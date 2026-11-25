//! `discard_stderr`, `stderr_to`, `stderr_append_to` and `to_stderr`
//! under every installed shell: stderr is redirected on its own while
//! stdout keeps streaming, and stdout can be sent to stderr.

mod support;

use std::fs;

use support::each_program;
use ush_compiler::UshCompiler;

const PRELUDE: &str = "use $bin::*\nuse std::io::{append_to, discard_stderr, stderr_append_to, stderr_to, to_file, to_stderr}\n";
const BOTH: &str = r#"sh "-c" "echo out; echo err >&2""#;

fn each_run(body: &str, check: impl FnMut(&str, &std::path::Path, (i32, String, String))) {
    each_program(&format!("{PRELUDE}{body}"), check);
}

#[test]
fn discard_stderr_keeps_stdout_streaming() {
    each_run(
        &format!("{BOTH} | discard_stderr\n{BOTH} | discard_stderr | tr \"a-z\" \"A-Z\"\n"),
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(stdout, "out\nOUT\n", "{shell}");
            assert_eq!(stderr, "", "{shell}");
        },
    );
}

#[test]
fn stdout_and_stderr_go_to_separate_files_with_spaces_in_their_names() {
    each_run(
        &format!(
            r#"let dir = "log dir"
mkdir $dir
{BOTH} | stderr_to (dir + "/err file.log") | to_file (dir + "/out file.log")
{BOTH} | stderr_append_to (dir + "/err file.log") | append_to (dir + "/out file.log")
let captured = {BOTH} | stderr_to (dir + "/captured err.log")
print "captured " + captured
"#
        ),
        |shell, dir, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(stdout, "captured out\n", "{shell}");
            assert_eq!(stderr, "", "{shell}");
            let read = |name: &str| fs::read_to_string(dir.join("log dir").join(name)).unwrap();
            assert_eq!(read("out file.log"), "out\nout\n", "{shell}");
            assert_eq!(read("err file.log"), "err\nerr\n", "{shell}");
            assert_eq!(read("captured err.log"), "err\n", "{shell}");
        },
    );
}

#[test]
fn to_stderr_sends_stdout_to_stderr() {
    each_run(
        r#"printf "warning\n" | to_stderr
"note\n" | to_stderr
sh "-c" "echo both; echo err >&2" |& to_stderr
print "done"
"#,
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(stdout, "done\n", "{shell}");
            assert_eq!(stderr, "warning\nnote\nboth\nerr\n", "{shell}");
        },
    );
}

#[test]
fn a_discarded_stderr_reads_well_in_a_condition() {
    each_run(
        r#"git "init" "-q" "repo"
cd "repo" {
  if git "cat-file" "-e" "HEAD:README.md" | discard_stderr {
    print "exists"
  }
  else {
    print "no such object"
  }
}
"#,
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(stdout, "no such object\n", "{shell}");
            assert_eq!(stderr, "", "{shell}");
        },
    );
}

#[test]
fn a_failing_stage_with_redirected_stderr_still_fails_and_is_named() {
    each_run(
        "sh \"-c\" \"echo hidden >&2; exit 3\" | stderr_to \"err.log\"\nprint \"unreachable\"\n",
        |shell, dir, (status, stdout, stderr)| {
            assert_eq!(status, 3, "{shell}: {stderr}");
            assert_eq!(stdout, "", "{shell}");
            assert!(
                stderr.contains("pipeline stage 1 failed: sh \"-c\""),
                "{shell}: {stderr}"
            );
            assert!(
                !stderr.lines().any(|line| line == "hidden"),
                "{shell}: {stderr}"
            );
            let log = fs::read_to_string(dir.join("err.log")).unwrap();
            assert_eq!(log, "hidden\n", "{shell}");
        },
    );
}

#[test]
fn misplaced_or_conflicting_redirections_are_rejected() {
    for (body, message) in [
        (r#"discard_stderr | cat"#, "needs a stage before it"),
        (r#"to_stderr | cat"#, "needs a stage before it"),
        (
            r#"printf "x" | to_stderr | cat"#,
            "can only be the last stage",
        ),
        (
            r#"printf "x" |& discard_stderr"#,
            "already redirects its stderr",
        ),
        (
            r#"printf "x" | discard_stderr |& cat"#,
            "cannot be followed by `|&`",
        ),
        (
            r#"printf "x" | discard_stderr | stderr_to "e""#,
            "already redirects its stderr",
        ),
        (r#"printf "x" | discard_stderr "e""#, "takes no arguments"),
        (
            r#"printf "x" | stderr_to """#,
            "a file path must not be empty",
        ),
        (
            r#"let x = printf "x" | to_stderr"#,
            "has no output to capture",
        ),
        (
            r#"discard_stderr"#,
            "redirect the stderr of the stage before them",
        ),
        (r#"to_stderr"#, "pipeline sinks"),
    ] {
        let error = UshCompiler::default()
            .compile_source(&format!("{PRELUDE}{body}"))
            .expect_err(body);
        let error = format!("{error:#}");
        assert!(error.contains(message), "{body}\n=> {error}");
    }
}
