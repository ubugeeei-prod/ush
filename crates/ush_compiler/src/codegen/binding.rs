use super::{FunctionRegistry, binding_for_name};
use crate::{
    ast::{BindingMode, Statement, StatementKind as S, Type},
    effects::FunctionErrorRegistry,
    env::{Binding, CodegenState, EnumRegistry, Env, Storage},
    sourcemap::OutputBuffer,
    traits::TraitImplRegistry,
    types::AstString as Name,
};
use anyhow::{Result, anyhow, ensure};

pub(super) fn target(s: &Statement, name: &str, ty: &Type, env: &Env) -> Result<Name> {
    if s.binding == BindingMode::Assign {
        let binding = env
            .get(name)
            .ok_or_else(|| anyhow!("undeclared binding `{name}`"))?;
        ensure!(
            binding.ty == *ty,
            "assignment to `{name}`: expected {}, found {}",
            binding.ty.render(),
            ty.render()
        );
        return Ok(match &binding.storage {
            Storage::Primitive(n)
            | Storage::Adt(n)
            | Storage::Aggregate(n)
            | Storage::Task(n, _) => n.clone(),
        });
    }
    Ok(format!("__ush_binding_{}_{name}", s.line).into())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn compile(
    s: &Statement,
    env: &mut Env,
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    errors: &FunctionErrorRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    let (name, ty) = match &s.kind {
        S::Let { name, expr } => (name, super::infer(expr, env, functions, impls, enums)?),
        S::Spawn { name, call } => (
            name,
            Type::Task(alloc::boxed::Box::new(super::calls::call_expr_type(
                call, functions,
            )?)),
        ),
        S::Await { name, task } => {
            let Some(Type::Task(ty)) = env.get(task).map(|b| &b.ty) else {
                anyhow::bail!("unknown task: {task}");
            };
            (name, (**ty).clone())
        }
        _ => unreachable!(),
    };
    let target = target(s, name, &ty, env)?;
    match &s.kind {
        S::Let { expr, .. } => {
            let destination = if s.binding == BindingMode::Assign && ty.is_structured() {
                state.temp_var("assignment")
            } else {
                target.clone()
            };
            super::blocks::compile_let(
                &destination,
                expr,
                &ty,
                env,
                globals,
                functions,
                impls,
                enums,
                errors,
                state,
                inside_function,
                out,
            )?;
            if destination != target {
                crate::matching::emit_value_to_target(
                    &target,
                    &crate::ast::Expr::Var(destination.clone()),
                    &ty,
                    env,
                    functions,
                    impls,
                    enums,
                    state,
                    inside_function,
                    out,
                )?;
                env.remove(&destination);
                env.insert(target.clone(), binding_for_name(&target, ty.clone()));
            }
        }
        S::Spawn { call, .. } => {
            let binding = super::tasks::compile_spawn(
                call,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?;
            store_task(&target, &binding, out);
            env.insert(target.clone(), binding_for_name(&target, ty.clone()));
        }
        S::Await { task, .. } => super::tasks::bind_await(&target, task, env, out)?,
        _ => unreachable!(),
    }
    let binding = env.remove(&target).expect("binding was emitted");
    if !ty.is_structured() && !matches!(ty, Type::Task(_)) {
        // Raw shell escapes can still read the source spelling. Typed code uses isolated storage.
        out.push_str(&format!(
            "# shellcheck disable=SC2034\n{name}=\"${{{target}}}\"\n"
        ));
    }
    env.insert(name.clone(), binding);
    Ok(())
}

pub(super) fn store_task(target: &str, binding: &Binding, out: &mut OutputBuffer) {
    let Storage::Task(source, _) = &binding.storage else {
        unreachable!()
    };
    for field in ["pid", "result", "awaited"] {
        out.push_str(&format!("{target}__{field}=\"${{{source}__{field}}}\"\n"));
    }
}
