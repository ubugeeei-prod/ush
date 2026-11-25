use alloc::boxed::Box;

use super::{Expr, Pattern, Statement};
use crate::types::HeapVec as Vec;

#[derive(Debug, Clone)]
pub(crate) enum Condition {
    Expr(Expr),
    Let {
        pattern: Pattern,
        expr: Expr,
    },
    And(Vec<Condition>),
    Or(Vec<Condition>),
    /// `! condition`: true when the inner condition is false, including a
    /// command that exits unsuccessfully.
    Not(Box<Condition>),
}

#[derive(Debug, Clone)]
pub(crate) struct IfBranch {
    pub condition: Condition,
    pub then_body: Vec<Statement>,
    pub else_body: Option<Vec<Statement>>,
}
