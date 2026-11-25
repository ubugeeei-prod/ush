mod support;
use std::{fs, process::Command};
use tempfile::tempdir;
use ush_compiler::{ShellTarget, UshCompiler};

#[test]
fn argv_and_process_calls_preserve_literal_bytes_on_every_available_shell() {
    let mut ran = 0;
    for (shell, target) in [
        ("sh", ShellTarget::Sh),
        ("bash", ShellTarget::Bash),
        ("zsh", ShellTarget::Zsh),
    ] {
        if Command::new(shell).arg("--version").output().is_err() {
            continue;
        }
        ran += 1;
        let dir = tempdir().unwrap();
        let script = dir.path().join("program.sh");
        let source = r#"
use std::process::{args, arg, output, status, redirect}
use std::string::{lines, split, trim, to_int}
use std::fs::{write_text, read_text}
use std::test::{assert, assert_eq}
let arguments = args
assert_eq (arg 0) "--help"
assert_eq (arg 1) "a b'\"$PATH;*\n日本語"
assert_eq (arg 2) ""
assert_eq arguments ["--help", "a b'\"$PATH;*\n日本語", ""]
assert_eq (output "printf" ["%s", arg 1]) (arg 1)
assert_eq (status "test" ["a", "=", "b"]) 1
assert_eq (status "exit" ["17"]) 17
assert_eq (status "ush_missing_command_xyz" ["a"]) 127
assert_eq (redirect "printf" ["%s", arg 1] "out" "err") 0
assert_eq (read_text "out") (arg 1)
assert_eq (read_text "err") ""
write_text "joined" ("a" + "\n\n")
assert_eq (trim (output "wc" ["-l", "joined"])) "2 joined"
assert_eq (split "a::b:" ":") ["a", "", "b", ""]
assert_eq (lines "a\r\nb\n") ["a", "b"]
assert_eq (to_int (trim " \t0042\r\n")) 42
fn nested() -> String {
  let text = ""
  for value in ["first", "second"] {
    let text = text + value + "\n"
  }
  text
}
assert_eq (nested) "first\nsecond"
assert_eq (output "printf" ["%s", 'raw\n']) 'raw\n'
print "passed"
"#;
        fs::write(
            &script,
            UshCompiler::for_target(target)
                .compile_source(source)
                .unwrap(),
        )
        .unwrap();
        let result = Command::new(shell)
            .arg(script)
            .args(["--help", "a b'\"$PATH;*\n日本語", ""])
            .current_dir(dir.path())
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{shell}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(String::from_utf8_lossy(&result.stdout), "passed\n");
    }
    assert!(ran > 0, "no supported shell was available");
}

#[test]
fn temporary_directories_are_removed_on_success_and_failure() {
    for tail in ["print \"ok\"", "fail 23 \"expected failure\""] {
        let (status, stdout, stderr) =
            support::try_run(&format!("use std::fs::temp_dir\nprint temp_dir\n{tail}\n"));
        assert_eq!(status, if tail.starts_with("fail") { 23 } else { 0 });
        let path = std::path::Path::new(
            stdout
                .lines()
                .next()
                .unwrap_or_else(|| panic!("missing temporary path: {stderr}")),
        );
        assert!(!path.exists());
        assert!(!path.parent().unwrap().exists());
    }
}

#[test]
fn failed_native_boundaries_do_not_resume_a_captured_function() {
    for call in [
        "std::process::output \"ush_missing_command_xyz\" [\"x\"]",
        "std::string::to_int \"bad\"",
    ] {
        let (status, stdout, stderr) = support::try_run(&format!(
            "fn work() -> String / unknown, exec, env, fs, stdin, stderr, net {{\n  let value = {call}\n  \"unexpected\"\n}}\nprint work\n"
        ));
        assert_ne!(status, 0, "{stderr}");
        assert!(!stdout.contains("unexpected"));
    }
}

#[test]
fn an_empty_split_separator_reports_a_source_error() {
    let (status, _, stderr) =
        support::try_run("for item in std::string::split \"abc\" \"\" {\n print item\n}\n");
    assert_ne!(status, 0);
    assert!(stderr.contains("separator must not be empty"), "{stderr}");
}
