use std::{fs, process::Command};
use tempfile::tempdir;
use ush_compiler::{ShellTarget, UshCompiler};

fn run(shell: &str, source: &str, help: bool) -> std::process::Output {
    let dir = tempdir().unwrap();
    let script = dir.path().join("program.sh");
    let target = match shell {
        "bash" => ShellTarget::Bash,
        "zsh" => ShellTarget::Zsh,
        _ => ShellTarget::Sh,
    };
    fs::write(
        &script,
        UshCompiler::for_target(target)
            .compile_source(source)
            .unwrap(),
    )
    .unwrap();
    let mut command = Command::new(shell);
    command.arg(script).current_dir(dir.path());
    if help {
        command
            .arg("--help")
            .env("TMPDIR", dir.path().join("missing"));
    }
    command.output().unwrap()
}

#[test]
fn mixed_boolean_conditions_follow_source_precedence_and_short_circuit() {
    let mut source = String::new();
    let mut expected = String::new();
    for a in [false, true] {
        for b in [false, true] {
            for c in [false, true] {
                for (expression, value) in [
                    (format!("{a} || {b} && {c}"), a || b && c),
                    (format!("{a} && {b} || {c}"), a && b || c),
                ] {
                    source.push_str(&format!(
                        "if {expression} {{\n print \"yes\"\n}}\nelse {{\n print \"no\"\n}}\n"
                    ));
                    expected.push_str(if value { "yes\n" } else { "no\n" });
                }
            }
        }
    }
    source.push_str("fn crash() -> Bool / unknown, stderr {\n fail 23 \"must not run\"\n}\nif true || crash && false {\n print \"short-circuit\"\n}\n");
    expected.push_str("short-circuit\n");
    for shell in ["sh", "bash", "zsh"] {
        if Command::new(shell).arg("--version").output().is_err() {
            continue;
        }
        let result = run(shell, &source, false);
        assert!(
            result.status.success(),
            "{shell}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(String::from_utf8_lossy(&result.stdout), expected, "{shell}");
    }
}

#[test]
fn user_call_failures_abort_captured_functions_on_bash() {
    let source = r#"
fn measure() -> String / unknown, stderr {
  fail 23 "measurement failed"
}
fn compare() -> Int / unknown, stderr, stdout {
  let value = measure
  print "must not run"
  0
}
print compare
"#;
    for shell in ["sh", "bash", "zsh"] {
        if Command::new(shell).arg("--version").output().is_err() {
            continue;
        }
        let result = run(shell, source, false);
        assert_eq!(result.status.code(), Some(23), "{shell}");
        assert!(!String::from_utf8_lossy(&result.stdout).contains("must not run"));
        assert!(String::from_utf8_lossy(&result.stderr).contains("measurement failed"));
    }
}

#[test]
fn help_precedes_temporary_storage_and_nested_calls_still_get_storage() {
    let source = r#"
use std::process::{arg, exit}
use std::fs::temp_dir
fn create() -> String / unknown, fs {
  temp_dir
}
fn nested() -> String / unknown, fs {
  create
}
if arg 0 == "--help" {
  print "usage"
  exit 0
}
print nested
"#;
    for shell in ["sh", "bash", "zsh"] {
        if Command::new(shell).arg("--version").output().is_err() {
            continue;
        }
        let result = run(shell, source, true);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(String::from_utf8_lossy(&result.stdout), "usage\n");
        let result = run(shell, source, false);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let output = String::from_utf8_lossy(&result.stdout);
        assert!(!output.trim().is_empty());
        assert!(!std::path::Path::new(output.trim()).exists());
    }
}

#[test]
fn invalid_process_indices_and_statuses_infer_errors() {
    for call in ["std::process::arg -1", "std::process::exit 256"] {
        let ty = if call.contains("::arg") {
            "String"
        } else {
            "()"
        };
        let error = UshCompiler
            .compile_source(&format!("fn bad() -> {ty} / env {{\n {call}\n}}\n"))
            .unwrap_err();
        assert!(
            error.to_string().contains("can raise `unknown`"),
            "{error:#}"
        );
    }
}

#[test]
fn legacy_typed_fail_keeps_its_expression_fallback() {
    UshCompiler
        .compile_source(
            r#"
enum Problem {
  Nope,
}
fn fail_legacy() -> () / Problem, stderr {
  let error = Problem::Nope
  fail $error
}
"#,
        )
        .unwrap();
}
