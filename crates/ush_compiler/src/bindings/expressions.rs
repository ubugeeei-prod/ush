use super::{Scope, block};
use crate::ast::{Call, ComprehensionClause, Expr, ExprFields};
use anyhow::Result;

pub(super) fn call(c: &Call, scope: &Scope, globals: &Scope) -> Result<()> {
    for arg in &c.args {
        expr(&arg.expr, scope, globals)?;
    }
    Ok(())
}

pub(super) fn expr(e: &Expr, scope: &Scope, globals: &Scope) -> Result<()> {
    match e {
        Expr::AsyncBlock(body) => block(body, &mut scope.clone(), globals)?,
        Expr::Comprehension(c) => {
            let mut inner = scope.clone();
            for clause in &c.clauses {
                match clause {
                    ComprehensionClause::For { name, iterable } => {
                        expr(iterable, &inner, globals)?;
                        inner.insert(name.clone(), false);
                    }
                    ComprehensionClause::If(e) => expr(e, &inner, globals)?,
                }
            }
            expr(&c.value, &inner, globals)?;
        }
        Expr::Call(c) => call(c, scope, globals)?,
        Expr::MethodCall(c) => {
            expr(&c.receiver, scope, globals)?;
            for arg in &c.args {
                expr(&arg.expr, scope, globals)?;
            }
        }
        Expr::Add(es) | Expr::Tuple(es) | Expr::List(es) => {
            for e in es {
                expr(e, scope, globals)?;
            }
        }
        Expr::Compare { lhs, rhs, .. }
        | Expr::Range {
            start: lhs,
            end: rhs,
        } => {
            expr(lhs, scope, globals)?;
            expr(rhs, scope, globals)?;
        }
        Expr::Try(e) | Expr::Field { base: e, .. } => expr(e, scope, globals)?,
        Expr::Pipeline(p) => {
            for e in p.exprs() {
                expr(e, scope, globals)?;
            }
        }
        Expr::Variant(v) => match &v.fields {
            ExprFields::Tuple(es) => {
                for e in es {
                    expr(e, scope, globals)?;
                }
            }
            ExprFields::Struct(es) => {
                for e in es {
                    expr(&e.expr, scope, globals)?;
                }
            }
            ExprFields::Unit => {}
        },
        _ => {}
    }
    Ok(())
}
