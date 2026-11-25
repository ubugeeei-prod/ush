//! `allow_fail command` and `defer { … }`.

use anyhow::{Result, bail};

use super::{
    SourceLine, declaration_support::finish_block, expr::parse_expr, inline::parse_inline_body,
};
use crate::{
    ast::{Expr, StatementKind},
    types::HeapVec as Vec,
};

/// Parse a statement that starts with `allow_fail` or `defer`.
pub(super) fn parse(
    line_no: usize,
    trimmed: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<Option<StatementKind>> {
    if let Some(command) = trimmed.strip_prefix("allow_fail ") {
        return Ok(Some(StatementKind::AllowFail(parse_expr(command)?)));
    }
    if trimmed == "allow_fail" {
        bail!("`allow_fail` needs a command to run");
    }
    if trimmed == "defer {" {
        *cursor += 1;
        let body = super::statement::parse_block(lines, cursor, None, false)?;
        let _ = finish_block(lines, cursor, "defer block")?;
        return Ok(Some(StatementKind::Defer(body)));
    }
    let Some(rest) = trimmed.strip_prefix("defer ") else {
        if trimmed == "defer" {
            bail!("`defer` needs a statement or a block");
        }
        return Ok(None);
    };
    let statement = parse_inline_body(line_no, rest, false)?;
    *cursor += 1;
    Ok(Some(StatementKind::Defer(Vec::from([statement]))))
}

/// `spawn command` is `async { command }` with the command run as a
/// statement: its output streams, and the task's value is `()`.
pub(super) fn spawn(line_no: usize, command: &str) -> Result<Expr> {
    let statement = parse_inline_body(line_no, command, false)?;
    if !matches!(
        statement.kind,
        StatementKind::Call(_) | StatementKind::TryCall(_) | StatementKind::Pipeline(_)
    ) || matches!(&statement.kind, StatementKind::Call(call) if call.asynchronous)
    {
        bail!("`spawn` runs a command, a pipeline or a function call: spawn {command}");
    }
    Ok(Expr::AsyncBlock(Vec::from([statement])))
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::ast::{Expr, StatementKind};

    fn parse(source: &str) -> StatementKind {
        let lines: Vec<_> = source
            .lines()
            .enumerate()
            .map(|(i, l)| (i + 1, l))
            .collect();
        let mut cursor = 0;
        let kind = super::parse(1, lines[0].1.trim(), &lines, &mut cursor)
            .unwrap()
            .expect("a process statement");
        // `defer` consumes its lines; the block parser advances past others.
        let consumed = if matches!(kind, StatementKind::Defer(_)) {
            lines.len()
        } else {
            0
        };
        assert_eq!(cursor, consumed, "{source}");
        kind
    }

    #[test]
    fn allow_fail_takes_a_whole_pipeline() {
        let StatementKind::AllowFail(Expr::Pipeline(pipeline)) =
            parse(r#"allow_fail git "log" | head "-n1""#)
        else {
            panic!("expected allow_fail of a pipeline");
        };
        assert_eq!(pipeline.stages.len(), 2);
    }

    #[test]
    fn defer_takes_a_block_or_one_statement() {
        let StatementKind::Defer(body) = parse("defer {\n  print \"a\"\n  print \"b\"\n}") else {
            panic!("expected a defer block");
        };
        assert_eq!(body.len(), 2);
        let StatementKind::Defer(body) = parse(r#"defer rm "-rf" "tmp""#) else {
            panic!("expected a one-line defer");
        };
        assert!(matches!(body[0].kind, StatementKind::Call(_)));
    }

    #[test]
    fn bare_keywords_are_rejected() {
        for source in ["defer", "allow_fail"] {
            let lines = [(1, source)];
            assert!(super::parse(1, source, &lines, &mut 0).is_err(), "{source}");
        }
        let lines = [(1, "deferred")];
        assert!(
            super::parse(1, "deferred", &lines, &mut 0)
                .unwrap()
                .is_none()
        );
    }
}
