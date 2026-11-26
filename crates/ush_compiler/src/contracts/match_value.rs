use super::{Checker, Locals, types::Ty};
use crate::{ast::MatchValue, types::HeapVec as Vec};
use anyhow::{Context, Result, anyhow};

impl Checker {
    pub(super) fn match_value(&mut self, value: &mut MatchValue, locals: &Locals) -> Result<Ty> {
        let subject = self.expr(&mut value.subject, locals)?;
        let mut found: Option<Ty> = None;
        for (pattern, arm) in &mut value.arms {
            let mut scope = locals.clone();
            self.pattern(pattern, &subject, &mut scope)?;
            let mut values = Vec::new();
            self.statement(arm, &mut scope, &mut values, true)
                .with_context(|| format!("line {}", arm.line))?;
            for ty in values {
                if let Some(expected) = &mut found {
                    join_constraints(expected, &ty);
                    expected
                        .check(&ty)
                        .with_context(|| format!("line {}: match arm type mismatch", arm.line))?;
                } else {
                    found = Some(ty);
                }
            }
        }
        found.ok_or_else(|| anyhow!("a match binding needs at least one value-producing arm"))
    }
}

fn join_constraints(left: &mut Ty, right: &Ty) {
    left.min = left.min.min(right.min);
    left.max = left.max.max(right.max);
    if left.length != right.length {
        left.length = None;
    }
    for (left, right) in left.args.iter_mut().zip(&right.args) {
        join_constraints(left, right);
    }
}
