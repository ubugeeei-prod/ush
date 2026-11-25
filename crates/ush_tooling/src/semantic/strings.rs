//! Interpolating strings, `$"…"` and `$"""…"""`: the text is a string
//! token, `$name` and `$env::NAME` are variables, `${` and `}` are
//! operators and the code between them is tokenized like any other code.

use super::scan::{string_end, triple_string_span};
use super::{push, tokenize_span};
use crate::token::{SemanticToken, SemanticTokenKind};

/// A string that continues onto the next line.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Multiline {
    No,
    /// `"""…"""`, raw text.
    Raw,
    /// `$"""…"""`, which interpolates.
    Template,
}

pub(super) fn tokenize_line(
    line_no: u32,
    line: &str,
    out: &mut Vec<SemanticToken>,
    multiline: Multiline,
) -> Multiline {
    let (start, closed) = match multiline {
        Multiline::No => (0, true),
        Multiline::Raw => {
            let (end, closed) = triple_string_span(line, 0);
            push(out, line_no, 0, end, SemanticTokenKind::String);
            (end, closed)
        }
        Multiline::Template => interpolated(line_no, line, 0, 0, true, out),
    };
    if !closed {
        return multiline;
    }
    tokenize_span(line_no, line, start, line.len(), out)
}

/// Tokenize the string literal starting at `index`, if one does: the index
/// after it, and the kind of string left open at the end of the line.
pub(super) fn string(
    line_no: u32,
    line: &str,
    index: usize,
    out: &mut Vec<SemanticToken>,
) -> Option<(usize, Multiline)> {
    let rest = &line[index..];
    let (next, open) = if rest.starts_with("$\"\"\"") {
        let (next, closed) = interpolated(line_no, line, index, index + 4, true, out);
        (
            next,
            if closed {
                Multiline::No
            } else {
                Multiline::Template
            },
        )
    } else if rest.starts_with("\"\"\"") {
        let (next, closed) = triple_string_span(line, index + 3);
        push(out, line_no, index, next - index, SemanticTokenKind::String);
        (
            next,
            if closed {
                Multiline::No
            } else {
                Multiline::Raw
            },
        )
    } else if rest.starts_with("$\"") {
        (
            interpolated(line_no, line, index, index + 2, false, out).0,
            Multiline::No,
        )
    } else if rest.starts_with(['"', '\'']) {
        let next = string_end(line, index);
        push(out, line_no, index, next - index, SemanticTokenKind::String);
        (next, Multiline::No)
    } else {
        return None;
    };
    Some((next, open))
}

/// Tokenize a string whose body starts at `body` (after `$"`, `$"""`, or at
/// the start of a template's continuation line) and whose first token
/// starts at `start`. Returns the index past the string and whether it
/// closed on this line.
fn interpolated(
    line_no: u32,
    line: &str,
    start: usize,
    body: usize,
    triple: bool,
    out: &mut Vec<SemanticToken>,
) -> (usize, bool) {
    let mut text_start = start;
    let mut index = body;
    while let Some(ch) = line[index..].chars().next() {
        let closes = if triple {
            line[index..].starts_with("\"\"\"").then_some(3)
        } else {
            (ch == '"').then_some(1)
        };
        if let Some(width) = closes {
            push_text(out, line_no, text_start, index + width);
            return (index + width, true);
        }
        if ch == '\\' {
            index += 1 + line[index + 1..].chars().next().map_or(0, char::len_utf8);
            continue;
        }
        if line[index..].starts_with("${") {
            push_text(out, line_no, text_start, index);
            push(out, line_no, index, 2, SemanticTokenKind::Operator);
            let close = hole_end(line, index + 2);
            tokenize_span(line_no, line, index + 2, close, out);
            if close < line.len() {
                push(out, line_no, close, 1, SemanticTokenKind::Operator);
            }
            index = (close + 1).min(line.len());
            text_start = index;
            continue;
        }
        if ch == '$' {
            let name = index + 1 + usize::from(line[index + 1..].starts_with("env::")) * 5;
            let end = super::scan::take_while(line, name, super::scan::is_ident);
            if end > name {
                push_text(out, line_no, text_start, index);
                push(
                    out,
                    line_no,
                    index,
                    end - index,
                    SemanticTokenKind::Variable,
                );
                index = end;
                text_start = index;
                continue;
            }
        }
        index += ch.len_utf8();
    }
    push_text(out, line_no, text_start, line.len());
    (line.len(), false)
}

fn push_text(out: &mut Vec<SemanticToken>, line_no: u32, start: usize, end: usize) {
    if end > start {
        push(out, line_no, start, end - start, SemanticTokenKind::String);
    }
}

/// The index of the `}` closing a hole whose code starts at `start`, or the
/// line's length when it does not close on this line.
pub(super) fn hole_end(line: &str, start: usize) -> usize {
    let bytes = line.as_bytes();
    let mut depth = 0usize;
    let mut index = start;
    while index < line.len() {
        match bytes[index] {
            b'}' if depth == 0 => return index,
            b'{' => depth += 1,
            b'}' => depth -= 1,
            b'\'' => {
                index = line[index + 1..]
                    .find('\'')
                    .map_or(line.len(), |offset| index + offset + 1);
            }
            b'"' if index > start && bytes[index - 1] == b'$' => {
                index = skip_double(line, index + 1, true);
            }
            b'"' => index = skip_double(line, index + 1, false),
            _ => {}
        }
        index += 1;
    }
    line.len()
}

/// The index of the closing quote of a `"…"` string whose body starts at
/// `start`, skipping escapes and, in a `$"…"`, nested holes.
fn skip_double(line: &str, start: usize, interpolating: bool) -> usize {
    let bytes = line.as_bytes();
    let mut index = start;
    while index < line.len() {
        match bytes[index] {
            b'\\' => index += 1,
            b'"' => return index,
            b'$' if interpolating && bytes.get(index + 1) == Some(&b'{') => {
                index = hole_end(line, index + 2);
            }
            _ => {}
        }
        index += 1;
    }
    line.len()
}
