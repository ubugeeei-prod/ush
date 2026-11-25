//! The CLI subcommands read a script's local modules.

use std::{fs, path::Path, process::Command};

use tempfile::tempdir;

fn ush(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_ush"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run ush");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

fn project(files: &[(&str, &str)]) -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    for (path, text) in files {
        let path = dir.path().join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    dir
}

const LIB: &str = "pub fn shout(text: String) -> String {\n  text + \"!\"\n}\n\npub fn fail() {\n  print \"x\"\n}\n";

#[test]
fn run_check_test_and_effects_follow_module_imports() {
    let dir = project(&[
        ("lib/text.ush", LIB),
        (
            "main.ush",
            "mod \"lib/text.ush\"\nuse std::test::{assert_eq}\nprint text::shout \"hi\"\ntest \"shout\" {\n  assert_eq (text::shout \"a\") \"a!\"\n}\n",
        ),
    ]);
    assert_eq!(ush(dir.path(), &["main.ush"]).1, "hi!\n");
    assert!(ush(dir.path(), &["check", "main.ush"]).0);
    let (passed, stdout, _) = ush(dir.path(), &["test", "main.ush"]);
    assert!(passed, "{stdout}");
    assert!(stdout.contains("1 passed; 0 failed"), "{stdout}");
    let (_, effects, _) = ush(dir.path(), &["effects", "main.ush"]);
    assert!(effects.contains("text::shout"), "{effects}");
}

#[test]
fn check_reports_a_module_error_on_the_importing_line() {
    let dir = project(&[
        (
            "lib/bad.ush",
            "pub fn f() -> String {\n  \"a\" + missing\n}\n",
        ),
        ("main.ush", "print 1\nmod \"lib/bad.ush\"\nprint bad::f\n"),
    ]);
    let (passed, _, stderr) = ush(dir.path(), &["check", "main.ush"]);
    assert!(!passed);
    assert_eq!(
        stderr,
        "main.ush:2: lib/bad.ush: line 2: unknown variable: missing\n"
    );
}

#[test]
fn explain_maps_a_shell_line_into_the_module_file() {
    let dir = project(&[
        ("lib/text.ush", LIB),
        ("main.ush", "mod \"lib/text.ush\"\ntext::fail\n"),
    ]);
    let (_, shell, _) = ush(dir.path(), &["compile", "main.ush"]);
    let line = shell
        .lines()
        .position(|line| line.contains("__ush_at 6 'print \"x\"' 'lib/text.ush'"))
        .expect("module statement marker")
        + 1;
    let (passed, stdout, stderr) = ush(dir.path(), &["explain", "main.ush", &line.to_string()]);
    assert!(passed, "{stderr}");
    assert!(stdout.contains("source : lib/text.ush:6"), "{stdout}");
    assert!(stdout.contains("->    6 |   print \"x\""), "{stdout}");
}
