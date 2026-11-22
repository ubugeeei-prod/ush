use anyhow::{Result, anyhow, bail};

use super::{
    super::{
        ast::{Expr, StatementKind},
        util::{split_once_top_level, strip_top_level_suffix},
    },
    SourceLine,
    expr::parse_expr,
};
use crate::scan;

pub(super) fn split_assignment(source: &str) -> Option<(&str, &str)> {
    let (name, expr) = split_once_top_level(source, '=')?;
    Some((name.trim(), expr.trim()))
}

pub(super) fn parse_alias(source: &str) -> Result<StatementKind> {
    let (name, value) = split_assignment(source).ok_or_else(|| anyhow!("invalid alias binding"))?;
    Ok(StatementKind::Alias {
        name: name.into(),
        value: parse_expr(value)?,
    })
}

pub(super) fn parse_statement_expr(source: &str) -> Result<Expr> {
    if source.trim_start().starts_with("$(") || source.trim_start().starts_with("$bin::") {
        return parse_expr(source.trim());
    }
    parse_expr(source.trim().strip_prefix('$').unwrap_or(source).trim())
}

pub(super) fn parse_shell_statement(source: &str) -> Result<StatementKind> {
    if let Some(inner) = strip_top_level_suffix(source, '?') {
        return Ok(StatementKind::TryShell(parse_statement_expr(inner)?));
    }
    Ok(StatementKind::Shell(parse_statement_expr(source)?))
}

pub(super) fn parse_shell_escape(source: &str) -> Result<Option<StatementKind>> {
    let Some(rest) = source.strip_prefix('$') else {
        return Ok(None);
    };
    if !rest.is_empty() && !rest.starts_with(char::is_whitespace) {
        return Ok(None);
    }
    let command = rest.trim();
    if command.is_empty() {
        bail!("shell escape requires a command");
    }
    if let Some(inner) = strip_top_level_suffix(command, '?') {
        if inner.is_empty() {
            bail!("shell escape requires a command");
        }
        return Ok(Some(StatementKind::TryShell(Expr::String(inner.into()))));
    }
    Ok(Some(StatementKind::Shell(Expr::String(command.into()))))
}

pub(super) fn trim_statement_terminator(source: &str) -> (&str, bool) {
    match strip_top_level_suffix(source, ';') {
        Some(inner) => (inner, true),
        None => (source.trim(), false),
    }
}

pub(super) fn is_tail_position(lines: &[SourceLine<'_>], cursor: usize) -> bool {
    let mut index = cursor;
    let mut depth = 0isize;
    let mut started = false;

    while index < lines.len() {
        let trimmed = lines[index].1.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            index += 1;
            continue;
        }
        started = true;
        depth += brace_delta(trimmed);
        index += 1;
        if depth <= 0 && !next_is_else(lines, index) {
            break;
        }
    }

    if !started {
        return false;
    }

    while index < lines.len() {
        let trimmed = lines[index].1.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            index += 1;
            continue;
        }
        return matches!(trimmed, "}" | "};");
    }

    true
}

fn next_is_else(lines: &[SourceLine<'_>], mut index: usize) -> bool {
    while index < lines.len() {
        let trimmed = lines[index].1.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            index += 1;
            continue;
        }
        return trimmed == "else {" || trimmed.starts_with("else if ");
    }
    false
}

fn brace_delta(line: &str) -> isize {
    scan::brace_delta(line)
}

pub(super) fn parse_failure(source: &str) -> Result<Option<StatementKind>> {
    if source == "fail ()" {
        return Ok(None);
    }
    if let Some(rest) = source
        .strip_prefix("fail ")
        .or_else(|| source.strip_prefix("raise "))
    {
        return Ok(Some(StatementKind::Raise(parse_statement_expr(rest)?)));
    }
    if let Some(args) = source
        .strip_prefix("fail(")
        .and_then(|text| text.strip_suffix(')'))
    {
        let parts = crate::util::split_top_level(args, ',');
        if parts.len() == 2 {
            return Ok(Some(StatementKind::Fail {
                status: parse_expr(parts[0])?,
                message: parse_expr(parts[1])?,
            }));
        }
    }
    Ok(None)
}
