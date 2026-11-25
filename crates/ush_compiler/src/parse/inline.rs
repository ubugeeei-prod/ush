use anyhow::{Result, bail};

use super::{
    super::ast::{Statement, StatementKind},
    control,
    expr::parse_expr,
    signature,
    statement_support::{
        parse_alias, parse_shell_escape, parse_shell_statement, parse_statement_expr,
        trim_statement_terminator,
    },
};

pub(super) fn parse_inline_body(
    line_no: usize,
    source: &str,
    tail_position: bool,
) -> Result<Statement> {
    let (source, terminated) = trim_statement_terminator(source.trim());
    let tail_position = tail_position && !terminated;
    if let Some(kind) = parse_shell_escape(source)? {
        return Ok(Statement::new(line_no, kind));
    }
    if let Some(rest) = source.strip_prefix("print ") {
        return Ok(Statement::new(
            line_no,
            StatementKind::Print(parse_statement_expr(rest)?),
        ));
    }
    if let Some(rest) = source.strip_prefix("shell ") {
        return Ok(Statement::new(line_no, parse_shell_statement(rest)?));
    }
    if let Some(kind) = super::statement_support::parse_failure(source)? {
        return Ok(Statement::new(line_no, kind));
    }
    if let Some(rest) = source.strip_prefix("return ") {
        return Ok(Statement::new(
            line_no,
            StatementKind::Return(parse_statement_expr(rest)?),
        ));
    }
    if let Some(rest) = source.strip_prefix("let ") {
        return Ok(Statement::new(line_no, control::parse_let_binding(rest)?));
    }
    if let Some(rest) = source.strip_prefix("alias ") {
        return Ok(Statement::new(line_no, parse_alias(rest)?));
    }
    if let Some(rest) = source.strip_prefix("async ")
        && !rest.trim_start().starts_with('{')
    {
        return Ok(Statement::new(
            line_no,
            StatementKind::Call(signature::parse_call(rest, true)?),
        ));
    }
    if tail_position && let Ok(expr) = parse_expr(source) {
        return Ok(Statement::new(line_no, StatementKind::Expr(expr)));
    }
    if let Some(pipeline) = super::pipeline::parse_pipeline(source)? {
        return Ok(Statement::new(line_no, StatementKind::Pipeline(pipeline)));
    }
    if let Some(base) = super::super::util::strip_top_level_suffix(source, '?')
        && signature::looks_like_call(base)
    {
        return Ok(Statement::new(
            line_no,
            StatementKind::TryCall(signature::parse_call(base, false)?),
        ));
    }
    if signature::looks_like_call(source) {
        return Ok(Statement::new(
            line_no,
            StatementKind::Call(signature::parse_call(source, false)?),
        ));
    }
    if let Ok(expr) = parse_expr(source) {
        return Ok(Statement::new(line_no, StatementKind::Expr(expr)));
    }
    bail!("unsupported inline statement: {source}")
}
