mod alias;
mod fallback;
#[cfg(test)]
mod tests;

use std::{borrow::Cow, collections::BTreeMap};

use anyhow::{Result, bail};

use crate::commands;
use crate::helpers::HelperInvocation;
use alias::expand_alias;
use fallback::{contains_unquoted_keyword, needs_posix_fallback};

#[derive(Debug, Clone)]
pub enum ParsedLine {
    Empty,
    Background(String),
    Fallback(String),
    Pipeline(Pipeline),
    /// `a && b`, `a || b`, `a; b` — an and-or list whose parts are
    /// parsed (and executed) by `ush` itself rather than handed to
    /// `/bin/sh` as one opaque chunk.
    List(Vec<ListItem>),
}

/// One element of an and-or list, together with the operator that
/// joined it to the element before it.
#[derive(Debug, Clone)]
pub struct ListItem {
    pub connector: Connector,
    pub line: ParsedLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Connector {
    /// `;` — run regardless of the previous status. Also the
    /// connector of the first element.
    Always,
    /// `&&` — run only when the previous status was `0`.
    And,
    /// `||` — run only when the previous status was non-zero.
    Or,
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub raw: String,
    pub stages: Vec<Stage>,
}

#[derive(Debug, Clone)]
pub enum Stage {
    Builtin(CommandSpec),
    External(CommandSpec),
    Helper(HelperInvocation),
    Assignments(Vec<(String, String)>),
}

#[derive(Debug, Clone)]
pub struct CommandSpec {
    pub raw: String,
    pub command: String,
    pub args: Vec<String>,
    pub assignments: Vec<(String, String)>,
}

pub fn parse_line(line: &str, aliases: &BTreeMap<String, String>) -> Result<ParsedLine> {
    let source = strip_comment(line);
    let stripped = source.trim();
    if stripped.is_empty() {
        return Ok(ParsedLine::Empty);
    }

    if let Some(background) = split_background_job(stripped) {
        return Ok(ParsedLine::Background(background.to_string()));
    }

    if let Some(segments) = split_and_or(stripped) {
        let mut items = Vec::with_capacity(segments.len());
        for (connector, segment) in segments {
            items.push(ListItem {
                connector,
                line: parse_segment(segment, aliases)?,
            });
        }
        return Ok(ParsedLine::List(items));
    }

    parse_segment(stripped, aliases)
}

/// Parses one element of an and-or list: a pipeline `ush` runs
/// itself, or a chunk that still needs `/bin/sh`.
fn parse_segment(stripped: &str, aliases: &BTreeMap<String, String>) -> Result<ParsedLine> {
    if stripped.is_empty() {
        return Ok(ParsedLine::Empty);
    }

    if needs_posix_fallback(stripped) {
        return Ok(ParsedLine::Fallback(stripped.to_string()));
    }

    let mut stages = Vec::new();
    for raw_stage in split_unquoted(stripped, '|')? {
        let expanded = expand_alias(raw_stage.trim(), aliases)?;
        if let Some(helper) = HelperInvocation::parse(&expanded) {
            stages.push(Stage::Helper(helper?));
            continue;
        }

        let tokens = shell_words::split(&expanded)?;
        if tokens.is_empty() {
            continue;
        }

        let (assignments, rest) = split_assignments(tokens);
        if rest.is_empty() {
            stages.push(Stage::Assignments(assignments));
            continue;
        }

        let command = rest[0].clone();
        let args = rest[1..].to_vec();
        let spec = CommandSpec {
            raw: expanded,
            command: command.clone(),
            args,
            assignments,
        };

        if commands::is_builtin(&command) {
            stages.push(Stage::Builtin(spec));
        } else {
            stages.push(Stage::External(spec));
        }
    }

    if stages.is_empty() {
        return Ok(ParsedLine::Empty);
    }

    Ok(ParsedLine::Pipeline(Pipeline {
        raw: stripped.to_string(),
        stages,
    }))
}

/// Splits `source` into an and-or list on unquoted `;`, `&&`, and
/// `||`, or reports `None` when the line has to stay whole.
///
/// Splitting is what lets `mkdir build && cd build` reach the `cd`
/// builtin at all: handing the whole line to `/bin/sh` runs every
/// part in a child process, where `ush` builtins (`cd`, `sammary`,
/// `tasks`, the structured helpers, session aliases) do not exist.
///
/// Lines that open a POSIX compound command are left alone. `if x;
/// then y; fi` uses `;` as an *inner* separator, so splitting it
/// would hand `/bin/sh` fragments that do not parse. The same goes
/// for a leading `(`, `{`, or `!`.
fn split_and_or(source: &str) -> Option<Vec<(Connector, &str)>> {
    let trimmed = source.trim_start();
    if trimmed.starts_with('!') || trimmed.starts_with('(') || trimmed.starts_with('{') {
        return None;
    }
    if contains_unquoted_keyword(source) {
        return None;
    }

    let mut segments: Vec<(Connector, &str)> = Vec::new();
    let mut connector = Connector::Always;
    let mut start = 0usize;
    let mut single = false;
    let mut double = false;
    let mut escaped = false;
    let mut backtick = false;
    let mut depth = 0usize;

    let bytes = source.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        let ch = bytes[index] as char;
        if escaped {
            escaped = false;
            index += 1;
            continue;
        }
        match ch {
            '\\' if !single => escaped = true,
            '\'' if !double && !backtick => single = !single,
            '"' if !single && !backtick => double = !double,
            '`' if !single && !double => backtick = !backtick,
            _ if single || double || backtick => {}
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            _ if depth > 0 => {}
            '&' if bytes.get(index + 1) == Some(&b'&') => {
                segments.push((connector, source[start..index].trim()));
                connector = Connector::And;
                index += 2;
                start = index;
                continue;
            }
            '|' if bytes.get(index + 1) == Some(&b'|') => {
                segments.push((connector, source[start..index].trim()));
                connector = Connector::Or;
                index += 2;
                start = index;
                continue;
            }
            // A newline separates commands exactly like `;` does.
            // Without this, `ush -c $'cd build\nmake'` — and every
            // multi-line paste at the prompt — was parsed as one
            // enormous command.
            ';' | '\n' => {
                segments.push((connector, source[start..index].trim()));
                connector = Connector::Always;
                index += 1;
                start = index;
                continue;
            }
            _ => {}
        }
        index += 1;
    }

