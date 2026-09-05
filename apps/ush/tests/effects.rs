//! `ush effects` and the effect rows it checks.

use std::fs;
use std::process::Command;

use tempfile::tempdir;

fn ush() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ush"))
}

fn write(source: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("script.ush");
    fs::write(&script, source).expect("write script");
    (dir, script)
}

#[test]
fn effects_lists_every_function_and_the_top_level() {
    let (_dir, script) = write(concat!(
        "fn slug(name: String) -> String {\n  name + \"-x\"\n}\n",
        "fn fetch(url: String) -> String {\n  std::http::get(url)\n}\n",
        "print fetch(\"u\") + slug(\"y\")\n",
    ));

    let output = ush()
        .args(["effects", script.to_str().expect("utf-8")])
        .output()
        .expect("run ush effects");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success(), "{stdout}");
    assert!(stdout.contains("fetch"), "{stdout}");
    assert!(stdout.contains("net"), "{stdout}");
    assert!(stdout.contains("slug"), "{stdout}");
    assert!(stdout.contains("pure"), "{stdout}");
    assert!(stdout.contains("(top level)"), "{stdout}");
}

#[test]
fn undeclared_hides_the_functions_that_already_have_a_row() {
    let (_dir, script) = write(concat!(
        "#[effects(net)]\nfn fetch(url: String) -> String {\n  std::http::get(url)\n}\n",
        "fn slug(name: String) -> String {\n  name + \"-x\"\n}\n",
        "print fetch(\"u\") + slug(\"y\")\n",
    ));

    let output = ush()
        .args(["effects", "--undeclared", script.to_str().expect("utf-8")])
        .output()
        .expect("run ush effects");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success(), "{stdout}");
    assert!(stdout.contains("slug"), "{stdout}");
    assert!(!stdout.contains("fetch"), "{stdout}");
}

#[test]
fn an_under_declared_row_fails_ush_check() {
    let (_dir, script) = write(
        "#[effects(fs)]\nfn fetch(url: String) -> String {\n  std::http::get(url)\n}\nprint fetch(\"u\")\n",
    );

    let output = ush()
        .args(["check", script.to_str().expect("utf-8")])
        .output()
        .expect("run ush check");
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr.contains("performs `net`"), "{stderr}");
    assert!(stderr.contains("#[effects(net)]"), "{stderr}");
}

#[test]
fn an_under_declared_row_also_refuses_to_run() {
    let (_dir, script) =
        write("#[pure]\nfn shout(name: String) {\n  print name\n}\nshout(\"x\")\n");

    let output = ush()
        .arg(script.to_str().expect("utf-8"))
        .output()
        .expect("run ush");

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("performs `io`"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
