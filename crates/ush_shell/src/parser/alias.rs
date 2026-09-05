use std::collections::BTreeMap;

use anyhow::{Result, bail};

use super::tokens::is_assignment;

/// Upper bound on alias chains such as `a -> b -> c`. Self- and
/// mutually recursive chains stop earlier, through `expanded`.
const MAX_ALIAS_DEPTH: usize = 8;

pub(super) fn expand_alias(stage: &str, aliases: &BTreeMap<String, String>) -> Result<String> {
    if aliases.is_empty() {
        return Ok(stage.to_string());
    }

    let mut current = stage.to_string();
    // POSIX forbids re-expanding a name that is already being
    // expanded, which is what makes `alias ls='ls --color'` resolve
    // to `ls --color` instead of repeating the flag until the depth
    // limit runs out.
    let mut expanded: Vec<String> = Vec::new();

    for _ in 0..MAX_ALIAS_DEPTH {
        let Some(span) = alias_expansion_span(&current)? else {
            return Ok(current);
        };
        let word = &current[span.start..span.end];
        let Some(alias) = aliases.get(word) else {
            return Ok(current);
        };
        if expanded.iter().any(|seen| seen == word) {
            return Ok(current);
        }
        let replaced = format!(
            "{}{}{}",
            &current[..span.start],
            alias,
            &current[span.end..]
        );
        expanded.push(word.to_string());
        current = replaced;
    }
    Ok(current)
}

#[derive(Clone, Copy, Debug)]
struct ShellWordSpan {
    start: usize,
    end: usize,
    quoted: bool,
}

/// Locates the command word an alias would replace, skipping any
/// leading `NAME=value` assignments. A quoted command word suppresses
/// expansion, matching POSIX.
fn alias_expansion_span(stage: &str) -> Result<Option<ShellWordSpan>> {
    let mut cursor = skip_whitespace(stage, 0);
    while let Some(span) = next_shell_word_span(stage, cursor)? {
        let raw = &stage[span.start..span.end];
        if !span.quoted {
            // An unquoted word carries no escapes, so it already is
            // its own parsed form — no tokenizer round trip needed.
            if is_assignment(raw) {
                cursor = skip_whitespace(stage, span.end);
                continue;
            }
            return Ok(Some(span));
        }
        if is_assignment(&parse_single_shell_word(raw)?) {
            cursor = skip_whitespace(stage, span.end);
            continue;
        }
        return Ok(None);
    }
    Ok(None)
}

fn skip_whitespace(source: &str, from: usize) -> usize {
    source[from..]
        .char_indices()
        .find_map(|(offset, ch)| (!ch.is_whitespace()).then_some(from + offset))
        .unwrap_or(source.len())
}

fn next_shell_word_span(source: &str, from: usize) -> Result<Option<ShellWordSpan>> {
    let start = skip_whitespace(source, from);
    if start >= source.len() {
        return Ok(None);
    }

    let mut single = false;
    let mut double = false;
    let mut escaped = false;
    let mut quoted = false;

    for (offset, ch) in source[start..].char_indices() {
        let index = start + offset;
        match ch {
            '\\' if !single => {
                escaped = !escaped;
                quoted = true;
            }
            '\'' if !double && !escaped => {
                single = !single;
                escaped = false;
                quoted = true;
            }
            '"' if !single && !escaped => {
                double = !double;
                escaped = false;
                quoted = true;
            }
            _ if ch.is_whitespace() && !single && !double && !escaped => {
                return Ok(Some(ShellWordSpan {
                    start,
                    end: index,
                    quoted,
                }));
            }
            _ => escaped = false,
        }
    }

    if single || double {
        bail!("unterminated quoted string");
    }

    Ok(Some(ShellWordSpan {
        start,
        end: source.len(),
        quoted,
    }))
}

fn parse_single_shell_word(raw: &str) -> Result<String> {
    let mut tokens = shell_words::split(raw)?;
    match tokens.len() {
        1 => Ok(tokens.remove(0)),
        _ => bail!("invalid shell word: {raw}"),
    }
}
