//! Commands used for their exit status: conditions, `status_of` and
//! `allow_fail`. The command streams its stdout and stderr as it would as a
//! statement; only its status is used, and a failure does not stop the
//! script.

use anyhow::{Result, anyhow, bail};

use super::{FunctionRegistry, rendered_call_runtime, stdlib};
use crate::{
    ast::{Call, Expr, Pipeline, Type},
    commands::Output,
    env::{Binding, CodegenState, EnumRegistry, Env, Storage},
    sourcemap::OutputBuffer,
    traits::TraitImplRegistry,
    types::OutputString as String,
};

/// Something whose exit status can be tested.
pub(crate) enum Operand<'a> {
    /// A `$bin::` command other than `test`, or `std::process::exec` and
    /// `output`.
    Command(&'a Call),
    /// A function returning `()`, run in a process of its own.
    Function(&'a Call),
    Pipeline(&'a Pipeline),
    /// `task.await`; only `status_of` and `allow_fail` take one.
    Await(&'a str),
}

/// `expr` as a command, or `None` when it is an ordinary value.
pub(crate) fn operand<'a>(
    expr: &'a Expr,
    env: &Env,
    functions: &FunctionRegistry,
    allow_await: bool,
) -> Result<Option<Operand<'a>>> {
    Ok(match expr {
        Expr::Pipeline(pipeline) => Some(Operand::Pipeline(pipeline)),
        Expr::Call(call) if !call.asynchronous => call_operand(call, functions),
        Expr::Field { base, name } if allow_await && name == "await" => match &**base {
            Expr::Var(task)
                if env
                    .get(task)
                    .is_some_and(|binding| matches!(binding.ty, Type::Task(_))) =>
            {
                Some(Operand::Await(task))
            }
            _ => None,
        },
        Expr::Try(inner) if operand(inner, env, functions, allow_await)?.is_some() => bail!(
            "`?` would stop the script when the command fails, but its status is being tested; \
             remove the `?`"
        ),
        _ => None,
    })
}

fn call_operand<'a>(call: &'a Call, functions: &FunctionRegistry) -> Option<Operand<'a>> {
    if let Some(command) = crate::commands::lookup(&call.name) {
        return (command.output != Output::Predicate).then_some(Operand::Command(call));
    }
    if matches!(
        call.name.as_str(),
        "std::process::exec" | "std::process::output" | "std::io::echo"
    ) {
        return Some(Operand::Command(call));
    }
    if call.name == "format" || stdlib::pipe::misuse(&call.name).is_some() {
        return None;
    }
    let def = functions.get(&call.name)?;
    (def.return_type == Some(Type::Unit)).then_some(Operand::Function(call))
}

/// A condition is a `Bool`, or a command whose success it tests.
pub(crate) fn check_condition(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<()> {
    match operand(expr, env, functions, false)? {
        Some(Operand::Pipeline(pipeline)) => {
            super::pipeline::check_statement(pipeline, env, functions, impls, enums)
        }
        Some(_) => Ok(()),
        None => match super::infer(expr, env, functions, impls, enums)? {
            Type::Bool => Ok(()),
            ty => bail!(
                "a condition must be a Bool or a command, found {}; compare the value, \
                 for example `value != \"\"`",
                ty.render()
            ),
        },
    }
}

/// Shell whose exit status is the operand's. Arguments are evaluated into
/// `out` first.
#[allow(clippy::too_many_arguments)]
pub(crate) fn render(
    operand: Operand<'_>,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<String> {
    let call = |call: &Call, state: &mut CodegenState, out: &mut OutputBuffer| {
        rendered_call_runtime(
            call,
            env,
            functions,
            impls,
            enums,
            state,
            inside_function,
            out,
        )
    };
    Ok(match operand {
        Operand::Command(command) => call(command, state, out)?,
        Operand::Function(function) if stdlib::runs_in_parent(&function.name) => {
            call(function, state, out)?
        }
        Operand::Function(function) => format!(
            "( __ush_capture_return='0' __ush_return_path=''; {} )",
            call(function, state, out)?
        ),
        Operand::Pipeline(pipeline) => super::pipeline::compile_status(
            pipeline,
            env,
            functions,
            impls,
            enums,
            state,
            inside_function,
            out,
        )?,
        Operand::Await(task) => {
            let Some(Binding {
                storage: Storage::Task(prefix, _),
                ..
            }) = env.get(task)
            else {
                bail!("await expects a task handle: {task}");
            };
            format!(
                "{{ [ \"${{{prefix}__awaited}}\" = '1' ] || {{ __ush_wait_job \"${{{prefix}__pid}}\" \"${{{prefix}__result}}.status\" && {prefix}__awaited='1'; }}; }}"
            )
        }
    })
}

/// `status_of command`: a fresh `Int` variable holding its exit status.
#[allow(clippy::too_many_arguments)]
pub(super) fn hoist_status(
    command: &Expr,
    env: &mut Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<Expr> {
    let operand = operand(command, env, functions, true)?.ok_or_else(not_a_command("status_of"))?;
    let test = render(
        operand,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )?;
    let temp = state.temp_var("status");
    out.push_str(&format!("if {test}; then {temp}=0; else {temp}=$?; fi\n"));
    env.insert(
        temp.clone(),
        Binding {
            ty: Type::Int,
            storage: Storage::Primitive(temp.clone()),
        },
    );
    Ok(Expr::Var(temp))
}

/// `allow_fail command`: run it and carry on whatever its status.
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
    let operand =
        operand(command, env, functions, true)?.ok_or_else(not_a_command("allow_fail"))?;
    let test = render(
        operand,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )?;
    out.push_str(&format!("{test} || :\n"));
    Ok(())
}

fn not_a_command(keyword: &'static str) -> impl FnOnce() -> anyhow::Error {
    move || {
        anyhow!("`{keyword}` runs a command, a pipeline, a function returning `()` or `task.await`")
    }
}
