//! `try { … } with <effect> { … }` — effect handlers.

use anyhow::{Result, anyhow, bail};

use super::{
    super::ast::{EffectHandler, StatementKind},
    SourceLine,
};
use crate::types::HeapVec as Vec;

/// `try { … } with log(message) { … }`
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
    let signature = header
        .strip_suffix('{')
        .ok_or_else(|| anyhow!("line {line_no}: expected `with name(args) {{`"))?
        .trim();
    let (effect, args, tail) = super::signature::split_paren_form(signature)
        .ok_or_else(|| anyhow!("line {line_no}: expected `with name(args) {{`"))?;
    let effect = super::path::parse_identifier(effect)?;
    if !tail.is_empty() {
        bail!("line {line_no}: unexpected text after handler parameters");
    }
    let mut params = Vec::new();
    if !args.trim().is_empty() {
        for arg in args.split(',') {
            let name = super::path::parse_identifier(arg.trim())?;
            if params.contains(&name) {
                bail!("line {line_no}: duplicate handler parameter: {name}");
            }
            params.push(name);
        }
    }
    *cursor += 1;
    let body = super::statement::parse_block(lines, cursor, false, true)?;
    Ok(EffectHandler {
        effect,
        params,
        body,
    })
}
