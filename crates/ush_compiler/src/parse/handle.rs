//! `try { … } with <effect> { … }` — effect handlers.

use anyhow::{Result, anyhow, bail};

use super::{
    super::ast::{EffectHandler, StatementKind},
    SourceLine,
};
use crate::types::{AstString as String, HeapVec as Vec};

/// `try { … } with log { (message) => … }`
///
/// A handler discharges an effect for the block it wraps: inside the
/// `try`, `do log(…)` is answered by this arm, and the row of the
/// enclosing function no longer mentions `log`.
pub(super) fn parse_handle(lines: &[SourceLine<'_>], cursor: &mut usize) -> Result<StatementKind> {
    *cursor += 1;
    let body = super::statement::parse_block(lines, cursor, false, false)?;
    let mut handlers = Vec::new();

    loop {
        let Some((line_no, line)) = lines.get(*cursor) else {
            bail!("unterminated try block");
        };
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix('}') else {
            bail!("line {line_no}: expected `}} with <effect> {{` after a try block");
        };
        let rest = rest.trim();
        if rest.is_empty() {
            *cursor += 1;
            break;
        }
        let Some(header) = rest.strip_prefix("with ") else {
            bail!("line {line_no}: expected `}} with <effect> {{` after a try block");
        };
        handlers.push(parse_handler(header.trim(), *line_no, lines, cursor)?);
    }

    if handlers.is_empty() {
        bail!("a try block needs at least one `with <effect> {{ … }}` handler");
    }
    Ok(StatementKind::Handle { body, handlers })
}

/// `log { (message) =>` … the arm header plus its body.
fn parse_handler(
    header: &str,
    line_no: usize,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<EffectHandler> {
    let (effect, rest) = header
        .split_once('{')
        .ok_or_else(|| anyhow!("line {line_no}: expected `{{` after the effect name"))?;
    let effect = String::from(effect.trim());
    if effect.is_empty() {
        bail!("line {line_no}: a handler needs an effect name");
    }

    let params = parse_handler_params(rest.trim(), line_no)?;
    *cursor += 1;
    let body = super::statement::parse_block(lines, cursor, false, true)?;
    Ok(EffectHandler {
        effect,
        params,
        body,
    })
}

/// `(message) =>`, `(a, b) =>`, or nothing at all for an operation
/// that takes no arguments.
fn parse_handler_params(source: &str, line_no: usize) -> Result<Vec<String>> {
    if source.is_empty() {
        return Ok(Vec::new());
    }
    let arrow = source
        .strip_suffix("=>")
        .ok_or_else(|| anyhow!("line {line_no}: a handler binds its arguments with `=>`"))?
        .trim();
    let inner = arrow
        .strip_prefix('(')
        .and_then(|rest| rest.strip_suffix(')'))
        .ok_or_else(|| anyhow!("line {line_no}: handler arguments are written `(a, b) =>`"))?;

    let mut params = Vec::new();
    for part in crate::util::split_top_level(inner, ',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if !crate::util::is_identifier(part) {
            bail!("line {line_no}: invalid handler argument: {part}");
        }
        params.push(String::from(part));
    }
    Ok(params)
}
