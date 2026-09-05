//! Error effects of the looping and branching statements.
//!
//! Split out of `analyze` so each file stays readable; the arms are
//! verbatim, they just live behind one dispatch.

use anyhow::Result;

use crate::traits::TraitImplRegistry;
use crate::{
    ast::{Statement, StatementKind},
    codegen::FunctionRegistry,
    env::{EnumRegistry, Env},
    errors::ErrorSet,
};

use super::{
    FunctionErrorRegistry, TaskErrorRegistry,
    analyze::block_errors,
    control::{analyze_condition, body_errors_with_binding, iterable_item_type},
    support::expr_errors,
};
use crate::codegen::infer;

/// The `if` / `while` / `for` / `loop` arms of [`statement_errors`].
///
/// [`statement_errors`]: super::analyze
#[allow(clippy::too_many_arguments)]
pub(super) fn loop_errors(
    statement: &Statement,
    env: &mut Env,
    tasks: &mut TaskErrorRegistry,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    match &statement.kind {
        StatementKind::If { branch, .. } => {
            let effect = analyze_condition(
                &branch.condition,
                env,
                tasks,
                functions,
                impls,
                enums,
                function_errors,
            )?;
            let mut errors = effect.errors;
            let mut then_tasks = tasks.clone();
            let mut then_env = effect.env;
            errors.extend(&block_errors(
                &branch.then_body,
                &mut then_env,
                &mut then_tasks,
                functions,
                impls,
                enums,
                function_errors,
            )?);
            if let Some(else_body) = &branch.else_body {
                let mut else_tasks = tasks.clone();
                let mut else_env = env.clone();
                errors.extend(&block_errors(
                    else_body,
                    &mut else_env,
                    &mut else_tasks,
                    functions,
                    impls,
                    enums,
                    function_errors,
                )?);
            }
            Ok(errors)
        }
        StatementKind::While { condition, body } => {
            let effect = analyze_condition(
                condition,
                env,
                tasks,
                functions,
                impls,
                enums,
                function_errors,
            )?;
            let mut errors = effect.errors;
            let mut body_tasks = tasks.clone();
            let mut body_env = effect.env;
            errors.extend(&block_errors(
                body,
                &mut body_env,
                &mut body_tasks,
                functions,
                impls,
                enums,
                function_errors,
            )?);
            Ok(errors)
        }
        StatementKind::For {
            name,
            iterable,
            body,
        } => {
            let mut errors = expr_errors(iterable, env, functions, impls, enums, function_errors)?;
            let ty = infer(iterable, env, functions, impls, enums)?;
            errors.extend(&body_errors_with_binding(
                name,
                iterable_item_type(&ty)?,
                body,
                env,
                tasks,
                functions,
                impls,
                enums,
                function_errors,
            )?);
            Ok(errors)
        }
        StatementKind::Loop { body } => {
            let mut body_tasks = tasks.clone();
            let mut body_env = env.clone();
            block_errors(
                body,
                &mut body_env,
                &mut body_tasks,
                functions,
                impls,
                enums,
                function_errors,
            )
        }
        _ => Ok(ErrorSet::default()),
    }
}
