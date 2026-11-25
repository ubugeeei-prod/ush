use super::{
    FunctionErrorRegistry,
    support::{binding_for_type, expr_errors},
};
use crate::{
    ast::{Comprehension, Type},
    codegen::{FunctionRegistry, infer},
    env::{EnumRegistry, Env},
    errors::ErrorSet,
    traits::TraitImplRegistry,
};
use anyhow::{Result, bail};
pub(super) fn errors(
    c: &Comprehension,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    let mut errors = ErrorSet::default();
    let mut inner = env.clone();
    for clause in &c.clauses {
        match clause {
            crate::ast::ComprehensionClause::For { name, iterable } => {
                errors.extend(&expr_errors(
                    iterable,
                    &inner,
                    functions,
                    impls,
                    enums,
                    function_errors,
                )?);
                let Type::List(item) = infer(iterable, &inner, functions, impls, enums)? else {
                    bail!("comprehension expects a list or range");
                };
                inner.insert(name.clone(), binding_for_type(name, *item));
            }
            crate::ast::ComprehensionClause::If(e) => errors.extend(&expr_errors(
                e,
                &inner,
                functions,
                impls,
                enums,
                function_errors,
            )?),
        }
    }
    errors.extend(&expr_errors(
        &c.value,
        &inner,
        functions,
        impls,
        enums,
        function_errors,
    )?);
    Ok(errors)
}
