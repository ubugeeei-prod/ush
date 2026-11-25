use std::{fs, process::Command};
use ush_compiler::UshCompiler;
#[test]
fn source_links_escape_paths_and_checked_in_artifacts_remain_relocatable() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("日本語 #.ush");
    let artifact = dir.path().join("demo.sh");
    fs::write(&source, "print \"hi\"\n").unwrap();
    let absolute = UshCompiler.compile_file(&source).unwrap();
    assert!(absolute.contains("# DO NOT EDIT MANUALLY."));
    assert!(absolute.contains("%E6%97%A5%E6%9C%AC%E8%AA%9E%20%23.ush"));
    let portable = UshCompiler
        .compile_checked_in_file(&source, &artifact)
        .unwrap();
    assert!(!portable.contains(dir.path().to_str().unwrap()));
    assert!(portable.contains("# Source: %E6%97%A5"));
}
#[test]
fn failures_explain_status_and_preserve_source_and_native_stderr() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("failure.ush");
    let artifact = dir.path().join("failure.sh");
    fs::write(
        &source,
        "print \"start\"\nshell \"missing_feedback_command\"\n",
    )
    .unwrap();
    fs::write(
        &artifact,
        UshCompiler
            .compile_checked_in_file(&source, &artifact)
            .unwrap(),
    )
    .unwrap();
    let out = Command::new("sh").arg(&artifact).output().unwrap();
    assert_eq!(out.status.code(), Some(127));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("failure.ush:2: command not found; check the command name and PATH"),
        "{stderr}"
    );
    assert!(stderr.contains("missing_feedback_command"));
    assert!(!stderr.contains("__ush_"));
}
#[test]
fn multiline_foreign_code_maps_to_its_actual_source_line() {
    let compiled = UshCompiler
        .compile_source_with_sourcemap("shell \"\"\"\nprintf first\nprintf second\n\"\"\"\n")
        .unwrap();
    let line = compiled
        .sourcemap
        .lines
        .iter()
        .find(|l| l.generated_text == "printf second")
        .unwrap();
    assert_eq!(line.source_line, Some(3));
}

#[test]
fn whitespace_after_multiline_opener_preserves_source_line() {
    let compiled = UshCompiler
        .compile_source_with_sourcemap("shell \"\"\" \nprintf ok\n\"\"\"\n")
        .unwrap();
    let line = compiled
        .sourcemap
        .lines
        .iter()
        .find(|l| l.generated_text == "printf ok")
        .unwrap();
    assert_eq!(line.source_line, Some(2));
}

#[test]
fn effect_defaults_register_the_builtins_they_use() {
    let source = r#"effect log(#[default(std::env::get("HOME"))] message: String) -> ()
try {
  do log()
} with log(message) {
  print message
}
"#;
    UshCompiler.compile_source(source).unwrap();
}
