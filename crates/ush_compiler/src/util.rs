use alloc::{boxed::Box, string::ToString};

use memchr::{memchr, memmem, memrchr};
use phf::phf_map;

use super::ast::Type;
use crate::scan::{ScanState, advance};
use crate::string_literal;
use crate::types::{AstString as String, AstVec as Vec, OutputString};

#[derive(Clone, Copy)]
enum PrimitiveType {
    String,
    Int,
    Bool,
}

static PRIMITIVE_TYPES: phf::Map<&'static str, PrimitiveType> = phf_map! {
    "String" => PrimitiveType::String,
    "Int" => PrimitiveType::Int,
    "Bool" => PrimitiveType::Bool,
};

pub(crate) fn split_top_level(source: &str, separator: char) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0usize;
    let mut state = ScanState::default();
    let mut index = 0usize;

    while index < source.len() {
        let ch = source[index..]
            .chars()
            .next()
            .expect("split index must be valid");
        if state.top_level() && ch == separator {
            result.push(source[start..index].trim());
            start = index + ch.len_utf8();
        }
        index = advance(source, index, &mut state);
    }

    if start == 0 {
        let mut single = Vec::new();
        single.push(source.trim());
        return single;
    }
    result.push(source[start..].trim());
    result
}

pub(crate) fn split_top_level_whitespace(source: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = None;
    let mut state = ScanState::default();
    let mut index = 0usize;

    while index < source.len() {
        let ch = source[index..]
            .chars()
            .next()
            .expect("split index must be valid");
        if !ch.is_whitespace() && start.is_none() {
            start = Some(index);
        }
        if ch.is_whitespace()
            && state.top_level()
            && let Some(begin) = start.take()
        {
            result.push(source[begin..index].trim());
        }
        index = advance(source, index, &mut state);
    }

    if let Some(begin) = start {
        result.push(source[begin..].trim());
    }
    result
}

pub(crate) fn split_once_top_level(source: &str, separator: char) -> Option<(&str, &str)> {
    let mut state = ScanState::default();
    let mut index = 0usize;
    while index < source.len() {
        let ch = source[index..]
            .chars()
            .next()
            .expect("split index must be valid");
        if state.top_level() && ch == separator {
            return Some((source[..index].trim(), source[index + 1..].trim()));
        }
        index = advance(source, index, &mut state);
    }
    None
}

pub(crate) fn strip_top_level_suffix(source: &str, suffix: char) -> Option<&str> {
    let trimmed = source.trim();
    let mut state = ScanState::default();
    let mut candidate = None;
    let mut index = 0usize;

    while index < trimmed.len() {
        let ch = trimmed[index..]
            .chars()
            .next()
            .expect("suffix index must be valid");
        if state.top_level() && ch == suffix {
            candidate = Some(index);
        }
        index = advance(trimmed, index, &mut state);
    }

    let index = candidate?;
    (index + suffix.len_utf8() == trimmed.len()).then_some(trimmed[..index].trim())
}

pub(crate) fn split_path(source: &str) -> Option<(String, String)> {
    let index = memmem::find(source.as_bytes(), b"::")?;
    Some((
        source[..index].trim().into(),
        source[index + 2..].trim().into(),
    ))
}

pub(crate) fn is_path(source: &str) -> bool {
    let mut count = 0usize;
    for part in source.split("::") {
        if !is_identifier(part) {
            return false;
        }
        count += 1;
    }
    count > 1
}

pub(crate) fn path_tail(source: &str) -> Option<&str> {
    source.rsplit("::").next()
}

pub(crate) fn parse_paren_body(source: &str) -> Option<(&str, &str)> {
    let open = memchr(b'(', source.as_bytes())?;
    let close = memrchr(b')', source.as_bytes())?;
    (close > open).then_some((source[..open].trim(), source[open + 1..close].trim()))
}

pub(crate) fn strip_wrapping_parens(source: &str) -> Option<&str> {
    let trimmed = source.trim();
    if !trimmed.starts_with('(') || !trimmed.ends_with(')') {
        return None;
    }

    let mut state = ScanState::default();
    let mut index = 0usize;
    while index < trimmed.len() {
        if !state.in_string() && trimmed.as_bytes()[index] == b'(' {
            state.paren += 1;
            index += 1;
            continue;
        }
        if !state.in_string() && trimmed.as_bytes()[index] == b')' {
            state.paren = state.paren.saturating_sub(1);
            if state.paren == 0 && index + 1 != trimmed.len() {
                return None;
            }
            index += 1;
            continue;
        }
        index = advance(trimmed, index, &mut state);
    }

    Some(trimmed[1..trimmed.len() - 1].trim())
}

pub(crate) fn parse_brace_body(source: &str) -> Option<(&str, &str)> {
    let open = memchr(b'{', source.as_bytes())?;
    let close = memrchr(b'}', source.as_bytes())?;
    // `then_some` would slice eagerly, and a `}` that comes before
    // its `{` — as in `} with log {` — makes that range backwards.
    (close > open).then(|| (source[..open].trim(), source[open + 1..close].trim()))
}

pub(crate) fn parse_string_literal(source: &str) -> Option<String> {
    string_literal::parse_string_literal(source)
}

pub(crate) fn is_identifier(source: &str) -> bool {
    let mut chars = source.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

pub(crate) fn parse_type(source: &str) -> Option<Type> {
    let trimmed = source.trim();
    if matches!(trimmed, "()" | "Unit") {
        return Some(Type::Unit);
    }
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        return parse_type(&trimmed[1..trimmed.len() - 1]).map(|inner| Type::List(Box::new(inner)));
    }
    if let Some(inner) = tuple_type_body(trimmed) {
        let parts = split_top_level(inner, ',');
        if parts.len() > 1 {
            return parts
                .into_iter()
                .map(parse_type)
                .collect::<Option<Vec<_>>>()
                .map(|items| Type::Tuple(items.into_iter().collect()));
        }
    }
    match PRIMITIVE_TYPES.get(trimmed).copied() {
        Some(PrimitiveType::String) => Some(Type::String),
        Some(PrimitiveType::Int) => Some(Type::Int),
        Some(PrimitiveType::Bool) => Some(Type::Bool),
        None if is_identifier(trimmed) => Some(Type::Adt(trimmed.into())),
        None => None,
    }
}

pub(crate) fn shell_quote(value: &str) -> OutputString {
    if value.is_empty() {
        return "''".to_string();
    }
    format!("'{}'", value.replace('\'', r#"'"'"'"#))
}

fn tuple_type_body(source: &str) -> Option<&str> {
    let inner = parse_paren_body(source)?;
    (inner.0.is_empty()).then_some(inner.1)
}
