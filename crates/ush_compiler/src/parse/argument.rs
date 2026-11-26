//! Whitespace call arguments are shell words; expressions opt in explicitly.

use alloc::string::ToString;
use anyhow::{Result, bail};

use super::expr::parse_expr;
use crate::{
    ast::{Call, CallArg, Expr},
    types::HeapVec as Vec,
    util::{is_identifier, split_top_level_whitespace},
};

pub(super) fn parse(source: &str) -> Result<Expr> {
    let text = source.trim();
    if text.starts_with(['(', '[', '"', '\'', '&'])
        || is_integer(text)
        || matches!(text, "true" | "false")
        || super::signature::is_paren_call(text)
        || text
            .find(|c: char| c.is_ascii_alphabetic())
            .is_some_and(|i| is_integer(&text[..i]) && is_identifier(&text[i..]))
    {
        return parse_expr(text);
    }
    if text.starts_with("$(") || text.starts_with("$\"") {
        return parse_expr(text);
    }
    if text.contains('$') {
        return interpolate(text);
    }
    Ok(Expr::String(text.into()))
}

/// An integer written the way it prints. `0755`, `-0`, `007` and `+1` stay
/// words, so `chmod 0755` and `xargs -0` reach the program as written.
fn is_integer(text: &str) -> bool {
    text.parse::<i64>()
        .is_ok_and(|number| number.to_string() == text)
}

fn interpolate(text: &str) -> Result<Expr> {
    let mut parts = Vec::new();
    let mut rest = text;
    while let Some(at) = rest.find('$') {
        if at > 0 {
            parts.push(Expr::String(rest[..at].into()));
        }
        rest = &rest[at + 1..];
        let (name, tail) = if let Some(braced) = rest.strip_prefix('{') {
            let close = braced.find('}').ok_or_else(|| {
                anyhow::anyhow!("unclosed variable reference; use $name or ${{name}}")
            })?;
            (&braced[..close], &braced[close + 1..])
        } else {
            // `$env::NAME` reads the environment; the prefix joins the name.
            let skip = if rest.starts_with("env::") { 5 } else { 0 };
            let end = rest[skip..]
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .map_or(rest.len(), |end| skip + end);
            (&rest[..end], &rest[end..])
        };
        if let Some(variable) = name.strip_prefix("env::") {
            parts.push(super::atoms::env_read(variable)?);
            rest = tail;
            continue;
        }
        if !is_identifier(name) {
            bail!(
                "invalid variable reference; use $name or ${{name}}, and (expression) for computed arguments"
            );
        }
        parts.push(Expr::Var(format!("${name}").into()));
        rest = tail;
    }
    if !rest.is_empty() {
        parts.push(Expr::String(rest.into()));
    }
    if parts.len() == 1 {
        return Ok(parts.remove(0));
    }
    for part in &mut parts {
        if !matches!(part, Expr::String(_)) {
            *part = Expr::Call(Call {
                name: "format".into(),
                args: vec![CallArg {
                    label: None,
                    expr: part.clone(),
                }],
                asynchronous: false,
            });
        }
    }
    Ok(Expr::Add(parts))
}

/// A whitespace call starts before any operator in its arguments. Arithmetic
/// at the expression level still starts with a value followed by an operator.
pub(super) fn starts_call(source: &str) -> bool {
    if !source.contains(char::is_whitespace) {
        return false;
    }
    let parts = split_top_level_whitespace(source);
    parts.len() > 1
        && super::path::looks_like_call_target(parts[0])
        && !parts[1].starts_with('{')
        && !parts[1..]
            .iter()
            .any(|part| matches!(*part, "+" | "==" | "!=" | "<" | ">" | "<=" | ">=" | ".."))
}

pub(super) fn propagation(source: &str) -> Option<&str> {
    let base = crate::util::strip_top_level_suffix(source, '?')?;
    let words = split_top_level_whitespace(base);
    let explicit = words
        .last()
        .is_some_and(|word| word.starts_with(['$', '"', '\'']));
    (base.ends_with(')')
        || explicit
        || source.ends_with(" ?")
        || !source.contains(char::is_whitespace))
    .then_some(base)
}
