use std::{fs, process::Command};

use tempfile::tempdir;

fn ush() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ush"))
}

fn normalize_path(text: &str, path: &std::path::Path) -> String {
    text.replace(&path.display().to_string(), "<SCRIPT>")
}

#[test]
fn ush_script_raise_exits_with_typed_error() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("raise.ush");
    fs::write(
        &script,
        concat!(
            "enum Problem {\n",
            "  Nope,\n",
            "}\n",
            "fn fail() -> String / Problem, stderr {\n",
            "  fail (Problem::Nope)\n",
            "}\n",
            "fn wrap(message: String) -> String {\n",
            "  message\n",
            "}\n",
            "print wrap (fail ())\n",
        ),
    )
    .expect("write script");

    let output = ush().arg(&script).output().expect("run ush");
    let stderr = normalize_path(&String::from_utf8_lossy(&output.stderr), &script);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(stdout, "");
    assert_eq!(stderr, include_str!("fixtures/error_raise.stderr"));
}

#[test]
fn ush_script_try_operator_propagates_function_failure() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("try.ush");
    fs::write(
        &script,
        concat!(
            "enum Problem {\n",
            "  Nope,\n",
            "}\n",
            "fn fail() -> () / Problem, stderr {\n",
            "  fail (Problem::Nope)\n",
            "}\n",
            "fn outer() -> () / Problem, stderr, stdout {\n",
            "  fail()?\n",
            "  print \"unreachable\"\n",
            "}\n",
            "outer ()\n",
        ),
    )
    .expect("write script");

    let output = ush().arg(&script).output().expect("run ush");
    let stderr = normalize_path(&String::from_utf8_lossy(&output.stderr), &script);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(stdout, "");
    assert_eq!(stderr, include_str!("fixtures/error_try.stderr"));
}
