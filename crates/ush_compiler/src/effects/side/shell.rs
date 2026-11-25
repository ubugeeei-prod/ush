//! Effects of pipelines and scoped blocks.

use crate::ast::{Expr, Pipeline, Scope, Statement};

use super::infer::{Context, block_effects, call_effects, expr_effects};
use super::{Effect, EffectSet};

/// Every stage runs. Only the last stage of a streaming pipeline writes
/// to the script's stdout; earlier stages write into the pipe, and a
/// captured pipeline's last stage writes into the captured value.
pub(super) fn pipeline_effects(pipeline: &Pipeline, ctx: Context<'_>, captured: bool) -> EffectSet {
    let last = pipeline.stages.len() - 1;
    let mut effects = EffectSet::empty();
    for (index, stage) in pipeline.stages.iter().enumerate() {
        let into_pipe = captured || index < last;
        effects.merge(&match &stage.expr {
            Expr::Call(call) => call_effects(call, ctx, into_pipe),
            other => expr_effects(other, ctx),
        });
    }
    effects
}

/// A scoped block changes the process environment, and `cd` also asks
/// the filesystem for the directory.
pub(super) fn scoped_effects(scope: &Scope, body: &[Statement], ctx: Context<'_>) -> EffectSet {
    let mut effects = EffectSet::of(Effect::Env);
    if matches!(scope, Scope::Cd(_)) {
        effects.insert(Effect::Fs);
    }
    for value in scope.exprs() {
        effects.merge(&expr_effects(value, ctx));
    }
    effects.union(&block_effects(body, ctx))
}
