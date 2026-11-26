use super::{FunctionErrorRegistry, callee_errors, support::expr_errors};
use crate::{
    ast::Call,
    codegen::FunctionRegistry,
    env::{EnumRegistry, Env},
    errors::ErrorSet,
    traits::TraitImplRegistry,
};
use anyhow::Result;

pub(super) fn call_errors(
    call: &Call,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    let mut errors = call_arg_errors(call, env, functions, impls, enums, function_errors)?;
    errors.extend(&callee_errors(&call.name, function_errors));
    Ok(errors)
}

pub(super) fn call_arg_errors(
    call: &Call,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    let mut errors = ErrorSet::default();
    for arg in &call.args {
        errors.extend(&expr_errors(
            &arg.expr,
            env,
            functions,
            impls,
            enums,
            function_errors,
        )?);
    }
    Ok(errors)
}
