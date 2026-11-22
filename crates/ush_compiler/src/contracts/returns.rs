use crate::ast::{Statement, StatementKind as S};

/// Whether every path either produces a value or leaves the function.
pub(crate) fn completes(body: &[Statement], tail: bool) -> bool {
    body.iter()
        .enumerate()
        .any(|(i, s)| ends(s, tail && i + 1 == body.len()))
}

fn ends(s: &Statement, tail: bool) -> bool {
    match &s.kind {
        S::Return(_) | S::Raise(_) | S::Fail { .. } => true,
        S::Expr(_) => tail,
        S::If {
            branch,
            returns_value,
        } => {
            let tail = tail && *returns_value;
            completes(&branch.then_body, tail)
                && branch
                    .else_body
                    .as_ref()
                    .is_some_and(|body| completes(body, tail))
        }
        S::Match {
            arms,
            returns_value,
            ..
        } => {
            !arms.is_empty()
                && arms
                    .iter()
                    .all(|(_, arm)| ends(arm, tail && *returns_value))
        }
        S::Loop { body } => completes(body, false),
        _ => false,
    }
}
