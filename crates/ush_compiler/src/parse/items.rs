//! Top-level item rules: `pub` exports and what a module may contain.

use anyhow::{Result, bail};

use super::FileRole;
use crate::ast::{Statement, StatementKind as S};

/// Splits a leading `pub` from a top-level line.
pub(super) fn strip_pub(line: &str) -> (&str, bool) {
    match line.strip_prefix("pub ") {
        Some(rest) => (rest.trim_start(), true),
        None => (line, false),
    }
}

/// The directory `extern` libraries are sourced from, relative to the
/// root script's directory.
pub(super) fn library_dir(role: FileRole<'_>) -> &str {
    match role {
        FileRole::Script => "",
        FileRole::Module { dir } => dir,
    }
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

/// `pub extern "x.sh" { … }` exports every declared function.
pub(super) fn export_all(statements: &mut [Statement], public: bool) {
    for statement in statements {
        statement.public = public && matches!(statement.kind, S::Function(_));
    }
}
