use super::Locals;
use crate::ast::{CompareOp, Expr};

/// Carry a proven comparison into the true branch, including later terms
/// of an AND. The enclosing scope keeps its original bounds.
pub(super) fn narrow(expr: &Expr, env: &mut Locals) {
    let Expr::Compare { lhs, op, rhs } = expr else {
        return;
    };
    let (Expr::Var(name), Expr::Int(value)) = (&**lhs, &**rhs) else {
        return;
    };
    let Some(ty) = env.get_mut(name) else {
        return;
    };
    match op {
        CompareOp::Gt => ty.min = ty.min.max(value.saturating_add(1)),
        CompareOp::Ge => ty.min = ty.min.max(*value),
        CompareOp::Lt => ty.max = ty.max.min(value.saturating_sub(1)),
        CompareOp::Le => ty.max = ty.max.min(*value),
        CompareOp::Eq => {
            ty.min = ty.min.max(*value);
            ty.max = ty.max.min(*value);
        }
        CompareOp::Ne => {}
    }
}
