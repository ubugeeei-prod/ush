use std::{fs, process::Command};

use tempfile::tempdir;
use ush_compiler::UshCompiler;

fn run_program(source: &str) -> String {
    let compiled = UshCompiler::default()
        .compile_source(source)
        .expect("compile ush program");
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("program.sh");
    fs::write(&script, compiled).expect("write script");

    let output = Command::new("/bin/sh")
        .arg(&script)
        .output()
        .expect("run compiled script");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn tuple_variant_match_binds_inner_value() {
    let output = run_program(
        r#"
        enum Option {
          None,
          Some(String),
        }
        let value = Option::Some("hello")
        match value {
          Option::Some(message) => print message
          _ => print "fallback"
        }
    "#,
    );

    assert_eq!(output, "hello\n");
}

#[test]
fn struct_variant_supports_shorthand_binding() {
    let output = run_program(
        r#"
        enum Response {
          Http { code: Int, body: String },
          Empty,
        }
        let code = 200
        let body = "ok"
        let response = Response::Http { code, body }
        match response {
          Response::Http { code, body } => print body + ":" + code
          _ => print "fallback"
        }
    "#,
    );

    assert_eq!(output, "ok:200\n");
}

#[test]
fn nested_variants_can_be_composed() {
    let output = run_program(
        r#"
        enum Response {
          Ok(String),
          Err(String),
        }
        enum Envelope {
          Wrap(Response),
          Missing,
        }
        let wrapped = Envelope::Wrap(Response::Ok("done"))
        match wrapped {
          Envelope::Wrap(Response::Ok(message)) => print message
          _ => print "fallback"
        }
    "#,
    );

    assert_eq!(output, "done\n");
}

#[test]
fn adt_variables_can_be_reused_in_other_constructors() {
    let output = run_program(
        r#"
        enum Response {
          Http { code: Int, body: String },
          Empty,
        }
        enum Envelope {
          Wrap(Response),
          Missing,
        }
        let response = Response::Http { code: 200, body: "ok" }
        let wrapped = Envelope::Wrap(response)
        match wrapped {
          Envelope::Wrap(Response::Http { body, code }) => print body + ":" + code
          _ => print "fallback"
        }
    "#,
    );

    assert_eq!(output, "ok:200\n");
}

#[test]
fn wildcard_arms_still_work_for_enums() {
    let output = run_program(
        r#"
        enum Option {
          None,
          Some(String),
        }
        let value = Option::None
        match value {
          Option::Some(message) => print message
          _ => print "none"
        }
    "#,
    );

    assert_eq!(output, "none\n");
}

#[test]
fn sync_function_calls_execute_in_order() {
    let output = run_program(
        r#"
        fn greet(message: String, count: Int) {
          print message + ":" + count
        }
        print "before"
        greet "hello" 2
        print "after"
    "#,
    );

    assert_eq!(output, "before\nhello:2\nafter\n");
}

#[test]
fn await_returns_async_function_result() {
    let output = run_program(
        r#"
        fn worker(message: String) -> String {
          message
        }
        print "main"
        let task = async worker "worker"
        print "after"
        let result = task.await
        print result
    "#,
    );

    assert_eq!(output, "main\nafter\nworker\n");
}

#[test]
fn awaited_values_keep_their_declared_type() {
    let output = run_program(
        r#"
        fn compute(value: Int) -> Int {
          value + 2
        }
        let task = async compute 40
        let result = task.await
        print result + 2
    "#,
    );

    assert_eq!(output, "44\n");
}

#[test]
fn call_expressions_support_space_separated_arguments() {
    let output = run_program(
        r#"
        fn greet(message: String, count: Int) -> String {
          message + ":" + count
        }
        let value = greet "hello" 2
        print value
    "#,
    );

    assert_eq!(output, "hello:2\n");
}

#[test]
fn omitted_unit_return_types_behave_like_explicit_unit() {
    let output = run_program(
        r#"
        fn note(message: String) {
          print message
        }
        let call_result = note "hello"
        print call_result == ()
        let task = async note "worker"
        let awaited = task.await
        print awaited == ()
    "#,
    );

    assert_eq!(output, "hello\ntrue\nworker\ntrue\n");
}

#[test]
fn dollar_operator_and_grouped_calls_work() {
    let output = run_program(
        r#"
        fn greet(name: String) -> String {
          "hi " + name
        }
        fn wrap(message: String) -> String {
          "<" + message + ">"
        }
        print $ wrap $ greet "ush"
        print $ wrap (greet "team")
    "#,
    );

    assert_eq!(output, "<hi ush>\n<hi team>\n");
}

#[test]
fn unit_style_application_calls_zero_arg_functions() {
    let output = run_program(
        r#"
        fn name() -> String {
          "ush"
        }
        print $ name ()
    "#,
    );

    assert_eq!(output, "ush\n");
}

#[test]
fn unit_functions_can_end_in_a_conditional_failure() {
    assert_eq!(
        run_program(
            r#"
fn require_name(name: String) -> () / unknown, stderr {
  if name == "" {
    fail(2, "missing name")
  }
}
require_name("ok")?
print "ok"
"#
        ),
        "ok\n"
    );
}
