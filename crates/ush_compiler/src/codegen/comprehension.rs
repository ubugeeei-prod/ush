use super::{FunctionRegistry, binding_for_name};
use crate::{
    ast::{Comprehension, ComprehensionClause, Type},
    env::{EnumRegistry, Env},
    traits::TraitImplRegistry,
};
use alloc::boxed::Box;
use anyhow::{Result, ensure};

pub(crate) fn infer(
    c: &Comprehension,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    let inner = scope(c, env, functions, impls, enums)?;
    Ok(Type::List(Box::new(super::infer(
        &c.value, &inner, functions, impls, enums,
    )?)))
}
pub(crate) fn scope(
    c: &Comprehension,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Env> {
    let mut inner = env.clone();
    for clause in &c.clauses {
        match clause {
            ComprehensionClause::For { name, iterable } => {
                let ty = super::infer(iterable, &inner, functions, impls, enums)?;
                let Type::List(item) = ty else {
                    anyhow::bail!("comprehension expects a list or range");
                };
                inner.insert(name.clone(), binding_for_name(name, *item));
            }
            ComprehensionClause::If(e) => ensure!(
                super::infer(e, &inner, functions, impls, enums)? == Type::Bool,
                "comprehension filter must be Bool"
            ),
        }
    }
    Ok(inner)
}
