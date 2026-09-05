//! Error effects of a `try … with` block.
//!
//! A handler does not change which errors can escape: both the body
//! and every handler arm can raise, so the set is their union.

use anyhow::Result;

use crate::traits::TraitImplRegistry;
use crate::{
    ast::{EffectHandler, Statement},
    codegen::FunctionRegistry,
    env::{EnumRegistry, Env},
    errors::ErrorSet,
};

use super::{FunctionErrorRegistry, analyze::block_errors};

#[allow(clippy::too_many_arguments)]
pub(super) fn handle_errors(
    body: &[Statement],
    handlers: &[EffectHandler],
    env: &Env,
    tasks: &mut super::TaskErrorRegistry,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    let mut errors = block_errors(
        body,
        &mut env.clone(),
        tasks,
        functions,
        impls,
        enums,
        function_errors,
    )?;
    for handler in handlers {
        errors.extend(&block_errors(
            &handler.body,
            &mut env.clone(),
            tasks,
            functions,
            impls,
            enums,
            function_errors,
        )?);
    }
    Ok(errors)
}
