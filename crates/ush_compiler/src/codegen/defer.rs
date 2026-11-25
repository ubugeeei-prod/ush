//! `defer { … }`: run a block when the enclosing function returns or the
//! script exits, whether it succeeds, fails or is stopped by a handled
//! signal. Deferred blocks run last registered first.
//!
//! The block becomes a shell function, defined where the `defer` runs, and
//! its name is pushed onto `__ush_defers`. The script's exit trap and the
//! scope of a function that defers run the list, before joining unfinished
//! tasks and removing temporary directories.

use anyhow::{Result, bail};

use super::{control::ControlContext, control_support::compile_body, shared::push_output};
use crate::{
    ast::{Statement, StatementKind as S},
    env::Env,
    sourcemap::OutputBuffer,
};

pub(super) fn compile(
    body: &[Statement],
    env: &Env,
    ctx: &mut ControlContext<'_>,
    out: &mut OutputBuffer,
) -> Result<()> {
    if let Some(place) = ctx.state.defer_blocked {
        bail!("`defer` cannot be used {place}");
    }
    if ctx.state.loop_depth > 0 {
        bail!(
            "`defer` cannot be used inside a loop: every iteration would add a block that runs \
             only at exit; defer once before the loop"
        );
    }
    if !ctx.state.scopes.is_empty() {
        bail!(
            "`defer` cannot be used inside a `cd` or `export` block: the block's directory and \
             variables are restored before the deferred block runs"
        );
    }
    check_body(body, false)?;
    let name = ctx.state.temp_var("defer");
    let blocked = ctx.state.defer_blocked.replace("inside a deferred block");
    let inside_function = core::mem::replace(&mut ctx.inside_function, true);
    let return_type = ctx.return_type.take();
    let compiled = compile_body(body, &mut env.clone(), false, ctx);
    ctx.state.defer_blocked = blocked;
    ctx.inside_function = inside_function;
    ctx.return_type = return_type;
    out.push_str(&format!("{name}() {{\n"));
    push_output(out, &compiled?, 2);
    out.push_str(&format!(
        "}}\n__ush_defers=\"{name} ${{__ush_defers:-}}\"\n"
    ));
    ctx.state.deferred = true;
    Ok(())
}

/// A deferred block runs after its scope has finished, so it cannot
/// return from that scope or leave a loop outside itself.
fn check_body(body: &[Statement], in_loop: bool) -> Result<()> {
    for statement in body {
        match &statement.kind {
            S::Return(_) => bail!(
                "line {}: a deferred block runs after its function has returned, so it cannot \
                 `return`",
                statement.line
            ),
            S::Break | S::Continue if !in_loop => bail!(
                "line {}: `break` and `continue` in a deferred block need a loop inside it",
                statement.line
            ),
            S::If { branch, .. } => {
                check_body(&branch.then_body, in_loop)?;
                if let Some(other) = &branch.else_body {
                    check_body(other, in_loop)?;
                }
            }
            S::Match { arms, .. } => {
                for (_, arm) in arms {
                    check_body(core::slice::from_ref(&**arm), in_loop)?;
                }
            }
            S::While { body, .. } | S::For { body, .. } | S::Loop { body } => {
                check_body(body, true)?;
            }
            S::Scoped { body, .. } | S::Handle { body, .. } => check_body(body, in_loop)?,
            _ => {}
        }
    }
    Ok(())
}
