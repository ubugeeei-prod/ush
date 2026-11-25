mod case;
mod emit;
mod exhaustiveness;
mod pattern;
mod read;
pub(crate) use read::read_expr;
mod comprehension;
mod sequence;

pub(crate) use case::case_word;
pub(crate) use exhaustiveness::check_exhaustive;

use anyhow::{Result, bail};

use super::{
    ast::{Expr, Pattern, Type},
    codegen::FunctionRegistry,
    codegen::infer,
    env::{Binding, CodegenState, EnumRegistry, Env, Storage, expect_adt},
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;

use crate::types::{AstVec as Vec, OutputString as String};
use emit::{emit_copy, emit_variant};
use pattern::bind_pattern;
use sequence::emit_sequence;

pub(crate) struct PatternPlan {
    pub condition: String,
    pub prelude: Vec<String>,
    pub env: Env,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn materialize_expr(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<Binding> {
    let ty = infer(expr, env, functions, impls, enums)?;
    match ty.clone() {
        Type::Adt(enum_name) => {
            let prefix = state.temp_var("match");
            emit_value_to_target(
                &prefix,
                expr,
                &Type::Adt(enum_name.clone()),
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?;
            Ok(Binding {
                ty: Type::Adt(enum_name),
                storage: Storage::Adt(prefix),
            })
        }
        Type::Tuple(_) | Type::List(_) => {
            let prefix = state.temp_var("match");
            emit_value_to_target(
                &prefix,
                expr,
                &ty,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?;
            Ok(Binding {
                ty,
                storage: Storage::Aggregate(prefix),
            })
        }
        primitive => {
            let temp = state.temp_var("match");
            let rendered = super::codegen::compile_runtime_primitive_expr(
                expr,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?;
            out.push_str(&format!("{temp}={rendered}\n"));
            Ok(Binding {
                ty: primitive,
                storage: Storage::Primitive(temp),
            })
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn emit_value_to_target(
    target: &str,
    expr: &Expr,
    expected: &Type,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    if expected.is_structured() {
        let low = crate::codegen::list::Lowering::new(functions, impls, enums, inside_function);
        if crate::codegen::list::handles(expr, env, &low) {
            return crate::codegen::list::emit_to_target(
                target, expr, expected, env, &low, state, out,
            );
        }
        if let Expr::Try(inner) = expr {
            return emit_value_to_target(
                target,
                inner,
                expected,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            );
        }
        if let Expr::Field { base, name } = expr
            && name == "await"
            && let Expr::Var(task) = &**base
            && env.get(task).is_some_and(|b| matches!(b.ty, Type::Task(_)))
        {
            crate::codegen::bind_await(target, task, &mut env.clone(), out)?;
            return Ok(());
        }
        if let Expr::Call(call) = expr {
            let command = crate::codegen::rendered_call_runtime(
                call,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?;
            let temp = state.temp_var("packed");
            let fds = if call.name.starts_with("std::") {
                ""
            } else {
                " 4>&1 >&3"
            };
            out.push_str(&format!("{temp}=\"$(__ush_return_path='' __ush_capture_return=1 {command}{fds})\" || {} \"$?\"\n",if inside_function{"return"}else{"exit"}));
            crate::codegen::wire::register(expected, state);
            crate::codegen::wire::decode(expected, target, &format!("\"${{{temp}}}\""), out);
            return Ok(());
        }
    }
    match expected {
        Type::String | Type::Int | Type::Bool | Type::Unit => {
            let rendered = super::codegen::compile_runtime_primitive_expr(
                expr,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?;
            // Aggregate cells are read indirectly by the wire-format helpers.
            if target.contains("__") {
                out.push_str("# shellcheck disable=SC2034\n");
            }
            out.push_str(&format!("{target}={rendered}\n"));
        }
        Type::Adt(enum_name) => match expr {
            Expr::Variant(variant) => emit_variant(
                target,
                variant,
                enum_name,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?,
            Expr::Var(name) => {
                let binding = env
                    .get(name)
                    .ok_or_else(|| anyhow::anyhow!("unknown variable: {name}"))?;
                if binding.ty != *expected {
                    bail!("type mismatch for {name}");
                }
                emit_copy(target, expect_adt(&binding.ty)?, binding, enums, state, out)?;
            }
            _ => bail!("expected ADT expression for {enum_name}"),
        },
        Type::Tuple(_) | Type::List(_) => emit_sequence(
            target,
            expr,
            expected,
            env,
            functions,
            impls,
            enums,
            state,
            inside_function,
            out,
        )?,
        Type::Task(_) => bail!("task handles cannot be materialized as match values"),
    }
    Ok(())
}

pub(crate) fn compile_pattern(
    pattern: &Pattern,
    subject: &Binding,
    env: &Env,
    enums: &EnumRegistry,
) -> Result<PatternPlan> {
    let mut plan = PatternPlan {
        condition: String::from(":"),
        prelude: Vec::new(),
        env: env.clone(),
    };
    bind_pattern(pattern, subject, enums, &mut plan)?;
    Ok(plan)
}
