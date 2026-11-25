//! Top-level item rules: `pub` exports and what a module may contain.

use anyhow::{Result, bail};

use super::{FileRole, SourceLine};
use crate::{
    ast::{Statement, StatementKind as S},
    types::HeapVec as Vec,
};

/// Splits a leading `pub` from a top-level line.
pub(super) fn strip_pub(line: &str) -> (&str, bool) {
    match line.strip_prefix("pub ") {
        Some(rest) => (rest.trim_start(), true),
        None => (line, false),
    }
}

/// `extern` and `test` blocks, which only a file's top level holds.
pub(super) fn parse_file_block(
    role: FileRole<'_>,
    line: usize,
    header: &str,
    public: bool,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<Option<Vec<Statement>>> {
    if header.starts_with("extern ") {
        // A module's libraries are sourced from beside the module.
        let dir = match role {
            FileRole::Script => "",
            FileRole::Module { dir } => dir,
        };
        let mut declared = super::external::parse(line, header, lines, cursor, dir)?;
        for statement in &mut declared {
            // `pub extern` exports every declared function.
            statement.public = public && matches!(statement.kind, S::Function(_));
        }
        return Ok(Some(declared));
    }
    if header == "test {" || header.starts_with("test ") {
        if public {
            bail!("line {line}: test blocks cannot be `pub`");
        }
        return Ok(Some(vec![super::testing::parse(
            line, header, lines, cursor,
        )?]));
    }
    Ok(None)
}

/// Records `pub` on a parsed top-level statement and keeps executable
/// statements out of modules.
pub(super) fn finish(
    role: FileRole<'_>,
    line: usize,
    statement: &mut Statement,
    public: bool,
) -> Result<()> {
    if public
        && !matches!(
            statement.kind,
            S::Function(_) | S::Enum(_) | S::Trait(_) | S::Effect(_)
        )
    {
        bail!(
            "line {line}: `pub` applies to fn, type, enum, trait, effect and extern declarations"
        );
    }
    statement.public = public;
    if matches!(role, FileRole::Module { .. }) {
        match statement.kind {
            S::Use(_)
            | S::Module(_)
            | S::Enum(_)
            | S::Trait(_)
            | S::Impl(_)
            | S::Function(_)
            | S::Effect(_)
            | S::Alias { .. } => {}
            S::UnitDef(_) => bail!("line {line}: `unit` declarations belong in the root script"),
            _ => bail!(
                "line {line}: a module contains only declarations (fn, type, enum, trait, impl, effect, alias, use, mod, extern); move this statement into a function"
            ),
        }
    }
    Ok(())
}
