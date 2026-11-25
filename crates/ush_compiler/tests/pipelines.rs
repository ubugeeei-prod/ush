//! Pipelines run under every installed shell: streaming, capture,
//! portable pipefail, readers that stop early, `|&`, function stages and
//! continuation lines. File stages and String stdin are in
//! `pipeline_files.rs`.

mod support;

use std::fs;

use support::{each_program, each_shell, run_with_sigpipe};
use tempfile::tempdir;

const PRELUDE: &str = "use $bin::*\nuse std::io::{append_to, from_file, to_file}\n";

fn each_run(body: &str, check: impl FnMut(&str, &std::path::Path, (i32, String, String))) {
    each_program(&format!("{PRELUDE}{body}"), check);
}

fn succeeds(body: &str, expected: &str) {
    each_run(body, |shell, _, (status, stdout, stderr)| {
        assert_eq!(status, 0, "{shell}: {stderr}");
        assert_eq!(stdout, expected, "{shell}");
    });
}

#[test]
fn statements_stream_and_values_capture_the_last_stage() {
    succeeds(
        r#"printf "b\na\n" | sort
let first = printf "x\ny\n" | head "-n1"
print "first=" + first
if (printf "abc" | tr "a-z" "A-Z") == "ABC" {
  print "matched"
}
std::process::exec "printf" ["%s\n", "via exec"] | tr "a-z" "A-Z"
"#,
        "a\nb\nfirst=x\nmatched\nVIA EXEC\n",
    );
}

#[test]
fn a_failing_middle_stage_fails_the_pipeline_and_names_the_stage() {
    each_run(
        "printf \"x\" | grep \"nomatch\" | cat\nprint \"unreachable\"\n",
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 1, "{shell}: {stderr}");
            assert_eq!(stdout, "", "{shell}");
            assert!(
                stderr.contains("<memory>:3: pipeline stage 2 failed: grep \"nomatch\" (exit 1)"),
                "{shell}: {stderr}"
            );
            assert_eq!(stderr.matches("failed").count(), 1, "{shell}: {stderr}");
        },
    );
}

#[test]
fn a_failing_first_stage_keeps_its_status_even_when_captured() {
    each_run(
        "let value = sh \"-c\" \"exit 7\" | cat\nprint \"unreachable \" + value\n",
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 7, "{shell}: {stderr}");
            assert_eq!(stdout, "", "{shell}");
            assert!(
                stderr.contains("pipeline stage 1 failed: sh \"-c\" \"exit 7\" (exit 7)"),
                "{shell}: {stderr}"
            );
            assert_eq!(stderr.matches("failed").count(), 1, "{shell}: {stderr}");
        },
    );
}

#[test]
fn a_producer_stopped_by_an_early_closing_consumer_is_not_a_failure() {
    // With SIGPIPE ignored, producers see a write error instead of dying
    // with 141 and exit with a status of their own choosing.
    for ignore_sigpipe in [false, true] {
        each_shell(|shell, target| {
            let dir = tempdir().unwrap();
            let source = format!("{PRELUDE}{EARLY_CLOSE}");
            let (status, stdout, stderr) =
                run_with_sigpipe(shell, target, dir.path(), &source, ignore_sigpipe);
            assert_eq!(status, 0, "{shell} ignore={ignore_sigpipe}: {stderr}");
            assert_eq!(stdout, "1\nline 1\ntext\n1\ny\n", "{shell}");
        });
    }
    each_run(
        "seq \"1\" \"1000000\" | sh \"-c\" \"exit 3\"\n",
        |shell, _, (status, _, stderr)| {
            assert_eq!(status, 3, "{shell}: {stderr}");
            assert!(stderr.contains("stage 2 failed"), "{shell}: {stderr}");
        },
    );
}

const EARLY_CLOSE: &str = r#"fn lines() {
  for n in 1..20000 {
    print "line " + n
  }
}
seq "1" "1000000" | head "-n1"
lines | head "-n1"
std::string::repeat "text\n" 20000 | head "-n1"
seq "1" "1000000" | cat | head "-n1"
sh "-c" "while echo y; do :; done; exit 9" | head "-n1"
"#;

#[test]
fn stderr_joins_the_pipe_only_after_a_merging_stage() {
    each_run(
        r#"sh "-c" "echo out; echo err >&2" |& sort
sh "-c" "echo out; echo err >&2" | sort
sh "-c" "echo out; echo err >&2" |& to_file "both.log"
"#,
        |shell, dir, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(stdout, "err\nout\nout\n", "{shell}");
            assert_eq!(stderr, "err\n", "{shell}");
            let log = fs::read_to_string(dir.join("both.log")).unwrap();
            assert_eq!(log.lines().count(), 2, "{shell}: {log}");
        },
    );
}

#[test]
fn function_stages_read_stdin_and_failures_propagate_out_of_functions() {
    each_run(
        r#"fn shout() -> () / unknown, io {
  print (std::string::to_upper_ascii (std::io::read_all))
}
fn broken() -> () / unknown, exec, io, fs, env, net {
  printf "x" | sh "-c" "exit 4" | cat
  print "unreachable"
}
printf "hi" | shout
broken
print "unreachable"
"#,
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 4, "{shell}: {stderr}");
            assert_eq!(stdout, "HI\n", "{shell}");
            assert!(
                stderr.contains("<memory>:7: pipeline stage 2 failed"),
                "{shell}: {stderr}"
            );
        },
    );
}

#[test]
fn a_trailing_pipe_continues_the_pipeline_on_the_next_line() {
    succeeds(
        "printf \"b\\na\\nc\\n\" |\n  sort |\n\n  # keep the first\n  head \"-n1\"\nprint \"done\"\n",
        "a\ndone\n",
    );
}

#[test]
fn the_last_line_of_a_unit_function_streams() {
    succeeds(
        "fn swap() -> () / unknown, exec, io, fs, env, net {\n  printf \"a\\n\" | tr \"a\" \"b\"\n}\nswap\n",
        "b\n",
    );
}

#[test]
fn a_string_function_feeds_its_value_and_question_marks_cover_the_pipeline() {
    succeeds(
        r#"fn greeting(name: String) -> String {
  "hello " + name + "\n"
}
# A returned String loses its trailing newline, like any captured value.
greeting "ush" | tr "a-z" "A-Z"
print ""
let upper = (greeting "ada" | tr "a-z" "A-Z")?
print (printf "%s" upper | wc "-c" | tr "-d" " ")
printf "x\n" | cat?
"#,
        "HELLO USH\n9\nx\n",
    );
}
