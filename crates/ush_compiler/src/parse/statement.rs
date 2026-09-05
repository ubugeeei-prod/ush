use anyhow::{Context, Result, bail};

use super::{
    super::{
        ast::{Attribute, Expr, Statement, StatementKind},
        util::strip_top_level_suffix,
    },
    SourceLine, attr, control, declaration,
    expr::parse_expr,
    handle, signature,
    statement_support::{
        is_tail_position, parse_alias, parse_shell_escape, parse_shell_statement,
        parse_statement_expr, trim_statement_terminator,
    },
};
use crate::types::HeapVec as Vec;

pub(super) fn parse_block(
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
    allow_declarations: bool,
    allow_tail_expr: bool,
) -> Result<Vec<Statement>> {
    let mut statements = Vec::new();
    let mut attrs = Vec::new();
    while *cursor < lines.len() {
        let (line_no, line) = &lines[*cursor];
        let trimmed = line.trim();
        if trimmed.starts_with("#[") {
            attrs.push(
                attr::parse_attribute_line(trimmed).with_context(|| format!("line {line_no}"))?,
            );
            *cursor += 1;
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') {
            *cursor += 1;
            continue;
        }
        // `} with <effect> {` closes this block and opens a handler
        // arm, so the block ends here and `parse_handle` reads the
        // rest of the line.
        if trimmed == "}" || trimmed.starts_with("} with ") {
            if !attrs.is_empty() {
                bail!("line {line_no}: dangling attributes before closing brace");
            }
            break;
        }
        let (trimmed, terminated) = trim_statement_terminator(trimmed);
        let tail_position = allow_tail_expr && !terminated && is_tail_position(lines, *cursor);
        let statement = parse_statement(
            *line_no,
            trimmed,
            lines,
            cursor,
            allow_declarations,
            &attrs,
            tail_position,
        )
        .with_context(|| format!("line {line_no}"))?;
        if !attrs.is_empty() && !accepts_attributes(&statement) {
            bail!("line {line_no}: attributes are only valid on declarations");
        }
        if !is_block_statement(&statement) {
            *cursor += 1;
        }
        attrs.clear();
        statements.push(statement);
    }
    Ok(statements)
}

fn parse_statement(
    line_no: usize,
    trimmed: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
    allow_declarations: bool,
    attrs: &[Attribute],
    tail_position: bool,
) -> Result<Statement> {
    if let Some(kind) = parse_shell_escape(trimmed)? {
        return Ok(Statement::new(line_no, kind));
    }
    if allow_declarations
        && let Some(kind) =
            declaration::parse_declaration(line_no, trimmed, lines, cursor, attrs, tail_position)?
    {
        return Ok(Statement::new(line_no, kind));
    }
    if let Some(rest) = trimmed.strip_prefix("let ") {
        return Ok(Statement::new(
            line_no,
            control::parse_let_binding_block(rest, lines, cursor)?,
        ));
    }
    if trimmed.starts_with("if ") {
        return Ok(Statement::new(
            line_no,
            control::parse_if(line_no, trimmed, lines, cursor, tail_position)?,
        ));
    }
    if trimmed.starts_with("while ") {
        return Ok(Statement::new(
            line_no,
            control::parse_while(trimmed, lines, cursor)?,
        ));
    }
    if trimmed.starts_with("for ") {
        return Ok(Statement::new(
            line_no,
            control::parse_for(trimmed, lines, cursor)?,
        ));
    }
    if trimmed == "try {" {
        return Ok(Statement::new(
            line_no,
            handle::parse_handle(lines, cursor)?,
        ));
    }
    if let Some(rest) = trimmed.strip_prefix("do ") {
        // `do log("hi")` lowers to a plain call: the effect
        // declaration generates a shell function of that name which
        // dispatches to whichever handler is installed. `do` is
        // required at the call site so performing an effect is
        // visible in the source, the way it is in Effekt.
        return Ok(Statement::new(
            line_no,
            StatementKind::Call(signature::parse_call(rest.trim(), false)?),
        ));
    }
    if trimmed == "loop {" {
        return Ok(Statement::new(
            line_no,
            control::parse_loop(trimmed, lines, cursor)?,
        ));
    }
    if trimmed == "break" {
        return Ok(Statement::new(line_no, StatementKind::Break));
    }
    if trimmed == "continue" {
        return Ok(Statement::new(line_no, StatementKind::Continue));
    }
    if let Some(rest) = trimmed.strip_prefix("print ") {
        return Ok(Statement::new(
            line_no,
            StatementKind::Print(parse_statement_expr(rest)?),
        ));
    }
    if let Some(rest) = trimmed.strip_prefix("shell ") {
        return Ok(Statement::new(line_no, parse_shell_statement(rest)?));
    }
    if let Some(rest) = trimmed.strip_prefix("raise ") {
        return Ok(Statement::new(
            line_no,
            StatementKind::Raise(parse_statement_expr(rest)?),
        ));
    }
    if let Some(rest) = trimmed.strip_prefix("return ") {
        return Ok(Statement::new(
            line_no,
            StatementKind::Return(parse_statement_expr(rest)?),
        ));
    }
    if let Some(rest) = trimmed.strip_prefix("alias ") {
        return Ok(Statement::new(line_no, parse_alias(rest)?));
    }
    if let Some(rest) = trimmed.strip_prefix("async ")
        && !rest.trim_start().starts_with('{')
    {
        return Ok(Statement::new(
            line_no,
            StatementKind::Call(signature::parse_call(rest, true)?),
        ));
    }
    if let Some(subject) = trimmed.strip_prefix("match ") {
        return Ok(Statement::new(
            line_no,
            declaration::parse_match(subject, lines, cursor, tail_position)?,
        ));
    }
    if tail_position && let Ok(expr) = parse_expr(trimmed) {
        return Ok(Statement::new(line_no, StatementKind::Expr(expr)));
    }
    if let Some(base) = strip_top_level_suffix(trimmed, '?')
        && signature::looks_like_call(base)
    {
        return Ok(Statement::new(
            line_no,
            StatementKind::TryCall(signature::parse_call(base, false)?),
        ));
    }
    if signature::looks_like_call(trimmed) {
        return Ok(Statement::new(
            line_no,
            StatementKind::Call(signature::parse_call(trimmed, false)?),
        ));
    }
    if let Ok(expr) = parse_expr(trimmed) {
        return Ok(Statement::new(line_no, StatementKind::Expr(expr)));
    }
    bail!("unsupported syntax: {trimmed}")
}

fn is_block_statement(statement: &Statement) -> bool {
    matches!(
        statement.kind,
        StatementKind::Enum(_)
            | StatementKind::Trait(_)
            | StatementKind::Impl(_)
            | StatementKind::Function(_)
            | StatementKind::Match { .. }
            | StatementKind::If { .. }
            | StatementKind::While { .. }
            | StatementKind::For { .. }
            | StatementKind::Loop { .. }
            | StatementKind::Handle { .. }
            | StatementKind::Let {
                expr: Expr::AsyncBlock(_),
                ..
            }
    )
}

fn accepts_attributes(statement: &Statement) -> bool {
    matches!(statement.kind, StatementKind::Function(_))
}
