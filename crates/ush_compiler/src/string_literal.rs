//! String literals: `'raw'`, `"escaped"`, `"""raw multi-line"""`, and the
//! two interpolating forms, `$"…"` and `$"""…"""`.
//!
//! An interpolating string reads variables the way CLI-style words do:
//! `$name` and `${name}` read a binding, `$env::NAME` reads the
//! environment, and `${expr}` evaluates any expression.

use anyhow::{Result, bail};

use crate::scan::{ScanState, advance, starts_interpolated, starts_template, starts_triple_quote};
use crate::types::{AstString as String, AstVec as Vec};
use crate::util::is_identifier;

/// One run of an interpolating string: decoded text, or the source of the
/// expression a hole evaluates.
#[derive(Debug)]
pub(crate) enum Piece {
    Text(String),
    Hole(String),
}

/// A literal that does not interpolate: `'…'`, `"…"` or `"""…"""`.
pub(crate) fn parse_string_literal(source: &str) -> Option<String> {
    if source.len() >= 6
        && starts_triple_quote(source, 0)
        && starts_triple_quote(source, source.len() - 3)
    {
        return Some(dedent(&source[3..source.len() - 3]));
    }
    if source.len() < 2 {
        return None;
    }
    let quote = source.as_bytes()[0];
    if !matches!(quote, b'\'' | b'"') || source.as_bytes()[source.len() - 1] != quote {
        return None;
    }
    let body = &source[1..source.len() - 1];
    if quote == b'\'' {
        return Some(body.into());
    }
    let mut out = String::default();
    let mut chars = body.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            decode_escape(chars.next(), false, &mut out);
        } else {
            out.push(ch);
        }
    }
    Some(out)
}

/// `\n`, `\r`, `\t`, `\"` and `\\`, plus `\$` in an interpolating string.
/// Any other escape keeps its backslash.
fn decode_escape(next: Option<char>, interpolating: bool, out: &mut String) {
    match next {
        Some('n') => out.push('\n'),
        Some('r') => out.push('\r'),
        Some('t') => out.push('\t'),
        Some(quoted @ ('"' | '\\')) => out.push(quoted),
        Some('$') if interpolating => out.push('$'),
        Some(other) => {
            out.push('\\');
            out.push(other);
        }
        None => out.push('\\'),
    }
}

/// The pieces of a `$"…"` string or a `$"""…"""` template, or `None` when
/// `source` is neither.
pub(crate) fn parse_interpolated(source: &str) -> Option<Result<Vec<Piece>>> {
    let template = starts_template(source, 0);
    let body = if template && source.len() >= 7 && starts_triple_quote(source, source.len() - 3) {
        &source[4..source.len() - 3]
    } else if starts_interpolated(source, 0) && source.len() >= 3 && source.ends_with('"') {
        &source[2..source.len() - 1]
    } else {
        return None;
    };
    if ends_early(source) {
        return None;
    }
    if !template {
        return Some(pieces(body));
    }
    // Dedent the lines first, so holes and escapes see the final text.
    let mut out = Vec::new();
    for (index, line) in dedented_lines(body).iter().enumerate() {
        if index > 0 {
            push_text(&mut out, "\n");
        }
        match pieces(line) {
            Ok(line_pieces) => {
                for piece in line_pieces {
                    match piece {
                        Piece::Text(text) => push_text(&mut out, &text),
                        hole => out.push(hole),
                    }
                }
            }
            Err(error) => return Some(Err(error)),
        }
    }
    Some(Ok(out))
}

/// Whether the string that starts `source` closes before its last byte, as
/// in `$"a" + $"b"`. A string whose `${` never closes runs to the end, so
/// it is parsed and reported as one literal.
fn ends_early(source: &str) -> bool {
    let mut state = ScanState::default();
    let mut index = advance(source, 0, &mut state);
    while index < source.len() {
        if !state.in_string() {
            return true;
        }
        index = advance(source, index, &mut state);
    }
    false
}

fn pieces(body: &str) -> Result<Vec<Piece>> {
    let mut out = Vec::new();
    let mut text = String::default();
    let mut index = 0usize;
    while let Some(ch) = body[index..].chars().next() {
        index += ch.len_utf8();
        if ch == '\\' {
            let next = body[index..].chars().next();
            decode_escape(next, true, &mut text);
            index += next.map_or(0, char::len_utf8);
            continue;
        }
        if ch != '$' {
            text.push(ch);
            continue;
        }
        let (code, next) = hole(body, index)?;
        if !text.is_empty() {
            out.push(Piece::Text(core::mem::take(&mut text)));
        }
        out.push(Piece::Hole(code));
        index = next;
    }
    if !text.is_empty() {
        out.push(Piece::Text(text));
    }
    Ok(out)
}

/// The expression source of the hole whose `$` ends just before `start`,
/// and the index after the hole.
fn hole(body: &str, start: usize) -> Result<(String, usize)> {
    let rest = &body[start..];
    if rest.starts_with('{') {
        let end = hole_end(body, start + 1)?;
        let code = body[start + 1..end].trim();
        if code.is_empty() {
            bail!("empty interpolation `${{}}`: write a name or an expression between the braces");
        }
        // `${name}` reads a binding exactly as `$name` does.
        let code = if is_identifier(code) || code.starts_with("env::") {
            format!("${code}").into()
        } else {
            code.into()
        };
        return Ok((code, end + 1));
    }
    let name = rest.strip_prefix("env::").unwrap_or(rest);
    let length = name
        .find(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
        .unwrap_or(name.len());
    if !is_identifier(&name[..length]) {
        bail!(
            "`$` in an interpolating string starts `$name`, `${{expression}}` or `$env::NAME`; write `\\$` for a dollar sign"
        );
    }
    let consumed = rest.len() - name.len() + length;
    Ok((format!("${}", &rest[..consumed]).into(), start + consumed))
}

/// The index of the `}` that closes a hole whose code starts at `start`.
fn hole_end(body: &str, start: usize) -> Result<usize> {
    let mut state = ScanState::default();
    let mut index = start;
    while index < body.len() {
        if state.top_level() && body.as_bytes()[index] == b'}' {
            return Ok(index);
        }
        index = advance(body, index, &mut state);
    }
    bail!("unterminated interpolation: `${{` needs a closing `}}` (write `\\$` for a dollar sign)")
}

fn push_text(pieces: &mut Vec<Piece>, text: &str) {
    if let Some(Piece::Text(last)) = pieces.last_mut() {
        last.push_str(text);
    } else if !text.is_empty() {
        pieces.push(Piece::Text(text.into()));
    }
}

/// A multi-line body without its first line break, its last blank line
/// and the indentation its non-blank lines share.
fn dedented_lines(body: &str) -> Vec<&str> {
    let body = body.strip_prefix('\n').unwrap_or(body);
    let mut lines = body.split('\n').collect::<Vec<_>>();
    if lines.last().is_some_and(|line| line.trim().is_empty()) {
        lines.pop();
    }
    let indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.chars()
                .take_while(|ch| matches!(ch, ' ' | '\t'))
                .count()
        })
        .min()
        .unwrap_or(0);
    lines
        .into_iter()
        .map(|line| strip_indent(line, indent))
        .collect()
}

fn dedent(body: &str) -> String {
    dedented_lines(body).join("\n").into()
}

fn strip_indent(line: &str, indent: usize) -> &str {
    let mut offset = 0usize;
    for (removed, ch) in line.chars().enumerate() {
        if removed == indent || !matches!(ch, ' ' | '\t') {
            break;
        }
        offset += ch.len_utf8();
    }
    &line[offset..]
}

#[cfg(test)]
mod tests;
