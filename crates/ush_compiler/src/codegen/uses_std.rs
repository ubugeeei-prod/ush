//! Only construct builtin signatures when the resolved program actually refers to them.
use crate::ast::{Condition, Expr, Statement, StatementKind as S};
pub(super) fn references(body: &[Statement]) -> bool {
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
            c.name.starts_with("std::") || c.args.iter().any(|a| expr(&a.expr))
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
        _ => false,
    }
}
fn condition(c: &Condition) -> bool {
    match c {
        Condition::Expr(e) | Condition::Let { expr: e, .. } => expr(e),
        Condition::And(cs) | Condition::Or(cs) => cs.iter().any(condition),
    }
}
fn expr(e: &Expr) -> bool {
    match e {
        Expr::FunctionRef(name) => name.starts_with("std::"),
        Expr::Comprehension(c) => {
            expr(&c.value)
                || c.clauses.iter().any(|c| match c {
                    crate::ast::ComprehensionClause::For { iterable, .. } => expr(iterable),
                    crate::ast::ComprehensionClause::If(e) => expr(e),
                })
        }
        Expr::Call(c) => c.name.starts_with("std::") || c.args.iter().any(|a| expr(&a.expr)),
        Expr::MethodCall(_) => true, // Builtin methods lower to std functions after type inference.

        Expr::Compare { lhs, rhs, .. }
        | Expr::Range {
            start: lhs,
            end: rhs,
        } => expr(lhs) || expr(rhs),
        Expr::Try(e) | Expr::Field { base: e, .. } => expr(e),
        Expr::Add(es) | Expr::List(es) | Expr::Tuple(es) => es.iter().any(expr),
        Expr::AsyncBlock(body) => references(body),
        Expr::Variant(v) => match &v.fields {
            crate::ast::ExprFields::Tuple(es) => es.iter().any(expr),
            crate::ast::ExprFields::Struct(es) => es.iter().any(|e| expr(&e.expr)),
            _ => false,
        },
        _ => false,
    }
}
