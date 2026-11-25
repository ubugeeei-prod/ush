//! Name clashes the linker rules out before code generation.

use anyhow::{Result, bail};

use super::Scope;
use crate::{
    ast::{Statement, StatementKind as S},
    codegen::shell_function_name,
    types::{AstString as Name, Map},
};

/// `mod "x.ush" as h` next to `extern "h.sh" as h` would make `h::f` ambiguous.
pub(super) fn ensure_no_shadowed_namespace(program: &[Statement], scope: &Scope<'_>) -> Result<()> {
    for statement in program {
        if let S::Function(def) = &statement.kind
            && let Some((namespace, _)) = def.name.split_once("::")
            && scope.modules.contains_key(namespace)
        {
            bail!(
                "line {}: module alias `{namespace}` is also an extern namespace",
                statement.line
            );
        }
    }
    Ok(())
}

/// Shell function names drop `::`; two items must not collapse into one.
pub(super) fn distinct_shell_names(program: &[Statement]) -> Result<()> {
    let mut seen: Map<Name, &str> = Map::default();
    for statement in program {
        let name = match &statement.kind {
            S::Function(def) => &def.name,
            S::Effect(def) => &def.name,
            _ => continue,
        };
        if let Some(other) = seen.insert(shell_function_name(name), name)
            && other != name.as_str()
        {
            bail!("`{other}` and `{name}` compile to the same shell function; rename one");
        }
    }
    Ok(())
}
