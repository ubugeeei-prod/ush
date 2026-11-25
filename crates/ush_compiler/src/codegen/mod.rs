mod alias;
mod bin;
mod binding;
mod blocks;
mod builtin_methods;
mod call_capture;
mod call_support;
pub(crate) mod calls;
mod compare;
pub(crate) mod comprehension;
mod control;
mod control_support;
mod defer;
mod docs;
mod echo;
mod effects_lowering;
mod enum_registry;
mod functions;
mod io;
pub(crate) mod list;
mod method_fields;
mod methods;
mod methods_support;
mod pipeline;
mod primitive;
pub(crate) mod process;
mod registries;
mod render;
mod runtime_expr;
mod runtime_member;
mod runtime_support;
mod scoped;
mod shared;
mod source_context;
mod statement;
mod statement_control;
mod stdlib;
mod task_block;
mod task_init;
mod task_methods;
mod tasks;
mod text;
mod uses_std;
pub(crate) mod wire;

use anyhow::Result;

use super::{
    ast::{Statement, StatementKind},
    effects,
    env::{CodegenState, Env},
};
use crate::ScriptDocs;
use crate::sourcemap::{OutputBuffer, SourceMapSection};
pub(crate) use source_context::SourceContext;

pub(crate) use effects_lowering::effect_dispatcher;
pub(crate) use functions::FunctionRegistry;
pub(crate) use list::list_method_fails;
pub(crate) use primitive::{compile_primitive_expr, infer};
use registries::Registries;
pub(crate) use registries::error_rows;
pub(crate) use runtime_expr::{compile_runtime_primitive_expr, rendered_call_runtime};
pub(crate) use shared::{binding_for_name, shell_function_name};
pub(crate) use stdlib::host_required;
pub(crate) use stdlib::pipe::is_redirection as redirection_stage;
pub(crate) use stdlib::register_builtins;
pub(crate) use task_block::infer_async_block_type;
pub(crate) use tasks::bind_await;
pub(crate) use uses_std::references as references_std;

pub(crate) fn compile_program(
    program: &[Statement],
    docs: &ScriptDocs,
    space: &crate::modules::SourceSpace,
    script_name: Option<&str>,
    source_context: SourceContext<'_>,
    target: crate::ShellTarget,
    mapped: bool,
) -> Result<OutputBuffer> {
    let mut env = Env::default();
    let Registries {
        functions,
        enums,
        trait_impls,
    } = Registries::build(program)?;
    let mut state = CodegenState::new(space, mapped);
    let mut out = OutputBuffer::for_program(SourceMapSection::RuntimeSupport, mapped);

    let bin_entry = script_name
        .filter(|name| *name == "bin.ush")
        .and_then(|_| functions.get("bin"));
    let extra_completion = bin_entry
        .map(bin::completion_candidates)
        .unwrap_or_default();
    let globals = functions::analyze_globals(program, &functions, &trait_impls, &enums)?;
    let function_errors =
        effects::analyze_function_errors(program, &globals, &functions, &trait_impls, &enums)?;
    // Side effects are checked alongside error effects: a declared
    // effect row that no longer covers the body is a compile error,
    // the same way an undeclared raise is.
    effects::analyze_function_effects(program)?;
    let previous_section = out.set_section(SourceMapSection::UserCode);
    let needs_tasks = task_init::present(program, &functions);
    let mut initialized_tasks = false;
    for statement in program {
        if matches!(statement.kind, StatementKind::Enum(_)) {
            continue;
        }
        if needs_tasks && !initialized_tasks && task_init::needs(statement, &functions) {
            io::track_statement(statement, &state, &mut out);
            out.push_str("__ush_task_init\n");
            initialized_tasks = true;
        }
        statement::compile_statement(
            statement,
            &mut env,
            &globals,
            &functions,
            &trait_impls,
            &enums,
            &function_errors,
            &mut state,
            None,
            false,
            false,
            &mut out,
        )?;
    }
    if let Some(def) = bin_entry {
        if needs_tasks && !initialized_tasks {
            out.push_str("__ush_task_init\n");
        }
        bin::push_bin_entry(&mut out, def, &globals, &functions, &trait_impls, &enums)?;
    }
    out.set_section(previous_section);
    let body = out;
    let mut out = OutputBuffer::for_program(SourceMapSection::RuntimeSupport, mapped);
    // Deferred blocks run from the exit trap the task runtime installs.
    let uses_tasks = body.as_str().contains("__ush_jobs=")
        || body.as_str().contains("__ush_task_init")
        || body.as_str().contains("__ush_defers=");
    source_context::emit(
        &mut out,
        source_context,
        target,
        source_context::Prelude {
            tasks: uses_tasks,
            literal_newlines: body.as_str().contains("${__ush_lf}"),
            modules: space.has_modules(),
        },
    );
    let previous_section = out.set_section(SourceMapSection::DocSupport);
    // Programs that read argv own their CLI, including --help and --man.
    if !body.as_str().contains("ush_fn_std__process__arg") {
        docs::push_doc_support(
            &mut out,
            docs,
            script_name.unwrap_or("script"),
            &extra_completion,
        );
    }
    out.set_section(previous_section);
    if body.as_str().contains("4>&1 >&3") {
        out.push_str("exec 3>&1\n\n");
    }
    if body.as_str().contains("__ush_pipe_check") {
        out.push_str(include_str!("pipeline_runtime.sh"));
        out.push('\n');
    }
    if body.as_str().contains("__ush_defers=") {
        out.push_str(include_str!("defer_runtime.sh"));
        out.push('\n');
    }
    if body.as_str().contains("__ush_task_kill") {
        out.push_str(include_str!("kill_runtime.sh"));
        out.push('\n');
    }
    wire::emit(state.wire_types, &enums, &mut out)?;
    stdlib::emit_builtins(&mut out, body.as_str());
    if body.as_str().contains("ush_fn_std__process__arg") {
        out.push_str("__ush_argv=\"$(__ush_std_encode_args \"$@\")\"\n");
    }
    out.append(body);
    out.finish();
    Ok(out)
}
