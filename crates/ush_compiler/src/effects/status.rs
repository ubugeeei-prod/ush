//! Error rows of commands used for their exit status. A condition,
//! `status_of` or `allow_fail` handles the command's own failure, so only
//! the evaluation of its arguments can still raise.

use anyhow::Result;

use crate::traits::TraitImplRegistry;
use crate::{
    ast::Expr,
    codegen::{
        FunctionRegistry,
        process::{Operand, operand},
    },
    env::{EnumRegistry, Env},
    errors::ErrorSet,
};

use super::{
    FunctionErrorRegistry,
    support::{call_arg_errors, expr_errors},
};

pub(super) fn errors(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    let args = |expr: &Expr| match expr {
        Expr::Call(call) => call_arg_errors(call, env, functions, impls, enums, function_errors),
        other => expr_errors(other, env, functions, impls, enums, function_errors),
    };
    match operand(expr, env, functions, true)? {
        Some(Operand::Command(_) | Operand::Function(_)) => args(expr),
        Some(Operand::Pipeline(pipeline)) => {
            let mut errors = ErrorSet::default();
            for stage in pipeline.exprs() {
                errors.extend(&args(stage)?);
            }
            Ok(errors)
        }
        Some(Operand::Await(_)) => Ok(ErrorSet::default()),
        None => expr_errors(expr, env, functions, impls, enums, function_errors),
    }
}
