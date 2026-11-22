use anyhow::{Result, anyhow};

use super::super::{
    ast::{
        Expr, ExprFields, NamedExpr, NamedFieldType, NamedPattern, Pattern, PatternFields,
        VariantExpr, VariantPattern,
    },
    util::{
        is_identifier, parse_brace_body, parse_paren_body, parse_type, split_once_top_level,
        split_path, split_top_level,
    },
};
use crate::types::{AstString as String, HeapVec as Vec};

use super::expr::{parse_expr, parse_pattern};

pub(super) fn parse_named_type_list(source: &str) -> Result<Vec<NamedFieldType>> {
    parse_list(source, |part| {
        let (name, ty) = split_once_top_level(part, ':')
            .ok_or_else(|| anyhow!("invalid struct field: {part}"))?;
        Ok(NamedFieldType {
            name: parse_name(name)?,
            ty: parse_type(ty).ok_or_else(|| anyhow!("invalid type: {ty}"))?,
        })
    })
}

pub(super) fn parse_variant_expr(source: &str) -> Result<Option<VariantExpr>> {
    if let Some((head, inner)) = parse_paren_body(source)
        && let Some((enum_name, variant_name)) = split_variant_path(head)
    {
        return Ok(Some(VariantExpr {
            enum_name,
            variant_name,
            fields: ExprFields::Tuple(parse_list(inner, parse_expr)?),
        }));
    }
    if let Some((head, inner)) = parse_brace_body(source) {
        if let Some((enum_name, variant_name)) = split_variant_path(head) {
            return Ok(Some(VariantExpr {
                enum_name,
                variant_name,
                fields: ExprFields::Struct(parse_named_expr_list(inner)?),
            }));
        }
        if is_identifier(head) {
            let name = parse_name(head)?;
            return Ok(Some(VariantExpr {
                enum_name: name.clone(),
                variant_name: name,
                fields: ExprFields::Struct(parse_named_expr_list(inner)?),
            }));
        }
    }
    Ok(
        split_variant_path(source).map(|(enum_name, variant_name)| VariantExpr {
            enum_name,
            variant_name,
            fields: ExprFields::Unit,
        }),
    )
}

pub(super) fn parse_variant_pattern(source: &str) -> Result<Option<VariantPattern>> {
    if let Some((head, inner)) = parse_paren_body(source)
        && let Some((enum_name, variant_name)) = split_variant_path(head)
    {
        return Ok(Some(VariantPattern {
            enum_name,
            variant_name,
            fields: PatternFields::Tuple(parse_list(inner, parse_pattern)?),
        }));
    }
    if let Some((head, inner)) = parse_brace_body(source) {
        if let Some((enum_name, variant_name)) = split_variant_path(head) {
            return Ok(Some(VariantPattern {
                enum_name,
                variant_name,
                fields: PatternFields::Struct(parse_named_pattern_list(inner)?),
            }));
        }
        if is_identifier(head) {
            let name = parse_name(head)?;
            return Ok(Some(VariantPattern {
                enum_name: name.clone(),
                variant_name: name,
                fields: PatternFields::Struct(parse_named_pattern_list(inner)?),
            }));
        }
    }
    Ok(
        split_variant_path(source).map(|(enum_name, variant_name)| VariantPattern {
            enum_name,
            variant_name,
            fields: PatternFields::Unit,
        }),
    )
}

fn split_variant_path(source: &str) -> Option<(String, String)> {
    let (enum_name, variant_name) = split_path(source)?;
    (is_identifier(&enum_name) && is_identifier(&variant_name)).then_some((enum_name, variant_name))
}

fn parse_named_expr_list(source: &str) -> Result<Vec<NamedExpr>> {
    parse_list(source, |part| {
        if let Some((name, expr)) = split_once_top_level(part, ':') {
            return Ok(NamedExpr {
                name: parse_name(name)?,
                expr: parse_expr(expr)?,
            });
        }
        let name = parse_name(part)?;
        Ok(NamedExpr {
            name: name.clone(),
            expr: Expr::Var(name),
        })
    })
}

fn parse_named_pattern_list(source: &str) -> Result<Vec<NamedPattern>> {
    parse_list(source, |part| {
        if let Some((name, pattern)) = split_once_top_level(part, ':') {
            return Ok(NamedPattern {
                name: parse_name(name)?,
                pattern: parse_pattern(pattern)?,
            });
        }
        let name = parse_name(part)?;
        Ok(NamedPattern {
            name: name.clone(),
            pattern: Pattern::Binding(name),
        })
    })
}

fn parse_list<T>(source: &str, mut parse: impl FnMut(&str) -> Result<T>) -> Result<Vec<T>> {
    split_top_level(source, ',')
        .into_iter()
        .filter(|part| !part.is_empty())
        .map(&mut parse)
        .collect()
}

fn parse_name(source: &str) -> Result<String> {
    let trimmed = source.trim();
    Ok(trimmed
        .chars()
        .all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
        .then_some(trimmed)
        .filter(|name| {
            name.chars()
                .next()
                .is_some_and(|first| first == '_' || first.is_ascii_alphabetic())
        })
        .ok_or_else(|| anyhow!("invalid identifier: {trimmed}"))?
        .into())
}
