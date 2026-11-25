use std::{fs, process::Command};

use serde_json::Value;
use tempfile::tempdir;

fn ush() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ush"))
}

#[test]
fn format_check_detects_unformatted_script() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("format.ush");
    fs::write(&script, "fn greet(name: String)->String {\nprint name\n}\n").expect("write");

    let output = ush()
        .args(["format", script.to_str().unwrap(), "--check"])
        .output()
        .expect("run format check");

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("formatting required"));
}

#[test]
fn format_stdout_prints_formatted_source() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("format.ush");
    fs::write(&script, "let value=1\n").expect("write");

    let output = ush()
        .args(["format", script.to_str().unwrap(), "--stdout"])
        .output()
        .expect("run format stdout");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "let value = 1\n");
}

#[test]
fn check_reports_type_errors() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("check.ush");
    fs::write(&script, "let value = missing.await\n").expect("write");

    let output = ush()
        .args(["check", script.to_str().unwrap()])
        .output()
        .expect("run check");

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("missing"));
}

#[test]
fn compile_can_write_sourcemap_json() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("map.ush");
    let compiled = dir.path().join("map.sh");
    let sourcemap = dir.path().join("map.sh.map.json");
    fs::write(&script, "let greeting = \"hello\"\nprint greeting\n").expect("write");

    let output = ush()
        .args([
            "compile",
            script.to_str().unwrap(),
            "-o",
            compiled.to_str().unwrap(),
            "--sourcemap",
            sourcemap.to_str().unwrap(),
        ])
        .output()
        .expect("run compile");

    assert!(output.status.success());
    let shell = fs::read_to_string(&compiled).expect("read shell");
    let map: Value = serde_json::from_str(&fs::read_to_string(&sourcemap).expect("read map"))
        .expect("parse map");
    let assign_line = shell
        .lines()
        .position(|line| line == "__ush_binding_1_greeting='hello'")
        .map(|index| index + 1)
        .expect("assignment line");
    let print_line = shell
        .lines()
        .position(|line| line == "printf '%s\\n' \"${__ush_binding_1_greeting}\"")
        .map(|index| index + 1)
        .expect("print line");

    let sources = map["sources"].as_array().expect("sources array");
    let summary = &map["summary"];

    assert_eq!(map["version"], 2);
    assert_eq!(map["source"], script.display().to_string());
    assert_eq!(map["generated"], compiled.display().to_string());
    assert_eq!(summary["mapped_line_count"], 6);
    assert_eq!(summary["source_line_count"], 2);
    assert!(
        summary["generated_line_count"]
            .as_u64()
            .expect("generated line count")
            >= 2
    );
    assert!(
        summary["sections"]
            .as_array()
            .expect("summary sections")
            .iter()
            .any(|section| {
                section["section"] == "user-code" && section["mapped_line_count"] == 6
            })
    );
    assert!(sources.iter().any(|line| {
        line["source_line"] == 1
            && line["source_text"] == "let greeting = \"hello\""
            && line["generated_lines"]
                .as_array()
                .expect("generated lines")
                .contains(&Value::from(assign_line))
    }));
    assert!(sources.iter().any(|line| {
        line["source_line"] == 2
            && line["source_text"] == "print greeting"
            && line["generated_lines"]
                .as_array()
                .expect("generated lines")
                .contains(&Value::from(print_line))
    }));
    assert_eq!(map["lines"][assign_line - 1]["section"], "user-code");
    assert_eq!(map["lines"][assign_line - 1]["source_line"], 1);
    assert_eq!(
        map["lines"][assign_line - 1]["generated_text"],
        "__ush_binding_1_greeting='hello'"
    );
    assert_eq!(
        map["lines"][assign_line - 1]["source_text"],
        "let greeting = \"hello\""
    );
    assert_eq!(map["lines"][print_line - 1]["section"], "user-code");
    assert_eq!(map["lines"][print_line - 1]["source_line"], 2);
    assert_eq!(
        map["lines"][print_line - 1]["generated_text"],
        "printf '%s\\n' \"${__ush_binding_1_greeting}\""
    );
    assert_eq!(
        map["lines"][print_line - 1]["source_text"],
        "print greeting"
    );
}
