//! Shared harness for the end-to-end compiler tests: compile a
//! `.ush` program to POSIX `sh` and run it, so assertions describe
//! observable behaviour rather than generated text.

use std::{fs, process::Command};

use tempfile::{TempDir, tempdir};
use ush_compiler::UshCompiler;

/// Compiles and runs `source`, returning `(status, stdout, stderr)`.
#[allow(dead_code)]
pub fn try_run_in(dir: &TempDir, source: &str) -> (i32, String, String) {
    let compiled = UshCompiler::default()
        .compile_source(source)
        .expect("compile ush program");
    let script = dir.path().join("program.sh");
    fs::write(&script, compiled).expect("write script");

    let output = Command::new("/bin/sh")
        .arg(&script)
        .current_dir(dir.path())
        .output()
        .expect("run compiled script");
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

/// Compiles and runs `source` in `dir`, asserting it succeeds.
#[allow(dead_code)]
pub fn run_in(dir: &TempDir, source: &str) -> String {
    let (status, stdout, stderr) = try_run_in(dir, source);
    assert_eq!(status, 0, "stderr: {stderr}");
    stdout
}

/// Compiles and runs `source` in a fresh temporary directory.
#[allow(dead_code)]
pub fn run(source: &str) -> String {
    run_in(&tempdir().expect("tempdir"), source)
}

/// Compiles and runs `source`, returning the status and stdout even
/// when the program fails.
#[allow(dead_code)]
pub fn try_run(source: &str) -> (i32, String, String) {
    try_run_in(&tempdir().expect("tempdir"), source)
}
