//! `ush effects` — print the effect row `ush` infers for each
//! function, next to the one it declares.
//!
//! Inference runs whether or not anything is annotated, so this is
//! also the way to find out what an `#[effects(...)]` row would have
//! to say before adding one.

use std::path::Path;

use anyhow::Result;
use ush_compiler::{EffectSet, UshCompiler};

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
            function.inferred,
            declared_suffix(function.declared.as_ref(), &function.inferred),
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
fn declared_suffix(declared: Option<&EffectSet>, inferred: &EffectSet) -> String {
    match declared {
        Some(declared) if declared != inferred => format!("  (declared: {declared})"),
        Some(_) => "  (declared)".to_string(),
        None => String::new(),
    }
}
