use alloc::boxed::Box;

use anyhow::{Result, anyhow, bail};

use super::{
    super::{
        ast::{Call, Expr, Type},
        env::{Binding, CodegenState, EnumRegistry, Env, Storage},
    },
    calls::{call_return_type, ensure_value_type},
    compile_runtime_primitive_expr,
    functions::FunctionRegistry,
    infer, rendered_call_runtime,
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_spawn(
    call: &Call,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<Binding> {
    let return_type = call_return_type(&call.name, functions)?
        .ok_or_else(|| anyhow!("async bindings require a return type: {}", call.name))?;
    ensure_value_type(&return_type)?;

    let rendered = rendered_call_runtime(
        call,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )?;
    let stdout_value = call.name.starts_with("std::") || call.name.starts_with("$bin::");
    Ok(start_task(&rendered, return_type, stdout_value, state, out))
}

pub(super) fn start_task(
    rendered: &str,
    return_type: Type,
    stdout_value: bool,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Binding {
    let prefix = state.temp_var("task");
    out.push_str("__ush_task_slot\n");
    out.push_str(&format!(
        "{prefix}__result=$__ush_slot/result\n: > \"${{{prefix}__result}}\"\n"
    ));
    out.push_str(&format!(": > \"${{{prefix}__result}}.status\"\n"));
    out.push_str(&format!(
        "( __ush_task_scope \"${{{prefix}__result}}.status\"; __ush_return_path=\"${{{prefix}__result}}\"; {rendered}"
    ));
    if stdout_value {
        out.push_str(" > \"$__ush_return_path\"");
    }
    out.push_str(" ) &\n");
    out.push_str(&format!("{prefix}__pid=$!\n{prefix}__awaited=0\n"));
    out.push_str("__ush_jobs=\"${__ush_jobs} $!\"\n");
    Binding {
        ty: Type::Task(Box::new(return_type)),
        storage: Storage::Task(prefix, Default::default()),
    }
}

pub(crate) fn compile_await(task: &str, env: &Env, out: &mut OutputBuffer) -> Result<Binding> {
    let binding = env
        .get(task)
        .ok_or_else(|| anyhow!("unknown task: {task}"))?;
    let (Type::Task(inner), Storage::Task(prefix, _)) = (&binding.ty, &binding.storage) else {
        bail!("await expects a task handle: {task}");
    };

    out.push_str(&format!("if [ \"${{{prefix}__awaited}}\" = '0' ]; then\n"));
    out.push_str(&format!(
        "  __ush_wait_job \"${{{prefix}__pid}}\" \"${{{prefix}__result}}.status\" || exit \"$?\"\n"
    ));
    out.push_str(&format!("  {prefix}__awaited='1'\n"));
    out.push_str("fi\n");

    Ok(Binding {
        ty: (*inner.clone()),
        storage: Storage::Primitive(format!("\"$(cat \"${{{prefix}__result}}\")\"").into()),
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_return(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    return_type: Option<&Type>,
    out: &mut OutputBuffer,
) -> Result<()> {
    let Some(declared) = return_type else {
        bail!("return is only valid inside functions");
    };
    let actual = infer(expr, env, functions, impls, enums)?;
    if actual != *declared {
        bail!("return type mismatch: expected {declared:?}, found {actual:?}");
    }
    ensure_value_type(declared)?;

    let rendered = if actual.is_structured() {
        let value =
            crate::matching::read_expr(expr, env, functions, impls, enums, state, true, out)?;
        let (Storage::Aggregate(prefix) | Storage::Adt(prefix)) = value.storage else {
            unreachable!()
        };
        super::wire::register(&actual, state);
        super::wire::encode(&actual, &prefix)
    } else {
        compile_runtime_primitive_expr(expr, env, functions, impls, enums, state, true, out)?
    };
    out.push_str("if [ -n \"${__ush_return_path:-}\" ]; then\n");
    out.push_str("  printf '%s' ");
    out.push_str(&rendered);
    out.push_str(" > \"$__ush_return_path\"\n");
    out.push_str("elif [ \"${__ush_capture_return:-0}\" = '1' ]; then\n");
    out.push_str("  printf '%s' ");
    out.push_str(&rendered);
    out.push_str(" >&4\n");
    out.push_str("fi\n");
    out.push_str("return 0\n");
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_expr_statement(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    let ty = infer(expr, env, functions, impls, enums)?;
    ensure_value_type(&ty)?;
    let _ = compile_runtime_primitive_expr(
        expr,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )?;
    Ok(())
}

pub(crate) fn bind_await(
    name: &str,
    task: &str,
    env: &mut Env,
    out: &mut OutputBuffer,
) -> Result<()> {
    let binding = compile_await(task, env, out)?;
    let Storage::Primitive(value) = binding.storage else {
        unreachable!()
    };
    if binding.ty.is_structured() {
        super::wire::decode(&binding.ty, name, &value, out);
    } else {
        out.push_str(&format!("{name}={value}\n"));
    }
    env.insert(name.into(), super::binding_for_name(name, binding.ty));
    Ok(())
}
