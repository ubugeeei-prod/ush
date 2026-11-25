//! Error rows of pipelines, `export` and scoped blocks.

use anyhow::Result;

use crate::traits::TraitImplRegistry;
use crate::{
    ast::{Expr, Pipeline, Scope, Statement, StatementKind},
    codegen::FunctionRegistry,
    env::{EnumRegistry, Env},
    errors::{ErrorSet, ErrorType},
};

use super::{
    FunctionErrorRegistry, TaskErrorRegistry, analyze::block_errors, support::expr_errors,
};

/// Any stage can exit unsuccessfully, so a pipeline can always fail with
/// `unknown`, on top of what its stages' calls and arguments raise.
pub(super) fn pipeline_errors(
    pipeline: &Pipeline,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    let mut errors = ErrorSet::default();
    errors.insert(ErrorType::Unknown);
    for stage in pipeline.exprs() {
        errors.extend(&expr_errors(
            stage,
            env,
            functions,
            impls,
            enums,
            function_errors,
        )?);
    }
    Ok(errors)
}

/// `export` raises what its values raise. A `cd` block can also fail to
/// enter or to leave its directory. Bindings made inside a block stay
/// inside it, as they do for `if` bodies.
pub(super) fn scoped_errors(
    statement: &Statement,
    env: &Env,
    tasks: &TaskErrorRegistry,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    let (values, body): (alloc::vec::Vec<&Expr>, &[Statement]) = match &statement.kind {
        StatementKind::Export(bindings) => (bindings.iter().map(|b| &b.value).collect(), &[]),
        StatementKind::Scoped { scope, body } => (scope.exprs().collect(), body),
        _ => return Ok(ErrorSet::default()),
    };
    let mut errors = ErrorSet::default();
    if let StatementKind::Scoped {
        scope: Scope::Cd(_),
        ..
    } = &statement.kind
    {
        errors.insert(ErrorType::Unknown);
    }
    for value in values {
        errors.extend(&expr_errors(
            value,
            env,
            functions,
            impls,
            enums,
            function_errors,
        )?);
    }
    errors.extend(&block_errors(
        body,
        &mut env.clone(),
        &mut tasks.clone(),
        functions,
        impls,
        enums,
        function_errors,
    )?);
    Ok(errors)
}
