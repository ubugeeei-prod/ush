mod alias;
mod comment;
mod fallback;
mod list;
#[cfg(test)]
mod tests;
mod tokens;

use std::collections::BTreeMap;

use anyhow::{Result, bail};

use crate::commands;
use crate::helpers::HelperInvocation;
use alias::expand_alias;
use comment::strip_comment;
use fallback::needs_posix_fallback_with;
pub use list::{Connector, ListItem};
use list::{Split, split_and_or};
use tokens::split_assignments;

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

    match split_and_or(stripped) {
        Split::List(segments) => {
            let mut items = Vec::with_capacity(segments.len());
            for (connector, segment) in segments {
                items.push(ListItem {
                    connector,
                    // Splitting only happens on a line with no
                    // unquoted POSIX keyword, so no segment has one
                    // either and the fallback check can skip its own
                    // keyword scan.
                    line: parse_segment(segment, aliases, Some(false))?,
                });
            }
            Ok(ParsedLine::List(items))
        }
        Split::Whole { keyword } => parse_segment(stripped, aliases, keyword),
    }
}

/// Parses one element of an and-or list: a pipeline `ush` runs
/// itself, or a chunk that still needs `/bin/sh`.
fn parse_segment(
    stripped: &str,
    aliases: &BTreeMap<String, String>,
    keyword: Option<bool>,
) -> Result<ParsedLine> {
    if stripped.is_empty() {
        return Ok(ParsedLine::Empty);
    }

    if needs_posix_fallback_with(stripped, keyword) {
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
