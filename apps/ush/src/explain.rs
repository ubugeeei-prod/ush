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

use ush_compiler::ShellTarget;

/// How many `.ush` lines of context to print on each side.
const CONTEXT_RADIUS: usize = 2;

pub fn explain(input: &Path, targets: &[String], target: ShellTarget) -> Result<i32> {
    let compiled = UshCompiler::for_target(target).compile_file_with_sourcemap(input)?;
    let source =
        fs::read_to_string(input).with_context(|| format!("failed to read {}", input.display()))?;
    let source_lines = source.lines().collect::<Vec<_>>();

    let mut status = 0;
    for (index, target) in targets.iter().enumerate() {
        if index > 0 {
            println!();
        }
        if !explain_one(input, target, &compiled.sourcemap, &source_lines)? {
            status = 1;
        }
    }
    Ok(status)
}

fn explain_one(
    input: &Path,
    target: &str,
    sourcemap: &SourceMap,
    source_lines: &[&str],
) -> Result<bool> {
    let (generated_line, requested) = parse_target(target)?;
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
        generated_line,
        generated_line,
        entry.section.label()
    );
    println!("  shell  : {}", entry.generated_text);

    let Some(source_line) = entry.source_line else {
        println!("  source : (generated support code, no `.ush` line behind it)");
        return Ok(true);
    };

    // A module line names its file; read it for the context lines.
    let module_text;
    let (file, lines) = match &entry.source_file {
        Some(file) => {
            module_text = fs::read_to_string(file).unwrap_or_default();
            (file.clone(), module_text.lines().collect::<Vec<_>>())
        }
        None => (input.display().to_string(), source_lines.to_vec()),
    };
    println!("  source : {file}:{source_line}");
    print_context(&lines, source_line);
    print_siblings(sourcemap, entry, generated_line);
    Ok(true)
}

/// Accepts either a `/bin/sh` line number or a sourcemap `G` id, so
/// both halves of the runtime report can be pasted back in.
fn parse_target(target: &str) -> Result<(usize, String)> {
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
    if shell_line == 0 {
        bail!("shell lines start at 1");
    }
    Ok((shell_line, format!("shell line {shell_line}")))
}

fn print_context(source_lines: &[&str], source_line: usize) {
    let first = source_line.saturating_sub(CONTEXT_RADIUS).max(1);
    let last = (source_line + CONTEXT_RADIUS).min(source_lines.len());
    if first > last {
        return;
    }
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
fn print_siblings(sourcemap: &SourceMap, entry: &SourceMapLine, generated_line: usize) {
    let group = sourcemap
        .lines
        .iter()
        .filter(|line| {
            line.source_file == entry.source_file && line.source_line == entry.source_line
        })
        .map(|line| line.generated_line)
        .collect::<Vec<_>>();
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
        println!("  {marker} line {:>5} | G{line:04} | {text}", line);
    }
}
