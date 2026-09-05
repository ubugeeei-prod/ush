use anyhow::{Result, anyhow, bail};

use super::{
    super::{
        ast::Attribute,
        util::{is_identifier, parse_paren_body},
    },
    expr::parse_expr,
};
use crate::scan::{ScanState, advance};
use crate::types::HeapVec as Vec;

pub(super) fn parse_attribute_line(source: &str) -> Result<Attribute> {
    parse_attribute(
        source
            .trim()
            .strip_prefix("#[")
            .and_then(|rest| rest.strip_suffix(']'))
            .ok_or_else(|| anyhow!("invalid attribute: {source}"))?,
    )
}

pub(super) fn parse_inline_attrs(source: &str) -> Result<(Vec<Attribute>, &str)> {
    let mut attrs = Vec::new();
    let mut rest = source.trim_start();
    while rest.starts_with("#[") {
        let end = attribute_end(rest).ok_or_else(|| anyhow!("unterminated attribute"))?;
        attrs.push(parse_attribute(&rest[2..end - 1])?);
        rest = rest[end..].trim_start();
    }
    Ok((attrs, rest))
}

fn parse_attribute(source: &str) -> Result<Attribute> {
    let trimmed = source.trim();
    if let Some((name, inner)) = parse_paren_body(trimmed) {
        return Ok(Attribute {
            name: parse_name(name)?,
            value: Some(parse_expr(inner)?),
        });
    }
    Ok(Attribute {
        name: parse_name(trimmed)?,
        value: None,
    })
}

fn attribute_end(source: &str) -> Option<usize> {
    let mut state = ScanState::default();
    let mut index = 2usize;
    while index < source.len() {
        if state.top_level() && source.as_bytes()[index] == b']' {
            return Some(index + 1);
        }
        index = advance(source, index, &mut state);
    }
    None
}

fn parse_name(source: &str) -> Result<crate::types::AstString> {
    let trimmed = source.trim();
    if is_identifier(trimmed) {
        Ok(trimmed.into())
    } else {
        bail!("invalid attribute name: {trimmed}")
    }
}
