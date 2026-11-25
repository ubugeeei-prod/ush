use anyhow::{Result, bail};

use crate::{
    ast::FunctionDef,
    env::EnumRegistry,
    errors::{ErrorSet, ErrorType},
};

/// The errors callers see: the written row, or the inferred one when
/// the signature has no row.
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
                // `-> String / ()` declares no errors at all.
                let row = match declared.is_empty() {
                    true => format!(
                        "{} / ()",
                        def.return_type
                            .as_ref()
                            .map_or("()".into(), |ty| ty.render())
                    ),
                    false => declared
                        .render_union(def.return_type.as_ref())
                        .as_str()
                        .into(),
                };
                bail!(
                    "function `{}` declares `{row}` but can raise `{}`; widen the row or drop it to infer one",
                    def.name,
                    inferred.render()
                );
            }
            Ok(())
        }
        // No row written: the inferred row is the function's row.
        None => Ok(()),
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
