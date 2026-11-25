mod collect;
mod condition;
mod pattern;
mod qualified;
use crate::{
    ast::{Call, Expr, FunctionDef, Statement, StatementKind as S},
    types::{AstString as Name, Map, Set},
};
use alloc::vec::Vec;
use anyhow::Result;
use pattern::bind_pattern;

pub(crate) fn resolve_program(mut program: Vec<Statement>) -> Result<Vec<Statement>> {
    let locals = collect::local_functions(&program);
    if program.is_empty() {
        return Ok(program);
    }
    let mut functions = Map::default();
    if program.iter().any(|s| matches!(&s.kind, S::Use(items) if items.iter().any(|i| i.alias == "*" && i.path.starts_with("std::")))) {
        crate::codegen::register_builtins(&mut functions)?;
    }
    for s in &program {
        if let S::Function(f) = &s.kind
            && f.name.contains("::")
        {
            functions.insert(f.name.clone(), f.clone());
        }
    }
    let imports = collect::collect_imports(&program, &locals, &functions)?;
    let mut known: Set<Name> = functions.into_keys().collect();
    known.extend(locals);
    known.extend(imports.values().cloned());
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
    };
    resolver.block(&mut program, &mut Set::default());
    Ok(program)
}
struct Resolver {
    imports: Map<Name, Name>,
    known: Set<Name>,
    globals: Set<Name>,
}
impl Resolver {
    fn name(&self, name: &mut Name) {
        if !name.contains("::") {
            if let Some(path) = self.imports.get(name) {
                *name = path.clone();
            } else if self.imports.contains_key("*") && crate::commands::contains(name) {
                *name = format!("$bin::{name}").into();
            }
        }
    }
    fn block(&self, body: &mut [Statement], scope: &mut Set<Name>) {
        for statement in body {
            self.statement(statement, scope);
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
    fn statement(&self, statement: &mut Statement, scope: &mut Set<Name>) {
        match &mut statement.kind {
            S::Function(def) => self.function(def),
            S::Impl(item) => {
                for method in &mut item.methods {
                    self.function(method);
                }
            }
            S::Handle { body, handlers } => {
                self.block(body, &mut scope.clone());
                for handler in handlers {
                    let mut inner = scope.clone();
                    inner.extend(handler.params.iter().cloned());
                    self.block(&mut handler.body, &mut inner);
                }
            }
            S::Let { name, expr } => {
                self.expr(expr, scope);
                scope.insert(name.clone());
            }
            S::Spawn { name, call } => {
                self.call(call, scope);
                scope.insert(name.clone());
            }
            S::Await { name, .. } => {
                scope.insert(name.clone());
            }
            S::Fail { status, message } => {
                self.expr(status, scope);
                self.expr(message, scope);
            }
            S::Alias { value, .. }
            | S::Print(value)
            | S::Shell(value)
            | S::TryShell(value)
            | S::Raise(value)
            | S::Expr(value)
            | S::Return(value) => self.expr(value, scope),
            S::Call(call) | S::TryCall(call) => self.call(call, scope),
            S::Match { expr, arms, .. } => {
                self.expr(expr, scope);
                for (pattern, arm) in arms {
                    let mut inner = scope.clone();
                    bind_pattern(pattern, &mut inner);
                    self.statement(arm, &mut inner);
                }
            }
            S::If { branch, .. } => {
                let mut inner = scope.clone();
                self.condition(&mut branch.condition, &mut inner);
                self.block(&mut branch.then_body, &mut inner);
                if let Some(body) = &mut branch.else_body {
                    self.block(body, &mut scope.clone());
                }
            }
            S::While { condition, body } => {
                let mut inner = scope.clone();
                self.condition(condition, &mut inner);
                self.block(body, &mut inner);
            }
            S::For {
                name,
                iterable,
                body,
            } => {
                self.expr(iterable, scope);
                let mut inner = scope.clone();
                inner.insert(name.clone());
                self.block(body, &mut inner);
            }
            S::Loop { body } => self.block(body, &mut scope.clone()),
            _ => {}
        }
    }
    fn call(&self, call: &mut Call, scope: &Set<Name>) {
        if !scope.contains(&call.name) {
            self.name(&mut call.name);
        }
        for arg in &mut call.args {
            self.expr(&mut arg.expr, scope);
        }
    }
    fn expr(&self, expr: &mut Expr, scope: &Set<Name>) {
        self.qualified_call(expr);
        match expr {
            Expr::Var(name) if !scope.contains(name) => {
                let mut target = name.clone();
                self.name(&mut target);
                if self.known.contains(&target)
                    || target.starts_with("std::")
                    || crate::commands::lookup(&target).is_some()
                {
                    *expr = Expr::Call(Call {
                        name: target,
                        args: Vec::new(),
                        asynchronous: false,
                    });
                }
            }
            Expr::FunctionRef(name) => {
                if !scope.contains(name) {
                    self.name(name);
                }
            }
            Expr::Comprehension(c) => {
                let mut inner = scope.clone();
                for clause in &mut c.clauses {
                    match clause {
                        crate::ast::ComprehensionClause::For { name, iterable } => {
                            self.expr(iterable, &inner);
                            inner.insert(name.clone());
                        }
                        crate::ast::ComprehensionClause::If(e) => self.expr(e, &inner),
                    }
                }
                self.expr(&mut c.value, &inner);
            }
            Expr::Call(call) => self.call(call, scope),
            Expr::MethodCall(call) => {
                self.expr(&mut call.receiver, scope);
                for arg in &mut call.args {
                    self.expr(&mut arg.expr, scope);
                }
            }
            Expr::Add(es) | Expr::Tuple(es) | Expr::List(es) => {
                for e in es {
                    self.expr(e, scope);
                }
            }
            Expr::Compare { lhs, rhs, .. }
            | Expr::Range {
                start: lhs,
                end: rhs,
            } => {
                self.expr(lhs, scope);
                self.expr(rhs, scope);
            }
            Expr::Try(e) | Expr::Field { base: e, .. } => self.expr(e, scope),
            Expr::AsyncBlock(body) => self.block(body, &mut scope.clone()),
            Expr::Variant(v) => match &mut v.fields {
                crate::ast::ExprFields::Tuple(es) => {
                    for e in es {
                        self.expr(e, scope);
                    }
                }
                crate::ast::ExprFields::Struct(es) => {
                    for e in es {
                        self.expr(&mut e.expr, scope);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
