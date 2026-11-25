//! Puts a file's types, traits and effects into the namespace the rest
//! of the compiler sees, and renames a module's own declarations.
//!
//! Calls are resolved by the import resolver, which knows about local
//! variables; type names have no shadowing, so this walk is plain.

use anyhow::{Error, Result, bail};

use super::{
    scope::Scope,
    visit::{VisitMut, walk_expr, walk_function, walk_pattern, walk_statement, walk_ty},
};
use crate::{
    ast::{Expr, FunctionDef, Pattern, Statement, StatementKind as S, Type},
    types::{AstString as Name, HeapVec as Vec, Map},
};

pub(super) fn rename(program: &mut [Statement], scope: &Scope<'_>) -> Result<()> {
    // A file's own types, traits and effects: renamed into a module's
    // namespace, and in the root script kept as they are, ahead of imports.
    let mut own = Map::default();
    for statement in program.iter() {
        let name = match &statement.kind {
            S::Enum(def) => &def.name,
            S::Trait(def) => &def.name,
            S::Effect(def) => &def.name,
            _ => continue,
        };
        if scope
            .explicit
            .get(name)
            .is_some_and(|item| item.kind.is_type())
        {
            bail!(
                "line {}: `{name}` is both declared here and imported with `use`",
                statement.line
            );
        }
        let internal = match scope.namespace {
            Some(namespace) => format!("{namespace}::{name}").into(),
            None => name.clone(),
        };
        own.insert(name.clone(), internal);
    }
    scope.ensure_unambiguous_types(&own)?;
    let mut renamer = Renamer {
        scope,
        own,
        bound: Vec::new(),
        line: 0,
        error: None,
    };
    for statement in program.iter_mut() {
        renamer.statement(statement);
        if let (Some(namespace), S::Function(def)) = (scope.namespace, &mut statement.kind) {
            def.name = format!("{namespace}::{}", def.name).into();
        }
    }
    renamer.error.map_or(Ok(()), Err)
}

struct Renamer<'s> {
    scope: &'s Scope<'s>,
    /// The file's own types, traits and effects.
    own: Map<Name, Name>,
    /// Type variables of the enclosing generic functions.
    bound: Vec<Name>,
    line: usize,
    error: Option<Error>,
}

impl Renamer<'_> {
    fn name(&mut self, name: &mut Name) {
        if self.bound.contains(name) {
            return;
        }
        if let Some(internal) = self.scope.type_name(&self.own, name) {
            *name = internal;
        }
    }

    /// `Status::Ok` or, for a record, `Config { … }` and `gh::Config { … }`.
    fn variant(&mut self, enum_name: &mut Name, variant_name: &str) {
        let path = format!("{enum_name}::{variant_name}");
        match self.scope.qualify(&path) {
            Some(Ok(item)) if item.kind.is_type() => *enum_name = item.internal.clone(),
            Some(Ok(_)) => {}
            Some(Err(error)) => self.fail(error),
            None => self.name(enum_name),
        }
    }

    fn fail(&mut self, error: Error) {
        if self.error.is_none() {
            self.error = Some(error.context(format!("line {}", self.line)));
        }
    }
}

impl VisitMut for Renamer<'_> {
    fn statement(&mut self, statement: &mut Statement) {
        let outer = core::mem::replace(&mut self.line, statement.line);
        match &mut statement.kind {
            S::Enum(def) => self.name(&mut def.name),
            S::Trait(def) => self.name(&mut def.name),
            S::Effect(def) => self.name(&mut def.name),
            S::Impl(item) => {
                if let Some(name) = &mut item.trait_name {
                    self.name(name);
                }
            }
            S::Handle { handlers, .. } => {
                for handler in handlers {
                    self.name(&mut handler.effect);
                }
            }
            _ => {}
        }
        walk_statement(self, statement);
        self.line = outer;
    }

    fn function(&mut self, def: &mut FunctionDef) {
        let depth = self.bound.len();
        if let Some(scheme) = &mut def.scheme {
            self.bound.extend(scheme.variables.keys().cloned());
            let (scope, own, bound) = (self.scope, &self.own, &self.bound);
            scheme.rename_types(&mut |name| {
                if !bound.contains(name)
                    && let Some(internal) = scope.type_name(own, name)
                {
                    *name = internal;
                }
            });
        }
        // Rows are split into errors and effects after linking, so the
        // written names are renamed here, before either is known.
        if let Some(effects) = &mut def.declared_effects {
            effects.iter_mut().for_each(|name| self.name(name));
        }
        walk_function(self, def);
        self.bound.truncate(depth);
    }

    fn expr(&mut self, expr: &mut Expr) {
        if let Expr::Variant(variant) = expr {
            let variant_name = variant.variant_name.clone();
            self.variant(&mut variant.enum_name, &variant_name);
        }
        walk_expr(self, expr);
    }

    fn pattern(&mut self, pattern: &mut Pattern) {
        if let Pattern::Variant(variant) = pattern {
            let variant_name = variant.variant_name.clone();
            self.variant(&mut variant.enum_name, &variant_name);
        }
        walk_pattern(self, pattern);
    }

    fn ty(&mut self, ty: &mut Type) {
        if let Type::Adt(name) = ty {
            self.name(name);
        }
        walk_ty(self, ty);
    }
}
