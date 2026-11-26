use anyhow::{Result, bail};

use super::{
    super::ast::Condition,
    expr::{parse_expr, parse_pattern},
};
use crate::scan::{ScanState, advance, parenthesized};
use crate::types::HeapVec as Vec;
use alloc::boxed::Box;

fn binds(condition: &Condition) -> bool {
    match condition {
        Condition::Let { .. } => true,
        Condition::Expr(_) => false,
        Condition::Not(inner) => binds(inner),
        Condition::And(items) | Condition::Or(items) => items.iter().any(binds),
    }
}

pub(super) fn parse_condition(source: &str) -> Result<Condition> {
    let source = source.trim();
    if let Some(parts) = split_logic(source, "||") {
        return Ok(Condition::Or(
            parts
                .into_iter()
                .map(parse_condition)
                .collect::<Result<Vec<_>>>()?,
        ));
    }
    if let Some(parts) = split_logic(source, "&&") {
        return Ok(Condition::And(
            parts
                .into_iter()
                .map(parse_condition)
                .collect::<Result<Vec<_>>>()?,
        ));
    }
    if let Some(rest) = source.strip_prefix('!') {
        let inner = parse_condition(rest)?;
        if binds(&inner) {
            bail!("a pattern binding cannot be negated");
        }
        return Ok(Condition::Not(Box::new(inner)));
    }
    if let Some(inner) = parenthesized(source)
        && split_logic(inner, ",").is_none()
    {
        return parse_condition(inner);
    }
    if let Some(rest) = source.strip_prefix("let ") {
        return parse_binding_condition(rest);
    }
    if let Some(condition) = parse_optional_binding(source)? {
        return Ok(condition);
    }
    Ok(Condition::Expr(parse_expr(source)?))
}

fn parse_binding_condition(source: &str) -> Result<Condition> {
    let Some((pattern, expr)) = split_binding(source) else {
        bail!("expected `let pattern = expr`");
    };
    Ok(Condition::Let {
        pattern: parse_pattern(pattern)?,
        expr: parse_expr(expr)?,
    })
}

fn parse_optional_binding(source: &str) -> Result<Option<Condition>> {
    let Some((pattern, expr)) = split_binding(source) else {
        return Ok(None);
    };
    Ok(Some(Condition::Let {
        pattern: parse_pattern(pattern)?,
        expr: parse_expr(expr)?,
    }))
}

pub(super) fn split_logic<'a>(source: &'a str, token: &str) -> Option<Vec<&'a str>> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut state = ScanState::default();
    let mut index = 0usize;
    while index + token.len() <= source.len() {
        if state.top_level() && source[index..].starts_with(token) {
            parts.push(source[start..index].trim());
            start = index + token.len();
            index += token.len();
            continue;
        }
        index = advance(source, index, &mut state);
    }
    if parts.is_empty() {
        return None;
    }
    parts.push(source[start..].trim());
    Some(parts)
}

fn split_binding(source: &str) -> Option<(&str, &str)> {
    let mut state = ScanState::default();
    let bytes = source.as_bytes();
    let mut index = 0usize;

    while index < bytes.len() {
        if state.top_level() && bytes[index] == b'=' {
            let prev = index.checked_sub(1).and_then(|it| bytes.get(it)).copied();
            let next = bytes.get(index + 1).copied();
            if !matches!(prev, Some(b'=') | Some(b'!') | Some(b'<') | Some(b'>'))
                && !matches!(next, Some(b'='))
            {
                return Some((source[..index].trim(), source[index + 1..].trim()));
            }
        }
        index = advance(source, index, &mut state);
    }
    None
}
