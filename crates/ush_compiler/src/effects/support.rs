use anyhow::Result;

use crate::traits::TraitImplRegistry;
use crate::{
    ast::{Call, Expr, ExprFields},
    codegen::FunctionRegistry,
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
        Expr::FunctionRef(_)
        | Expr::Quantity(..)
        | Expr::String(_)
        | Expr::Int(_)
        | Expr::Bool(_)
        | Expr::Unit
        | Expr::Var(_)
        | Expr::AsyncBlock(_) => {}
        Expr::Comprehension(c) => errors.extend(&super::comprehension::errors(
            c,
            env,
            functions,
            impls,
            enums,
            function_errors,
        )?),
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
            if crate::codegen::list_method_fails(call, env, functions, impls, enums) {
                errors.insert(ErrorType::Unknown);
            }
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
        Expr::Tuple(items) | Expr::List(items) | Expr::Template(items) => {
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
        Expr::Index { base, index } => {
            errors.insert(ErrorType::Unknown);
            for part in [base, index] {
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
        Expr::Compare { lhs, rhs, .. }
        | Expr::Range {
            start: lhs,
            end: rhs,
        } => {
            for part in [lhs, rhs] {
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
        Expr::ShellCapture(_) | Expr::Env(_) => errors.insert(ErrorType::Unknown),
        Expr::StatusOf(command) => errors.extend(&super::status::errors(
            command,
            env,
            functions,
            impls,
            enums,
            function_errors,
        )?),
        Expr::Pipeline(p) => errors.extend(&super::shell::pipeline_errors(
            p,
            env,
            functions,
            impls,
            enums,
            function_errors,
        )?),
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

pub(super) use super::bindings::{binding_for_type, raised_error};
