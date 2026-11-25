//! What a task handle offers besides `.await`: `task.pid` and
//! `task.kill()`.

use anyhow::{Result, bail};

use super::{FunctionRegistry, compile_primitive_expr, infer};
use crate::{
    ast::{Expr, MethodCall, Type},
    env::{Binding, EnumRegistry, Env, Storage},
    sourcemap::OutputBuffer,
    traits::TraitImplRegistry,
    types::OutputString as String,
};

/// Signals `kill` accepts, by name without the `SIG` prefix.
pub(crate) const SIGNALS: &[&str] = &[
    "HUP", "INT", "QUIT", "KILL", "USR1", "USR2", "ALRM", "TERM", "STOP", "CONT",
];

/// The type of `task.name`, when `name` is a task member.
pub(crate) fn infer_field(base: &Type, name: &str) -> Option<Type> {
    (matches!(base, Type::Task(_)) && name == "pid").then_some(Type::Int)
}

/// `task.pid` reads the process ID recorded when the task started.
pub(crate) fn pid(task: &str, env: &Env) -> Option<Binding> {
    let Some(Binding {
        ty: Type::Task(_),
        storage: Storage::Task(prefix, _),
    }) = env.get(task)
    else {
        return None;
    };
    Some(Binding {
        ty: Type::Int,
        storage: Storage::Primitive(format!("{prefix}__pid").into()),
    })
}

/// The type of `task.kill(…)`, or `None` for any other method.
pub(crate) fn infer_method(
    call: &MethodCall,
    receiver: &Type,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Option<Type>> {
    if !matches!(receiver, Type::Task(_)) || call.method != "kill" {
        return Ok(None);
    }
    match call.args.as_slice() {
        [] => {}
        [arg] if arg.label.is_none() => {
            if infer(&arg.expr, env, functions, impls, enums)? != Type::String {
                bail!("`kill` takes a signal name such as \"INT\"");
            }
            if let Expr::String(signal) = &arg.expr
                && !SIGNALS.contains(&signal.as_str())
            {
                bail!(
                    "unknown signal `{signal}`; use one of {}",
                    SIGNALS.join(", ")
                );
            }
        }
        _ => bail!("`kill` takes at most one signal name"),
    }
    Ok(Some(Type::Unit))
}

/// Whether `receiver.method` is `task.kill` on a task handle.
pub(crate) fn is_kill(receiver: &Expr, method: &str, env: &Env) -> bool {
    method == "kill"
        && matches!(receiver, Expr::Var(task)
            if env.get(task).is_some_and(|b| matches!(b.ty, Type::Task(_))))
}

/// `task.kill(…)` in value position, with its arguments already hoisted:
/// signal the task and evaluate to `()`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_kill(
    receiver: &Expr,
    method: &str,
    args: &[Expr],
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<Option<String>> {
    if !is_kill(receiver, method, env) {
        return Ok(None);
    }
    let signal = match args {
        [] => "TERM".into(),
        [signal] => compile_primitive_expr(signal, env, functions, impls, enums)?,
        _ => bail!("`kill` takes at most one signal name"),
    };
    push_kill(receiver, &signal, env, inside_function, out)?;
    Ok(Some("''".into()))
}

/// Signal the task and every process it started.
pub(crate) fn push_kill(
    receiver: &Expr,
    signal: &str,
    env: &Env,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    let Expr::Var(task) = receiver else {
        bail!("`kill` needs a task handle");
    };
    let Some(Binding {
        storage: Storage::Task(prefix, _),
        ..
    }) = env.get(task)
    else {
        bail!("`kill` needs a task handle: {task}");
    };
    out.push_str(&format!(
        "__ush_task_kill \"${{{prefix}__pid}}\" {signal} \"${{{prefix}__result}}.status\""
    ));
    super::io::push_failure_guard(out, inside_function);
    Ok(())
}
