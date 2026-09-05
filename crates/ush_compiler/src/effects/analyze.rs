use alloc::boxed::Box;

use anyhow::{Result, anyhow, bail};

use crate::traits::TraitImplRegistry;
use crate::{
    ast::{Expr, Statement, StatementKind, Type},
    codegen::{FunctionRegistry, infer, infer_async_block_type},
    env::{EnumRegistry, Env},
    errors::{ErrorSet, ErrorType},
};

use super::{
    FunctionErrorRegistry, TaskErrorRegistry,
    matching::match_errors,
    support::{binding_for_type, call_arg_errors, call_errors, expr_errors, raised_error},
};

pub(super) fn block_errors(
    statements: &[Statement],
    env: &mut Env,
    tasks: &mut TaskErrorRegistry,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    let mut errors = ErrorSet::default();
    for statement in statements {
        let current = statement_errors(
            statement,
            env,
            tasks,
            functions,
            impls,
            enums,
            function_errors,
        )?;
        errors.extend(&current);
    }
    Ok(errors)
}

fn statement_errors(
    statement: &Statement,
    env: &mut Env,
    tasks: &mut TaskErrorRegistry,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    match &statement.kind {
        StatementKind::Use(_)
        | StatementKind::Enum(_)
        | StatementKind::Trait(_)
        | StatementKind::Impl(_)
        | StatementKind::Function(_)
        | StatementKind::Effect(_) => Ok(ErrorSet::default()),
        StatementKind::Handle { body, handlers } => super::handled::handle_errors(
            body,
            handlers,
            env,
            tasks,
            functions,
            impls,
            enums,
            function_errors,
        ),
        StatementKind::Alias { value, .. } => {
            expr_errors(value, env, functions, impls, enums, function_errors)
        }
        StatementKind::Let {
            name,
            expr: Expr::AsyncBlock(body),
        } => {
            let mut task_env = env.clone();
            let mut task_errors = tasks.clone();
            let deferred = block_errors(
                body,
                &mut task_env,
                &mut task_errors,
                functions,
                impls,
                enums,
                function_errors,
            )?;
            let ty = infer_async_block_type(body, env, functions, impls, enums)?;
            env.insert(
                name.clone(),
                binding_for_type(name, Type::Task(Box::new(ty))),
            );
            tasks.insert(name.clone(), deferred);
            Ok(ErrorSet::default())
        }
        StatementKind::Let { name, expr } => {
            let errors = expr_errors(expr, env, functions, impls, enums, function_errors)?;
            let ty = infer(expr, env, functions, impls, enums)?;
            env.insert(name.clone(), binding_for_type(name, ty));
            Ok(errors)
        }
        StatementKind::Spawn { name, call } => {
            let errors = call_arg_errors(call, env, functions, impls, enums, function_errors)?;
            let def = functions
                .get(&call.name)
                .ok_or_else(|| anyhow!("unknown function: {}", call.name))?;
            let ty = def
                .return_type
                .clone()
                .ok_or_else(|| anyhow!("async bindings require a return type: {}", call.name))?;
            env.insert(
                name.clone(),
                binding_for_type(name, Type::Task(Box::new(ty))),
            );
            tasks.insert(
                name.clone(),
                function_errors.get(&call.name).cloned().unwrap_or_default(),
            );
            Ok(errors)
        }
        StatementKind::Await { name, task } => {
            let binding = env
                .get(task)
                .ok_or_else(|| anyhow!("unknown task: {task}"))?;
            let Type::Task(inner) = &binding.ty else {
                bail!("await expects a task handle: {task}");
            };
            let errors = tasks.get(task).cloned().unwrap_or_default();
            env.insert(name.clone(), binding_for_type(name, *inner.clone()));
            Ok(errors)
        }
        StatementKind::Print(expr) | StatementKind::Expr(expr) | StatementKind::Return(expr) => {
            expr_errors(expr, env, functions, impls, enums, function_errors)
        }
        StatementKind::Shell(expr) | StatementKind::TryShell(expr) => {
            let mut errors = expr_errors(expr, env, functions, impls, enums, function_errors)?;
            errors.insert(ErrorType::Unknown);
            Ok(errors)
        }
        StatementKind::Call(call) | StatementKind::TryCall(call) => {
            call_errors(call, env, functions, impls, enums, function_errors)
        }
        StatementKind::Raise(expr) => {
            let mut errors = expr_errors(expr, env, functions, impls, enums, function_errors)?;
            errors.insert(raised_error(expr, env, functions, impls, enums)?);
            Ok(errors)
        }
        StatementKind::Match { expr, arms, .. } => match_errors(
            expr,
            arms,
            env,
            tasks,
            functions,
            impls,
            enums,
            function_errors,
        ),
        StatementKind::If { .. }
        | StatementKind::While { .. }
        | StatementKind::For { .. }
        | StatementKind::Loop { .. } => super::loops::loop_errors(
            statement,
            env,
            tasks,
            functions,
            impls,
            enums,
            function_errors,
        ),
        StatementKind::Break | StatementKind::Continue => Ok(ErrorSet::default()),
    }
}
