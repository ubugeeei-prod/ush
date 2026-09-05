use anyhow::{Result, anyhow, bail};

use crate::{
    ast::Type,
    errors::{ErrorSet, ErrorType},
    types::{AstString as String, HeapVec as Vec},
    util::{
        is_identifier, parse_type, split_once_top_level, split_top_level, strip_wrapping_parens,
    },
};

pub(super) type FunctionReturn = (Option<Type>, Option<ErrorSet>, Option<Vec<String>>);

/// Parses the part of a signature after the parameter list.
///
/// The shape is `-> <errors>!<type> / { <effects> }`, where both the
/// error set and the effect row are optional. The row follows
/// Effekt: it lists what the function still needs from its caller,
/// and `/ {}` says it needs nothing.
pub(super) fn parse_function_return(source: &str) -> Result<FunctionReturn> {
    if source.is_empty() {
        return Ok((Some(Type::Unit), None, None));
    }
    let ty = source
        .strip_prefix("->")
        .ok_or_else(|| anyhow!("invalid function signature suffix: {source}"))?
        .trim();

    let (ty, effects) = match split_effect_row(ty)? {
        Some((head, row)) => (head, Some(row)),
        None => (ty, None),
    };
    if ty.is_empty() {
        // `-> / { io }` still returns unit; the row just moved the
        // type away.
        return Ok((Some(Type::Unit), None, effects));
    }

    if let Some((errors, value)) = split_once_top_level(ty, '!') {
        return Ok((
            Some(parse_type_name(value)?),
            Some(parse_error_set(errors)?),
            effects,
        ));
    }
    Ok((Some(parse_type_name(ty)?), None, effects))
}

/// Splits `String / { fs, log }` into the type and the effect names.
fn split_effect_row(source: &str) -> Result<Option<(&str, Vec<String>)>> {
    let Some((head, row)) = split_once_top_level(source, '/') else {
        return Ok(None);
    };
    let row = row.trim();
    let inner = row
        .strip_prefix('{')
        .and_then(|rest| rest.strip_suffix('}'))
        .ok_or_else(|| anyhow!("an effect row is written `/ {{ name, … }}`, got `/ {row}`"))?;

    let mut effects = Vec::new();
    for part in split_top_level(inner, ',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if !is_identifier(part) {
            bail!("invalid effect name: {part}");
        }
        let name = String::from(part);
        if effects.contains(&name) {
            bail!("duplicate effect in row: {part}");
        }
        effects.push(name);
    }
    Ok(Some((head.trim(), effects)))
}

fn parse_type_name(source: &str) -> Result<Type> {
    parse_type(source).ok_or_else(|| anyhow!("invalid type: {source}"))
}

fn parse_error_set(source: &str) -> Result<ErrorSet> {
    let inner = strip_wrapping_parens(source).unwrap_or(source).trim();
    if inner.is_empty() {
        bail!("error set cannot be empty");
    }

    let mut errors = ErrorSet::default();
    for part in split_top_level(inner, '|') {
        let part = part.trim();
        if part.is_empty() {
            bail!("invalid error set: {source}");
        }
        errors.insert(parse_error_type(part)?);
    }
    Ok(errors)
}

fn parse_error_type(source: &str) -> Result<ErrorType> {
    let trimmed = source.trim();
    if trimmed == "unknown" {
        return Ok(ErrorType::Unknown);
    }
    if is_identifier(trimmed) {
        return Ok(ErrorType::Known(String::from(trimmed)));
    }
    bail!("invalid error type: {trimmed}")
}
