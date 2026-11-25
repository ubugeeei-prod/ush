//! The row a function has — written or inferred — for editor hovers.

use std::path::Path;

use ush_compiler::UshCompiler;

/// `row: unknown, exec (inferred)` for `name`, when `source` compiles.
pub(crate) fn function_row(source: &str, path: Option<&Path>, name: &str) -> Option<String> {
    let report = match path.filter(|path| path.exists()) {
        Some(path) => UshCompiler.effects_source_at(source, path),
        None => UshCompiler.effects_source(source),
    }
    .ok()?;
    let function = report
        .functions
        .iter()
        .find(|function| function.name == name)?;
    let origin = match function.declared_row() {
        Some(_) => "declared",
        None => "inferred",
    };
    Some(format!("\n\nrow: `{}` ({origin})", function.row()))
}
