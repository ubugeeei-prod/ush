use super::{SourceLine, declaration_support::finish_block};
use crate::{
    ast::{Attribute, Expr, FunctionDef, Statement, StatementKind, Type},
    util::parse_string_literal,
};
use anyhow::{Result, anyhow};

pub(super) fn parse(
    line: usize,
    header: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<Statement> {
    let label = header
        .strip_prefix("test ")
        .and_then(|s| s.strip_suffix('{'))
        .ok_or_else(|| anyhow!("expected `test \"name\" {{` or `test {{`"))?
        .trim();
    let label = if label.is_empty() {
        format!("test at line {line}").into()
    } else {
        parse_string_literal(label).ok_or_else(|| anyhow!("test name must be a string literal"))?
    };
    *cursor += 1;
    let body = super::statement::parse_block(lines, cursor, false, false)?;
    finish_block(lines, cursor, "test block")?;
    Ok(Statement::new(
        line,
        StatementKind::Function(FunctionDef {
            name: format!("__ush_test_{line}").into(),
            attrs: vec![Attribute {
                name: "test".into(),
                value: Some(Expr::String(label)),
            }],
            params: vec![],
            receiver: None,
            scheme: None,
            return_type: Some(Type::Unit),
            declared_errors: None,
            declared_effects: None,
            body,
        }),
    ))
}
