//! The resolver's walk over statements and expressions.

use super::{Call, Expr, Name, Resolver, Set, Vec, bind_pattern};
use crate::ast::{Statement, StatementKind as S};

impl Resolver<'_> {
    pub(super) fn statement(&self, statement: &mut Statement, scope: &mut Set<Name>) {
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
            S::Pipeline(pipeline) => pipeline.exprs_mut().for_each(|e| self.expr(e, scope)),
            S::Export(bindings) => bindings
                .iter_mut()
                .for_each(|b| self.expr(&mut b.value, scope)),
            S::Scoped { scope: kind, body } => {
                kind.exprs_mut().for_each(|e| self.expr(e, scope));
                self.block(body, &mut scope.clone());
            }
            S::AllowFail(command) => self.expr(command, scope),
            S::Defer(body) => self.block(body, &mut scope.clone()),
            _ => {}
        }
    }
    pub(super) fn call(&self, call: &mut Call, scope: &Set<Name>) {
        if !scope.contains(&call.name) {
            self.name(&mut call.name);
        }
        for arg in &mut call.args {
            self.expr(&mut arg.expr, scope);
        }
    }
    pub(super) fn expr(&self, expr: &mut Expr, scope: &Set<Name>) {
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
            Expr::Try(e) | Expr::Field { base: e, .. } | Expr::StatusOf(e) => self.expr(e, scope),
            Expr::AsyncBlock(body) => self.block(body, &mut scope.clone()),
            Expr::Pipeline(pipeline) => pipeline.exprs_mut().for_each(|e| self.expr(e, scope)),
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
