//! The items a file declares and which of them other files may use.

use anyhow::{Result, bail};

use crate::{
    ast::{Statement, StatementKind as S},
    types::{AstString as Name, HeapVec as Vec, Map},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ItemKind {
    Function,
    /// `type` records and `enum`s.
    Type,
    Trait,
    /// Both a call target (`do log`) and an effect name (`/ log`).
    Effect,
}

impl ItemKind {
    /// Called, referenced with `&`, or performed with `do`.
    pub(crate) fn is_value(self) -> bool {
        matches!(self, Self::Function | Self::Effect)
    }

    /// Named in types, patterns, rows and handlers.
    pub(crate) fn is_type(self) -> bool {
        !matches!(self, Self::Function)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Item {
    pub kind: ItemKind,
    /// The name every pass after linking sees: `gh::build`.
    pub internal: Name,
    pub public: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Exports {
    /// The module file, for diagnostics.
    pub display: alloc::string::String,
    /// Keyed by the name written inside the module (`build`, `helpers::run`),
    /// in declaration order so wildcard imports are deterministic.
    items: Vec<(Name, Item)>,
    index: Map<Name, usize>,
}

impl Exports {
    /// Collects a file's declarations. `namespace` prefixes a module's
    /// items; the root script's items keep their names.
    pub(crate) fn collect(program: &[Statement], namespace: Option<&str>, display: &str) -> Self {
        let mut exports = Self {
            display: display.into(),
            ..Self::default()
        };
        for statement in program {
            let (name, kind) = match &statement.kind {
                S::Function(def) => (&def.name, ItemKind::Function),
                S::Enum(def) => (&def.name, ItemKind::Type),
                S::Trait(def) => (&def.name, ItemKind::Trait),
                S::Effect(def) => (&def.name, ItemKind::Effect),
                _ => continue,
            };
            let internal = match namespace {
                Some(namespace) => format!("{namespace}::{name}").into(),
                None => name.clone(),
            };
            let item = Item {
                kind,
                internal,
                public: statement.public,
            };
            // Duplicate functions and types are reported by the passes
            // that own them; the first declaration names the item here.
            if !exports.index.contains_key(name) {
                exports.index.insert(name.clone(), exports.items.len());
                exports.items.push((name.clone(), item));
            }
        }
        exports
    }

    pub(crate) fn get(&self, name: &str) -> Option<&Item> {
        self.index.get(name).map(|&index| &self.items[index].1)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&Name, &Item)> {
        self.items.iter().map(|(name, item)| (name, item))
    }

    /// The item `alias::name` names from outside the module.
    pub(crate) fn lookup(&self, alias: &str, name: &str) -> Result<&Item> {
        let Some(item) = self.get(name) else {
            bail!("module `{alias}` ({}) has no item `{name}`", self.display);
        };
        if !item.public {
            bail!(
                "`{alias}::{name}` is private to {}; mark it `pub` to use it here",
                self.display
            );
        }
        Ok(item)
    }
}
