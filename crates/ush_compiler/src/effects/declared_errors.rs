use anyhow::{Result, bail};

use crate::{
    ast::FunctionDef,
    env::EnumRegistry,
    errors::{ErrorSet, ErrorType},
};

pub(super) fn exposed_errors(def: &FunctionDef, inferred: &ErrorSet) -> ErrorSet {
    def.declared_errors
        .clone()
        .unwrap_or_else(|| inferred.clone())
}

pub(super) fn validate_function_errors(
    def: &FunctionDef,
    inferred: &ErrorSet,
    enums: &EnumRegistry,
) -> Result<()> {
    if def.attrs.iter().any(|a| a.name == "test") {
        return Ok(());
    }
    match &def.declared_errors {
        Some(declared) => {
            validate_declared_error_types(def, declared, enums)?;
            if !inferred.is_subset_of(declared) {
                bail!(
                    "function `{}` declares `{}` but can raise `{}`",
                    def.name,
                    declared.render_union(def.return_type.as_ref()),
                    inferred.render()
                );
            }
            Ok(())
        }
        None if inferred.is_empty() => Ok(()),
        None => bail!(
            "function `{}` can raise `{}` but its signature does not declare an error set; write `-> {}`",
            def.name,
            inferred.render(),
            inferred.render_union(def.return_type.as_ref())
        ),
    }
}

fn validate_declared_error_types(
    def: &FunctionDef,
    declared: &ErrorSet,
    enums: &EnumRegistry,
) -> Result<()> {
    for error in declared.iter() {
        let ErrorType::Known(name) = error else {
            continue;
        };
        if !enums.contains_key(name) {
            bail!(
                "function `{}` declares unknown error type `{}`",
                def.name,
                name
            );
        }
    }
    Ok(())
}
