mod collect;
mod condition;
mod pattern;
mod qualified;
mod walk;
use crate::{
    ast::{Call, Expr, FunctionDef, Statement, StatementKind as S},
    modules::Scope,
    types::{AstString as Name, Map, Set},
};
use alloc::vec::Vec;
use anyhow::{Error, Result};
use core::cell::{Cell, RefCell};
use pattern::bind_pattern;

/// Rewrites every call and function reference in one file to the name
/// the rest of the compiler uses: std imports to `std::…` paths, module
/// imports and `gh::item` paths to internal names, and a module's own
/// functions into its namespace.
pub(crate) fn resolve_file(
    mut program: Vec<Statement>,
    scope: &Scope<'_>,
) -> Result<Vec<Statement>> {
    let locals = collect::local_functions(&program);
    if program.is_empty() {
        return Ok(program);
    }
    let mut functions = Set::default();
    if program.iter().any(|s| matches!(&s.kind, S::Use(items) if items.iter().any(|i| i.alias == "*" && i.path.starts_with("std::")))) {
        let mut builtins = Map::default();
        crate::codegen::register_builtins(&mut builtins)?;
        functions.extend(builtins.into_keys());
    }
    for s in &program {
        if let S::Function(f) = &s.kind
            && f.name.contains("::")
        {
            functions.insert(f.name.clone());
        }
    }
    let imports = collect::collect_imports(&program, &locals, &functions, scope)?;
    let mut known: Set<Name> = functions;
    known.extend(locals);
    known.extend(imports.values().cloned());
    for exports in scope.modules.values() {
        known.extend(
            exports
                .iter()
                .filter(|(_, item)| item.kind.is_value())
                .map(|(_, item)| item.internal.clone()),
        );
    }
    let globals = program
        .iter()
        .filter_map(|s| match &s.kind {
            S::Let { name, .. } | S::Spawn { name, .. } | S::Await { name, .. } => {
                Some(name.clone())
            }
            _ => None,
        })
        .collect();
    let resolver = Resolver {
        imports,
        known,
        globals,
        scope,
        line: Cell::new(0),
        error: RefCell::new(None),
    };
    resolver.block(&mut program, &mut Set::default());
    match resolver.error.into_inner() {
        Some(error) => Err(error),
        None => Ok(program),
    }
}
struct Resolver<'s> {
    imports: Map<Name, Name>,
    known: Set<Name>,
    globals: Set<Name>,
    scope: &'s Scope<'s>,
    /// The statement being resolved, for diagnostics.
    line: Cell<usize>,
    /// The first error; resolution keeps walking so the walk stays simple.
    error: RefCell<Option<Error>>,
}
impl Resolver<'_> {
    fn name(&self, name: &mut Name) {
        if let Some(path) = self.imports.get(name) {
            *name = path.clone();
        } else if !name.contains("::") {
            if self.imports.contains_key("*") && crate::commands::contains(name) {
                *name = format!("$bin::{name}").into();
            }
        } else if let Some(item) = self.scope.qualify(name) {
            match item {
                Ok(item) if item.kind.is_value() => *name = item.internal.clone(),
                Ok(_) => self.fail(anyhow::anyhow!("`{name}` is a type, not a function")),
                Err(error) => self.fail(error),
            }
        }
    }
    fn fail(&self, error: Error) {
        let mut slot = self.error.borrow_mut();
        if slot.is_none() {
            *slot = Some(error.context(format!("line {}", self.line.get())));
        }
    }
    fn block(&self, body: &mut [Statement], scope: &mut Set<Name>) {
        for statement in body {
            let outer = self.line.replace(statement.line);
            self.statement(statement, scope);
            self.line.set(outer);
        }
    }
    fn function(&self, def: &mut FunctionDef) {
        let mut scope = self.globals.clone();
        if def.receiver.is_some() {
            scope.insert("self".into());
        }
        for param in &mut def.params {
            if let Some(default) = &mut param.default {
                self.expr(default, &scope);
            }
            scope.insert(param.name.clone());
        }
        self.block(&mut def.body, &mut scope);
    }
}
