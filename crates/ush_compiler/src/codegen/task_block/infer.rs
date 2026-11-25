use alloc::boxed::Box;

use anyhow::{Result, anyhow, bail};

use crate::traits::TraitImplRegistry;
use crate::types::HeapVec as Vec;
use crate::{
    ast::{IfBranch, MatchArm, Statement, StatementKind, Type},
    env::{EnumRegistry, Env},
    matching::compile_pattern,
};

use super::super::{
    calls::call_return_type, functions::FunctionRegistry, infer, shared::binding_for_name,
};
use super::support::{condition_env, iterable_item_type, mismatch};

pub(crate) fn infer_async_block_type(
    body: &[Statement],
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    let mut scope = env.clone();
    let mut returns = Vec::new();
    let mut tail = Type::Unit;
    for (index, statement) in body.iter().enumerate() {
        collect_return_types(statement, &scope, functions, impls, enums, &mut returns)?;
        if index + 1 == body.len() {
            tail = infer_statement_value(statement, &scope, functions, impls, enums)?;
        }
        apply_statement_bindings(statement, &mut scope, functions, impls, enums)?;
    }
    finish_return_type(returns, tail)
}

fn finish_return_type(returns: Vec<Type>, tail: Type) -> Result<Type> {
    let Some(expected) = returns.first() else {
        return Ok(tail);
    };
    for actual in &returns[1..] {
        if actual != expected {
            return mismatch("async block return", expected, actual);
        }
    }
    if tail != Type::Unit && tail != *expected {
        return mismatch("async block return", expected, &tail);
    }
    Ok(if tail == Type::Unit {
        expected.clone()
    } else {
        tail
    })
}

fn infer_statement_value(
    statement: &Statement,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    match &statement.kind {
        StatementKind::Expr(expr) | StatementKind::Return(expr) => {
            infer(expr, env, functions, impls, enums)
        }
        StatementKind::If {
            branch,
            returns_value,
        } if *returns_value => infer_if(branch, env, functions, impls, enums),
        StatementKind::Match {
            expr,
            arms,
            returns_value,
        } if *returns_value => infer_match(expr, arms, env, functions, impls, enums),
        _ => Ok(Type::Unit),
    }
}

fn infer_if(
    branch: &IfBranch,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    let Some(else_body) = &branch.else_body else {
        bail!("async blocks require an else branch for value-returning if expressions");
    };
    let then_ty = infer_async_block_type(
        &branch.then_body,
        &condition_env(&branch.condition, env, functions, impls, enums)?,
        functions,
        impls,
        enums,
    )?;
    let else_ty = infer_async_block_type(else_body, env, functions, impls, enums)?;
    if then_ty != else_ty {
        return mismatch("if branch", &then_ty, &else_ty);
    }
    Ok(then_ty)
}

fn infer_match(
    expr: &crate::ast::Expr,
    arms: &[MatchArm],
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    let subject = binding_for_name("__ush_match", infer(expr, env, functions, impls, enums)?);
    let mut found: Option<Type> = None;
    for (pattern, arm) in arms {
        let arm_env = compile_pattern(pattern, &subject, env, enums)?.env;
        let ty = infer_statement_value(arm, &arm_env, functions, impls, enums)?;
        if let Some(expected) = &found {
            if *expected != ty {
                return mismatch("match arm", expected, &ty);
            }
        } else {
            found = Some(ty);
        }
    }
    found.ok_or_else(|| anyhow!("match must have at least one arm"))
}

fn apply_statement_bindings(
    statement: &Statement,
    env: &mut Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<()> {
    match &statement.kind {
        StatementKind::Let { name, expr } => {
            let ty = infer(expr, env, functions, impls, enums)?;
            env.insert(name.clone(), binding_for_name(name, ty));
        }
        StatementKind::Spawn { name, call } => {
            let ty = call_return_type(&call.name, functions)?
                .ok_or_else(|| anyhow!("async bindings require a return type: {}", call.name))?;
            env.insert(
                name.clone(),
                binding_for_name(name, Type::Task(Box::new(ty))),
            );
        }
        StatementKind::Await { name, task } => {
            let binding = env
                .get(task)
                .ok_or_else(|| anyhow!("unknown task: {task}"))?;
            let Type::Task(inner) = &binding.ty else {
                bail!("await expects a task handle: {task}");
            };
            env.insert(name.clone(), binding_for_name(name, *inner.clone()));
        }
        _ => {}
    }
    Ok(())
}

fn collect_return_types(
    statement: &Statement,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    returns: &mut Vec<Type>,
) -> Result<()> {
    match &statement.kind {
        StatementKind::Return(expr) => returns.push(infer(expr, env, functions, impls, enums)?),
        StatementKind::If { branch, .. } => {
            collect_body_returns(
                &branch.then_body,
                &condition_env(&branch.condition, env, functions, impls, enums)?,
                functions,
                impls,
                enums,
                returns,
            )?;
            if let Some(body) = &branch.else_body {
                collect_body_returns(body, env, functions, impls, enums, returns)?;
            }
        }
        StatementKind::Match { expr, arms, .. } => {
            let subject =
                binding_for_name("__ush_match", infer(expr, env, functions, impls, enums)?);
            for (pattern, arm) in arms {
                let arm_env = compile_pattern(pattern, &subject, env, enums)?.env;
                collect_return_types(arm, &arm_env, functions, impls, enums, returns)?;
            }
        }
        StatementKind::While { body, .. }
        | StatementKind::Loop { body }
        | StatementKind::Scoped { body, .. } => {
            collect_body_returns(body, env, functions, impls, enums, returns)?;
        }
        StatementKind::For {
            name,
            iterable,
            body,
        } => {
            let item_ty = iterable_item_type(&infer(iterable, env, functions, impls, enums)?)?;
            let mut body_env = env.clone();
            body_env.insert(name.clone(), binding_for_name(name, item_ty));
            collect_body_returns(body, &body_env, functions, impls, enums, returns)?;
        }
        _ => {}
    }
    Ok(())
}

fn collect_body_returns(
    body: &[Statement],
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    returns: &mut Vec<Type>,
) -> Result<()> {
    for statement in body {
        collect_return_types(statement, env, functions, impls, enums, returns)?;
    }
    Ok(())
}
