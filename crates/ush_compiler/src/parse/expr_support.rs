use alloc::boxed::Box;

use anyhow::Result;

use super::{
    super::ast::{Expr, MethodCall},
    expr::parse_atom,
    path::parse_identifier,
    signature,
};
use crate::scan::{ScanState, advance};
use crate::types::HeapVec as Vec;
use crate::util::parse_paren_body;

pub(super) fn parse_member_chain(source: &str) -> Result<Option<Expr>> {
    let words = crate::util::split_top_level_whitespace(source);
    if words.len() > 1 && super::path::looks_like_call_target(words[0]) {
        return Ok(None);
    }
    let parts = split_members(source);
    if parts.len() <= 1 {
        return Ok(None);
    }
    let mut expr = parse_atom(parts[0])?;
    for part in &parts[1..] {
        if let Some((head, inner)) = parse_paren_body(part) {
            expr = Expr::MethodCall(MethodCall {
                receiver: Box::new(expr),
                method: parse_identifier(head.trim())?,
                args: signature::parse_call_args(inner)?,
            });
            continue;
        }
        expr = Expr::Field {
            base: Box::new(expr),
            name: parse_identifier(part.trim())?,
        };
    }
    Ok(Some(expr))
}

pub(super) fn split_range(source: &str) -> Option<(&str, &str)> {
    let mut state = ScanState::default();
    let bytes = source.as_bytes();
    let mut index = 0usize;

    while index + 1 < bytes.len() {
        if state.top_level() && bytes[index] == b'.' && bytes[index + 1] == b'.' {
            return Some((source[..index].trim(), source[index + 2..].trim()));
        }
        index = advance(source, index, &mut state);
    }
    None
}

fn split_members(source: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut state = ScanState::default();
    let bytes = source.as_bytes();
    let mut index = 0usize;

    while index < bytes.len() {
        if state.top_level() && bytes[index] == b'.' {
            let prev = index.checked_sub(1).and_then(|it| bytes.get(it)).copied();
            let next = bytes.get(index + 1).copied();
            if !matches!(
                (prev, next),
                (Some(b'.'), _) | (_, Some(b'.')) | (_, Some(b':'))
            ) {
                parts.push(source[start..index].trim());
                start = index + 1;
            }
        }
        index = advance(source, index, &mut state);
    }

    if parts.is_empty() {
        return parts;
    }
    parts.push(source[start..].trim());
    parts
}
