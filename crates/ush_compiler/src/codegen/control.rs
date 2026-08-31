use anyhow::{Result, bail};

use super::{
    super::{
        ast::{Condition, Expr, IfBranch, Statement, Type},
        effects::FunctionErrorRegistry,
        env::{CodegenState, Env, Storage},
        matching::{emit_value_to_target, materialize_expr},
    },
    compile_runtime_primitive_expr,
    control_support::{
        bind_loop_var, compile_body, compile_condition, homogeneous_type, tuple_item_binding,
    },
    functions::FunctionRegistry,
    shared::push_output,
};
use crate::env::EnumRegistry;
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;

pub(crate) struct ControlContext<'a> {
    pub globals: &'a Env,
    pub functions: &'a FunctionRegistry,
    pub impls: &'a TraitImplRegistry,
    pub enums: &'a EnumRegistry,
    pub function_errors: &'a FunctionErrorRegistry,
    pub state: &'a mut CodegenState,
    pub return_type: Option<&'a Type>,
    pub inside_function: bool,
}

pub(crate) fn compile_if(
    branch: &IfBranch,
    returns_value: bool,
    env: &Env,
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<()> {
    if returns_value && branch.else_body.is_none() {
        bail!("if expressions require an else branch");
    }
    let plan = compile_condition(&branch.condition, env, ctx)?;
    out.push_str("if ");
    out.push_str(&plan.script);
    out.push_str("; then\n");
    let mut then_env = plan.env;
    push_output(
        out,
        &compile_body(&branch.then_body, &mut then_env, returns_value, ctx)?,
        2,
    );
    if let Some(else_body) = &branch.else_body {
        out.push_str("else\n");
        let mut else_env = env.clone();
        push_output(
            out,
            &compile_body(else_body, &mut else_env, returns_value, ctx)?,
            2,
        );
    }
    out.push_str("fi\n");
    Ok(())
}

pub(crate) fn compile_while(
    condition: &Condition,
    body: &[Statement],
    env: &Env,
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<()> {
    let plan = compile_condition(condition, env, ctx)?;
    out.push_str("while ");
    out.push_str(&plan.script);
    out.push_str("; do\n");
    let mut body_env = plan.env;
    push_output(out, &compile_body(body, &mut body_env, false, ctx)?, 2);
    out.push_str("done\n");
    Ok(())
}

pub(crate) fn compile_loop(
    body: &[Statement],
    env: &Env,
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<()> {
    out.push_str("while :; do\n");
    let mut body_env = env.clone();
    push_output(out, &compile_body(body, &mut body_env, false, ctx)?, 2);
    out.push_str("done\n");
    Ok(())
}

pub(crate) fn compile_for(
    name: &str,
    iterable: &Expr,
    env: &Env,
    body: &[Statement],
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<()> {
    if let Expr::Range { start, end } = iterable {
        return compile_range_for(name, start, end, env, body, ctx, out);
    }
    let binding = materialize_expr(
        iterable,
        env,
        ctx.functions,
        ctx.impls,
        ctx.enums,
        ctx.state,
        ctx.inside_function,
        out,
    )?;
    let Storage::Aggregate(prefix) = binding.storage else {
        bail!("for-in expects a list, tuple, or range");
    };
    match binding.ty {
        Type::Tuple(types) => compile_tuple_for(name, &prefix, &types, env, body, ctx, out),
        Type::List(item_ty) => compile_list_for(name, &prefix, &item_ty, env, body, ctx, out),
        _ => bail!("for-in expects a list, tuple, or range"),
    }
}

fn compile_range_for(
    name: &str,
    start: &Expr,
    end: &Expr,
    env: &Env,
    body: &[Statement],
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<()> {
    let start = compile_runtime_primitive_expr(
        start,
        env,
        ctx.functions,
        ctx.impls,
        ctx.enums,
        ctx.state,
        ctx.inside_function,
        out,
    )?;
    let end = compile_runtime_primitive_expr(
        end,
        env,
        ctx.functions,
        ctx.impls,
        ctx.enums,
        ctx.state,
        ctx.inside_function,
        out,
    )?;
    let index = ctx.state.temp_var("for_index");
    out.push_str(&format!(
        "{index}={start}\nwhile [ \"${{{index}}}\" -lt {end} ]; do\n"
    ));
    let mut body_env = bind_loop_var(name, &Type::Int, env);
    // The counter advances before the body so `continue` cannot skip
    // it and spin forever.
    out.push_str(&format!(
        "  {name}=\"${{{index}}}\"\n  {index}=$(({index} + 1))\n"
    ));
    push_output(out, &compile_body(body, &mut body_env, false, ctx)?, 2);
    out.push_str("done\n");
    Ok(())
}

fn compile_tuple_for(
    name: &str,
    prefix: &str,
    types: &[Type],
    env: &Env,
    body: &[Statement],
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<()> {
    let Some(item_ty) = homogeneous_type(types) else {
        bail!("for-in over tuples requires all items to share one type");
    };
    for (index, _) in types.iter().enumerate() {
        let source_name = format!("__ush_tuple_item_{index}");
        let mut item_env = env.clone();
        item_env.insert(
            source_name.clone().into(),
            tuple_item_binding(prefix, index, &item_ty),
        );
        emit_value_to_target(
            name,
            &Expr::Var(source_name.into()),
            &item_ty,
            &item_env,
            ctx.functions,
            ctx.impls,
            ctx.enums,
            ctx.state,
            ctx.inside_function,
            out,
        )?;
        let mut body_env = bind_loop_var(name, &item_ty, env);
        push_output(out, &compile_body(body, &mut body_env, false, ctx)?, 0);
    }
    Ok(())
}

fn compile_list_for(
    name: &str,
    prefix: &str,
    item_ty: &Type,
    env: &Env,
    body: &[Statement],
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<()> {
    match item_ty {
        Type::String | Type::Int | Type::Bool | Type::Unit => {}
        _ => bail!("for-in over lists of structured values is not implemented yet"),
    }
    let index = ctx.state.temp_var("for_list_index");
    let value = ctx.state.temp_var("for_value");
    out.push_str(&format!(
        "{index}=0\nwhile [ \"${{{index}}}\" -lt \"${{{prefix}__len}}\" ]; do\n"
    ));
    out.push_str(&format!(
        "  eval \"{value}=\\\"\\${{{prefix}__${{{index}}}}}\\\"\"\n"
    ));
    let mut body_env = bind_loop_var(name, item_ty, env);
    // Same as the range lowering: advance before the body so
    // `continue` still makes progress.
    out.push_str(&format!(
        "  {name}=\"${{{value}}}\"\n  {index}=$(({index} + 1))\n"
    ));
    push_output(out, &compile_body(body, &mut body_env, false, ctx)?, 2);
    out.push_str("done\n");
    Ok(())
}
