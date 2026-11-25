//! `cd path { … }` and `export NAME = value, … [{ … }]`.

use anyhow::{Result, bail};

use super::{SourceLine, declaration_support::finish_block, expr::parse_expr};
use crate::{
    ast::{EnvBinding, Scope, StatementKind},
    scan::brace_delta,
    types::HeapVec as Vec,
    util::{is_identifier, split_once_top_level, split_top_level},
};

/// Parse a `cd` block or an `export` statement. Plain `cd path` and any
/// `export` that is not `NAME = value` keep their command meaning.
pub(super) fn parse(
    trimmed: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<Option<StatementKind>> {
    if let Some(path) = trimmed.strip_prefix("cd ").and_then(block_header) {
        let scope = Scope::Cd(parse_expr(path)?);
        return block(scope, lines, cursor).map(Some);
    }
    let Some(rest) = trimmed.strip_prefix("export ") else {
        return Ok(None);
    };
    let (rest, scoped) = block_header(rest).map_or((rest, false), |head| (head, true));
    let Some(bindings) = parse_bindings(rest)? else {
        return Ok(None);
    };
    if scoped {
        return block(Scope::Env(bindings), lines, cursor).map(Some);
    }
    Ok(Some(StatementKind::Export(bindings)))
}

/// The text before a block-opening `{` that ends the line.
fn block_header(source: &str) -> Option<&str> {
    let head = source.trim().strip_suffix('{')?;
    (brace_delta(source) == 1).then(|| head.trim())
}

fn block(scope: Scope, lines: &[SourceLine<'_>], cursor: &mut usize) -> Result<StatementKind> {
    *cursor += 1;
    let body = super::statement::parse_block(lines, cursor, None, false)?;
    let _ = finish_block(lines, cursor, "scoped block")?;
    Ok(StatementKind::Scoped { scope, body })
}

/// `NAME = value, OTHER = value`, or `None` when the first binding does
/// not have that shape (`export "A=b"` is still the `export` command).
fn parse_bindings(source: &str) -> Result<Option<Vec<EnvBinding>>> {
    let mut bindings: Vec<EnvBinding> = Vec::new();
    for (index, part) in split_top_level(source, ',').into_iter().enumerate() {
        let assignment = split_once_top_level(part, '=')
            .filter(|(name, value)| is_identifier(name) && !value.starts_with('='));
        let Some((name, value)) = assignment else {
            if index == 0 {
                return Ok(None);
            }
            bail!("expected `NAME = value` in export, found `{part}`");
        };
        if bindings.iter().any(|binding| binding.name == name) {
            bail!("export binds `{name}` twice");
        }
        bindings.push(EnvBinding {
            name: name.into(),
            value: parse_expr(value)?,
        });
    }
    Ok(Some(bindings))
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use crate::ast::{Scope, StatementKind};

    fn parse(source: &str) -> Option<StatementKind> {
        let lines: Vec<_> = source
            .lines()
            .enumerate()
            .map(|(i, l)| (i + 1, l))
            .collect();
        super::parse(lines[0].1.trim(), &lines, &mut 0).unwrap()
    }

    #[test]
    fn cd_without_a_block_is_the_cd_command() {
        assert!(parse(r#"cd "dir""#).is_none());
        assert!(parse(r#"cd "{""#).is_none());
        let Some(StatementKind::Scoped { scope, body }) = parse("cd \"dir\" {\n  print \"x\"\n}")
        else {
            panic!("expected a cd block");
        };
        assert!(matches!(scope, Scope::Cd(_)));
        assert_eq!(body.len(), 1);
    }

    #[test]
    fn export_binds_names_with_or_without_a_block() {
        let Some(StatementKind::Export(bindings)) = parse(r#"export A = "1", B = ("x" + ",")"#)
        else {
            panic!("expected export");
        };
        let names: Vec<_> = bindings.iter().map(|b| b.name.as_str()).collect();
        assert_eq!(names, ["A", "B"]);
        let scoped = parse("export A = \"1\" {\n}");
        assert!(matches!(
            scoped,
            Some(StatementKind::Scoped {
                scope: Scope::Env(_),
                ..
            })
        ));
    }

    #[test]
    fn other_export_shapes_stay_commands() {
        assert!(parse(r#"export "A=b""#).is_none());
        assert!(parse(r#"export A == "b""#).is_none());
    }

    #[test]
    fn duplicate_and_malformed_bindings_are_rejected() {
        let lines = [(1, r#"export A = "1", A = "2""#)];
        let error = super::parse(lines[0].1, &lines, &mut 0).unwrap_err();
        assert!(format!("{error}").contains("twice"));
        let lines = [(1, r#"export A = "1", "B""#)];
        assert!(super::parse(lines[0].1, &lines, &mut 0).is_err());
    }
}
