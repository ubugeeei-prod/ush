//! `allow_fail command` and `detach command`: statements that run a command
//! without letting its status stop the script.

use anyhow::Result;

use super::{
    FunctionRegistry,
    process::{not_a_command, operand, render},
};
use crate::{
    ast::Expr,
    env::{CodegenState, EnumRegistry, Env},
    sourcemap::OutputBuffer,
    traits::TraitImplRegistry,
    types::OutputString as String,
};
use alloc::format;

#[allow(clippy::too_many_arguments)]
pub(super) fn compile_allow_fail(
    command: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    let low = (env, functions, impls, enums);
    let test = render_operand("allow_fail", command, low, state, inside_function, out)?;
    out.push_str(&format!("{test} || :\n"));
    Ok(())
}

/// `detach command`: run it in the background and never wait for it. It
/// reads nothing, and writes only where its own redirections send it: a
/// process that outlives the script must not hold the script's output open.
#[allow(clippy::too_many_arguments)]
pub(super) fn compile_detach(
    command: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    let low = (env, functions, impls, enums);
    let run = render_operand("detach", command, low, state, inside_function, out)?;
    out.push_str(&format!("( {run} ) </dev/null >/dev/null 2>&1 &\n"));
    Ok(())
}

fn render_operand(
    keyword: &'static str,
    command: &Expr,
    (env, functions, impls, enums): (&Env, &FunctionRegistry, &TraitImplRegistry, &EnumRegistry),
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<String> {
    let allow_await = keyword == "allow_fail";
    let operand =
        operand(command, env, functions, allow_await)?.ok_or_else(not_a_command(keyword))?;
    render(
        operand,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )
}
