use std::{fs, os::unix::fs::PermissionsExt, path::Path};

use tempfile::{TempDir, tempdir};
use ush_config::UshConfig;

use super::evaluate;
use crate::{Shell, ShellOptions};

fn shell() -> Shell {
    let config = UshConfig::default();
    let options = ShellOptions::resolve(false, false, false, false, &config);
    Shell::new(config, options).expect("shell")
}

fn eval(args: &[&str]) -> bool {
    let owned = args
        .iter()
        .map(|arg| (*arg).to_string())
        .collect::<Vec<_>>();
    evaluate(&shell(), &owned).expect("eval")
}

fn eval_error(args: &[&str]) -> String {
    let owned = args
        .iter()
        .map(|arg| (*arg).to_string())
        .collect::<Vec<_>>();
    evaluate(&shell(), &owned)
        .expect_err("expected an error")
        .to_string()
}

fn eval_path(op: &str, path: &Path) -> bool {
    eval(&[op, &path.display().to_string()])
}

fn fixture() -> TempDir {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("file.txt"), "hello").expect("write");
    fs::write(dir.path().join("empty.txt"), "").expect("write");
    fs::create_dir(dir.path().join("sub")).expect("mkdir");
    let script = dir.path().join("run.sh");
    fs::write(&script, "#!/bin/sh\n").expect("write");
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).expect("chmod");
    std::os::unix::fs::symlink(dir.path().join("file.txt"), dir.path().join("link"))
        .expect("symlink");
    dir
}

#[test]
fn no_arguments_is_false() {
    assert!(!eval(&[]));
}

#[test]
fn a_lone_word_is_true_when_it_is_not_empty() {
    assert!(eval(&["ok"]));
    assert!(!eval(&[""]));
}

#[test]
fn string_length_operators_work_both_ways() {
    assert!(eval(&["-n", "ok"]));
    assert!(!eval(&["-n", ""]));
    assert!(eval(&["-z", ""]));
    assert!(!eval(&["-z", "ok"]));
}

#[test]
fn string_comparisons_use_exact_equality() {
    assert!(eval(&["a", "=", "a"]));
    assert!(!eval(&["a", "=", "b"]));
    assert!(eval(&["a", "!=", "b"]));
    assert!(!eval(&["a", "!=", "a"]));
    assert!(eval(&["日本語", "=", "日本語"]));
}

#[test]
fn every_integer_comparison_is_supported() {
    assert!(eval(&["3", "-eq", "3"]));
    assert!(eval(&["3", "-ne", "4"]));
    assert!(eval(&["3", "-gt", "2"]));
    assert!(eval(&["3", "-ge", "3"]));
    assert!(eval(&["2", "-lt", "3"]));
    assert!(eval(&["3", "-le", "3"]));
    assert!(!eval(&["2", "-gt", "3"]));
}

#[test]
fn negative_numbers_compare_correctly() {
    assert!(eval(&["-1", "-lt", "0"]));
    assert!(eval(&["-2", "-lt", "-1"]));
}

#[test]
fn non_numeric_operands_are_rejected() {
    let message = eval_error(&["x", "-eq", "1"]);
    assert!(message.contains("invalid integer"), "{message}");
    assert!(message.contains('x'), "{message}");
}

#[test]
fn bang_negates_the_rest_of_the_expression() {
    assert!(eval(&["!", "-z", "ok"]));
    assert!(!eval(&["!", "-n", "ok"]));
    assert!(eval(&["!", "!", "-n", "ok"]));
}

#[test]
fn a_bare_bang_is_an_error() {
    assert!(eval_error(&["!"]).contains("requires an expression"));
}

#[test]
fn existence_operators_see_files_directories_and_links() {
    let dir = fixture();
    assert!(eval_path("-e", &dir.path().join("file.txt")));
    assert!(eval_path("-e", &dir.path().join("sub")));
    assert!(!eval_path("-e", &dir.path().join("missing")));
}

#[test]
fn file_and_directory_operators_are_distinct() {
    let dir = fixture();
    assert!(eval_path("-f", &dir.path().join("file.txt")));
    assert!(!eval_path("-f", &dir.path().join("sub")));
    assert!(eval_path("-d", &dir.path().join("sub")));
    assert!(!eval_path("-d", &dir.path().join("file.txt")));
}

#[test]
fn symlink_operators_do_not_follow_the_link() {
    let dir = fixture();
    assert!(eval_path("-h", &dir.path().join("link")));
    assert!(eval_path("-L", &dir.path().join("link")));
    assert!(!eval_path("-h", &dir.path().join("file.txt")));
}

#[test]
fn the_size_operator_distinguishes_empty_files() {
    let dir = fixture();
    assert!(eval_path("-s", &dir.path().join("file.txt")));
    assert!(!eval_path("-s", &dir.path().join("empty.txt")));
    assert!(!eval_path("-s", &dir.path().join("missing")));
}

#[test]
fn the_readable_operator_reports_openable_files() {
    let dir = fixture();
    assert!(eval_path("-r", &dir.path().join("file.txt")));
    assert!(!eval_path("-r", &dir.path().join("missing")));
}

#[test]
fn the_executable_operator_reads_the_permission_bits() {
    let dir = fixture();
    assert!(eval_path("-x", &dir.path().join("run.sh")));
    assert!(!eval_path("-x", &dir.path().join("file.txt")));
    assert!(!eval_path("-x", &dir.path().join("missing")));
}

#[test]
fn unknown_operators_are_rejected() {
    assert!(eval_error(&["-q", "value"]).contains("unsupported test operator"));
    assert!(eval_error(&["a", "-like", "b"]).contains("unsupported test operator"));
}

#[test]
fn expressions_longer_than_three_words_are_rejected() {
    assert!(eval_error(&["a", "=", "b", "c"]).contains("unsupported test expression"));
}
