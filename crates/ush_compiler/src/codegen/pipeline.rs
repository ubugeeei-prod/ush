//! Pipelines, streamed as statements or captured as values.
//!
//! POSIX `sh` has no `pipefail`, so every target uses one portable
//! lowering. The stages run inside a command substitution that captures
//! their exit statuses, not their output: each stage reports
//! ` INDEX:STATUS` on descriptor 7, the substitution's stdout, while the
//! last stage writes to descriptor 8, a copy of the enclosing stdout.
//! `__ush_pipe_check` then reports the leftmost failing stage.
//!
//! A stage holds its ends of the pipes until it has reported, so a
//! producer can only write into a closed pipe after its reader has
//! reported. A failure reported after the reader's is therefore the
//! reader stopping early, as `head` does: SIGPIPE (141), or a write error
//! where the caller ignores SIGPIPE, as CI runners do. It is not a
//! failure; the reader's own status decides.
//!
//! Every stage runs its program in a subshell of its own, so a builtin or
//! function killed by SIGPIPE cannot take the status report down with it.
//! The program does not inherit descriptor 7, so a daemon it starts cannot
//! hold the pipeline open.

mod lower;
mod render;
mod stage;

use anyhow::Result;

use super::{FunctionRegistry, control::ControlContext};
use crate::{
    ast::{Expr, Pipeline, Type},
    env::{Binding, CodegenState, EnumRegistry, Env, Storage},
    sourcemap::OutputBuffer,
    traits::TraitImplRegistry,
    types::OutputString as String,
};
use lower::lower;
use stage::Types;

/// The type of a captured pipeline, after checking every stage.
pub(crate) fn infer(
    pipeline: &Pipeline,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    let types = Types {
        env,
        functions,
        impls,
        enums,
    };
    stage::classify(pipeline, true, &types)?;
    Ok(Type::String)
}

/// Check every stage of a pipeline that runs as a statement.
pub(crate) fn check_statement(
    pipeline: &Pipeline,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<()> {
    let types = Types {
        env,
        functions,
        impls,
        enums,
    };
    stage::classify(pipeline, false, &types).map(drop)
}

pub(crate) fn compile_statement(
    pipeline: &Pipeline,
    env: &Env,
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<()> {
    let types = Types {
        env,
        functions: ctx.functions,
        impls: ctx.impls,
        enums: ctx.enums,
    };
    let block = lower(
        pipeline,
        false,
        false,
        &types,
        ctx.state,
        ctx.inside_function,
        out,
    )?;
    out.push_str(&block);
    out.push_str(&format!(" || {} \"$?\"\n", leave(ctx.inside_function)));
    Ok(())
}

/// Capture the last stage's stdout into a fresh variable, with the usual
/// removal of trailing newlines, and return that variable.
#[allow(clippy::too_many_arguments)]
pub(crate) fn hoist_capture(
    pipeline: &Pipeline,
    env: &mut Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<Expr> {
    let types = Types {
        env,
        functions,
        impls,
        enums,
    };
    let block = lower(pipeline, true, false, &types, state, inside_function, out)?;
    let temp = state.temp_var("value");
    out.push_str(&format!("{temp}=\"$(\n"));
    for line in block.lines() {
        out.push_str(&format!("  {line}\n"));
    }
    // The failing stage was reported inside the substitution.
    out.push_str(&format!(
        ")\" || {{ __ush_status=$?; __ush_line=''; {} \"$__ush_status\"; }}\n",
        leave(inside_function)
    ));
    let binding = Binding {
        ty: Type::String,
        storage: Storage::Primitive(temp.clone()),
    };
    env.insert(temp.clone(), binding);
    Ok(Expr::Var(temp))
}

fn leave(inside_function: bool) -> &'static str {
    if inside_function { "return" } else { "exit" }
}

/// The shell whose exit status is the pipeline's, with the same
/// left-to-right rule, but without reporting the failing stage: a
/// condition, `status_of` or `allow_fail` handles the status itself.
#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_status(
    pipeline: &Pipeline,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<String> {
    let types = Types {
        env,
        functions,
        impls,
        enums,
    };
    let block = lower(pipeline, false, true, &types, state, inside_function, out)?;
    Ok(if block.contains('\n') {
        format!("{{\n{block}\n}}")
    } else {
        block
    })
}
