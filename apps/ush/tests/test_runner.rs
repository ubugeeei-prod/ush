use std::{fs, process::Command};

use tempfile::tempdir;

fn ush() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ush"))
}

fn normalize_path(text: &str, path: &std::path::Path, marker: &str) -> String {
    // The runner canonicalises the scripts it discovers, so the
    // resolved form has to be normalised too — on macOS a temp dir
    // under `/tmp` or `/var` gains a `/private` prefix on the way
    // through `canonicalize`.
    let mut targets = vec![path.display().to_string()];
    if let Ok(canonical) = fs::canonicalize(path) {
        targets.push(canonical.display().to_string());
    }
    targets.sort_by_key(|target| std::cmp::Reverse(target.len()));

    text.lines()
        .map(|line| {
            let mut line = line.to_string();
            for target in &targets {
                line = line.replace(target, marker);
            }
            line
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

#[test]
fn test_command_runs_explicit_directory_and_reports_failures() {
    let dir = tempdir().expect("tempdir");
    let tests = dir.path().join("suite");
    fs::create_dir_all(&tests).expect("create test dir");
    fs::write(tests.join("pass.ush"), "print \"ok\"\n").expect("write pass test");
    fs::write(tests.join("fail.ush"), "shell \"false\"\n").expect("write fail test");

    let output = ush()
        .args(["test", tests.to_str().unwrap()])
        .current_dir(dir.path())
        .output()
        .expect("run ush test");

    let stdout = normalize_path(
        &String::from_utf8_lossy(&output.stdout),
        &tests.join("fail.ush"),
        "<FAIL_SCRIPT>",
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    assert_eq!(stdout, include_str!("fixtures/test_runner_failure.stdout"));
}

#[test]
fn test_command_defaults_to_tests_directory() {
    let dir = tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    fs::create_dir_all(&tests).expect("create tests dir");
    fs::write(tests.join("sample.ush"), "print \"ok\"\n").expect("write test");

    let output = ush()
        .arg("test")
        .current_dir(dir.path())
        .output()
        .expect("run ush test");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert_eq!(stdout, include_str!("fixtures/test_runner_success.stdout"));
}

#[test]
fn named_and_anonymous_blocks_are_isolated_and_all_run() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("blocks.ush");
    fs::write(
        &source,
        r#"
use std::test::*
fn inc(x: Int) -> Int {
  x + 1
}
test "generic list" {
  assert_eq [inc $x for x in 0..2] [1, 2]
  $ export USH_TEST_BLOCK_LEAK=yes
}
test {
  assert_eq (std::env::get_or "USH_TEST_BLOCK_LEAK" "no") "no"
}
test "failure" {
  assert_eq "actual" "expected"
  print "unreachable"
}
test "after failure" {
  assert true
}
"#,
    )
    .unwrap();
    let ordinary = ush().arg(&source).output().unwrap();
    assert!(
        ordinary.status.success(),
        "{}",
        String::from_utf8_lossy(&ordinary.stderr)
    );
    assert!(ordinary.stdout.is_empty());
    let output = ush().arg("test").arg(&source).output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(output.status.code(), Some(1), "{stdout}");
    assert!(stdout.contains("3 passed; 1 failed"), "{stdout}");
    assert!(stdout.contains("assert_eq failed"), "{stdout}");
    assert!(stdout.contains("after failure"), "{stdout}");
    assert!(!stdout.contains("unreachable"), "{stdout}");
}

#[test]
fn assertions_require_an_import_and_matching_types() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("assertions.ush");
    for (source, diagnostic) in [
        ("test {\n assert_eq 1 1\n}\n", "unknown function: assert_eq"),
        (
            "use std::test::assert_eq\ntest {\n assert_eq 1 \"1\"\n}\n",
            "same type",
        ),
    ] {
        fs::write(&path, source).unwrap();
        let output = ush().arg("check").arg(&path).output().unwrap();
        assert!(!output.status.success());
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(diagnostic),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
