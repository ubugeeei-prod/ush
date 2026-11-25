//! Effects of pipelines and scoped blocks.

use crate::ast::{Expr, Pipeline, Scope, Statement};

use super::infer::{Context, block_effects, call_effects, expr_effects};
use super::{Effect, EffectSet};

/// Every stage runs. Only the last process of a streaming pipeline writes
/// to the script's stdout; earlier stages write into the pipe, a file sink
/// or `to_stderr` takes the last process's output, and a captured
/// pipeline's last process writes into the captured value. Redirection
/// stages are not processes.
pub(super) fn pipeline_effects(pipeline: &Pipeline, ctx: Context<'_>, captured: bool) -> EffectSet {
    let redirect = |expr: &Expr| {
        matches!(expr, Expr::Call(call) if call.name.starts_with("std::io::")
            && crate::codegen::redirection_stage(&call.name))
    };
    let writer = pipeline.stages.iter().rposition(|s| !redirect(&s.expr));
    let sunk = pipeline.stages.last().is_some_and(|stage| {
        matches!(&stage.expr, Expr::Call(call)
            if matches!(call.name.as_str(), "std::io::to_file" | "std::io::append_to" | "std::io::to_stderr"))
    });
    let mut effects = EffectSet::empty();
    for (index, stage) in pipeline.stages.iter().enumerate() {
        let into_pipe = captured || sunk || Some(index) != writer;
        effects.merge(&match &stage.expr {
            Expr::Call(call) => call_effects(call, ctx, into_pipe),
            other => expr_effects(other, ctx),
        });
    }
    effects
}

/// A command run for its exit status streams as it does as a statement.
pub(super) fn status_effects(expr: &Expr, ctx: Context<'_>) -> EffectSet {
    match expr {
        Expr::Pipeline(pipeline) => pipeline_effects(pipeline, ctx, false),
        Expr::Call(call) => {
            let mut effects = call_effects(call, ctx, false);
            if call.name == "std::process::output" {
                effects.insert(Effect::Stdout);
            }
            effects
        }
        other => expr_effects(other, ctx),
    }
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
