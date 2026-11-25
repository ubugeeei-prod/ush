//! `len`, `is_empty`, `contains`, `join`, `first`, `last` and
//! `enumerate` in value position.

use anyhow::{Result, bail};

use super::{
    Lowering,
    access::{bind_element, element_index, read_list, scalar},
};
use crate::{
    ast::{Expr, MethodCall, Type},
    codegen::{binding_for_name, compile_runtime_primitive_expr},
    env::{CodegenState, Env},
    sourcemap::OutputBuffer,
};

pub(crate) fn hoist_method(
    call: &MethodCall,
    env: &mut Env,
    low: &Lowering<'_>,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<Expr> {
    let method = call.method.as_str();
    if method == "enumerate" {
        let ty = low.infer(&Expr::MethodCall(call.clone()), env)?;
        let temp = state.temp_var("list");
        super::build::emit_to_target(
            &temp,
            &Expr::MethodCall(call.clone()),
            &ty,
            env,
            low,
            state,
            out,
        )?;
        env.insert(temp.clone(), binding_for_name(&temp, ty));
        return Ok(Expr::Var(temp));
    }
    let (prefix, item) = read_list(&call.receiver, env, low, state, out)?;
    if matches!(method, "first" | "last") {
        let position = element_index(&prefix, None, method, env, low, state, out)?;
        return Ok(bind_element(&prefix, &position, &item, env, state, out));
    }
    let argument = match call.args.first() {
        Some(arg) => Some(compile_runtime_primitive_expr(
            &arg.expr,
            env,
            low.functions,
            low.impls,
            low.enums,
            state,
            low.inside_function,
            out,
        )?),
        None => None,
    };
    let temp = state.temp_var("value");
    let (code, ty) = match (method, argument) {
        ("len", None) => (format!("{temp}=${{{prefix}__len}}\n"), Type::Int),
        ("is_empty", None) => (
            format!(
                "if [ \"${{{prefix}__len}}\" -eq 0 ]; then {temp}=true; else {temp}=false; fi\n"
            ),
            Type::Bool,
        ),
        ("contains", Some(needle)) => (
            format!(
                "if __ush_std_list_contains {prefix} {needle}; then {temp}=true; else {temp}=false; fi\n"
            ),
            Type::Bool,
        ),
        ("join", Some(separator)) => (
            format!("__ush_std_list_join {prefix} {separator}\n{temp}=$__ush_std_list_joined\n"),
            Type::String,
        ),
        _ => bail!("unsupported List method `{method}`"),
    };
    out.push_str(&code);
    env.insert(temp.clone(), scalar(&temp, ty));
    Ok(Expr::Var(temp))
}
