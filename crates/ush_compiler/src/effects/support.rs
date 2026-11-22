use anyhow::{Result, bail};

use crate::traits::TraitImplRegistry;
use crate::{
    ast::{Call, Expr, ExprFields, Type},
    codegen::{FunctionRegistry, infer},
    env::{Binding, EnumRegistry, Env, Storage},
    errors::{ErrorSet, ErrorType},
};

use super::{FunctionErrorRegistry, callee_errors};

pub(super) fn expr_errors(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    let mut errors = ErrorSet::default();
    match expr {
        Expr::Quantity(..)
        | Expr::String(_)
        | Expr::Int(_)
        | Expr::Bool(_)
        | Expr::Unit
        | Expr::Var(_)
        | Expr::AsyncBlock(_) => {}
        Expr::Field { base, name } => {
            if name == "await"
                && let Expr::Var(n) = &**base
                && let Some(Binding {
                    storage: Storage::Task(_, deferred),
                    ..
                }) = env.get(n)
            {
                errors.extend(deferred);
            }
            errors.extend(&expr_errors(
                base,
                env,
                functions,
                impls,
                enums,
                function_errors,
            )?);
        }
        Expr::MethodCall(call) => {
            errors.extend(&expr_errors(
                &call.receiver,
                env,
                functions,
                impls,
                enums,
                function_errors,
            )?);
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
        }
        Expr::Tuple(items) | Expr::List(items) => {
            for item in items {
                errors.extend(&expr_errors(
                    item,
                    env,
                    functions,
                    impls,
                    enums,
                    function_errors,
                )?);
            }
        }
        Expr::Range { start, end } => {
            errors.extend(&expr_errors(
                start,
                env,
                functions,
                impls,
                enums,
                function_errors,
            )?);
            errors.extend(&expr_errors(
                end,
                env,
                functions,
                impls,
                enums,
                function_errors,
            )?);
        }
        Expr::Add(parts) => {
            for part in parts {
                errors.extend(&expr_errors(
                    part,
                    env,
                    functions,
                    impls,
                    enums,
                    function_errors,
                )?);
            }
        }
        Expr::Compare { lhs, rhs, .. } => {
            errors.extend(&expr_errors(
                lhs,
                env,
                functions,
                impls,
                enums,
                function_errors,
            )?);
            errors.extend(&expr_errors(
                rhs,
                env,
                functions,
                impls,
                enums,
                function_errors,
            )?);
        }
        Expr::ShellCapture(_) => errors.insert(ErrorType::Unknown),
        Expr::Try(inner) => {
            errors.extend(&expr_errors(
                inner,
                env,
                functions,
                impls,
                enums,
                function_errors,
            )?);
        }
        Expr::Call(call) => {
            if call.name == "format" {
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
                return Ok(errors);
            }
            errors.extend(&call_errors(
                call,
                env,
                functions,
                impls,
                enums,
                function_errors,
            )?);
        }
        Expr::Variant(variant) => match &variant.fields {
            ExprFields::Unit => {}
            ExprFields::Tuple(values) => {
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
            }
            ExprFields::Struct(values) => {
                for value in values {
                    errors.extend(&expr_errors(
                        &value.expr,
                        env,
                        functions,
                        impls,
                        enums,
                        function_errors,
                    )?);
                }
            }
        },
    }
    Ok(errors)
}

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

pub(super) fn raised_error(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<ErrorType> {
    match infer(expr, env, functions, impls, enums)? {
        Type::Adt(name) => Ok(ErrorType::Known(name)),
        other => bail!("raise expects an error ADT value, found {other:?}"),
    }
}

pub(super) fn binding_for_type(name: &str, ty: Type) -> Binding {
    let storage = match &ty {
        Type::Adt(_) => Storage::Adt(name.into()),
        Type::Tuple(_) | Type::List(_) => Storage::Aggregate(name.into()),
        Type::String | Type::Int | Type::Bool | Type::Unit => Storage::Primitive(name.into()),
        Type::Task(_) => Storage::Task(name.into(), Default::default()),
    };
    Binding { ty, storage }
}
