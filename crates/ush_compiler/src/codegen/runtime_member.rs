use anyhow::Result;

use super::{
    super::{
        ast::{Call, Expr, MethodCall, Type},
        env::{Binding, CodegenState, EnumRegistry, Env, Storage},
        matching::read_expr,
    },
    FunctionRegistry, infer,
    method_fields::field_binding,
    methods::compile_display_expr,
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;
use crate::types::HeapVec as Vec;

#[allow(clippy::too_many_arguments)]
pub(super) fn hoist_field_access(
    base: &Expr,
    name: &str,
    env: &mut Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<Expr> {
    if name == "await"
        && let Expr::Var(task) = base
        && env
            .get(task)
            .is_some_and(|b| matches!(b.ty, crate::ast::Type::Task(_)))
    {
        let temp = state.temp_var("await");
        super::tasks::bind_await(&temp, task, env, out)?;
        return Ok(Expr::Var(temp));
    }
    let binding = read_expr(
        base,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )?;
    let temp = state.temp_var("field");
    env.insert(temp.clone(), field_binding(&binding, name, enums)?);
    Ok(Expr::Var(temp))
}

#[allow(clippy::too_many_arguments)]
pub(super) fn hoist_method_capture(
    call: &MethodCall,
    env: &mut Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    propagate: bool,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<Expr> {
    let args = call
        .args
        .iter()
        .map(|arg| arg.expr.clone())
        .collect::<Vec<_>>();
    let rendered = super::methods::compile_method_capture(
        &call.receiver,
        &call.method,
        &args,
        env,
        functions,
        impls,
        enums,
        state,
        propagate,
        inside_function,
        out,
    )?;
    let temp = state.temp_var("value");
    out.push_str(&temp);
    out.push('=');
    out.push_str(&rendered);
    if propagate {
        out.push_str(" || ");
        out.push_str(if inside_function {
            "return \"$?\""
        } else {
            "exit \"$?\""
        });
    }
    out.push('\n');
    env.insert(
        temp.clone(),
        Binding {
            ty: infer(
                &Expr::MethodCall(call.clone()),
                env,
                functions,
                impls,
                enums,
            )?,
            storage: Storage::Primitive(temp.clone()),
        },
    );
    Ok(Expr::Var(temp))
}

#[allow(clippy::too_many_arguments)]
pub(super) fn hoist_format_capture(
    call: &Call,
    env: &mut Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    propagate: bool,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<Expr> {
    let rendered = compile_display_expr(
        &call.args[0].expr,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )?;
    let temp = state.temp_var("value");
    out.push_str(&temp);
    out.push('=');
    out.push_str(&rendered);
    if propagate {
        out.push_str(" || ");
        out.push_str(if inside_function {
            "return \"$?\""
        } else {
            "exit \"$?\""
        });
    }
    out.push('\n');
    env.insert(
        temp.clone(),
        Binding {
            ty: Type::String,
            storage: Storage::Primitive(temp.clone()),
        },
    );
    Ok(Expr::Var(temp))
}
