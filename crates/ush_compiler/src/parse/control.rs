use anyhow::{Result, anyhow, bail};

use super::{
    super::ast::{Expr, IfBranch, Statement, StatementKind},
    SourceLine, condition,
    declaration_support::finish_block,
    expr::parse_expr,
    signature,
};
use crate::types::{AstString as String, HeapVec as Vec};

pub(super) fn parse_if(
    _line_no: usize,
    trimmed: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
    returns_value: bool,
) -> Result<StatementKind> {
    let condition = parse_header(trimmed, "if ")?;
    *cursor += 1;
    let then_body = super::statement::parse_block(lines, cursor, None, returns_value)?;
    let terminated = finish_block(lines, cursor, "if body")?;
    let else_body = parse_else(lines, cursor, returns_value)?;
    Ok(StatementKind::If {
        branch: IfBranch {
            condition: condition::parse_condition(condition)?,
            then_body,
            else_body,
        },
        returns_value: returns_value && !terminated,
    })
}

pub(super) fn parse_let_binding(source: &str) -> Result<StatementKind> {
    let (name, expr) = super::statement_support::split_assignment(source)
        .ok_or_else(|| anyhow!("invalid let binding"))?;
    super::path::parse_identifier(name)?;
    if let Some(rest) = expr.strip_prefix("async ")
        && !rest.trim_start().starts_with('{')
    {
        return Ok(StatementKind::Spawn {
            name: name.into(),
            call: signature::parse_call(rest, true)?,
        });
    }
    if let Some(task) = signature::parse_await_task(expr)? {
        return Ok(StatementKind::Await {
            name: name.into(),
            task,
        });
    }
    Ok(StatementKind::Let {
        name: name.into(),
        expr: parse_expr(expr)?,
    })
}

pub(super) fn parse_let_binding_block(
    source: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<StatementKind> {
    let (name, expr) = super::statement_support::split_assignment(source)
        .ok_or_else(|| anyhow!("invalid let binding"))?;
    if expr != "async {" {
        return parse_let_binding(source);
    }

    *cursor += 1;
    let body = super::statement::parse_block(lines, cursor, None, true)?;
    let _ = finish_block(lines, cursor, "async block")?;
    Ok(StatementKind::Let {
        name: name.into(),
        expr: Expr::AsyncBlock(body),
    })
}

pub(super) fn parse_while(
    trimmed: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<StatementKind> {
    let condition = condition::parse_condition(parse_header(trimmed, "while ")?)?;
    *cursor += 1;
    let body = super::statement::parse_block(lines, cursor, None, false)?;
    let _ = finish_block(lines, cursor, "while body")?;
    Ok(StatementKind::While { condition, body })
}

pub(super) fn parse_for(
    trimmed: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<StatementKind> {
    let source = parse_header(trimmed, "for ")?;
    let (name, expr) = source
        .split_once(" in ")
        .ok_or_else(|| anyhow!("expected `for name in expr {{`"))?;
    let name = parse_name(name)?;
    *cursor += 1;
    let body = super::statement::parse_block(lines, cursor, None, false)?;
    let _ = finish_block(lines, cursor, "for body")?;
    Ok(StatementKind::For {
        name,
        iterable: parse_expr(expr)?,
        body,
    })
}

pub(super) fn parse_loop(
    trimmed: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<StatementKind> {
    if trimmed != "loop {" {
        bail!("expected `loop {{`");
    }
    *cursor += 1;
    let body = super::statement::parse_block(lines, cursor, None, false)?;
    let _ = finish_block(lines, cursor, "loop body")?;
    Ok(StatementKind::Loop { body })
}

fn parse_else(
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
    returns_value: bool,
) -> Result<Option<Vec<Statement>>> {
    let Some((index, line_no, trimmed)) = next_meaningful(lines, *cursor) else {
        return Ok(None);
    };
    if let Some(rest) = trimmed.strip_prefix("else if ") {
        *cursor = index;
        let nested = parse_if(line_no, &format!("if {rest}"), lines, cursor, returns_value)?;
        return Ok(Some(
            vec![Statement::new(line_no, nested)].into_iter().collect(),
        ));
    }
    if trimmed != "else {" {
        return Ok(None);
    }
    let mut nested_cursor = index + 1;
    let body = super::statement::parse_block(lines, &mut nested_cursor, None, returns_value)?;
    let _ = finish_block(lines, &mut nested_cursor, "else body")?;
    *cursor = nested_cursor;
    Ok(Some(body))
}

fn next_meaningful<'a>(
    lines: &'a [SourceLine<'a>],
    mut cursor: usize,
) -> Option<(usize, usize, &'a str)> {
    while cursor < lines.len() {
        let (line_no, line) = &lines[cursor];
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            return Some((cursor, *line_no, trimmed));
        }
        cursor += 1;
    }
    None
}

fn parse_header<'a>(trimmed: &'a str, keyword: &str) -> Result<&'a str> {
    trimmed
        .strip_prefix(keyword)
        .and_then(|rest| rest.strip_suffix('{'))
        .map(str::trim)
        .ok_or_else(|| anyhow!("expected `{keyword}... {{`"))
}

fn parse_name(source: &str) -> Result<String> {
    let name = source.trim();
    if super::super::util::is_identifier(name) {
        Ok(name.into())
    } else {
        bail!("invalid identifier: {name}")
    }
}
