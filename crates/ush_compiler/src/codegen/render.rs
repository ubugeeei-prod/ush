use alloc::string::ToString;

use anyhow::{Result, bail};

use super::{
    super::{
        ast::{Expr, Type},
        env::{Binding, EnumRegistry, Env, Storage},
    },
    calls::capture_call,
    compare::compile_compare_expr,
    functions::FunctionRegistry,
    infer,
};
use crate::{traits::TraitImplRegistry, types::AstVec as Vec, types::OutputString as String};

pub(crate) fn compile_string_expr(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<String> {
    match expr {
        Expr::Add(parts) => Ok(parts
            .iter()
            .map(|part| compile_string_fragment(part, env, functions, impls, enums))
            .collect::<Result<Vec<_>>>()?
            .join("")),
        _ => compile_string_fragment(expr, env, functions, impls, enums),
    }
}

pub(crate) fn compile_int_expr(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<String> {
    match expr {
        Expr::Int(value) => Ok(value.to_string()),
        Expr::Var(name) => Ok(format!("\"${{{}}}\"", primitive_int_var(env, name)?)),
        Expr::Try(inner) => compile_int_expr(inner, env, functions, impls, enums),
        Expr::Call(call) => capture_call(call, env, functions, impls, enums, &Type::Int),
        Expr::Add(parts) => Ok(format!(
            "$(({}))",
            int_terms(parts, env, functions, impls, enums)?.join(" + ")
        )),
        _ => bail!("unsupported integer expression"),
    }
}

pub(crate) fn compile_bool_expr(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<String> {
    match expr {
        Expr::Bool(value) => Ok(if *value { "'true'" } else { "'false'" }.into()),
        Expr::Var(name) => Ok(format!("\"${{{}}}\"", primitive_bool_var(env, name)?)),
        Expr::Try(inner) => compile_bool_expr(inner, env, functions, impls, enums),
        Expr::Compare { lhs, op, rhs } => {
            compile_compare_expr(lhs, rhs, *op, env, functions, impls, enums)
        }
        Expr::Call(call) => Ok(format!(
            "\"{}\"",
            capture_call(call, env, functions, impls, enums, &Type::Bool)?
        )),
        Expr::Field { .. } | Expr::MethodCall(_) => bail!("unsupported boolean expression"),
        _ => bail!("unsupported boolean expression"),
    }
}

pub(crate) fn compile_unit_expr(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<String> {
    match expr {
        Expr::Unit => Ok("''".into()),
        Expr::Var(name) if env.get(name).map(|binding| binding.ty.clone()) == Some(Type::Unit) => {
            Ok("''".into())
        }
        Expr::Try(inner) => compile_unit_expr(inner, env, functions, impls, enums),
        Expr::Call(call) => Ok(format!(
            "\"{}\"",
            capture_call(call, env, functions, impls, enums, &Type::Unit)?
        )),
        Expr::Field { .. } | Expr::MethodCall(_) => bail!("unsupported unit expression"),
        _ => bail!("unsupported unit expression"),
    }
}

fn compile_string_fragment(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<String> {
    match expr {
        Expr::FunctionRef(_) => bail!("function reference must be checked before lowering"),
        Expr::Quantity(..) => bail!("unit literal must be checked before lowering"),
        Expr::ShellCapture(command) => Ok(format!("\"$(\n{command}\n)\"")),
        Expr::String(value) => Ok(super::super::util::shell_value_quote(value)),
        Expr::Int(value) => Ok(super::super::util::shell_quote(&value.to_string())),
        Expr::Bool(value) => Ok(super::super::util::shell_quote(if *value {
            "true"
        } else {
            "false"
        })),
        Expr::Unit => Ok("''".into()),
        Expr::Var(name) => match env.get(name) {
            Some(binding) if binding.ty == Type::Unit => Ok("''".into()),
            Some(binding) => primitive_var(binding, name).map(|var| format!("\"${{{var}}}\"")),
            None => bail!("unknown variable: {name}"),
        },
        Expr::Try(inner) => compile_string_fragment(inner, env, functions, impls, enums),
        Expr::Add(_) => compile_string_expr(expr, env, functions, impls, enums),
        Expr::Compare { lhs, op, rhs } => {
            compile_compare_expr(lhs, rhs, *op, env, functions, impls, enums)
        }
        Expr::Call(call) => Ok(format!(
            "\"{}\"",
            capture_call(call, env, functions, impls, enums, &Type::String)?
        )),
        Expr::Template(parts) => super::text::compile_template(parts, env, functions, impls, enums),
        Expr::Field { .. }
        | Expr::MethodCall(_)
        | Expr::Pipeline(_)
        | Expr::Env(_)
        | Expr::Index { .. }
        | Expr::StatusOf(_) => {
            bail!("unsupported string expression before runtime hoisting")
        }
        Expr::AsyncBlock(_) => bail!("async blocks cannot be stringified implicitly"),
        Expr::Variant(_) => bail!("ADT values cannot be stringified implicitly"),
        Expr::Comprehension(_) | Expr::Tuple(_) | Expr::List(_) | Expr::Range { .. } => {
            bail!("structured values cannot be stringified implicitly")
        }
    }
}

fn int_terms(
    parts: &[Expr],
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Vec<String>> {
    let mut compiled = Vec::with_capacity(parts.len());
    for part in parts {
        match part {
            Expr::Int(value) => compiled.push(value.to_string()),
            Expr::Var(name) => compiled.push(primitive_int_var(env, name)?.to_string()),
            Expr::Try(inner) => match inner.as_ref() {
                Expr::Int(value) => compiled.push(value.to_string()),
                Expr::Var(name) => compiled.push(primitive_int_var(env, name)?.to_string()),
                Expr::Call(call) => compiled.push(capture_call(
                    call,
                    env,
                    functions,
                    impls,
                    enums,
                    &Type::Int,
                )?),
                Expr::Add(_) if infer(inner, env, functions, impls, enums)? == Type::Int => {
                    compiled.push(format!(
                        "({})",
                        compile_int_expr(inner, env, functions, impls, enums)?
                    ));
                }
                _ => bail!("only integer addition is supported"),
            },
            Expr::Call(call) => {
                compiled.push(capture_call(
                    call,
                    env,
                    functions,
                    impls,
                    enums,
                    &Type::Int,
                )?);
            }
            Expr::Field { .. } | Expr::MethodCall(_) => {
                bail!("unsupported integer expression before runtime hoisting")
            }
            Expr::Add(_) if infer(part, env, functions, impls, enums)? == Type::Int => {
                compiled.push(format!(
                    "({})",
                    compile_int_expr(part, env, functions, impls, enums)?
                ));
            }
            Expr::AsyncBlock(_) => bail!("async blocks cannot be used in integer expressions"),
            _ => bail!("only integer addition is supported"),
        }
    }
    Ok(compiled)
}

fn primitive_var<'a>(binding: &'a Binding, name: &str) -> Result<&'a str> {
    match &binding.storage {
        Storage::Primitive(var) => Ok(var.as_str()),
        Storage::Adt(_) => bail!("`{name}` is not a primitive value"),
        Storage::Aggregate(_) => bail!("`{name}` is a structured value"),
        Storage::Task(_, _) => bail!("`{name}` is a task handle"),
    }
}

fn primitive_int_var<'a>(env: &'a Env, name: &str) -> Result<&'a str> {
    match env.get(name) {
        Some(binding) if binding.ty == Type::Int => primitive_var(binding, name),
        Some(_) => bail!("`{name}` is not an integer"),
        None => bail!("unknown variable: {name}"),
    }
}

fn primitive_bool_var<'a>(env: &'a Env, name: &str) -> Result<&'a str> {
    match env.get(name) {
        Some(binding) if binding.ty == Type::Bool => primitive_var(binding, name),
        Some(_) => bail!("`{name}` is not a boolean"),
        None => bail!("unknown variable: {name}"),
    }
}
