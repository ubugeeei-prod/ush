mod alias;
mod fallback;
#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

use anyhow::{Result, bail};

use crate::commands;
use crate::helpers::HelperInvocation;
use alias::expand_alias;
use fallback::needs_posix_fallback;

#[derive(Debug, Clone)]
pub enum ParsedLine {
    Empty,
    Background(String),
    Fallback(String),
    Pipeline(Pipeline),
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
    let stripped = strip_comment(line).trim();
    if stripped.is_empty() {
        return Ok(ParsedLine::Empty);
    }

    if let Some(background) = split_background_job(stripped) {
        return Ok(ParsedLine::Background(background.to_string()));
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

/// Borrows the part of `line` before an unquoted `#` comment. The
/// interactive path calls this for every keystroke-completed line, so
/// it hands back a slice rather than a fresh `String`.
fn strip_comment(line: &str) -> &str {
    let mut single = false;
    let mut double = false;
    for (index, ch) in line.char_indices() {
        match ch {
            '\'' if !double => single = !single,
            '"' if !single => double = !double,
            '#' if !single
                && !double
                && (index == 0 || line[..index].ends_with(char::is_whitespace)) =>
            {
                return &line[..index];
            }
            _ => {}
        }
    }
    line
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
