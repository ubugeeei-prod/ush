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

#[test]
fn a_shell_line_number_maps_back_to_the_ush_source() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("boom.ush");
    fs::write(&script, SCRIPT).expect("write script");

    let run = ush().arg(&script).output().expect("run ush");
    let stderr = String::from_utf8_lossy(&run.stderr).into_owned();
    assert!(!run.status.success());

    assert!(
        stderr
            .contains("boom.ush:3: command not found; check the command name and PATH (exit 127)"),
        "{stderr}"
    );
    assert!(!stderr.contains("__ush_"), "{stderr}");
    let native = stderr
        .lines()
        .find(|s| s.contains("ush-explain-test-missing-tool") && !s.trim_start().starts_with('$'))
        .unwrap();
    let fields: Vec<_> = native.split(':').collect();
    let shell_line = fields
        .iter()
        .find_map(|s| s.trim().trim_start_matches("line ").parse::<usize>().ok())
        .expect("native shell line")
        .to_string();

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

    let compiled = ush_compiler::UshCompiler
        .compile_file_with_sourcemap(&script)
        .unwrap();
    let line = compiled.sourcemap.generated_lines_for_source(3)[0];
    let id = format!("G{line:04}");

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

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("generated support code"));
}
