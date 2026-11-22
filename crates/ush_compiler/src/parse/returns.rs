use anyhow::{Result, anyhow, bail};

use crate::{
    ast::{FunctionDef, Statement, StatementKind, Type},
    effects::Effect,
    errors::{ErrorSet, ErrorType},
    types::{AstString as String, HeapVec as Vec, Set},
    util::{is_identifier, parse_type},
};

pub(super) fn parse_function_return(source: &str) -> Result<(Option<Type>, Option<Vec<String>>)> {
    if source.is_empty() {
        return Ok((Some(Type::Unit), None));
    }
    let suffix = source
        .strip_prefix("->")
        .ok_or_else(|| anyhow!("invalid function signature suffix: {source}"))?
        .trim();
    let (value, row) = match suffix.split_once('/') {
        Some((value, effects)) => (value.trim(), Some(parse_row(effects.trim())?)),
        None => (suffix, None),
    };
    if value.contains('!') {
        bail!("write the success type first, followed by `/ ErrorType`");
    }
    Ok((
        Some(parse_type(value).ok_or_else(|| anyhow!("invalid return type: {value}"))?),
        row,
    ))
}

fn parse_row(row: &str) -> Result<Vec<String>> {
    let mut names = Vec::new();
    if row == "()" {
        return Ok(names);
    }
    for part in row.split(',').map(str::trim) {
        if !is_identifier(part) {
            bail!("expected an error type or effect after `/`; use `/ ()` for an empty row");
        }
        if names.iter().any(|name: &String| name == part) {
            bail!("duplicate name in effect row: {part}");
        }
        names.push(part.into());
    }
    Ok(names)
}

/// Resolve the shared row by declaration, including forward declarations.
pub(super) fn resolve_rows(program: &mut [Statement]) -> Result<()> {
    let mut types = Set::default();
    let mut operations = Set::default();
    for statement in program.iter() {
        match &statement.kind {
            StatementKind::Enum(def) => {
                types.insert(def.name.clone());
            }
            StatementKind::Effect(def) => {
                operations.insert(def.name.clone());
            }
            _ => {}
        }
    }
    for statement in program {
        let methods = match &mut statement.kind {
            StatementKind::Function(def) => core::slice::from_mut(def),
            StatementKind::Impl(def) => def.methods.as_mut_slice(),
            _ => continue,
        };
        for def in methods {
            resolve_function(def, &types, &operations)?;
        }
    }
    Ok(())
}

fn resolve_function(
    def: &mut FunctionDef,
    types: &Set<String>,
    operations: &Set<String>,
) -> Result<()> {
    let Some(names) = &mut def.declared_effects else {
        return Ok(());
    };
    let mut errors = ErrorSet::default();
    for name in names.iter() {
        if (types.contains(name) || name == "unknown")
            && (operations.contains(name) || Effect::parse(name).is_some())
        {
            bail!(
                "ambiguous name `{name}` in the effect row of `{}`",
                def.name
            );
        }
        if name == "unknown" {
            errors.insert(ErrorType::Unknown);
        } else if types.contains(name) {
            errors.insert(ErrorType::Known(name.clone()));
        }
    }
    names.retain(|name| name != "unknown" && !types.contains(name));
    def.declared_errors = Some(errors);
    Ok(())
}
