use alloc::string::ToString;

use anyhow::{Result, anyhow, bail};

use super::{
    super::{
        ast::{Call, FunctionDef, Type},
        env::{CodegenState, EnumRegistry, Env},
    },
    call_support::{function_for_call, rendered_args},
    command_words::{CommandWord, command_line, list_argument},
    compile_primitive_expr,
    functions::FunctionRegistry,
    infer, rendered_call_runtime,
    shared::shell_function_name,
    stdlib,
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;
use crate::types::OutputString as String;

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_call(
    call: &Call,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    let rendered = rendered_call_runtime(
        call,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )?;
    if !call.asynchronous
        && (call.name == "std::io::echo"
            || stdlib::runs_in_parent(&call.name)
            || call.name.starts_with("$bin::"))
    {
        out.push_str(&rendered);
        if inside_function {
            super::io::push_failure_guard(out, true);
        } else {
            out.push('\n');
        }
        return Ok(());
    }
    out.push_str("( __ush_capture_return='0' __ush_return_path=''; ");
    out.push_str(&rendered);
    out.push_str(" )");
    if call.asynchronous {
        out.push_str(" &\n__ush_jobs=\"${__ush_jobs}${__ush_jobs:+ }$!\"\n");
    } else if inside_function {
        super::io::push_failure_guard(out, true);
    } else {
        out.push('\n');
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_try_call(
    call: &Call,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    let rendered = rendered_call_runtime(
        call,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )?;
    if !call.asynchronous
        && (call.name == "std::io::echo"
            || stdlib::runs_in_parent(&call.name)
            || call.name.starts_with("$bin::"))
    {
        out.push_str(&rendered);
        out.push_str(" || ");
        out.push_str(if inside_function {
            "return \"$?\""
        } else {
            "exit \"$?\""
        });
        out.push('\n');
        return Ok(());
    }
    out.push_str("( __ush_capture_return='0' __ush_return_path=''; ");
    out.push_str(&rendered);
    out.push_str(" )");
    out.push_str(" || ");
    out.push_str(if inside_function {
        "return \"$?\""
    } else {
        "exit \"$?\""
    });
    out.push('\n');
    Ok(())
}

pub(crate) fn rendered_call(
    call: &Call,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<String> {
    if call.name == "std::io::echo" {
        return super::echo::render(call, env, functions, impls, enums);
    }
    if call.name == "std::test::assert_eq" {
        return stdlib::testing::render_eq(call, env, functions, impls, enums);
    }
    if let Some(command) = crate::commands::lookup(&call.name) {
        command.validate(call.args.len())?;
        let predicate = command.output == crate::commands::Output::Predicate;
        let mut words = Vec::new();
        let mut spreads = false;
        for arg in &call.args {
            if arg.label.is_some() {
                bail!("command arguments are positional");
            }
            let ty = infer(&arg.expr, env, functions, impls, enums)?;
            if command.word == "cd" && ty != Type::String {
                bail!("cd expects a String path");
            }
            if ty == Type::String || ty == Type::Int {
                words.push(CommandWord::One(compile_primitive_expr(
                    &arg.expr, env, functions, impls, enums,
                )?));
            } else if let Some(list) = list_argument(&arg.expr, &ty, env) {
                if predicate {
                    bail!("`{}` takes single words, not a List", command.word);
                }
                spreads = true;
                words.push(CommandWord::Each(list));
            } else {
                bail!("command arguments must be String, Int or a List of them, found {ty:?}");
            }
        }
        return Ok(command_line(command.word, predicate, spreads, &words));
    }
    let def = function_for_call(&call.name, functions)?;
    let args = rendered_args(call, env, functions, impls, enums, def)?;
    let target = shell_function_name(&call.name);
    Ok(if args.is_empty() {
        target.to_string()
    } else {
        format!("{target} {}", args.join(" "))
    })
}

pub(crate) fn call_return_type(name: &str, functions: &FunctionRegistry) -> Result<Option<Type>> {
    if name == "std::io::echo" {
        return Ok(Some(Type::String));
    }
    if let Some(command) = crate::commands::lookup(name) {
        return Ok(Some(command.ty()));
    }
    Ok(function_for_call(name, functions)?.return_type.clone())
}

pub(crate) fn call_expr_type(call: &Call, functions: &FunctionRegistry) -> Result<Type> {
    call_return_type(&call.name, functions)?
        .ok_or_else(|| anyhow!("function `{}` does not return a value", call.name))
}

pub(crate) fn capture_call(
    call: &Call,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    expected: &Type,
) -> Result<String> {
    let actual = call_expr_type(call, functions)?;
    ensure_value_type(&actual)?;
    if &actual != expected {
        bail!(
            "function `{}` returns {:?}, but {:?} was required",
            call.name,
            actual,
            expected
        );
    }
    Ok(format!(
        "$(__ush_return_path='' __ush_capture_return='1' {}{})",
        rendered_call(call, env, functions, impls, enums)?,
        super::call_support::capture_fds(&call.name)
    ))
}

pub(crate) fn ensure_signature(def: &FunctionDef) -> Result<()> {
    for param in &def.params {
        ensure_value_type(&param.ty)?;
    }
    if let Some(ty) = &def.return_type {
        ensure_value_type(ty)?;
    }
    Ok(())
}

pub(crate) fn ensure_value_type(ty: &Type) -> Result<()> {
    match ty {
        Type::String | Type::Int | Type::Bool | Type::Unit => Ok(()),
        Type::Adt(_) => Ok(()),
        Type::List(t) => ensure_value_type(t),
        Type::Tuple(ts) => ts.iter().try_for_each(ensure_value_type),
        Type::Task(_) => bail!("nested task types are not supported"),
    }
}
