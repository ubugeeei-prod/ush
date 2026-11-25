//! `ush effects` — print the row `ush` infers for each function, the
//! error types it can raise followed by its effects, next to the row it
//! declares.
//!
//! Inference runs whether or not a row is written, so this is also the
//! way to find out what a `-> T / …` row would have to say before
//! adding one.

use std::path::Path;

use anyhow::Result;
use ush_compiler::{FunctionEffects, UshCompiler};

pub fn report(input: &Path, undeclared_only: bool) -> Result<i32> {
    let report = UshCompiler.effects_file(input)?;

    let rows = report
        .functions
        .iter()
        .filter(|function| !undeclared_only || function.declared.is_none())
        .collect::<Vec<_>>();

    let width = rows
        .iter()
        .map(|function| function.name.chars().count())
        .max()
        .unwrap_or(0)
        .max("(top level)".len());

    for function in rows {
        println!(
            "{:width$}  {}{}",
            function.name,
            function.row(),
            declared_suffix(function),
        );
    }

    if !undeclared_only {
        println!("{:width$}  {}", "(top level)", report.top_level);
    }
    Ok(0)
}

/// Shows the declared row only when it says something the inferred
/// row does not — an over-declaration is the interesting case, since
/// an under-declaration is a compile error and never gets this far.
fn declared_suffix(function: &FunctionEffects) -> String {
    match function.declared_row() {
        Some(declared) if declared != function.row() => format!("  (declared: {declared})"),
        Some(_) => "  (declared)".to_string(),
        None => String::new(),
    }
}
