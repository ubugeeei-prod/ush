use alloc::boxed::Box;

use anyhow::{Context, Result, anyhow};

use super::{
    super::{
        ast::{Attribute, EnumDef, FunctionDef, StatementKind, VariantDef, VariantFields},
        util::{parse_brace_body, parse_paren_body},
    },
    SourceLine,
    declaration_support::{finish_block, parse_name},
    effect_decl,
    expr::{parse_expr, parse_named_type_list, parse_pattern, parse_type_list},
    impls, inline, signature, use_decl,
};
use crate::types::HeapVec as Vec;

pub(super) fn parse_declaration(
    line_no: usize,
    trimmed: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
    attrs: &[Attribute],
    _tail_position: bool,
) -> Result<Option<StatementKind>> {
    if let Some(rest) = trimmed.strip_prefix("type ") {
        return Ok(Some(parse_type_def(rest, lines, cursor)?));
    }
    if let Some(rest) = trimmed.strip_prefix("enum ") {
        return Ok(Some(parse_enum(rest, lines, cursor)?));
    }
    if let Some(rest) = trimmed.strip_prefix("trait ") {
        return Ok(Some(impls::parse_trait(rest, lines, cursor)?));
    }
    if let Some(rest) = trimmed.strip_prefix("impl ") {
        return Ok(Some(impls::parse_impl(rest, lines, cursor)?));
    }
    if let Some(rest) = trimmed.strip_prefix("use ") {
        return Ok(Some(use_decl::parse_use(rest)?));
    }
    if let Some(rest) = trimmed.strip_prefix("fn ") {
        return Ok(Some(parse_function(line_no, rest, lines, cursor, attrs)?));
    }
    if let Some(rest) = trimmed.strip_prefix("effect ") {
        return Ok(Some(effect_decl::parse_effect(rest)?));
    }
    Ok(None)
}

pub(super) fn parse_match(
    subject: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
    returns_value: bool,
) -> Result<StatementKind> {
    let subject = subject
        .strip_suffix('{')
        .ok_or_else(|| anyhow!("expected `{{` after match subject"))?
        .trim();
    let mut arms = Vec::new();
    *cursor += 1;

    while *cursor < lines.len() {
        let (line_no, line) = &lines[*cursor];
        let trimmed = line.trim().trim_end_matches(',').trim();
        if matches!(trimmed, "}" | "};") {
            break;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') {
            *cursor += 1;
            continue;
        }
        let (pattern, statement) = trimmed
            .split_once("=>")
            .ok_or_else(|| anyhow!("invalid match arm"))?;
        arms.push((
            parse_pattern(pattern.trim())?,
            Box::new(
                inline::parse_inline_body(*line_no, statement.trim(), returns_value)
                    .with_context(|| format!("line {line_no}: invalid match arm body"))?,
            ),
        ));
        *cursor += 1;
    }
    let terminated = finish_block(lines, cursor, "match expression")?;
    let returns_value = returns_value && !terminated;
    Ok(StatementKind::Match {
        expr: parse_expr(subject)?,
        arms,
        returns_value,
    })
}

fn parse_enum(header: &str, lines: &[SourceLine<'_>], cursor: &mut usize) -> Result<StatementKind> {
    let name = header
        .strip_suffix('{')
        .ok_or_else(|| anyhow!("expected `{{` after enum name"))?
        .trim();
    let mut variants = Vec::new();
    let name = parse_name(name)?;
    *cursor += 1;
    while *cursor < lines.len() {
        let variant_line = lines[*cursor].1.trim().trim_end_matches(',').trim();
        if variant_line == "}" {
            break;
        }
        if variant_line.is_empty() || variant_line.starts_with('#') {
            *cursor += 1;
            continue;
        }
        variants.push(parse_variant_def(variant_line)?);
        *cursor += 1;
    }
    let _ = finish_block(lines, cursor, "enum definition")?;
    Ok(StatementKind::Enum(EnumDef { name, variants }))
}

fn parse_type_def(
    header: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<StatementKind> {
    let name = header
        .strip_suffix('{')
        .ok_or_else(|| anyhow!("expected `{{` after type name"))?
        .trim();
    let name = parse_name(name)?;
    let mut fields = Vec::new();
    *cursor += 1;
    while *cursor < lines.len() {
        let field_line = lines[*cursor].1.trim().trim_end_matches(',').trim();
        if field_line == "}" {
            break;
        }
        if field_line.is_empty() || field_line.starts_with('#') {
            *cursor += 1;
            continue;
        }
        fields.extend(parse_named_type_list(field_line)?);
        *cursor += 1;
    }
    let _ = finish_block(lines, cursor, "type definition")?;
    Ok(StatementKind::Enum(EnumDef {
        name: name.clone(),
        variants: vec![VariantDef {
            name,
            fields: VariantFields::Struct(fields),
        }]
        .into_iter()
        .collect(),
    }))
}

fn parse_function(
    _line_no: usize,
    header: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
    attrs: &[Attribute],
) -> Result<StatementKind> {
    let head = header
        .strip_suffix('{')
        .ok_or_else(|| anyhow!("expected `{{` after function signature"))?
        .trim();
    let (name, params, return_type, declared_errors, declared_effects) =
        signature::parse_function_header(head)?;
    *cursor += 1;
    let body = super::statement::parse_block(lines, cursor, false, return_type.is_some())?;
    let _ = finish_block(lines, cursor, "function body")?;
    Ok(StatementKind::Function(FunctionDef {
        attrs: attrs.to_vec(),
        name,
        receiver: None,
        params,
        return_type,
        declared_errors,
        declared_effects,
        body,
    }))
}

fn parse_variant_def(source: &str) -> Result<VariantDef> {
    if let Some((head, inner)) = parse_paren_body(source) {
        return Ok(VariantDef {
            name: parse_name(head)?,
            fields: VariantFields::Tuple(parse_type_list(inner)?),
        });
    }
    if let Some((head, inner)) = parse_brace_body(source) {
        return Ok(VariantDef {
            name: parse_name(head)?,
            fields: VariantFields::Struct(parse_named_type_list(inner)?),
        });
    }
    Ok(VariantDef {
        name: parse_name(source)?,
        fields: VariantFields::Unit,
    })
}
