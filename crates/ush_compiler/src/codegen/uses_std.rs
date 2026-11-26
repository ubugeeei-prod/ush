//! Only construct builtin signatures when the resolved program actually refers to them.
use crate::ast::{Condition, Expr, Statement, StatementKind as S};
pub(crate) fn references(body: &[Statement]) -> bool {
    body.iter().any(statement)
}
fn statement(s: &Statement) -> bool {
    match &s.kind {
        S::Function(f) => {
            references(&f.body) || f.params.iter().filter_map(|p| p.default.as_ref()).any(expr)
        }
        S::Impl(i) => i.methods.iter().any(|f| {
            references(&f.body) || f.params.iter().filter_map(|p| p.default.as_ref()).any(expr)
        }),
        S::Effect(e) => e.params.iter().filter_map(|p| p.default.as_ref()).any(expr),
        S::Handle { body, handlers } => {
            references(body) || handlers.iter().any(|h| references(&h.body))
        }
        S::Let { expr: e, .. }
        | S::Alias { value: e, .. }
        | S::Print(e)
        | S::Shell(e)
        | S::TryShell(e)
        | S::Raise(e)
        | S::Expr(e)
        | S::Return(e) => expr(e),
        S::Call(c) | S::TryCall(c) | S::Spawn { call: c, .. } => {
            needs_signatures(&c.name) || c.args.iter().any(|a| expr(&a.expr))
        }
        S::Fail { status, message } => expr(status) || expr(message),
        S::Match { expr: e, arms, .. } => expr(e) || arms.iter().any(|(_, s)| statement(s)),
        S::If { branch, .. } => {
            condition(&branch.condition)
                || references(&branch.then_body)
                || branch.else_body.as_ref().is_some_and(|b| references(b))
        }
        S::While { condition: c, body } => condition(c) || references(body),
        S::For { iterable, body, .. } => expr(iterable) || references(body),
        S::Loop { body } => references(body),
        S::Pipeline(pipeline) => pipeline.exprs().any(expr),
        S::Export(bindings) => bindings.iter().any(|b| expr(&b.value)),
        // A `cd` block changes directory through `std::process::chdir`.
        S::Scoped { scope, body } => {
            matches!(scope, crate::ast::Scope::Cd(_)) || scope.exprs().any(expr) || references(body)
        }
        S::AllowFail(e) => expr(e),
        S::Defer(body) => references(body),
        _ => false,
    }
}
fn condition(c: &Condition) -> bool {
    match c {
        Condition::Expr(e) | Condition::Let { expr: e, .. } => expr(e),
        Condition::And(cs) | Condition::Or(cs) => cs.iter().any(condition),
        Condition::Not(c) => condition(c),
    }
}
fn expr(e: &Expr) -> bool {
    match e {
        Expr::FunctionRef(name) => needs_signatures(name),
        Expr::Comprehension(c) => {
            expr(&c.value)
                || c.clauses.iter().any(|c| match c {
                    crate::ast::ComprehensionClause::For { iterable, .. } => expr(iterable),
                    crate::ast::ComprehensionClause::If(e) => expr(e),
                })
        }
        Expr::Call(c) => needs_signatures(&c.name) || c.args.iter().any(|a| expr(&a.expr)),
        Expr::MethodCall(call) => {
            // Task awaiting needs no std signatures, but its receiver can.
            call.method != "await"
                || expr(&call.receiver)
                || call.args.iter().any(|arg| expr(&arg.expr))
        }

        Expr::Compare { lhs, rhs, .. }
        | Expr::Range {
            start: lhs,
            end: rhs,
        }
        | Expr::Index {
            base: lhs,
            index: rhs,
        } => expr(lhs) || expr(rhs),
        Expr::Try(e) | Expr::Field { base: e, .. } | Expr::StatusOf(e) => expr(e),
        Expr::Add(es) | Expr::List(es) | Expr::Tuple(es) | Expr::Template(es) => {
            es.iter().any(expr)
        }
        Expr::AsyncBlock(body) => references(body),
        Expr::Match(value) => {
            expr(&value.subject) || value.arms.iter().any(|(_, arm)| statement(arm))
        }
        Expr::Pipeline(pipeline) => pipeline.exprs().any(expr),
        Expr::Variant(v) => match &v.fields {
            crate::ast::ExprFields::Tuple(es) => es.iter().any(expr),
            crate::ast::ExprFields::Struct(es) => es.iter().any(|e| expr(&e.expr)),
            _ => false,
        },
        _ => false,
    }
}

fn needs_signatures(name: &str) -> bool {
    name != "std::io::echo" && name.starts_with("std::")
}
