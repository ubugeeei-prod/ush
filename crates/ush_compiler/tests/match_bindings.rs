mod support;

use support::each_program;
use ush_compiler::UshCompiler;

fn succeeds(source: &str, expected: &str) {
    each_program(source, |shell, _, (status, stdout, stderr)| {
        assert_eq!(status, 0, "{shell}: {stderr}");
        assert_eq!(stdout, expected, "{shell}");
    });
}

fn rejects(source: &str, expected: &str) {
    let error = UshCompiler.compile_source(source).expect_err("must reject");
    assert!(format!("{error:#}").contains(expected), "{error:#}");
}

#[test]
fn platform_selection_is_immutable_and_does_not_return_from_the_function() {
    succeeds(
        r#"fn target(os: String, arch: String) -> String {
  let os = match os {
    "Linux" => "unknown-linux-gnu"
    "Darwin" => "apple-darwin"
    other => fail 1 ("unsupported OS: " + other)
  }
  let arch = match arch {
    "arm64" | "aarch64" => "aarch64"
    "amd64" => "x86_64"
    other => other
  }
  arch + "-" + os
}
echo (target Linux amd64)
echo (target Darwin arm64)
echo (target Linux riscv64)
"#,
        "x86_64-unknown-linux-gnu\naarch64-apple-darwin\nriscv64-unknown-linux-gnu\n",
    );
}

#[test]
fn only_the_selected_arm_runs_and_the_subject_runs_once() {
    each_program(
        r#"use std::io::eprint
use std::string::to_int

fn source() -> String {
  eprint once
  "yes"
}
let selected = match source {
  "yes" => 42
  _ => to_int invalid
}
echo $selected
"#,
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(
                (status, stdout.as_str(), stderr.as_str()),
                (0, "42\n", "once\n"),
                "{shell}"
            );
        },
    );
}

#[test]
fn literals_preserve_newlines_and_patterns_do_not_escape_their_scope() {
    succeeds(
        r#"let other = "outer"
let text = match "inner" {
  other => other + "\n\n"
}
echo (text + "end") $other
let mut count = 1
count = match count {
  1 => count + 1
  _ => 0
};
echo $count
"#,
        "inner\n\nend outer\n2\n",
    );
}

#[test]
fn structured_results_work_with_generics_and_different_list_lengths() {
    succeeds(
        r#"fn choose<T>(yes: Bool, left: T, right: T) -> T {
  let result = match yes {
    true => left
    false => right
  }
  result
}
enum Item {
  Text(String),
  Empty,
}
let xs = match Item::Text("value") {
  Item::Text(text) => [text, "tail"]
  Item::Empty => ["empty"]
}
let tuple = match true {
  true => (1, xs)
  false => (2, ["other"])
}
echo (tuple.0) (tuple.1.join(","))
echo (choose false first second)
let item = match false {
  true => Item::Empty
  false => Item::Text("ok")
}
match item {
  Item::Text(text) => echo $text
  Item::Empty => echo empty
}
"#,
        "1 value,tail\nsecond\nok\n",
    );
}

#[test]
fn failed_arms_report_their_own_source_line() {
    each_program(
        "let os = match \"unsupported\" {\n  \"Linux\" => \"linux\"\n  other => fail 7 (\"unsupported: \" + other)\n}\necho unreachable\n",
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 7, "{shell}: {stderr}");
            assert!(stdout.is_empty());
            assert!(
                stderr.contains("<memory>:3: unsupported: unsupported (exit 7)"),
                "{shell}: {stderr}"
            );
        },
    );
}

#[test]
fn bindings_remain_typed_immutable_and_exhaustive() {
    rejects(
        "let x = match true {\n  true => 1\n  false => 2\n}\nx = 3\n",
        "immutable binding `x`",
    );
    rejects(
        "let x = match true {\n  true => 1\n  false => \"no\"\n}\n",
        "match arm type mismatch",
    );
    rejects("let x = match true {\n  true => 1\n}\n", "non-exhaustive");
    rejects(
        "let x = match \"a\" {\n  other => other\n}\necho $other\n",
        "unknown variable: other",
    );
    rejects(
        "let x = match true {\n  true => echo yes;\n  false => 2\n}\n",
        "must produce a value",
    );
    rejects(
        "fn f() -> String / () {\n  let x = match false {\n    true => \"ok\"\n    false => fail 1 \"no\"\n  }\n  x\n}\n",
        "unknown",
    );
}

#[test]
fn literal_selection_lowers_directly_to_case() {
    let generated = UshCompiler
        .compile_source("let x = match \"a\" {\n  \"a\" => \"one\"\n  _ => \"two\"\n}\necho $x\n")
        .unwrap();
    assert!(generated.contains("case \"${__ush_"), "{generated}");
    assert!(!generated.contains("task_block"));
    assert!(!generated.contains("__ush_capture_return"));
}
