//! Atoms introduced with the text and data features: interpolating
//! strings, `$env::NAME` environment reads and `list[index]`.

use alloc::boxed::Box;

use anyhow::{Context, Result, bail};

use super::expr::{parse_atom, parse_expr};
use crate::{
    ast::Expr,
    scan::{ScanState, advance},
    string_literal::{Piece, parse_interpolated},
    types::HeapVec as Vec,
    util::{is_identifier, split_top_level_whitespace},
};

/// `$"…"` or `$"""…"""`: a plain `Expr::String` without holes, otherwise
/// an `Expr::Template` whose holes are parsed as expressions.
pub(super) fn parse_string(source: &str) -> Option<Result<Expr>> {
    let parsed = parse_interpolated(source)?.and_then(template);
    Some(parsed.map_err(|error| anyhow::Error::msg(BadString(format!("{error:#}")))))
}

/// A malformed string literal. It is reported as it is, even where the
/// parser would otherwise try another reading of the line.
#[derive(Debug)]
pub(super) struct BadString(alloc::string::String);

impl core::fmt::Display for BadString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

/// The error for a statement no form accepts: a malformed string literal
/// inside it when there is one, since that is what needs fixing.
pub(super) fn unsupported(statement: &str, error: &anyhow::Error) -> anyhow::Error {
    let call = super::signature::parse_call(statement, false).err();
    match call
        .iter()
        .chain([error])
        .find_map(|e| e.downcast_ref::<BadString>())
    {
        Some(bad) => anyhow::anyhow!("{bad}"),
        None => anyhow::anyhow!("unsupported syntax: {statement}"),
    }
}

fn template(pieces: crate::types::AstVec<Piece>) -> Result<Expr> {
    if !pieces.iter().any(|piece| matches!(piece, Piece::Hole(_))) {
        let text = pieces
            .into_iter()
            .next()
            .map_or_else(Default::default, |piece| match piece {
                Piece::Text(text) => text,
                Piece::Hole(_) => unreachable!("checked above"),
            });
        return Ok(Expr::String(text));
    }
    let mut parts = Vec::with_capacity(pieces.len());
    for piece in pieces {
        parts.push(match piece {
            Piece::Text(text) => Expr::String(text),
            Piece::Hole(source) => parse_expr(&source).with_context(|| {
                format!("invalid interpolation `{source}` (write `\\$` for a dollar sign)")
            })?,
        });
    }
    Ok(Expr::Template(parts))
}

/// `$env::NAME`, an environment variable read. `$env::` names the process
/// environment the way `$bin::` names the command catalog. Anything after
/// the name makes the atom a CLI-style word, as `$name.txt` is.
pub(super) fn parse_env(source: &str) -> Option<Result<Expr>> {
    let name = source.strip_prefix("$env::")?;
    is_identifier(name).then(|| env_read(name))
}

/// Read the environment variable `name`.
pub(super) fn env_read(name: &str) -> Result<Expr> {
    if !is_identifier(name) {
        bail!("`$env::{name}`: write an environment variable name, as in `$env::HOME`");
    }
    if name.starts_with("__ush") {
        bail!("`$env::{name}`: names starting with `__ush` are reserved");
    }
    Ok(Expr::Env(name.into()))
}

/// `base[index]`, where `base` is one word written right before the `[`.
pub(super) fn parse_index(source: &str) -> Option<Result<Expr>> {
    let (base, index) = split_index(source)?;
    Some(index_expr(base, index))
}

fn index_expr(base: &str, index: &str) -> Result<Expr> {
    if index.trim().is_empty() {
        bail!("`{base}[]` needs an index");
    }
    Ok(Expr::Index {
        base: Box::new(parse_atom(base)?),
        index: Box::new(parse_expr(index)?),
    })
}

fn split_index(source: &str) -> Option<(&str, &str)> {
    if !source.ends_with(']') {
        return None;
    }
    let bytes = source.as_bytes();
    let mut state = ScanState::default();
    let (mut open, mut index) = (None, 0usize);
    while index < source.len() {
        if state.top_level() && bytes[index] == b'[' {
            open = Some(index);
        }
        index = advance(source, index, &mut state);
    }
    let open = open.filter(|&open| open > 0)?;
    let base = &source[..open];
    let attached = base
        .chars()
        .last()
        .is_some_and(|ch| ch == '_' || ch == ')' || ch == ']' || ch.is_ascii_alphanumeric());
    if !attached || split_top_level_whitespace(base).len() != 1 || !closes_at_end(source, open) {
        return None;
    }
    Some((base, &source[open + 1..source.len() - 1]))
}

/// Whether the bracket opened at `open` closes at the last byte.
fn closes_at_end(source: &str, open: usize) -> bool {
    let mut state = ScanState::default();
    let mut index = advance(source, open, &mut state);
    while index < source.len() {
        if state.top_level() {
            return false;
        }
        index = advance(source, index, &mut state);
    }
    state.top_level()
}
