use alloc::boxed::Box;

use anyhow::Result;

use crate::traits::TraitImplRegistry;
use crate::{
    ast::{Expr, Pattern, Statement, Type},
    codegen::{FunctionRegistry, infer},
    env::{Binding, EnumRegistry, Env, Storage},
    errors::ErrorSet,
    matching::{check_exhaustive, compile_pattern},
};

use super::{
    FunctionErrorRegistry, TaskErrorRegistry, analyze::block_errors, support::expr_errors,
};

pub(super) fn value_errors(
    value: &crate::ast::MatchValue,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    match_errors(
        &value.subject,
        &value.arms,
        &mut env.clone(),
        &mut Default::default(),
        functions,
        impls,
        enums,
        function_errors,
    )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn match_errors(
    expr: &Expr,
    arms: &[(Pattern, Box<Statement>)],
    env: &mut Env,
    tasks: &mut TaskErrorRegistry,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    let mut errors = expr_errors(expr, env, functions, impls, enums, function_errors)?;
    let subject_ty = infer(expr, env, functions, impls, enums)?;
    check_exhaustive(&subject_ty, arms, enums)?;
    let subject = match subject_ty {
        Type::Adt(name) => Binding {
            ty: Type::Adt(name),
            storage: Storage::Adt("__ush_effect_match".into()),
        },
        ty => Binding {
            ty,
            storage: Storage::Primitive("__ush_effect_match".into()),
        },
    };
    for (pattern, arm) in arms {
        let plan = compile_pattern(pattern, &subject, env, enums)?;
        let mut arm_tasks = tasks.clone();
        let mut arm_env = plan.env;
        errors.extend(&block_errors(
            core::slice::from_ref(arm),
            &mut arm_env,
            &mut arm_tasks,
            functions,
            impls,
            enums,
            function_errors,
        )?);
    }
    Ok(errors)
}
