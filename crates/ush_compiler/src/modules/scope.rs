//! What one file can name: its own items, its `mod` aliases, and the
//! items it imported from them with `use`.

use anyhow::{Result, bail};

use alloc::rc::Rc;

use super::exports::{Exports, Item};
use crate::{
    ast::{Statement, StatementKind as S},
    types::{AstString as Name, HeapVec as Vec, Map},
};

#[derive(Default)]
pub(crate) struct Scope<'a> {
    /// The internal namespace of a module's own items; `None` for the
    /// root script, whose items keep their names.
    pub namespace: Option<&'a str>,
    /// `mod` aliases.
    pub modules: Map<Name, Rc<Exports>>,
    /// `use gh::{build}` and `use gh::{build as b}`.
    pub explicit: Map<Name, Item>,
    /// `use gh::*`, in import order.
    pub wildcards: Vec<(Name, Item)>,
}

impl<'a> Scope<'a> {
    /// The item behind `alias::rest`, when `alias` names a module.
    pub(crate) fn qualify(&self, path: &str) -> Option<Result<&Item>> {
        let (alias, rest) = path.split_once("::")?;
        let exports = self.modules.get(alias)?;
        Some(exports.lookup(alias, rest))
    }

    /// Moves `use` items that name module aliases into the scope, leaving
    /// std and `$bin` imports for the import resolver.
    pub(crate) fn take_imports(&mut self, program: &mut [Statement]) -> Result<()> {
        for statement in program {
            let S::Use(items) = &mut statement.kind else {
                continue;
            };
            let mut kept = Vec::new();
            for item in items.drain(..) {
                let alias = item.path.split("::").next().unwrap_or(&item.path);
                let Some(exports) = self.modules.get(alias).cloned() else {
                    kept.push(item);
                    continue;
                };
                let line = statement.line;
                if item.alias == "*" {
                    for (name, export) in exports.iter() {
                        if export.public && !name.contains("::") {
                            self.wildcards.push((name.clone(), export.clone()));
                        }
                    }
                    continue;
                }
                let Some((_, rest)) = item.path.split_once("::") else {
                    bail!("line {line}: import items from a module with `use {alias}::{{item}}`");
                };
                let export = exports
                    .lookup(alias, rest)
                    .map_err(|error| error.context(format!("line {line}")))?;
                if self
                    .explicit
                    .insert(item.alias.clone(), export.clone())
                    .is_some()
                {
                    bail!("line {line}: duplicate import alias: {}", item.alias);
                }
            }
            *items = kept.into_iter().collect();
        }
        Ok(())
    }

    /// The internal name of a type, trait or effect written bare in this
    /// file. The file's own declarations win over imports, explicit
    /// imports over wildcards.
    pub(crate) fn type_name(&self, own: &Map<Name, Name>, name: &str) -> Option<Name> {
        if let Some(internal) = own.get(name) {
            return Some(internal.clone());
        }
        if let Some(item) = self.explicit.get(name) {
            return item.kind.is_type().then(|| item.internal.clone());
        }
        self.wildcards
            .iter()
            .find(|(alias, item)| alias == name && item.kind.is_type())
            .map(|(_, item)| item.internal.clone())
    }

    /// Two wildcards offering different types under one name conflict,
    /// as two wildcards offering different functions do, unless the file
    /// declares or explicitly imports that name.
    pub(crate) fn ensure_unambiguous_types(&self, own: &Map<Name, Name>) -> Result<()> {
        let mut seen: Map<&Name, &Name> = Map::default();
        for (name, item) in &self.wildcards {
            if !item.kind.is_type() || own.contains_key(name) || self.explicit.contains_key(name) {
                continue;
            }
            if seen
                .insert(name, &item.internal)
                .is_some_and(|previous| *previous != item.internal)
            {
                bail!("ambiguous wildcard import: {name}");
            }
        }
        Ok(())
    }
}
