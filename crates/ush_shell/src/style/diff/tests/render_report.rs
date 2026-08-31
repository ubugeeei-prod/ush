use super::{
    super::{
        parse::parse_diff_args,
        render::{render_diff_clean, render_diff_report},
    },
    args,
};
use crate::style::strip_ansi;

#[test]
fn identical_files_render_a_same_badge() {
    let options = parse_diff_args(&args(&["one.txt", "two.txt"])).expect("parse");
    let rendered = strip_ansi(&render_diff_clean(&options));

    assert_eq!(rendered, "diff one.txt two.txt\n[same] no differences\n");
}

#[test]
fn a_report_summarizes_files_hunks_and_line_counts() {
    let options = parse_diff_args(&args(&["one.txt", "two.txt"])).expect("parse");
    let rendered = strip_ansi(&render_diff_report(
        &options,
        "--- one.txt\n+++ two.txt\n@@ -1,2 +1,2 @@\n keep\n-old\n+new\n",
    ));

    assert!(rendered.starts_with("diff one.txt two.txt\n"));
    assert!(rendered.contains("1 changed file, 1 hunk, +1, -1\n"));
    assert!(rendered.contains("one.txt -> two.txt"));
    assert!(rendered.contains("[1 hunk]"));
    assert!(rendered.contains("[+1]"));
    assert!(rendered.contains("[-1]"));
    assert!(rendered.contains("@@ -1,2 +1,2 @@"));
}

#[test]
fn plural_counts_are_used_for_multiple_files() {
    let options = parse_diff_args(&args(&["a", "b"])).expect("parse");
    let rendered = strip_ansi(&render_diff_report(
        &options,
        "--- a\n+++ b\n@@ -1 +1 @@\n+x\n--- c\n+++ d\n@@ -1 +1 @@\n+y\n+z\n",
    ));

    assert!(rendered.contains("2 changed files, 2 hunks, +3"));
}

#[test]
fn a_report_without_sections_still_says_something() {
    let options = parse_diff_args(&args(&["a", "b"])).expect("parse");
    let rendered = strip_ansi(&render_diff_report(&options, ""));

    assert!(rendered.contains("differences detected"));
}

#[test]
fn notes_are_surfaced_above_the_sections() {
    let options = parse_diff_args(&args(&["a", "b"])).expect("parse");
    let rendered = strip_ansi(&render_diff_report(
        &options,
        "Binary files a and b differ\n",
    ));

    assert!(rendered.contains("[note] Binary files a and b differ"));
}

#[test]
fn added_and_removed_lines_keep_their_original_text() {
    let options = parse_diff_args(&args(&["a", "b"])).expect("parse");
    let rendered = strip_ansi(&render_diff_report(
        &options,
        "--- a\n+++ b\n@@ -1 +1 @@\n-removed line\n+added line\n",
    ));

    assert!(rendered.contains("-removed line"));
    assert!(rendered.contains("+added line"));
}
