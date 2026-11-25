//! Portable, variadic stdout output. Arguments never become a printf format.

use anyhow::{Result, ensure};

use super::{FunctionRegistry, compile_primitive_expr};
use crate::{
    ast::{Call, Type},
    env::{EnumRegistry, Env},
    traits::{TraitImplRegistry, ensure_trait},
    types::OutputString as String,
};

pub(super) fn infer(
    call: &Call,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    for arg in &call.args {
        ensure!(arg.label.is_none(), "echo arguments are positional");
        let ty = super::infer(&arg.expr, env, functions, impls, enums)?;
        ensure_trait(&ty, "Display", impls)?;
    }
    Ok(Type::String)
}

pub(super) fn render(
    call: &Call,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<String> {
    infer(call, env, functions, impls, enums)?;
    let mut format = String::new();
    let mut words = String::new();
    for (index, arg) in call.args.iter().enumerate() {
        if index > 0 {
            format.push(' ');
        }
        format.push_str("%s");
        words.push(' ');
        words.push_str(&compile_primitive_expr(
            &arg.expr, env, functions, impls, enums,
        )?);
    }
    Ok(format!("printf '{format}\\n'{words}"))
}
