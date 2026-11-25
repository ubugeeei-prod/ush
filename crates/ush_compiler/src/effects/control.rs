use anyhow::{Result, bail};

use crate::{
    ast::{Condition, Type},
    codegen::infer,
    env::{Binding, Env, Storage},
    errors::ErrorSet,
    matching::compile_pattern,
    traits::TraitImplRegistry,
};

use super::{
    FunctionErrorRegistry, TaskErrorRegistry,
    analyze::block_errors,
    support::{binding_for_type, expr_errors},
};
use crate::codegen::FunctionRegistry;
use crate::env::EnumRegistry;

pub(super) struct ConditionEffect {
    pub errors: ErrorSet,
    pub env: Env,
    pub binds: bool,
}

pub(super) fn analyze_condition(
    condition: &Condition,
    env: &Env,
    tasks: &TaskErrorRegistry,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ConditionEffect> {
    let _ = tasks;
    match condition {
        Condition::Expr(expr) => {
            crate::codegen::process::check_condition(expr, env, functions, impls, enums)?;
            Ok(ConditionEffect {
                errors: super::status::errors(expr, env, functions, impls, enums, function_errors)?,
                env: env.clone(),
                binds: false,
            })
        }
        Condition::Not(inner) => {
            let effect =
                analyze_condition(inner, env, tasks, functions, impls, enums, function_errors)?;
            Ok(ConditionEffect {
                errors: effect.errors,
                env: env.clone(),
                binds: false,
            })
        }
        Condition::Let { pattern, expr } => {
            let errors = expr_errors(expr, env, functions, impls, enums, function_errors)?;
            let ty = infer(expr, env, functions, impls, enums)?;
            let subject = match &ty {
                Type::Adt(_) => Binding {
                    ty: ty.clone(),
                    storage: Storage::Adt("__ush_cond".into()),
                },
                Type::Tuple(_) | Type::List(_) => Binding {
                    ty: ty.clone(),
                    storage: Storage::Aggregate("__ush_cond".into()),
                },
                _ => Binding {
                    ty: ty.clone(),
                    storage: Storage::Primitive("__ush_cond".into()),
                },
            };
            let plan = compile_pattern(pattern, &subject, env, enums)?;
            Ok(ConditionEffect {
                errors,
                env: plan.env,
                binds: true,
            })
        }
        Condition::And(items) => {
            let mut errors = ErrorSet::default();
            let mut current_env = env.clone();
            let mut binds = false;
            for item in items {
                let effect = analyze_condition(
                    item,
                    &current_env,
                    tasks,
                    functions,
                    impls,
                    enums,
                    function_errors,
                )?;
                errors.extend(&effect.errors);
                current_env = effect.env;
                binds |= effect.binds;
            }
            Ok(ConditionEffect {
                errors,
                env: current_env,
                binds,
            })
        }
        Condition::Or(items) => {
            let mut errors = ErrorSet::default();
            for item in items {
                let effect =
                    analyze_condition(item, env, tasks, functions, impls, enums, function_errors)?;
                if effect.binds {
                    bail!("pattern bindings are only supported in `&&` conditions");
                }
                errors.extend(&effect.errors);
            }
            Ok(ConditionEffect {
                errors,
                env: env.clone(),
                binds: false,
            })
        }
    }
}

pub(super) fn iterable_item_type(ty: &Type) -> Result<Type> {
    match ty {
        Type::List(item) => Ok((**item).clone()),
        Type::Tuple(items) => {
            let first = items
                .first()
                .ok_or_else(|| anyhow::anyhow!("empty tuple"))?;
            if items.iter().all(|item| item == first) {
                Ok(first.clone())
            } else {
                bail!("for-in over tuples requires all items to share one type")
            }
        }
        _ => bail!("for-in expects a list, tuple, or range"),
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn body_errors_with_binding(
    name: &str,
    ty: Type,
    body: &[crate::ast::Statement],
    env: &Env,
    tasks: &TaskErrorRegistry,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    let mut body_env = env.clone();
    let mut body_tasks = tasks.clone();
    body_env.insert(name.into(), binding_for_type(name, ty));
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
