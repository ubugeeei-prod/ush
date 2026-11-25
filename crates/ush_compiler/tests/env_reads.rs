//! `$env::NAME` reads a required environment variable under every
//! installed shell: a missing or empty variable stops the program with its
//! name and source line, and `std::env::get_or` supplies defaults.

mod support;

use support::each_program;
use ush_compiler::UshCompiler;

const PRELUDE: &str = "use std::env::{get_or, set, unset}\n";

fn each_run(body: &str, check: impl FnMut(&str, &std::path::Path, (i32, String, String))) {
    each_program(&format!("{PRELUDE}{body}"), check);
}

#[test]
fn a_set_variable_reads_as_a_string_in_expressions_words_and_strings() {
    each_run(
        r#"use $bin::*
set USH_TEST_DIR "/tmp/a b"
set USH_TEST_SIDE "$x `y` *"
let dir = $env::USH_TEST_DIR
print $"$env::USH_TEST_DIR/ambient-${env::USH_TEST_SIDE}.jsonl"
echo --dir=$env::USH_TEST_DIR
let side = $env::USH_TEST_SIDE
print side.starts_with("$x")
echo $env::USH_TEST_SIDE.txt
if $env::USH_TEST_DIR == dir {
  print "same"
}
fn read() -> String / unknown, env {
  $env::USH_TEST_SIDE
}
print read
"#,
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(
                stdout,
                "/tmp/a b/ambient-$x `y` *.jsonl\n--dir=/tmp/a b\ntrue\n$x `y` *.txt\nsame\n$x `y` *\n",
                "{shell}"
            );
        },
    );
}

#[test]
fn an_unset_variable_stops_with_its_name_and_line() {
    each_run(
        "unset USH_TEST_MISSING\nprint \"before\"\nlet dir = $env::USH_TEST_MISSING\nprint \"unreachable\"\n",
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 1, "{shell}: {stderr}");
            assert_eq!(stdout, "before\n", "{shell}");
            assert!(
                stderr.contains(
                    "<memory>:4: environment variable USH_TEST_MISSING is not set or is empty (exit 1)"
                ),
                "{shell}: {stderr}"
            );
        },
    );
}

#[test]
fn an_empty_variable_is_missing_too_and_get_or_supplies_defaults() {
    each_run(
        r#"set USH_TEST_EMPTY ""
unset USH_TEST_UNSET
print (get_or USH_TEST_UNSET fallback)
print $"[${get_or "USH_TEST_EMPTY" "fallback"}]"
fn tmp() -> String / unknown, env {
  $env::USH_TEST_EMPTY
}
print tmp
"#,
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 1, "{shell}: {stderr}");
            assert_eq!(stdout, "fallback\n[]\n", "{shell}");
            assert!(
                stderr.contains("environment variable USH_TEST_EMPTY is not set or is empty"),
                "{shell}: {stderr}"
            );
        },
    );
}

#[test]
fn reads_are_typed_and_inferred() {
    let error = UshCompiler
        .compile_source("fn dir() -> String / env {\n  $env::RUNNER_TEMP\n}\nprint dir\n")
        .expect_err("a read can fail, so the error row must say so");
    assert!(
        format!("{error:#}").contains("can raise `unknown`"),
        "{error:#}"
    );
    let error = UshCompiler
        .compile_source("let HOME = \"x\"\nprint $env::HOME\n")
        .expect_err("a binding named like the variable shadows it in the shell");
    assert!(
        format!("{error:#}")
            .contains("`$env::HOME` reads the environment, but `HOME` is also a ush binding"),
        "{error:#}"
    );
    let report = UshCompiler
        .effects_source("fn dir() {\n  $env::RUNNER_TEMP\n}\n")
        .unwrap();
    assert_eq!(report.functions[0].inferred.render_row(), "env");
}

#[test]
fn names_are_checked() {
    for (source, message) in [
        (
            "print $env::__ush_x\n",
            "names starting with `__ush` are reserved",
        ),
        (
            "use $bin::*\necho --dir=$env::\n",
            "write an environment variable name",
        ),
    ] {
        let error = UshCompiler.compile_source(source).unwrap_err();
        assert!(format!("{error:#}").contains(message), "{error:#}");
    }
}
