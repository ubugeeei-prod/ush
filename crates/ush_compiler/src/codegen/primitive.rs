use alloc::boxed::Box;

use anyhow::{Result, anyhow, bail};

use super::super::{
    ast::{Expr, ExprFields, Type, VariantExpr, VariantFields},
    env::{EnumRegistry, Env, lookup_variant},
};
use super::{
    compare::infer_compare,
    functions::FunctionRegistry,
    methods::{infer_field_expr, infer_format_call, infer_method_call},
    render::{compile_bool_expr, compile_int_expr, compile_string_expr, compile_unit_expr},
    task_block::infer_async_block_type,
};
use crate::traits::{TraitImplRegistry, ensure_trait};
use crate::types::OutputString as String;

pub(crate) fn infer(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    match expr {
        Expr::ShellCapture(_) | Expr::String(_) => Ok(Type::String),
        Expr::Quantity(..) => bail!("unit literal must be checked before lowering"),
        Expr::Int(_) => Ok(Type::Int),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Unit => Ok(Type::Unit),
        Expr::Var(name) => env
            .get(name)
            .map(|binding| binding.ty.clone())
            .ok_or_else(|| anyhow!("unknown variable: {name}")),
        Expr::Field { base, name } => infer_field_expr(base, name, env, functions, impls, enums),
        Expr::Try(inner) => infer(inner, env, functions, impls, enums),
        Expr::Add(parts) => infer_add(parts, env, functions, impls, enums),
        Expr::Compare { lhs, op, rhs } => {
            infer_compare(lhs, rhs, *op, env, functions, impls, enums)
        }
        Expr::Call(call) if call.name == "format" => {
            infer_format_call(call, env, functions, impls, enums)
        }
        Expr::Call(call) => super::calls::call_expr_type(call, functions),
        Expr::MethodCall(call) => infer_method_call(call, env, functions, impls, enums),
        Expr::Variant(variant) => infer_variant(variant, env, functions, impls, enums),
        Expr::Tuple(items) => items
            .iter()
            .map(|item| infer(item, env, functions, impls, enums))
            .collect::<Result<_>>()
            .map(Type::Tuple),
        Expr::List(items) => infer_list(items, env, functions, impls, enums),
        Expr::Range { start, end } => {
            if infer(start, env, functions, impls, enums)? != Type::Int
                || infer(end, env, functions, impls, enums)? != Type::Int
            {
                bail!("ranges require integer bounds");
            }
            Ok(Type::List(Box::new(Type::Int)))
        }
        Expr::AsyncBlock(body) => infer_async_block_type(body, env, functions, impls, enums)
            .map(|ty| Type::Task(Box::new(ty))),
    }
}

pub(crate) fn compile_primitive_expr(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<String> {
    match infer(expr, env, functions, impls, enums)? {
        Type::String => compile_string_expr(expr, env, functions, impls, enums),
        Type::Int => compile_int_expr(expr, env, functions, impls, enums),
        Type::Bool => compile_bool_expr(expr, env, functions, impls, enums),
        Type::Unit => compile_unit_expr(expr, env, functions, impls, enums),
        Type::Adt(name) => bail!("cannot use {name} as a primitive shell value"),
        Type::Tuple(_) | Type::List(_) => bail!("structured values are not primitive shell values"),
        Type::Task(_) => bail!("task handles cannot be used as primitive values"),
    }
}

fn infer_add(
    parts: &[Expr],
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    let mut saw_string = false;
    let mut saw_non_int = false;
    for part in parts {
        match infer(part, env, functions, impls, enums)? {
            Type::String => saw_string = true,
            Type::Int => {}
            Type::Unit => saw_non_int = true,
            Type::Bool => bail!("booleans cannot participate in `+` expressions"),
            Type::Adt(name) => bail!("ADT `{name}` cannot participate in `+` expressions"),
            Type::Tuple(_) => bail!("tuple values cannot participate in `+` expressions"),
            Type::List(_) => bail!("list values cannot participate in `+` expressions"),
            Type::Task(_) => bail!("task handles cannot participate in `+` expressions"),
        }
    }
    let ty = if saw_string {
        Type::String
    } else if saw_non_int {
        bail!("unit values can only participate in string concatenation");
    } else {
        Type::Int
    };
    ensure_trait(&ty, "Add", impls)?;
    Ok(ty)
}

fn infer_list(
    items: &[Expr],
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    let Some(first) = items.first() else {
        bail!("lists must contain at least one item");
    };
    let item_ty = infer(first, env, functions, impls, enums)?;
    for item in &items[1..] {
        let actual = infer(item, env, functions, impls, enums)?;
        if actual != item_ty {
            bail!("list item type mismatch: expected {item_ty:?}, found {actual:?}");
        }
    }
    Ok(Type::List(Box::new(item_ty)))
}

fn infer_variant(
    variant: &VariantExpr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    let def = lookup_variant(enums, &variant.enum_name, &variant.variant_name)?;
    match (&def.fields, &variant.fields) {
        (VariantFields::Unit, ExprFields::Unit) => {}
        (VariantFields::Tuple(types), ExprFields::Tuple(values)) if types.len() == values.len() => {
            for (ty, expr) in types.iter().zip(values) {
                if infer(expr, env, functions, impls, enums)? != *ty {
                    bail!(
                        "tuple constructor type mismatch for {}::{}",
                        variant.enum_name,
                        variant.variant_name
                    );
                }
            }
        }
        (VariantFields::Struct(fields), ExprFields::Struct(values)) => {
            for value in values {
                if !fields.iter().any(|field| field.name == value.name) {
                    bail!("unknown field {}", value.name);
                }
            }
            for field in fields {
                let value = values
                    .iter()
                    .find(|value| value.name == field.name)
                    .ok_or_else(|| anyhow!("missing field {}", field.name))?;
                if infer(&value.expr, env, functions, impls, enums)? != field.ty {
                    bail!(
                        "struct constructor type mismatch for {}::{}",
                        variant.enum_name,
                        variant.variant_name
                    );
                }
            }
        }
        _ => bail!(
            "variant field shape mismatch for {}::{}",
            variant.enum_name,
            variant.variant_name
        ),
    }
    Ok(Type::Adt(variant.enum_name.clone()))
}
