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

/// Runs `check` for every shell installed here, with the target the
/// shell needs. CI's Linux runner provides sh (dash), dash and bash, and
/// its macOS runner adds zsh; `scripts/test_targets.ush` adds BusyBox ash.
#[allow(dead_code)]
pub fn each_shell(mut check: impl FnMut(&str, ush_compiler::ShellTarget)) {
    use ush_compiler::ShellTarget;
    let mut ran = 0;
    for (shell, target) in [
        ("sh", ShellTarget::Sh),
        ("dash", ShellTarget::Sh),
        ("bash", ShellTarget::Bash),
        ("zsh", ShellTarget::Zsh),
    ] {
        if Command::new(shell).arg("-c").arg(":").status().is_ok() {
            ran += 1;
            check(shell, target);
        }
    }
    assert!(ran > 0, "no POSIX shell found");
}

/// Compiles `source` for `target`, runs it with `shell` in `dir` and
/// returns `(status, stdout, stderr)`.
#[allow(dead_code)]
pub fn run_with(
    shell: &str,
    target: ush_compiler::ShellTarget,
    dir: &std::path::Path,
    source: &str,
) -> (i32, String, String) {
    run_with_sigpipe(shell, target, dir, source, false)
}

/// [`run_with`], optionally starting the shell with SIGPIPE ignored.
#[allow(dead_code)]
pub fn run_with_sigpipe(
    shell: &str,
    target: ush_compiler::ShellTarget,
    dir: &std::path::Path,
    source: &str,
    ignore_sigpipe: bool,
) -> (i32, String, String) {
    let compiled = UshCompiler::for_target(target)
        .compile_source(source)
        .unwrap_or_else(|error| panic!("{error:#}"));
    let script = dir.join("program.sh");
    fs::write(&script, compiled).expect("write script");
    // A signal ignored on entry stays ignored in a non-interactive shell,
    // as SIGPIPE is under some CI runners.
    let mut command = Command::new(shell);
    if ignore_sigpipe {
        command.args(["-c", "trap '' PIPE; exec \"$0\" \"$1\"", shell]);
    }
    let output = command
        .arg(&script)
        .current_dir(dir)
        .output()
        .expect("run compiled script");
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

/// Runs `source` under every shell, in a fresh directory each time.
#[allow(dead_code)]
pub fn each_program(
    source: &str,
    mut check: impl FnMut(&str, &std::path::Path, (i32, String, String)),
) {
    each_shell(|shell, target| {
        let dir = tempdir().expect("tempdir");
        let result = run_with(shell, target, dir.path(), source);
        check(shell, dir.path(), result);
    });
}
