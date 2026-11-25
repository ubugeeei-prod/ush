//! `for (i, item) in list.enumerate()` without building the pair list:
//! each iteration writes the counter and the element into the loop's
//! tuple directly.

use anyhow::Result;

use super::{
    Lowering,
    access::{copy_element, read_list},
};
use crate::{
    ast::{Expr, Statement, Type},
    codegen::{
        control::ControlContext,
        control_support::{bind_loop_var, compile_body, loop_storage},
        shared::push_output,
    },
    env::Env,
    sourcemap::OutputBuffer,
};

pub(crate) fn compile_enumerate_for(
    name: &str,
    list: &Expr,
    env: &Env,
    body: &[Statement],
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<()> {
    let low = Lowering::new(ctx.functions, ctx.impls, ctx.enums, ctx.inside_function);
    let (prefix, item) = read_list(list, env, &low, ctx.state, out)?;
    let storage = loop_storage(name, env, ctx.state);
    let index = ctx.state.temp_var("for_list_index");
    out.push_str(&format!(
        "{index}=0\nwhile [ \"${{{index}}}\" -lt \"${{{prefix}__len}}\" ]; do\n  {storage}__len=2\n  {storage}__0=\"${{{index}}}\"\n"
    ));
    let mut element = OutputBuffer::new(ctx.state.mapped);
    copy_element(
        &prefix,
        &index,
        &item,
        &format!("{storage}__1"),
        ctx.state,
        &mut element,
    );
    push_output(out, &element, 2);
    // Advance before the body so `continue` makes progress.
    out.push_str(&format!("  {index}=$(({index} + 1))\n"));
    let pair = Type::Tuple(vec![Type::Int, item]);
    let mut body_env = bind_loop_var(name, &storage, &pair, env);
    push_output(out, &compile_body(body, &mut body_env, false, ctx)?, 2);
    out.push_str("done\n");
    Ok(())
}
