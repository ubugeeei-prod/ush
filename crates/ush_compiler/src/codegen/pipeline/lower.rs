//! Plan a pipeline's processes and evaluate every stage's arguments.

use anyhow::{Result, bail};

use super::{
    render::{Output, Process, process, render},
    stage::{self, StageKind, Types},
};
use crate::{
    ast::{Expr, Pipeline},
    codegen::{compile_runtime_primitive_expr, io::push_failure_guard, rendered_call_runtime},
    env::CodegenState,
    sourcemap::OutputBuffer,
    types::{HeapVec as Vec, OutputString as String},
    util::shell_value_quote,
};

/// Evaluate every stage's arguments, left to right, then return the
/// shell that runs the stages and checks their statuses.
pub(super) fn lower(
    pipeline: &Pipeline,
    captured: bool,
    quiet: bool,
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
                    last.output = Some(Output::File {
                        path,
                        append: *append,
                    });
                    last.last = index;
                }
                continue;
            }
            StageKind::ToStderr => {
                if processes.is_empty() {
                    processes.push(process("(command cat)".into(), source.take(), false, 0));
                }
                if let Some(last) = processes.last_mut() {
                    last.output = Some(Output::Stderr);
                    last.last = index;
                }
                continue;
            }
            StageKind::Stderr { path, append } => {
                let path = match path {
                    Some(path) => Some(path_word(path, types, state, inside_function, out)?),
                    None => None,
                };
                let Some(last) = processes.last_mut() else {
                    bail!(
                        "pipeline stage {}: stderr redirection needs a command before it",
                        index + 1
                    );
                };
                if last.errors.is_some() || last.merge_stderr {
                    bail!(
                        "pipeline stage {}: the stage before it already redirects its stderr",
                        index + 1
                    );
                }
                last.errors = Some((path, *append));
                last.last = index;
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
    Ok(render(pipeline, &processes, quiet))
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
