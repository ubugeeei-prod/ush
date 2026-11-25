use alloc::vec::Vec;

use anyhow::{Result, bail};

use super::{
    super::{
        ast::{Condition, Statement, Type},
        env::{Binding, Env, Storage},
        matching::{compile_pattern, read_expr},
    },
    compile_runtime_primitive_expr,
    control::ControlContext,
    shared::binding_for_name,
    statement::compile_statement,
};
use crate::sourcemap::OutputBuffer;
use crate::types::OutputString as String;

#[derive(Clone)]
pub(super) struct ConditionPlan {
    pub script: String,
    pub env: Env,
    pub binds: bool,
}

pub(super) fn compile_condition(
    condition: &Condition,
    env: &Env,
    ctx: &mut ControlContext<'_>,
) -> Result<ConditionPlan> {
    match condition {
        Condition::Expr(expr) => {
            super::process::check_condition(expr, env, ctx.functions, ctx.impls, ctx.enums)?;
            let mut buffer = OutputBuffer::new(ctx.state.mapped);
            if let Some(operand) = super::process::operand(expr, env, ctx.functions, false)? {
                let test = super::process::render(
                    operand,
                    env,
                    ctx.functions,
                    ctx.impls,
                    ctx.enums,
                    ctx.state,
                    ctx.inside_function,
                    &mut buffer,
                )?;
                return Ok(ConditionPlan {
                    script: block_script(buffer, &test),
                    env: env.clone(),
                    binds: false,
                });
            }
            let rendered = compile_runtime_primitive_expr(
                expr,
                env,
                ctx.functions,
                ctx.impls,
                ctx.enums,
                ctx.state,
                ctx.inside_function,
                &mut buffer,
            )?;
            Ok(ConditionPlan {
                script: block_script(buffer, &format!("[ {rendered} = 'true' ]")),
                env: env.clone(),
                binds: false,
            })
        }
        Condition::Let { pattern, expr } => {
            let mut buffer = OutputBuffer::new(ctx.state.mapped);
            let subject = read_expr(
                expr,
                env,
                ctx.functions,
                ctx.impls,
                ctx.enums,
                ctx.state,
                ctx.inside_function,
                &mut buffer,
            )?;
            let plan = compile_pattern(pattern, &subject, env, ctx.enums)?;
            Ok(ConditionPlan {
                script: block_script_with_lines(buffer, &plan.prelude, &plan.condition),
                env: plan.env,
                binds: true,
            })
        }
        Condition::And(items) => {
            let mut current_env = env.clone();
            let mut parts = Vec::new();
            let mut binds = false;
            for item in items {
                let plan = compile_condition(item, &current_env, ctx)?;
                current_env = plan.env.clone();
                binds |= plan.binds;
                parts.push(plan.script);
            }
            Ok(ConditionPlan {
                script: format!("{{\n{}\n}}", parts.join(" && ")),
                env: current_env,
                binds,
            })
        }
        Condition::Not(inner) => {
            let plan = compile_condition(inner, env, ctx)?;
            Ok(ConditionPlan {
                script: format!("! {}", plan.script),
                env: env.clone(),
                binds: false,
            })
        }
        Condition::Or(items) => {
            let plans = items
                .iter()
                .map(|item| compile_condition(item, env, ctx))
                .collect::<Result<Vec<_>>>()?;
            if plans.iter().any(|plan| plan.binds) {
                bail!("pattern bindings are only supported in `&&` conditions");
            }
            Ok(ConditionPlan {
                script: format!(
                    "{{\n{}\n}}",
                    plans
                        .iter()
                        .map(|plan| plan.script.clone())
                        .collect::<Vec<_>>()
                        .join(" || ")
                ),
                env: env.clone(),
                binds: false,
            })
        }
    }
}

pub(super) fn compile_body(
    statements: &[Statement],
    env: &mut Env,
    tail_position: bool,
    ctx: &mut ControlContext<'_>,
) -> Result<OutputBuffer> {
    let mut out = OutputBuffer::new(ctx.state.mapped);
    for (index, statement) in statements.iter().enumerate() {
        compile_statement(
            statement,
            env,
            ctx.globals,
            ctx.functions,
            ctx.impls,
            ctx.enums,
            ctx.function_errors,
            ctx.state,
            ctx.return_type,
            ctx.inside_function,
            tail_position && index + 1 == statements.len(),
            &mut out,
        )?;
    }
    if out.is_empty() {
        out.push_str(":\n");
    }
    Ok(out)
}

pub(super) fn bind_loop_var(name: &str, storage: &str, ty: &Type, env: &Env) -> Env {
    let mut body_env = env.clone();
    body_env.insert(name.into(), binding_for_name(storage, ty.clone()));
    body_env
}

pub(super) fn homogeneous_type(items: &[Type]) -> Option<Type> {
    let first = items.first()?.clone();
    items.iter().all(|item| *item == first).then_some(first)
}

pub(super) fn tuple_item_binding(prefix: &str, index: usize, ty: &Type) -> Binding {
    let storage = match ty {
        Type::Adt(_) => Storage::Adt(format!("{prefix}__{index}").into()),
        Type::Tuple(_) | Type::List(_) => Storage::Aggregate(format!("{prefix}__{index}").into()),
        Type::String | Type::Int | Type::Bool | Type::Unit => {
            Storage::Primitive(format!("{prefix}__{index}").into())
        }
        Type::Task(_) => Storage::Task(format!("{prefix}__{index}").into(), Default::default()),
    };
    Binding {
        ty: ty.clone(),
        storage,
    }
}

fn block_script(buffer: OutputBuffer, condition: &str) -> String {
    block_script_with_lines(buffer, &[], condition)
}

fn block_script_with_lines(buffer: OutputBuffer, lines: &[String], condition: &str) -> String {
    let compiled = buffer.into_shell();
    let mut out = String::from("{\n");
    out.push_str(&compiled);
    for line in lines {
        out.push_str(line);
        out.push('\n');
    }
    out.push_str(condition);
    out.push_str("\n}");
    out
}

pub(super) fn loop_storage(
    name: &str,
    env: &Env,
    state: &mut crate::env::CodegenState,
) -> crate::types::AstString {
    if env.contains_key(name) {
        state.temp_var("loop_binding")
    } else {
        name.into()
    }
}
