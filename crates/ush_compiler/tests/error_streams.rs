use std::{fs, process::Command};

use tempfile::tempdir;
use ush_compiler::UshCompiler;

fn compile_program(source: &str) -> String {
    UshCompiler::default()
        .compile_source(source)
        .expect("compile ush program")
        .to_string()
}

fn compile_error(source: &str) -> String {
    UshCompiler::default()
        .compile_source(source)
        .expect_err("program should fail to compile")
        .to_string()
}

fn run_program(source: &str) -> std::process::Output {
    let compiled = compile_program(source);
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("program.sh");
    fs::write(&script, compiled).expect("write script");
    Command::new("/bin/sh")
        .arg(&script)
        .output()
        .expect("run compiled script")
}

#[test]
fn raise_requires_an_error_adt() {
    let error = compile_error(
        r#"
        enum Problem {
          Nope,
        }

        fn fail() -> () / Problem, stderr {
          raise "oops"
        }
    "#,
    );

    assert!(error.contains("raise expects an error ADT value"));
}

#[test]
fn omitted_error_rows_are_inferred_for_callers() {
    let output = run_program(
        r#"
        enum Problem {
          Nope,
        }

        fn fail() -> String {
          fail (Problem::Nope)
        }

        fn caller() -> String {
          let value = fail?
          value
        }

        print caller
    "#,
    );

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Problem::Nope"));
}

#[test]
fn declared_error_sets_must_cover_inferred_errors() {
    let error = compile_error(
        r#"
        enum Problem {
          Nope,
        }

        enum Other {
          Nope,
        }

        fn fail() -> String / Other, stderr {
          fail (Problem::Nope)
        }
    "#,
    );

    assert!(error.contains("declares `String / Other`"));
    assert!(error.contains("can raise `Problem`"));
}

#[test]
fn call_sites_use_declared_error_sets_as_the_function_contract() {
    let error = compile_error(
        r#"
        enum Problem {
          Nope,
        }

        enum Other {
          Nope,
        }

        fn fail() -> String / Other, Problem, stderr {
          fail (Problem::Nope)
        }

        fn outer() -> String / Problem, stderr {
          let value = fail()?
          value
        }
    "#,
    );

    assert!(error.contains("function `outer` declares `String / Problem`"));
    assert!(error.contains("can raise `Other | Problem`"));
}

#[test]
fn function_error_streams_are_inferred_and_composed() {
    let compiled = compile_program(
        r#"
        enum Problem {
          Nope,
        }

        fn leaf() -> String / Problem, stderr {
          fail (Problem::Nope)
        }

        fn wrap(message: String) -> String {
          "<" + message + ">"
        }

        fn mixed() -> String / Problem, unknown, exec, fs, env, net, io {
          $ false
          wrap(leaf()?)
        }

        fn awaited() -> String / Problem, stderr, task {
          let task = async leaf ()
          let value = task.await
          value
        }
    "#,
    );

    assert!(compiled.contains("# raises: Problem\nush_fn_leaf()"));
    assert!(compiled.contains("# raises: Problem | unknown\nush_fn_mixed()"));
    assert!(compiled.contains("# raises: Problem\nush_fn_awaited()"));
}

#[test]
fn raise_aborts_runtime_with_typed_message() {
    let output = run_program(
        r#"
        enum Problem {
          Nope,
        }

        fn fail() -> String / Problem, stderr {
          fail (Problem::Nope)
        }

        print $ fail ()
    "#,
    );

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Problem::Nope (exit 1)"));
}

#[test]
fn piped_raise_aborts_runtime_with_typed_message() {
    let output = run_program(
        r#"
        enum Problem {
          Nope,
        }

        fn fail() -> String / Problem, stderr {
          fail (Problem::Nope)
        }

        fn wrap(message: String) -> String {
          message
        }

        print wrap (fail ())
    "#,
    );

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Problem::Nope (exit 1)"));
}

#[test]
fn try_operator_returns_from_the_current_function() {
    let compiled = compile_program(
        r#"
        enum Problem {
          Nope,
        }

        fn fail() -> String / Problem, stderr {
          fail (Problem::Nope)
        }

        fn outer() -> String / Problem, stderr {
          let value = fail()?
          value
        }
    "#,
    );

    assert!(compiled.contains("# raises: Problem\nush_fn_outer()"));
    assert!(compiled.contains(
        "$(__ush_return_path='' __ush_capture_return='1' ush_fn_fail 4>&1 >&3)\" || return \"$?\""
    ));
}

#[test]
fn try_statements_propagate_call_and_shell_failures() {
    let compiled = compile_program(
        r#"
        fn outer() -> () / unknown, exec, fs, env, net, io {
          let command = "false"
          shell command?
          helper()?
        }

        fn helper() -> () / unknown, exec, fs, env, net, io {
          $ false?
        }
    "#,
    );

    assert!(compiled.contains("eval \"${__ush_binding_3_command}\"\n"));
    // The last line of a `-> ()` function is a statement, not a value.
    assert!(compiled.contains(
        "( __ush_capture_return='0' __ush_return_path=''; ush_fn_helper ) || return \"$?\""
    ));
    assert!(compiled.contains("__ush_report \"$__ush_status\"; return \"$__ush_status\""));
}