    if segments.is_empty() {
        return None;
    }
    if single || double || backtick || depth > 0 {
        return None;
    }

    segments.push((connector, source[start..].trim()));
    segments.retain(|(_, segment)| !segment.is_empty());
    (segments.len() > 1).then_some(segments)
}

fn split_assignments(tokens: Vec<String>) -> (Vec<(String, String)>, Vec<String>) {
    let mut assignments = Vec::new();
    let mut rest = Vec::new();
    let mut assigning = true;

    for token in tokens {
        if assigning && is_assignment(&token) {
            if let Some((name, value)) = token.split_once('=') {
                assignments.push((name.to_string(), value.to_string()));
            }
            continue;
        }

        assigning = false;
        rest.push(token);
    }

    (assignments, rest)
}

fn is_assignment(token: &str) -> bool {
    let Some((name, _)) = token.split_once('=') else {
        return false;
    };
    is_identifier(name)
}

/// Removes unquoted `#` comments from `source`.
///
/// The interactive path calls this for every keystroke-completed
/// line, so it borrows when there is nothing to strip. Input can
/// span several lines (a multi-line `-c` string, a paste at the
/// prompt), and a comment ends at its own newline — truncating the
/// rest of the input at the first `#` would silently drop every
/// command after a commented one.
fn strip_comment(source: &str) -> Cow<'_, str> {
    let Some(first) = comment_start(source, 0) else {
        return Cow::Borrowed(source);
    };

    let mut out = String::with_capacity(source.len());
    let mut cursor = 0usize;
    let mut comment = Some(first);
    while let Some(start) = comment {
        out.push_str(&source[cursor..start]);
        cursor = source[start..]
            .find('\n')
            .map_or(source.len(), |offset| start + offset);
        comment = comment_start(source, cursor);
    }
    out.push_str(&source[cursor..]);
    Cow::Owned(out)
}

/// The index of the next unquoted `#` that starts a comment, scanning
/// from `from`. Quote state is tracked from the beginning of the
/// input so a `#` inside a string that opened on an earlier line is
/// still recognised as text.
fn comment_start(source: &str, from: usize) -> Option<usize> {
    let mut single = false;
    let mut double = false;
    for (index, ch) in source.char_indices() {
        match ch {
            '\'' if !double => single = !single,
            '"' if !single => double = !double,
            '#' if !single
                && !double
                && index >= from
                && (index == 0 || source[..index].ends_with(char::is_whitespace)) =>
            {
                return Some(index);
            }
            _ => {}
        }
    }
    None
}

fn split_unquoted(source: &str, separator: char) -> Result<Vec<&str>> {
    let mut result = Vec::new();
    let mut start = 0usize;
    let mut single = false;
    let mut double = false;
    let mut escaped = false;

    for (index, ch) in source.char_indices() {
        match ch {
            '\\' if !single => escaped = !escaped,
            '\'' if !double && !escaped => single = !single,
            '"' if !single && !escaped => double = !double,
            _ if ch == separator && !single && !double && !escaped => {
                result.push(source[start..index].trim());
                start = index + ch.len_utf8();
            }
            _ => escaped = false,
        }
    }

    if single || double {
        bail!("unterminated quoted string");
    }

    result.push(source[start..].trim());
    Ok(result)
}

fn is_identifier(source: &str) -> bool {
    let mut chars = source.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn split_background_job(line: &str) -> Option<&str> {
    let mut single = false;
    let mut double = false;
    let mut escaped = false;
    let mut background_index = None;

    for (index, ch) in line.char_indices() {
        match ch {
            '\\' if !single => escaped = !escaped,
            '\'' if !double && !escaped => {
                single = !single;
                background_index = None;
                escaped = false;
            }
            '"' if !single && !escaped => {
                double = !double;
                background_index = None;
                escaped = false;
            }
            _ if single || double => escaped = false,
            '&' => {
                background_index = Some(index);
                escaped = false;
            }
            _ if ch.is_whitespace() => escaped = false,
            _ => {
                background_index = None;
                escaped = false;
            }
        }
    }

    let index = background_index?;
    let command = line[..index].trim_end();
    if command.is_empty() {
        return None;
    }
    if command.ends_with('&') {
        return None;
    }
    Some(command)
}

pub fn is_builtin(command: &str) -> bool {
    commands::is_builtin(command)
}
