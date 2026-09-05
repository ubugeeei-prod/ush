use alloc::boxed::Box;

use anyhow::{Result, anyhow, bail};

use super::super::{
    ast::{Expr, NamedFieldType, Pattern, Type},
    util::{
        is_identifier, parse_paren_body, parse_string_literal, parse_type, split_top_level,
        strip_top_level_suffix,
    },
};
use crate::types::HeapVec as Vec;

use super::{
    compare::split_compare,
    expr_support::{parse_member_chain, split_range},
    record, signature,
};

pub(super) fn parse_expr(source: &str) -> Result<Expr> {
    // `let answer = do ask("name?")` — performing an effect in value
    // position. The operation is a generated shell function, so the
    // rest of the pipeline sees an ordinary call.
    if let Some(rest) = source.trim().strip_prefix("do ") {
        return Ok(Expr::Call(super::signature::parse_call(
            rest.trim(),
            false,
        )?));
    }
    if let Some((start, end)) = split_range(source) {
        return Ok(Expr::Range {
            start: Box::new(parse_atom(start)?),
            end: Box::new(parse_atom(end)?),
        });
    }
    if let Some((lhs, op, rhs)) = split_compare(source) {
        return Ok(Expr::Compare {
            lhs: Box::new(parse_expr(lhs)?),
            op,
            rhs: Box::new(parse_expr(rhs)?),
        });
    }
    let applications = split_top_level(source, '$');
    if applications.len() > 1 {
        return parse_dollar_chain(&applications);
    }
    let pieces = split_top_level(source, '+');
    if pieces.len() > 1 {
        return Ok(Expr::Add(
            pieces
                .into_iter()
                .map(parse_atom)
                .collect::<Result<Vec<_>>>()?,
        ));
    }
    parse_atom(source.trim())
}

pub(super) fn parse_pattern(source: &str) -> Result<Pattern> {
    let trimmed = source.trim();
    if trimmed == "_" {
        return Ok(Pattern::Wildcard);
    }
    if let Some(string) = parse_string_literal(trimmed) {
        return Ok(Pattern::String(string));
    }
    if trimmed == "true" {
        return Ok(Pattern::Bool(true));
    }
    if trimmed == "false" {
        return Ok(Pattern::Bool(false));
    }
    if trimmed == "()" {
        return Ok(Pattern::Unit);
    }
    if let Ok(number) = trimmed.parse::<i64>() {
        return Ok(Pattern::Int(number));
    }
    if let Some(pattern) = parse_variant_pattern(trimmed)? {
        return Ok(Pattern::Variant(pattern));
    }
    if is_identifier(trimmed) {
        return Ok(Pattern::Binding(trimmed.into()));
    }
    bail!("unsupported pattern: {trimmed}")
}

pub(super) fn parse_named_type_list(source: &str) -> Result<Vec<NamedFieldType>> {
    record::parse_named_type_list(source)
}

pub(super) fn parse_atom(source: &str) -> Result<Expr> {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        bail!("empty expression");
    }
    if let Some(inner) = strip_top_level_suffix(trimmed, '?') {
        if inner.is_empty() {
            bail!("try operator requires an expression");
        }
        return Ok(Expr::Try(Box::new(parse_expr(inner)?)));
    }
    if trimmed == "()" {
        return Ok(Expr::Unit);
    }
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        return Ok(Expr::List(parse_list(
            &trimmed[1..trimmed.len() - 1],
            parse_expr,
        )?));
    }
    if trimmed.starts_with('(')
        && trimmed.ends_with(')')
        && let Some((head, inner)) = parse_paren_body(trimmed)
        && head.is_empty()
    {
        let items = split_top_level(inner, ',');
        if items.len() > 1 {
            return Ok(Expr::Tuple(
                items
                    .into_iter()
                    .filter(|part| !part.is_empty())
                    .map(parse_expr)
                    .collect::<Result<Vec<_>>>()?,
            ));
        }
        return parse_expr(inner);
    }
    if let Some(string) = parse_string_literal(trimmed) {
        return Ok(Expr::String(string));
    }
    if let Some(expr) = parse_member_chain(trimmed)? {
        return Ok(expr);
    }
    if trimmed == "true" {
        return Ok(Expr::Bool(true));
    }
    if trimmed == "false" {
        return Ok(Expr::Bool(false));
    }
    if let Ok(number) = trimmed.parse::<i64>() {
        return Ok(Expr::Int(number));
    }
    if let Some(variant) = parse_variant_expr(trimmed)? {
        return Ok(Expr::Variant(variant));
    }
    if let Some(call) = signature::parse_expr_call(trimmed)? {
        return Ok(Expr::Call(call));
    }
    if is_identifier(trimmed) {
        return Ok(Expr::Var(trimmed.into()));
    }
    bail!("unsupported expression: {trimmed}")
}

fn parse_dollar_chain(parts: &[&str]) -> Result<Expr> {
    let Some(last) = parts.last() else {
        bail!("empty application");
    };
    let mut expr = parse_expr(last)?;
    for part in parts[..parts.len() - 1].iter().rev() {
        expr = Expr::Call(signature::parse_dollar_call(part, expr)?);
    }
    Ok(expr)
}

pub(super) fn parse_type_list(source: &str) -> Result<Vec<Type>> {
    parse_list(source, |part| {
        parse_type(part).ok_or_else(|| anyhow!("invalid type: {part}"))
    })
}

fn parse_list<T>(source: &str, mut parse: impl FnMut(&str) -> Result<T>) -> Result<Vec<T>> {
    split_top_level(source, ',')
        .into_iter()
        .filter(|part| !part.is_empty())
        .map(&mut parse)
        .collect()
}

fn parse_variant_expr(source: &str) -> Result<Option<super::super::ast::VariantExpr>> {
    record::parse_variant_expr(source)
}

fn parse_variant_pattern(source: &str) -> Result<Option<super::super::ast::VariantPattern>> {
    record::parse_variant_pattern(source)
}
