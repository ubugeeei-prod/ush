//! Commands used for their exit status under every installed shell: as
//! `if` and `while` conditions with `!`, `&&` and `||`, through
//! `status_of`, and through `allow_fail`. The command's output streams and
//! its failure never stops the script.

mod support;

use support::each_program;
use ush_compiler::UshCompiler;

const PRELUDE: &str = "use $bin::*\nuse std::io::{discard_stderr, to_file}\n";

fn succeeds(body: &str, expected: &str) {
    each_program(
        &format!("{PRELUDE}{body}"),
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(stdout, expected, "{shell}: {stderr}");
            assert!(!stderr.contains("failed"), "{shell}: {stderr}");
        },
    );
}

fn compile_error(body: &str) -> String {
    let error = UshCompiler::default()
        .compile_source(&format!("{PRELUDE}{body}"))
        .expect_err("the program should not compile");
    format!("{error:#}")
}

#[test]
fn a_failing_condition_command_takes_the_else_branch_and_streams_its_output() {
    succeeds(
        r#"if sh "-c" "echo checking; exit 1" {
  print "unreachable"
}
else {
  print "failed quietly"
}
if sh "-c" "echo passing" {
  print "passed"
}
print "after"
"#,
        "checking\nfailed quietly\npassing\npassed\nafter\n",
    );
}

#[test]
fn negation_and_logic_combine_commands_with_booleans() {
    succeeds(
        r#"let ready = true
if ! grep "-q" "absent" "notes.txt" | discard_stderr && ready {
  print "not found"
}
if false || test "-n" "x" {
  print "test is still a Bool"
}
if !(sh "-c" "exit 0") || sh "-c" "exit 1" {
  print "unreachable"
}
if ! ready {
  print "unreachable"
}
"#,
        "not found\ntest is still a Bool\n",
    );
}

#[test]
fn while_conditions_retry_a_command_until_it_succeeds() {
    succeeds(
        r#"let mut tries = 0
while ! test "-f" "ready.flag" {
  tries = tries + 1
  if tries == 3 {
    touch "ready.flag"
  }
}
print "ready after " + tries
"#,
        "ready after 3\n",
    );
}

#[test]
fn a_pipeline_condition_fails_when_any_stage_fails() {
    succeeds(
        r#"if printf "abc\n" | grep "-q" "b" {
  print "found"
}
if sh "-c" "echo x; exit 3" | cat {
  print "unreachable"
}
else {
  print "first stage failed"
}
if seq "1" "100000" | head "-n1" | grep "-q" "1" {
  print "early close is fine"
}
"#,
        "found\nx\nfirst stage failed\nearly close is fine\n",
    );
}

#[test]
fn functions_returning_unit_are_tested_by_their_success() {
    // The function body still reports where it failed; the caller decides
    // what the failure means.
    each_program(
        &format!(
            "{PRELUDE}fn check(path: String) -> () / unknown, exec, fs, stdout, stderr {{
  ls $path | discard_stderr | to_file \"/dev/null\"
  print \"listed \" + path
}}
if check \"/\" {{
  print \"root ok\"
}}
if ! check \"/definitely/missing\" {{
  print \"missing\"
}}
"
        ),
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(stdout, "listed /\nroot ok\nmissing\n", "{shell}");
            assert!(
                stderr.contains("<memory>:4: pipeline stage 1"),
                "{shell}: {stderr}"
            );
        },
    );
}

#[test]
fn status_of_returns_the_exit_status_without_stopping() {
    succeeds(
        r#"let code = status_of sh "-c" "echo streamed; exit 3"
print "code " + code
let piped = status_of printf "x\n" | grep "-q" "y"
print "piped " + piped
let ok = status_of sh "-c" "exit 0"
if (status_of sh "-c" "exit 4") == 4 {
  print "compared"
}
print "ok " + ok
"#,
        "streamed\ncode 3\npiped 1\ncompared\nok 0\n",
    );
}

#[test]
fn allow_fail_tolerates_failure_and_collects_results() {
    succeeds(
        r#"allow_fail sh "-c" "echo first; exit 2"
allow_fail grep "nothing" "missing.txt" | discard_stderr
let mut failed = ""
for check in ["true", "false", "true"] {
  if ! sh "-c" $check {
    failed = failed + check + " "
  }
}
print "failed: " + failed
"#,
        "first\nfailed: false \n",
    );
}

#[test]
fn argument_failures_still_stop_the_script() {
    each_program(
        &format!(
            "{PRELUDE}let code = status_of printf \"%s\" (sh \"-c\" \"exit 5\")\nprint \"unreachable\"\n"
        ),
        |shell, _, (status, stdout, _)| {
            assert_eq!(status, 5, "{shell}");
            assert_eq!(stdout, "", "{shell}");
        },
    );
}

#[test]
fn values_that_are_not_commands_are_rejected() {
    for (body, message) in [
        ("if \"text\" {\n}", "must be a Bool or a command"),
        (
            "fn name() -> String {\n  \"x\"\n}\nif name {\n}",
            "must be a Bool or a command",
        ),
        ("if git \"diff\"? {\n}", "remove the `?`"),
        (
            r#"let code = status_of "text""#,
            "`status_of` runs a command",
        ),
        (r#"allow_fail 1"#, "`allow_fail` runs a command"),
        (
            r#"let code = status_of test "-f" "x""#,
            "`status_of` runs a command",
        ),
        ("if ! let x = 1 {\n}", "cannot be negated"),
    ] {
        let error = compile_error(body);
        assert!(error.contains(message), "{body}\n=> {error}");
    }
}

#[test]
fn status_tests_do_not_add_unknown_to_a_function_row() {
    let source = format!(
        "{PRELUDE}fn probe() -> Bool / exec, stdout, stderr, stdin, fs, env, net {{\n  if git \"diff\" \"--quiet\" {{\n    return true\n  }}\n  allow_fail git \"fetch\"\n  (status_of git \"status\") == 0\n}}\nprint probe\n"
    );
    UshCompiler::default()
        .compile_source(&source)
        .unwrap_or_else(|error| panic!("{error:#}"));
}
