//! Capture a call's value into a fresh variable before the statement
//! that uses it.

use super::{
    super::{
        ast::{Call, Expr},
        env::{Binding, CodegenState, EnumRegistry, Env, Storage},
    },
    FunctionRegistry,
    calls::{call_expr_type, ensure_value_type, rendered_call},
    runtime_support::{FailureMode, hoist_call},
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;
use anyhow::Result;

#[allow(clippy::too_many_arguments)]
pub(super) fn hoist_call_capture(
    call: &Call,
    env: &mut Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    mode: FailureMode,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<Expr> {
    let prepared = hoist_call(
        call,
        env,
        functions,
        impls,
        enums,
        state,
        mode,
        inside_function,
        out,
    )?;
    let ty = call_expr_type(&prepared, functions)?;
    ensure_value_type(&ty)?;
    let rendered = rendered_call(&prepared, env, functions, impls, enums)?;

    let temp = state.temp_var("value");
    out.push_str(&temp);
    out.push_str("=\"$(__ush_return_path='' __ush_capture_return='1' ");
    out.push_str(&rendered);
    out.push_str(super::call_support::capture_fds(&prepared.name));
    out.push_str(")\"");
    // Bash clears errexit in command substitutions. Never rely on it to
    // propagate a failed value-producing call through a captured function.
    out.push_str(" || ");
    out.push_str(if inside_function {
        "return \"$?\""
    } else {
        "exit \"$?\""
    });
    out.push('\n');
    env.insert(
        temp.clone(),
        Binding {
            ty,
            storage: Storage::Primitive(temp.clone()),
        },
    );
    Ok(Expr::Var(temp))
}
