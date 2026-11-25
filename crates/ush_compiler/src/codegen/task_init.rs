//! Find the first executable statement that can reach task-backed storage.
use super::FunctionRegistry;
use crate::{
    ast::{Condition, Expr, Statement, StatementKind as S},
    types::Set,
};

pub(super) fn needs(statement: &Statement, functions: &FunctionRegistry) -> bool {
    Walker {
        functions,
        seen: Set::default(),
        declarations: false,
    }
    .statement(statement)
}

pub(super) fn present(program: &[Statement], functions: &FunctionRegistry) -> bool {
    Walker {
        functions,
        seen: Set::default(),
        declarations: true,
    }
    .body(program)
}

struct Walker<'a> {
    functions: &'a FunctionRegistry,
    seen: Set<crate::types::AstString>,
    declarations: bool,
}

impl Walker<'_> {
    fn call(&mut self, name: &str) -> bool {
        if name == "std::fs::temp_dir" {
            return true;
        }
        if !self.seen.insert(name.into()) {
            return false;
        }
        self.functions.get(name).is_some_and(|f| {
            self.body(&f.body)
                || f.params
                    .iter()
                    .filter_map(|p| p.default.as_ref())
                    .any(|e| self.expr(e))
        })
    }
    fn body(&mut self, body: &[Statement]) -> bool {
        body.iter().any(|s| self.statement(s))
    }
    fn statement(&mut self, s: &Statement) -> bool {
        match &s.kind {
            S::Function(f) if self.declarations => {
                self.body(&f.body)
                    || f.params
                        .iter()
                        .filter_map(|p| p.default.as_ref())
                        .any(|e| self.expr(e))
            }
            S::Impl(i) if self.declarations => i.methods.iter().any(|f| self.body(&f.body)),
            S::Spawn { .. } | S::Await { .. } => true,
            S::Handle { body, handlers } => {
                self.body(body) || handlers.iter().any(|h| self.body(&h.body))
            }
            S::Let { expr: e, .. }
            | S::Alias { value: e, .. }
            | S::Print(e)
            | S::Shell(e)
            | S::TryShell(e)
            | S::Raise(e)
            | S::Expr(e)
            | S::Return(e) => self.expr(e),
            S::Call(c) | S::TryCall(c) => {
                c.asynchronous || self.call(&c.name) || c.args.iter().any(|a| self.expr(&a.expr))
            }
            S::Fail { status, message } => self.expr(status) || self.expr(message),
            S::Match { expr, arms, .. } => {
                self.expr(expr) || arms.iter().any(|(_, s)| self.statement(s))
            }
            S::If { branch, .. } => {
                self.condition(&branch.condition)
                    || self.body(&branch.then_body)
                    || branch.else_body.as_ref().is_some_and(|b| self.body(b))
            }
            S::While { condition, body } => self.condition(condition) || self.body(body),
            S::For { iterable, body, .. } => self.expr(iterable) || self.body(body),
            S::Loop { body } => self.body(body),
            S::Pipeline(pipeline) => pipeline.exprs().any(|e| self.expr(e)),
            S::Export(bindings) => bindings.iter().any(|b| self.expr(&b.value)),
            S::Scoped { scope, body } => scope.exprs().any(|e| self.expr(e)) || self.body(body),
            _ => false,
        }
    }
    fn condition(&mut self, c: &Condition) -> bool {
        match c {
            Condition::Expr(e) | Condition::Let { expr: e, .. } => self.expr(e),
            Condition::And(cs) | Condition::Or(cs) => cs.iter().any(|c| self.condition(c)),
        }
    }
    fn expr(&mut self, e: &Expr) -> bool {
        match e {
            Expr::AsyncBlock(_) => true,
            Expr::FunctionRef(name) => self.call(name),
            Expr::Comprehension(c) => {
                self.expr(&c.value)
                    || c.clauses.iter().any(|c| match c {
                        crate::ast::ComprehensionClause::For { iterable, .. } => {
                            self.expr(iterable)
                        }
                        crate::ast::ComprehensionClause::If(e) => self.expr(e),
                    })
            }
            Expr::Call(c) => {
                c.asynchronous || self.call(&c.name) || c.args.iter().any(|a| self.expr(&a.expr))
            }
            // User methods may allocate storage; conservatively initialize before dispatch.
            Expr::MethodCall(c) => {
                !self.declarations
                    || c.method == "await"
                    || self.expr(&c.receiver)
                    || c.args.iter().any(|a| self.expr(&a.expr))
            }
            Expr::Compare { lhs, rhs, .. }
            | Expr::Range {
                start: lhs,
                end: rhs,
            } => self.expr(lhs) || self.expr(rhs),
            Expr::Try(e) | Expr::Field { base: e, .. } => self.expr(e),
            Expr::Add(es) | Expr::List(es) | Expr::Tuple(es) => es.iter().any(|e| self.expr(e)),
            Expr::Pipeline(pipeline) => pipeline.exprs().any(|e| self.expr(e)),
            Expr::Variant(v) => match &v.fields {
                crate::ast::ExprFields::Tuple(es) => es.iter().any(|e| self.expr(e)),
                crate::ast::ExprFields::Struct(es) => es.iter().any(|e| self.expr(&e.expr)),
                _ => false,
            },
            _ => false,
        }
    }
}
