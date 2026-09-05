//! `ush explain` closes the loop between a `/bin/sh` diagnostic and
//! the `.ush` source that produced it.

use std::fs;
use std::process::Command;

use tempfile::tempdir;

fn ush() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ush"))
}

const SCRIPT: &str = concat!(
    "let greeting = \"hello\"\n",
    "print greeting\n",
    "$ ush-explain-test-missing-tool\n",
    "print \"after\"\n",
);

/// The line number `/bin/sh` itself printed for the failure.
fn failing_shell_line(stderr: &str) -> String {
    let line = stderr
        .lines()
        .find(|line| line.starts_with("/bin/sh: line ") || line.contains(": line "))
        .unwrap_or_else(|| panic!("no `line N` diagnostic in: {stderr}"));
    line.split("line ")
        .nth(1)
        .and_then(|rest| rest.split(':').next())
        .expect("line number")
        .trim()
        .to_string()
}

#[test]
fn a_shell_line_number_maps_back_to_the_ush_source() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("boom.ush");
    fs::write(&script, SCRIPT).expect("write script");

    let run = ush().arg(&script).output().expect("run ush");
    let stderr = String::from_utf8_lossy(&run.stderr).into_owned();
    assert!(!run.status.success());

    // The number `/bin/sh` complains about and the number the runtime
    // report hands to `ush explain` have to be the same one, or the
    // mapping is decorative.
    let shell_line = failing_shell_line(&stderr);
    assert!(
        stderr.contains(&format!("shell  : line {shell_line} |")),
        "runtime report disagrees with /bin/sh: {stderr}"
    );

    let explained = ush()
        .args(["explain", script.to_str().expect("utf-8"), &shell_line])
        .output()
        .expect("run ush explain");
    let stdout = String::from_utf8_lossy(&explained.stdout);

    assert!(explained.status.success(), "stdout: {stdout}");
    assert!(stdout.contains("user-code"), "{stdout}");
    assert!(stdout.contains("ush-explain-test-missing-tool"), "{stdout}");
    assert!(
        stdout.contains("->    3 | $ ush-explain-test-missing-tool"),
        "expected the failing source line to be marked: {stdout}"
    );
}

#[test]
fn a_sourcemap_id_is_accepted_too() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("boom.ush");
    fs::write(&script, SCRIPT).expect("write script");

    let run = ush().arg(&script).output().expect("run ush");
    let stderr = String::from_utf8_lossy(&run.stderr).into_owned();
    let id = stderr
        .split("| G")
        .nth(1)
        .map(|rest| format!("G{}", &rest[..4]))
        .expect("sourcemap id in the report");

    let explained = ush()
        .args(["explain", script.to_str().expect("utf-8"), &id])
        .output()
        .expect("run ush explain");

    assert!(explained.status.success());
    assert!(
        String::from_utf8_lossy(&explained.stdout).contains("ush-explain-test-missing-tool"),
        "{}",
        String::from_utf8_lossy(&explained.stdout)
    );
}

#[test]
fn a_line_inside_the_scaffolding_is_reported_as_such() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("ok.ush");
    fs::write(&script, "print \"hi\"\n").expect("write script");

    let output = ush()
        .args(["explain", script.to_str().expect("utf-8"), "3"])
        .output()
        .expect("run ush explain");

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("runtime scaffolding"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
