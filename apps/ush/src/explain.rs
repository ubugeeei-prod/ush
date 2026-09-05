//! `ush explain` — turn a line number out of a `/bin/sh` diagnostic
//! back into the `.ush` code that produced it.
//!
//! Generated shell is the thing that actually runs, so the shell's
//! own errors point into it: `sh: line 495: foo: command not found`.
//! Without a way back, that number tells the author nothing about the
//! program they wrote. This command closes the loop, and the runtime
//! failure report prints the exact invocation to use.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use ush_compiler::{SourceMap, SourceMapLine, UshCompiler};

use crate::runtime_diagnostics::{InstrumentedScript, instrument_compiled_script};

/// How many `.ush` lines of context to print on each side.
const CONTEXT_RADIUS: usize = 2;

pub fn explain(input: &Path, targets: &[String]) -> Result<i32> {
    let compiled = UshCompiler.compile_file_with_sourcemap(input)?;
    let instrumented = instrument_compiled_script(input, &compiled);
    let source =
        fs::read_to_string(input).with_context(|| format!("failed to read {}", input.display()))?;
    let source_lines = source.lines().collect::<Vec<_>>();

    let mut status = 0;
    for (index, target) in targets.iter().enumerate() {
        if index > 0 {
            println!();
        }
        if !explain_one(
            input,
            target,
            &compiled.sourcemap,
            &instrumented,
            &source_lines,
        )? {
            status = 1;
        }
    }
    Ok(status)
}

fn explain_one(
    input: &Path,
    target: &str,
    sourcemap: &SourceMap,
    instrumented: &InstrumentedScript,
    source_lines: &[&str],
) -> Result<bool> {
    let (generated_line, requested) = parse_target(target, instrumented)?;
    let Some(entry) = sourcemap.line(generated_line) else {
        println!(
            "{requested}: outside the generated shell for {}",
            input.display()
        );
        return Ok(false);
    };

    println!(
        "{}: shell line {} | G{:04} | {}",
        input.display(),
        instrumented.shell_line(generated_line),
        generated_line,
        entry.section.label()
    );
    println!("  shell  : {}", entry.generated_text);

    let Some(source_line) = entry.source_line else {
        println!("  source : (generated support code, no `.ush` line behind it)");
        return Ok(true);
    };

    println!("  source : {}:{source_line}", input.display());
    print_context(source_lines, source_line);
    print_siblings(sourcemap, source_line, generated_line, instrumented);
    Ok(true)
}

/// Accepts either a `/bin/sh` line number or a sourcemap `G` id, so
/// both halves of the runtime report can be pasted back in.
fn parse_target(target: &str, instrumented: &InstrumentedScript) -> Result<(usize, String)> {
    let trimmed = target.trim();
    if let Some(rest) = trimmed
        .strip_prefix('G')
        .or_else(|| trimmed.strip_prefix('g'))
    {
        let generated: usize = rest
            .parse()
            .with_context(|| format!("invalid sourcemap id: {trimmed}"))?;
        return Ok((generated, format!("G{generated:04}")));
    }

    let shell_line: usize = trimmed
        .parse()
        .with_context(|| format!("expected a shell line number or a `G####` id, got {trimmed}"))?;
    match instrumented.generated_line(shell_line) {
        Some(generated) => Ok((generated, format!("shell line {shell_line}"))),
        None => bail!(
            "shell line {shell_line} falls inside the {} lines of runtime scaffolding `ush` adds \
             before the generated program",
            instrumented.header_lines
        ),
    }
}

fn print_context(source_lines: &[&str], source_line: usize) {
    let first = source_line.saturating_sub(CONTEXT_RADIUS).max(1);
    let last = (source_line + CONTEXT_RADIUS).min(source_lines.len());
    for line in first..=last {
        let marker = if line == source_line { "->" } else { "  " };
        println!("  {marker} {line:>4} | {}", source_lines[line - 1]);
    }
}

/// The other shell lines the same `.ush` line lowered into.
///
/// One source line routinely becomes a whole control-flow block, and
/// seeing the group is what makes a failure in the middle of one
/// readable.
fn print_siblings(
    sourcemap: &SourceMap,
    source_line: usize,
    generated_line: usize,
    instrumented: &InstrumentedScript,
) {
    let group = sourcemap.generated_lines_for_source(source_line);
    if group.len() <= 1 {
        return;
    }
    println!("  group  :");
    for line in group {
        let marker = if line == generated_line { "->" } else { "  " };
        let text = sourcemap
            .line(line)
            .map(|entry: &SourceMapLine| entry.generated_text.as_str())
            .unwrap_or("");
        println!(
            "  {marker} line {:>5} | G{line:04} | {text}",
            instrumented.shell_line(line)
        );
    }
}
