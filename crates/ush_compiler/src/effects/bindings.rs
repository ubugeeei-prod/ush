use crate::{
    ast::Expr,
    codegen::{FunctionRegistry, infer},
    env::{EnumRegistry, Env},
    errors::ErrorType,
    traits::TraitImplRegistry,
};
use crate::{
    ast::Type,
    env::{Binding, Storage},
};
use anyhow::{Result, bail};
pub(super) fn binding_for_type(name: &str, ty: Type) -> Binding {
    let storage = match &ty {
        Type::Adt(_) => Storage::Adt(name.into()),
        Type::Tuple(_) | Type::List(_) => Storage::Aggregate(name.into()),
        Type::String | Type::Int | Type::Bool | Type::Unit => Storage::Primitive(name.into()),
        Type::Task(_) => Storage::Task(name.into(), Default::default()),
    };
    Binding { ty, storage }
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
