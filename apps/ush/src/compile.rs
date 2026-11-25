//! `ush compile`: one script to stdout or a file, or many scripts into
//! output directories, written or checked for freshness.

mod freshness;

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use ush_compiler::{ShellTarget, UshCompiler};

use crate::sourcemap_json::write_sourcemap_file;

pub struct CompileRequest<'a> {
    pub inputs: &'a [PathBuf],
    pub output: Option<&'a Path>,
    pub out_dir: Option<&'a Path>,
    pub sourcemap: Option<&'a Path>,
    pub checked_in: bool,
    pub check: bool,
    pub target: ShellTarget,
}

/// Exit status: 0 when every input compiled (and, with `--check`, matched
/// its output); 1 otherwise. Every input is processed before returning, so
/// one run reports all stale or broken scripts.
pub fn run(request: &CompileRequest<'_>) -> Result<i32> {
    let jobs = plan(request)?;
    let compiler = UshCompiler::for_target(request.target);
    let mut failed = 0;
    for (output, input) in &jobs {
        let result = match output {
            Some(output) if request.check => freshness::ensure_dir(output).and_then(|()| {
                compile(request, &compiler, input, Some(output))
                    .and_then(|shell| freshness::check(&shell, output))
            }),
            Some(output) => compile(request, &compiler, input, Some(output))
                .and_then(|shell| write_if_changed(output, &shell)),
            None => compile(request, &compiler, input, None).map(|shell| print!("{shell}")),
        };
        if let Err(error) = result {
            eprintln!("ush: {error:#}");
            failed += 1;
        }
    }
    if failed > 0 && request.check {
        eprintln!("ush: {failed} output(s) out of date; run the same command without --check");
    }
    Ok(i32::from(failed > 0))
}

/// Pairs each input with its output path, rejecting combinations that
/// cannot be honoured before anything is written.
fn plan(request: &CompileRequest<'_>) -> Result<Vec<(Option<PathBuf>, PathBuf)>> {
    let destination = request.output.is_some() || request.out_dir.is_some();
    if request.checked_in && !destination {
        bail!("--checked-in needs -o FILE or --out-dir DIR");
    }
    if request.check && !destination {
        bail!("--check needs -o FILE or --out-dir DIR to compare against");
    }
    if request.inputs.len() > 1 && request.out_dir.is_none() {
        bail!("compiling several inputs needs --out-dir DIR");
    }
    if request.sourcemap.is_some() && (request.inputs.len() > 1 || request.check) {
        bail!("--sourcemap takes a single input and cannot be combined with --check");
    }
    let mut jobs = Vec::new();
    let mut writers = BTreeMap::new();
    for input in request.inputs {
        let output = match (request.output, request.out_dir) {
            (Some(output), _) => Some(output.to_path_buf()),
            (None, Some(dir)) => Some(output_in(dir, input)?),
            (None, None) => None,
        };
        if let Some(path) = &output
            && let Some(previous) = writers.insert(path.clone(), input)
        {
            bail!(
                "{} and {} would both write {}",
                previous.display(),
                input.display(),
                path.display()
            );
        }
        jobs.push((output, input.clone()));
    }
    Ok(jobs)
}

/// `DIR/<stem>.sh`. A relative DIR is taken from the input's directory,
/// so scripts spread over many directories each get their own
/// `compiled/` beside them.
fn output_in(dir: &Path, input: &Path) -> Result<PathBuf> {
    let stem = input
        .file_stem()
        .with_context(|| format!("{} has no file name", input.display()))?;
    let base = match input.parent() {
        Some(parent) if dir.is_relative() => parent.join(dir),
        _ => dir.to_path_buf(),
    };
    Ok(base.join(stem).with_extension("sh"))
}

fn compile(
    request: &CompileRequest<'_>,
    compiler: &UshCompiler,
    input: &Path,
    output: Option<&Path>,
) -> Result<String> {
    if request.checked_in
        && !request.check
        && let Some(dir) = output.and_then(Path::parent)
        && !dir.as_os_str().is_empty()
    {
        // The artifact records its path relative to its source, so its
        // directory must exist before compiling.
        fs::create_dir_all(dir).with_context(|| format!("failed to create {}", dir.display()))?;
    }
    let artifact = output.filter(|_| request.checked_in);
    let Some(map) = request.sourcemap else {
        return match artifact {
            Some(path) => compiler.compile_checked_in_file(input, path),
            None => compiler.compile_file(input),
        };
    };
    let compiled = match artifact {
        Some(path) => compiler.compile_checked_in_file_with_sourcemap(input, path)?,
        None => compiler.compile_file_with_sourcemap(input)?,
    };
    write_sourcemap_file(map, input, output, &compiled.sourcemap)?;
    Ok(compiled.shell)
}

/// Leaves an up-to-date file untouched, so timestamps only move when
/// the content does.
fn write_if_changed(path: &Path, shell: &str) -> Result<()> {
    if fs::read(path).is_ok_and(|current| current == shell.as_bytes()) {
        return Ok(());
    }
    if let Some(dir) = path.parent().filter(|dir| !dir.as_os_str().is_empty()) {
        fs::create_dir_all(dir).with_context(|| format!("failed to create {}", dir.display()))?;
    }
    fs::write(path, shell).with_context(|| format!("failed to write {}", path.display()))
}
