use std::fs;

use tempfile::tempdir;

use super::{count_display_lines, render_cat};
use crate::{helpers::ValueStream, style::strip_ansi};

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn rendered(cwd: &std::path::Path, args: &[String], input: ValueStream) -> String {
    strip_ansi(
        &render_cat(cwd, args, &input)
            .expect("render")
            .expect("stylish")
            .to_text()
            .expect("text"),
    )
}

#[test]
fn display_lines_count_a_trailing_newline_as_a_terminator() {
    assert_eq!(count_display_lines(""), 0);
    assert_eq!(count_display_lines("a"), 1);
    assert_eq!(count_display_lines("a\n"), 1);
    assert_eq!(count_display_lines("a\nb"), 2);
    assert_eq!(count_display_lines("a\nb\n"), 2);
    assert_eq!(count_display_lines("\n"), 1);
}

#[test]
fn piped_input_is_numbered_without_a_header() {
    let dir = tempdir().expect("tempdir");
    let text = rendered(
        dir.path(),
        &args(&[]),
        ValueStream::Text("one\ntwo\n".to_string()),
    );

    assert_eq!(text, "1 | one\n2 | two\n");
}

#[test]
fn empty_piped_input_renders_nothing() {
    let dir = tempdir().expect("tempdir");
    assert_eq!(rendered(dir.path(), &args(&[]), ValueStream::Empty), "");
}

#[test]
fn a_file_gets_a_header_with_its_line_count_and_size() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("a.txt"), "one\ntwo\n").expect("write");
    let text = rendered(dir.path(), &args(&["a.txt"]), ValueStream::Empty);

    assert!(text.contains("2 lines, 8 B\n"));
    assert!(text.ends_with("1 | one\n2 | two\n"));
}

#[test]
fn a_single_line_file_uses_singular_wording() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("a.txt"), "only\n").expect("write");
    let text = rendered(dir.path(), &args(&["a.txt"]), ValueStream::Empty);

    assert!(text.contains("1 line, 5 B\n"));
}

#[test]
fn an_empty_file_is_labelled_rather_than_left_blank() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("a.txt"), "").expect("write");
    let text = rendered(dir.path(), &args(&["a.txt"]), ValueStream::Empty);

    assert!(text.contains("0 lines, 0 B\n"));
    assert!(text.ends_with("(empty)\n"));
}

#[test]
fn line_numbers_are_right_aligned_to_the_widest_number() {
    let dir = tempdir().expect("tempdir");
    let body = (1..=10).map(|n| format!("line {n}\n")).collect::<String>();
    fs::write(dir.path().join("a.txt"), body).expect("write");
    let text = rendered(dir.path(), &args(&["a.txt"]), ValueStream::Empty);

    assert!(text.contains(" 1 | line 1\n"));
    assert!(text.contains("10 | line 10\n"));
}

#[test]
fn a_file_without_a_trailing_newline_still_ends_with_one() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("a.txt"), "no newline").expect("write");
    let text = rendered(dir.path(), &args(&["a.txt"]), ValueStream::Empty);

    assert!(text.ends_with("1 | no newline\n"));
}

#[test]
fn several_files_are_separated_by_a_blank_line() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("a.txt"), "a\n").expect("write");
    fs::write(dir.path().join("b.txt"), "b\n").expect("write");
    let text = rendered(dir.path(), &args(&["a.txt", "b.txt"]), ValueStream::Empty);

    assert!(text.contains("1 | a\n\ncat "));
    assert!(text.matches("cat ").count() == 2);
}

#[test]
fn unsupported_flags_leave_the_stylish_renderer_alone() {
    let dir = tempdir().expect("tempdir");
    assert!(
        render_cat(dir.path(), &args(&["-b"]), &ValueStream::Empty)
            .expect("render")
            .is_none()
    );
}

#[test]
fn a_missing_file_names_the_path_it_could_not_read() {
    let dir = tempdir().expect("tempdir");
    let error =
        render_cat(dir.path(), &args(&["missing.txt"]), &ValueStream::Empty).expect_err("missing");
    let message = format!("{error:#}");

    assert!(message.contains("failed to read"), "{message}");
    assert!(message.contains("missing.txt"), "{message}");
}

#[test]
fn multibyte_text_is_measured_in_bytes_and_kept_intact() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("a.txt"), "日本語\n").expect("write");
    let text = rendered(dir.path(), &args(&["a.txt"]), ValueStream::Empty);

    assert!(text.contains("1 line, 10 B\n"));
    assert!(text.ends_with("1 | 日本語\n"));
}
