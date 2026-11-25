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

mod render;
mod stage;

use anyhow::Result;

use super::{
    FunctionRegistry, compile_runtime_primitive_expr, control::ControlContext,
    io::push_failure_guard, rendered_call_runtime,
};
use crate::{
    ast::{Expr, Pipeline, Type},
    env::{Binding, CodegenState, EnumRegistry, Env, Storage},
    sourcemap::OutputBuffer,
    traits::TraitImplRegistry,
    types::{HeapVec as Vec, OutputString as String},
    util::shell_value_quote,
};
use render::{Process, process, render};
use stage::{StageKind, Types};

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
    let block = lower(pipeline, false, &types, ctx.state, ctx.inside_function, out)?;
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
    let block = lower(pipeline, true, &types, state, inside_function, out)?;
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

/// Evaluate every stage's arguments, left to right, then return the
/// shell that runs the stages and checks their statuses.
fn lower(
    pipeline: &Pipeline,
    captured: bool,
    types: &Types<'_>,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<String> {
    let kinds = stage::classify(pipeline, captured, types)?;
    let mut processes: Vec<Process> = Vec::new();
    let mut source = None;
    for (index, (stage, kind)) in pipeline.stages.iter().zip(&kinds).enumerate() {
        let command = match kind {
            StageKind::Source(path) => {
                source = Some((path_word(path, types, state, inside_function, out)?, index));
                continue;
            }
            StageKind::Sink { path, append } => {
                let path = path_word(path, types, state, inside_function, out)?;
                if processes.is_empty() {
                    // `from_file a | to_file b` copies a file.
                    processes.push(process("(command cat)".into(), source.take(), false, 0));
                }
                if let Some(last) = processes.last_mut() {
                    last.output = Some((path, *append));
                    last.last = index;
                }
                continue;
            }
            StageKind::Command(call) => format!(
                "({})",
                rendered_call_runtime(
                    call,
                    types.env,
                    types.functions,
                    types.impls,
                    types.enums,
                    state,
                    inside_function,
                    out,
                )?
            ),
            StageKind::Function(call) => format!(
                "( __ush_capture_return='0' __ush_return_path=''; {} )",
                rendered_call_runtime(
                    call,
                    types.env,
                    types.functions,
                    types.impls,
                    types.enums,
                    state,
                    inside_function,
                    out,
                )?
            ),
            StageKind::Text(expr) => format!(
                "(printf '%s' {})",
                compile_runtime_primitive_expr(
                    expr,
                    types.env,
                    types.functions,
                    types.impls,
                    types.enums,
                    state,
                    inside_function,
                    out,
                )?
            ),
        };
        let merge = stage.merge_stderr;
        processes.push(process(command, source.take(), merge, index));
    }
    Ok(render(pipeline, &processes))
}

/// A literal path is used as written. A computed path goes through the
/// same resolution as `std::fs`, which also rejects an empty path.
fn path_word(
    path: &Expr,
    types: &Types<'_>,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<String> {
    if let Expr::String(path) = path {
        anyhow::ensure!(!path.is_empty(), "a file path must not be empty");
        return Ok(shell_value_quote(path));
    }
    let value = compile_runtime_primitive_expr(
        path,
        types.env,
        types.functions,
        types.impls,
        types.enums,
        state,
        inside_function,
        out,
    )?;
    let temp = state.temp_var("path");
    out.push_str(&format!("__ush_std_fs_path {value}"));
    push_failure_guard(out, inside_function);
    out.push_str(&format!("{temp}=$__ush_path\n"));
    Ok(format!("\"${temp}\""))
}
