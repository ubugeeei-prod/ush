//! `export` statements, and the `cd` and `export` blocks that scope a
//! change to their body.
//!
//! A block saves what it changes, runs its body in the current shell and
//! restores the saved state. Running the body in place keeps assignments
//! to outer bindings visible after the block, as in every other block.
//! The restore runs when the body finishes and before a `break` or
//! `continue` leaves it. Every other early exit ends the process scope
//! the change was made in: `fail`, `?` and failing commands exit the
//! script at top level, and a function body runs in its own subshell.

use anyhow::{Result, bail};

use super::{
    compile_runtime_primitive_expr, control::ControlContext, control_support::compile_body, infer,
    io::push_failure_guard, shared::push_output,
};
use crate::{
    ast::{EnvBinding, Expr, Scope, Statement, Type},
    env::{CodegenState, Env, ScopeFrame},
    sourcemap::OutputBuffer,
    types::{HeapVec as Vec, OutputString as String},
};

pub(crate) fn compile_export(
    bindings: &[EnvBinding],
    env: &Env,
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<()> {
    for (binding, value) in bindings.iter().zip(env_values(bindings, env, ctx, out)?) {
        out.push_str(&format!("export {}={value}\n", binding.name));
    }
    Ok(())
}

pub(crate) fn compile_scoped(
    scope: &Scope,
    body: &[Statement],
    env: &Env,
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<()> {
    let restore = match scope {
        Scope::Cd(path) => enter_directory(path, env, ctx, out)?,
        Scope::Env(bindings) => enter_environment(bindings, env, ctx, out)?,
    };
    ctx.state.scopes.push(ScopeFrame {
        loop_depth: ctx.state.loop_depth,
        restore: restore.clone(),
    });
    let compiled = compile_body(body, &mut env.clone(), false, ctx);
    ctx.state.scopes.pop();
    push_output(out, &compiled?, 2);
    out.push_str(&restore);
    Ok(())
}

/// Restore every block entered inside the innermost loop, innermost first.
pub(crate) fn unwind_to_loop(state: &CodegenState, out: &mut OutputBuffer) {
    let inside_loop = state.scopes.iter().rev();
    for frame in inside_loop.take_while(|frame| frame.loop_depth == state.loop_depth) {
        out.push_str(&frame.restore);
    }
}

fn enter_directory(
    path: &Expr,
    env: &Env,
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<String> {
    let path = string_value(path, "a `cd` block's directory", env, ctx, out)?;
    let saved = ctx.state.temp_var("cwd");
    out.push_str(&format!("{saved}=$PWD\nush_fn_std__process__chdir {path}"));
    push_failure_guard(out, ctx.inside_function);
    let leave = if ctx.inside_function {
        "return"
    } else {
        "exit"
    };
    Ok(format!(
        "cd \"${saved}\" || {{ __ush_status=$?; __ush_report \"$__ush_status\" \
         'cannot return to the directory the cd block started in'; {leave} \"$__ush_status\"; }}\n"
    ))
}

fn enter_environment(
    bindings: &[EnvBinding],
    env: &Env,
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<String> {
    let values = env_values(bindings, env, ctx, out)?;
    let mut restore = String::new();
    for (binding, value) in bindings.iter().zip(values) {
        let name = &binding.name;
        let id = ctx.state.next_handler_id();
        let (had, saved) = (
            format!("__ush_had_{name}_{id}"),
            format!("__ush_saved_{name}_{id}"),
        );
        out.push_str(&format!("{had}=${{{name}+x}} {saved}=${{{name}-}}\n"));
        out.push_str(&format!("export {name}={value}\n"));
        restore.push_str(&format!(
            "if [ -n \"${had}\" ]; then export {name}=\"${saved}\"; else unset {name}; fi\n"
        ));
    }
    Ok(restore)
}

/// Evaluate every value before any variable changes, so a later value
/// never observes an earlier binding of the same statement.
fn env_values(
    bindings: &[EnvBinding],
    env: &Env,
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<Vec<String>> {
    bindings
        .iter()
        .map(|binding| {
            check_env_name(&binding.name, env)?;
            let what = format!("the value of `{}`", binding.name);
            string_value(&binding.value, &what, env, ctx, out)
        })
        .collect()
}

fn check_env_name(name: &str, env: &Env) -> Result<()> {
    if name.starts_with("__ush") {
        bail!("`{name}` is reserved for the generated script");
    }
    if matches!(name, "PWD" | "OLDPWD") {
        bail!("`{name}` is maintained by `cd`; use a `cd` block to change directory");
    }
    if name == "IFS" {
        bail!("`IFS` controls word splitting in the generated script and cannot be exported");
    }
    if env.contains_key(name) {
        bail!(
            "`{name}` is a ush binding in this scope; exporting it would change its value \
             behind the type checker, so give the environment variable another name"
        );
    }
    Ok(())
}

fn string_value(
    expr: &Expr,
    what: &str,
    env: &Env,
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<String> {
    let ty = infer(expr, env, ctx.functions, ctx.impls, ctx.enums)?;
    if ty != Type::String {
        bail!("{what} must be a String, found {ty:?}");
    }
    compile_runtime_primitive_expr(
        expr,
        env,
        ctx.functions,
        ctx.impls,
        ctx.enums,
        ctx.state,
        ctx.inside_function,
        out,
    )
}
