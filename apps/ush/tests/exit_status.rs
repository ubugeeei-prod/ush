//! `ush script.ush` has to hand the script's exit status back to the
//! caller: shell pipelines, CI steps, and `&&` chains all depend on
//! it, and the runtime source-map trap sits between the two.

use std::{fs, process::Command};

use tempfile::{TempDir, tempdir};

fn ush_binary() -> &'static str {
    env!("CARGO_BIN_EXE_ush")
}

fn run_script(dir: &TempDir, source: &str) -> (i32, String) {
    let script = dir.path().join("program.ush");
    fs::write(&script, source).expect("write script");
    let output = Command::new(ush_binary())
        .arg(&script)
        .current_dir(dir.path())
        .output()
        .expect("run ush");
    (
        output.status.code().expect("exit code"),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

#[test]
fn a_successful_script_exits_zero() {
    let dir = tempdir().expect("tempdir");
    let (status, stdout) = run_script(&dir, "print \"ok\"\n");

    assert_eq!(status, 0);
    assert_eq!(stdout, "ok\n");
}

#[test]
fn an_explicit_exit_code_is_handed_back_unchanged() {
    let dir = tempdir().expect("tempdir");
    assert_eq!(run_script(&dir, "$ exit 3\n").0, 3);
    assert_eq!(run_script(&dir, "$ exit 42\n").0, 42);
}

#[test]
fn a_failing_command_stops_the_script_with_a_non_zero_status() {
    let dir = tempdir().expect("tempdir");
    let (status, stdout) = run_script(&dir, "$ false\nprint \"after\"\n");

    assert_ne!(status, 0, "a failing command must not report success");
    assert_eq!(stdout, "", "the script must stop at the failure");
}

#[test]
fn a_missing_command_reports_a_non_zero_status() {
    let dir = tempdir().expect("tempdir");
    let (status, _) = run_script(&dir, "$ ush-definitely-not-a-real-command\n");

    assert_ne!(status, 0);
}

#[test]
fn a_raised_error_that_reaches_the_top_level_fails_the_script() {
    let dir = tempdir().expect("tempdir");
    let (status, stdout) = run_script(
        &dir,
        r#"
enum Problem {
  MissingConfig,
}

fn load() -> Problem!String {
  raise Problem::MissingConfig
}

load()?
print "unreachable"
"#,
    );

    assert_ne!(status, 0);
    assert_eq!(stdout, "");
}

#[test]
fn statements_after_a_failure_do_not_run() {
    let dir = tempdir().expect("tempdir");
    let (status, stdout) = run_script(&dir, "print \"before\"\n$ false\nprint \"after\"\n");

    assert_ne!(status, 0);
    assert_eq!(stdout, "before\n");
}
