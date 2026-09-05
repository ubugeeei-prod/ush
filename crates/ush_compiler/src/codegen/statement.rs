use anyhow::{Result, anyhow};

use super::{
    super::{
        ast::{Statement, StatementKind, Type},
        effects::FunctionErrorRegistry,
        env::{CodegenState, EnumRegistry, Env},
    },
    alias::compile_alias,
    blocks::{compile_let, compile_match},
    calls::{compile_call, compile_try_call},
    functions::{FunctionRegistry, compile_function},
    io::{compile_raise, compile_shell, push_print},
    shared::binding_for_name,
    statement_control::compile_control_statement,
    tasks::{compile_await, compile_expr_statement, compile_return, compile_spawn},
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_statement(
    statement: &Statement,
    env: &mut Env,
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
    state: &mut CodegenState,
    return_type: Option<&Type>,
    inside_function: bool,
    tail_position: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    let previous_origin = out.set_origin(Some(statement.line));
    let result = (|| -> Result<()> {
        if let Some(control) = compile_control_statement(
            &statement.kind,
            env,
            globals,
            functions,
            impls,
            enums,
            function_errors,
            state,
            return_type,
            inside_function,
            out,
        ) {
            control?;
            return Ok(());
        }
        match &statement.kind {
            StatementKind::Use(_) => {}
            StatementKind::Enum(_) => {}
            StatementKind::Trait(_) | StatementKind::Impl(_) => {}
            StatementKind::Effect(def) => super::effects_lowering::compile_effect(def, out),
            StatementKind::Handle { body, handlers } => super::effects_lowering::compile_handle(
                body,
                handlers,
                env,
                globals,
                functions,
                impls,
                enums,
                function_errors,
                state,
                inside_function,
                out,
            )?,
            StatementKind::Function(def) => compile_function(
                def,
                globals,
                functions,
                impls,
                enums,
                function_errors,
                state,
                out,
            )?,
            StatementKind::Alias { name, value } => compile_alias(
                name,
                value,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?,
            StatementKind::Let { name, expr } => compile_let(
                name,
                expr,
                env,
                globals,
                functions,
                impls,
                enums,
                function_errors,
                state,
                inside_function,
                out,
            )?,
            StatementKind::Spawn { name, call } => {
                let binding = compile_spawn(
                    call,
                    env,
                    functions,
                    impls,
                    enums,
                    state,
                    inside_function,
                    out,
                )?;
                env.insert(name.clone(), binding);
            }
            StatementKind::Await { name, task } => {
                let binding = compile_await(task, env, out)?;
                env.insert(name.clone(), binding_for_name(name, binding.ty));
                if let super::super::env::Storage::Primitive(value) = binding.storage {
                    out.push_str(name);
                    out.push('=');
                    out.push_str(&value);
                    out.push('\n');
                }
            }
            StatementKind::Print(expr) => push_print(
                expr,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?,
            StatementKind::Shell(expr) => compile_shell(
                expr,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                false,
                out,
            )?,
            StatementKind::TryShell(expr) => compile_shell(
                expr,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                true,
                out,
            )?,
            StatementKind::Raise(expr) => {
                compile_raise(expr, env, functions, impls, enums, inside_function, out)?
            }
            StatementKind::Expr(expr) => {
                if tail_position {
                    compile_return(expr, env, functions, impls, enums, state, return_type, out)?;
                } else {
                    compile_expr_statement(
                        expr,
                        env,
                        functions,
                        impls,
                        enums,
                        state,
                        inside_function,
                        out,
                    )?;
                }
            }
            StatementKind::Call(call) => compile_call(
                call,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?,
            StatementKind::TryCall(call) => compile_try_call(
                call,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?,
            StatementKind::Return(expr) => {
                compile_return(expr, env, functions, impls, enums, state, return_type, out)?
            }
            StatementKind::Match {
                expr,
                arms,
                returns_value,
            } => compile_match(
                expr,
                arms,
                *returns_value,
                env,
                globals,
                functions,
                impls,
                enums,
                function_errors,
                state,
                return_type,
                inside_function,
                out,
            )?,
            kind @ (StatementKind::If { .. }
            | StatementKind::While { .. }
            | StatementKind::For { .. }
            | StatementKind::Loop { .. }
            | StatementKind::Break
            | StatementKind::Continue) => {
                return Err(anyhow!(
                    "internal codegen invariant violated: control-flow statement {kind:?} must be \
                     dispatched via compile_control_statement before reaching compile_statement",
                ));
            }
        }
        Ok(())
    })();
    out.set_origin(previous_origin);
    result
}
