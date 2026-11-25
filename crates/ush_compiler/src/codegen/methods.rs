use anyhow::{Result, anyhow, bail};

use super::{
    super::{
        ast::{Call, Expr, MethodCall, Type},
        env::{CodegenState, EnumRegistry, Env},
        matching::materialize_expr,
    },
    FunctionRegistry,
    builtin_methods::{capture_builtin_method, infer_builtin_method},
    calls::ensure_value_type,
    compile_runtime_primitive_expr, infer,
    methods_support::{bind_method_args, compile_method_body, resolve_method_args},
    runtime_support::{FailureMode, hoist_expr},
};
use crate::sourcemap::OutputBuffer;
use crate::traits::{TraitImplRegistry, ensure_trait, lookup_method};
use crate::types::{HeapVec as Vec, OutputString as String};

pub(crate) fn infer_field_expr(
    base: &Expr,
    name: &str,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    let base_ty = infer(base, env, functions, impls, enums)?;
    if name == "await"
        && let Type::Task(result) = base_ty
    {
        return Ok(*result);
    }
    if let Some(ty) = super::task_methods::infer_field(&base_ty, name) {
        return Ok(ty);
    }
    super::method_fields::field_type(&base_ty, name, enums)
}

pub(crate) fn infer_method_call(
    call: &MethodCall,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    let receiver_ty = infer(&call.receiver, env, functions, impls, enums)?;
    if let Some(ty) =
        super::task_methods::infer_method(call, &receiver_ty, env, functions, impls, enums)?
    {
        return Ok(ty);
    }
    if let Some(ty) = infer_builtin_method(call, env, functions, impls, enums)? {
        return Ok(ty);
    }
    let def = lookup_method(&receiver_ty, &call.method, impls)?;
    let args = resolve_method_args(call, def)?;
    for (param, arg) in def.params.iter().zip(args) {
        let actual = infer(arg, env, functions, impls, enums)?;
        if actual != param.ty {
            bail!(
                "type mismatch for `{}.{}`: expected {:?}, found {:?}",
                receiver_ty.render(),
                def.name,
                param.ty,
                actual
            );
        }
    }
    def.return_type
        .clone()
        .ok_or_else(|| anyhow!("method `{}` does not return a value", def.name))
}

pub(crate) fn infer_format_call(
    call: &Call,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    let [arg] = resolve_format_args(call)?;
    let ty = infer(arg, env, functions, impls, enums)?;
    ensure_trait(&ty, "Display", impls)?;
    Ok(Type::String)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_display_expr(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<String> {
    let ty = infer(expr, env, functions, impls, enums)?;
    ensure_trait(&ty, "Display", impls)?;
    match ty {
        Type::String | Type::Int | Type::Bool | Type::Unit => compile_runtime_primitive_expr(
            expr,
            env,
            functions,
            impls,
            enums,
            state,
            inside_function,
            out,
        ),
        _ => compile_method_capture(
            expr,
            "fmt",
            &[],
            env,
            functions,
            impls,
            enums,
            state,
            false,
            inside_function,
            out,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_method_capture(
    receiver: &Expr,
    method: &str,
    args: &[Expr],
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    propagate: bool,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<String> {
    let mode = if propagate {
        FailureMode::Propagate
    } else {
        FailureMode::Abort
    };
    let mut runtime_env = env.clone();
    let receiver = hoist_expr(
        receiver,
        &mut runtime_env,
        functions,
        impls,
        enums,
        state,
        mode,
        inside_function,
        out,
    )?;
    let args = args
        .iter()
        .map(|arg| {
            hoist_expr(
                arg,
                &mut runtime_env,
                functions,
                impls,
                enums,
                state,
                mode,
                inside_function,
                out,
            )
        })
        .collect::<Result<Vec<_>>>()?;
    if let Some(rendered) = super::task_methods::compile_kill(
        &receiver,
        method,
        &args,
        &runtime_env,
        functions,
        impls,
        enums,
        inside_function,
        out,
    )? {
        return Ok(rendered);
    }
    if let Some(rendered) = capture_builtin_method(
        &receiver,
        method,
        &args,
        &runtime_env,
        functions,
        impls,
        enums,
    )? {
        return Ok(rendered);
    }
    let receiver_ty = infer(&receiver, &runtime_env, functions, impls, enums)?;
    let def = lookup_method(&receiver_ty, method, impls)?;
    let return_type = def
        .return_type
        .clone()
        .ok_or_else(|| anyhow!("method `{method}` does not return a value"))?;
    ensure_value_type(&return_type)?;
    let receiver_binding = materialize_expr(
        &receiver,
        &runtime_env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )?;
    let mut method_env = runtime_env.clone();
    method_env.insert("self".into(), receiver_binding);
    bind_method_args(
        &args,
        def,
        &mut method_env,
        &runtime_env,
        functions,
        impls,
        enums,
        state,
        out,
    )?;
    let fn_name = state.temp_var("method");
    out.push_str(&fn_name);
    out.push_str("() {\n");
    let body = compile_method_body(def, &mut method_env, env, functions, impls, enums, state)?;
    out.push_str(&body);
    out.push_str("}\n");
    let temp = state.temp_var("value");
    out.push_str(&temp);
    out.push_str("=\"$(__ush_capture_return='1' __ush_return_path='' ");
    out.push_str(&fn_name);
    out.push_str(" 4>&1 >&3)\"\n");
    Ok(format!("\"${{{temp}}}\""))
}

fn resolve_format_args(call: &Call) -> Result<[&Expr; 1]> {
    match call.args.as_slice() {
        [arg] if arg.label.is_none() => Ok([&arg.expr]),
        _ => bail!("`format` expects exactly one positional argument"),
    }
}
