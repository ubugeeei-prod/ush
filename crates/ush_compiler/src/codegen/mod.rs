mod alias;
mod bin;
mod blocks;
mod builtin_methods;
mod call_support;
pub(crate) mod calls;
mod compare;
mod control;
mod control_support;
mod docs;
mod effects_lowering;
mod enum_registry;
mod functions;
mod io;
mod method_fields;
mod methods;
mod methods_support;
mod primitive;
mod render;
mod runtime_expr;
mod runtime_member;
mod runtime_support;
mod shared;
mod source_context;
mod statement;
mod statement_control;
mod stdlib;
mod task_block;
mod tasks;
pub(crate) mod wire;

use anyhow::Result;

use super::{
    ast::{Statement, StatementKind},
    effects,
    env::{CodegenState, EnumRegistry, Env},
};
use crate::ScriptDocs;
use crate::sourcemap::{OutputBuffer, SourceMapSection};
use crate::traits::{TraitImplRegistry, TraitRegistry, register_trait, register_trait_impl};
pub(crate) use source_context::SourceContext;

pub(crate) use effects_lowering::effect_dispatcher;
pub(crate) use functions::FunctionRegistry;
pub(crate) use primitive::{compile_primitive_expr, infer};
pub(crate) use runtime_expr::{compile_runtime_primitive_expr, rendered_call_runtime};
pub(crate) use shared::binding_for_name;
pub(crate) use stdlib::register_builtins;
pub(crate) use task_block::infer_async_block_type;
pub(crate) use tasks::bind_await;

pub(crate) fn compile_program(
    program: &[Statement],
    docs: &ScriptDocs,
    source: &str,
    script_name: Option<&str>,
    source_context: SourceContext<'_>,
    target: crate::ShellTarget,
) -> Result<OutputBuffer> {
    let mut env = Env::default();
    let mut functions = functions::FunctionRegistry::default();
    let mut enums = EnumRegistry::default();
    let mut traits = TraitRegistry::default();
    let mut trait_impls = TraitImplRegistry::default();
    let mut state = CodegenState::new(source);
    let mut out = OutputBuffer::for_program(SourceMapSection::RuntimeSupport);

    stdlib::register_builtins(&mut functions)?;

    for statement in program {
        match &statement.kind {
            StatementKind::Use(_) => {}
            StatementKind::Enum(def) => enum_registry::register_enum(def, &mut enums)?,
            StatementKind::Trait(def) => register_trait(def, &mut traits)?,
            StatementKind::Impl(item) => register_trait_impl(item, &traits, &mut trait_impls)?,
            StatementKind::Function(def) => functions::register_function(def, &mut functions)?,
            // An effect operation is a call target like any other:
            // the declaration generates the dispatcher function that
            // `do name(…)` resolves to.
            StatementKind::Effect(def) => functions::register_function(
                &effects_lowering::effect_dispatcher(def),
                &mut functions,
            )?,
            _ => {}
        }
    }

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
    for statement in program {
        if matches!(statement.kind, StatementKind::Enum(_)) {
            continue;
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
        bin::push_bin_entry(&mut out, def, &globals, &functions, &trait_impls, &enums)?;
    }
    out.set_section(previous_section);
    let body = out;
    let mut out = OutputBuffer::for_program(SourceMapSection::RuntimeSupport);
    let uses_tasks = body.as_str().contains("__ush_jobs=");
    source_context::emit(&mut out, source_context, target, uses_tasks);
    let previous_section = out.set_section(SourceMapSection::DocSupport);
    docs::push_doc_support(
        &mut out,
        docs,
        script_name.unwrap_or("script"),
        &extra_completion,
    );
    out.set_section(previous_section);
    if body.as_str().contains("4>&1 >&3") {
        out.push_str("exec 3>&1\n\n");
    }
    wire::emit(state.wire_types, &enums, &mut out)?;
    stdlib::emit_builtins(&mut out, body.as_str());
    out.append(body);
    out.finish();
    Ok(out)
}
