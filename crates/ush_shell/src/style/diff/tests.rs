mod render_report;

use super::{
    model::DiffLineKind,
    parse::{build_diff_command_args, parse_diff_args, parse_unified_diff},
};

pub(super) fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

#[test]
fn two_positional_targets_are_required() {
    assert!(parse_diff_args(&args(&["a", "b"])).is_some());
    assert!(parse_diff_args(&args(&["a"])).is_none());
    assert!(parse_diff_args(&args(&["a", "b", "c"])).is_none());
    assert!(parse_diff_args(&args(&[])).is_none());
}

#[test]
fn long_flags_are_recognized() {
    let options = parse_diff_args(&args(&[
        "--recursive",
        "--new-file",
        "--text",
        "--ignore-all-space",
        "--ignore-space-change",
        "--ignore-blank-lines",
        "--ignore-case",
        "a",
        "b",
    ]))
    .expect("parse");

    assert!(options.recursive);
    assert!(options.new_file);
    assert!(options.text);
    assert!(options.ignore_all_space);
    assert!(options.ignore_space_change);
    assert!(options.ignore_blank_lines);
    assert!(options.ignore_case);
    assert_eq!(options.targets, args(&["a", "b"]));
}

#[test]
fn short_flags_can_be_bundled() {
    let options = parse_diff_args(&args(&["-rNaw", "a", "b"])).expect("parse");
    assert!(options.recursive);
    assert!(options.new_file);
    assert!(options.text);
    assert!(options.ignore_all_space);
}

#[test]
fn context_defaults_to_three_lines() {
    let options = parse_diff_args(&args(&["a", "b"])).expect("parse");
    assert_eq!(options.context, 3);
}

#[test]
fn context_can_be_set_three_different_ways() {
    assert_eq!(
        parse_diff_args(&args(&["-U", "7", "a", "b"]))
            .expect("parse")
            .context,
        7
    );
    assert_eq!(
        parse_diff_args(&args(&["-U7", "a", "b"]))
            .expect("parse")
            .context,
        7
    );
    assert_eq!(
        parse_diff_args(&args(&["--unified=7", "a", "b"]))
            .expect("parse")
            .context,
        7
    );
}

#[test]
fn a_dangling_context_flag_is_rejected() {
    assert!(parse_diff_args(&args(&["a", "b", "-U"])).is_none());
    assert!(parse_diff_args(&args(&["-U", "nope", "a", "b"])).is_none());
    assert!(parse_diff_args(&args(&["--unified=nope", "a", "b"])).is_none());
}

#[test]
fn unknown_flags_bail_out_to_the_plain_renderer() {
    assert!(parse_diff_args(&args(&["--brief", "a", "b"])).is_none());
    assert!(parse_diff_args(&args(&["-Z", "a", "b"])).is_none());
}

#[test]
fn a_double_dash_forces_the_rest_to_be_paths() {
    let options = parse_diff_args(&args(&["--", "-a", "-b"])).expect("parse");
    assert_eq!(options.targets, args(&["-a", "-b"]));
}

#[test]
fn command_args_always_pin_the_context_and_close_the_flags() {
    let options = parse_diff_args(&args(&["-r", "a", "b"])).expect("parse");
    let built = build_diff_command_args(&options);
    assert_eq!(built[0], "--unified=3");
    assert!(built.contains(&"--recursive".to_string()));
    let separator = built.iter().position(|arg| arg == "--").expect("separator");
    assert_eq!(&built[separator + 1..], args(&["a", "b"]).as_slice());
}

#[test]
fn command_args_omit_flags_that_were_not_requested() {
    let options = parse_diff_args(&args(&["a", "b"])).expect("parse");
    let built = build_diff_command_args(&options);
    assert_eq!(built, args(&["--unified=3", "--", "a", "b"]));
}

#[test]
fn a_unified_diff_is_split_into_sections_and_hunks() {
    let report = parse_unified_diff(
        "--- a/one.txt\t2024-01-01\n+++ b/one.txt\t2024-01-02\n@@ -1,2 +1,2 @@\n keep\n-old\n+new\n",
    );

    assert_eq!(report.sections.len(), 1);
    let section = &report.sections[0];
    assert_eq!(section.old_label, "a/one.txt");
    assert_eq!(section.new_label, "b/one.txt");
    assert_eq!(section.hunks.len(), 1);
    assert_eq!(section.additions, 1);
    assert_eq!(section.deletions, 1);
    assert_eq!(section.hunks[0].header, "@@ -1,2 +1,2 @@");
    assert_eq!(section.hunks[0].lines.len(), 3);
    assert_eq!(section.hunks[0].lines[0].kind, DiffLineKind::Context);
    assert_eq!(section.hunks[0].lines[1].kind, DiffLineKind::Removed);
    assert_eq!(section.hunks[0].lines[2].kind, DiffLineKind::Added);
}

#[test]
fn several_files_become_several_sections() {
    let report = parse_unified_diff(
        "--- a\n+++ b\n@@ -1 +1 @@\n-x\n+y\n--- c\n+++ d\n@@ -1 +1 @@\n-p\n+q\n",
    );

    assert_eq!(report.sections.len(), 2);
    assert_eq!(report.sections[0].new_label, "b");
    assert_eq!(report.sections[1].new_label, "d");
}

#[test]
fn a_missing_old_header_falls_back_to_a_placeholder() {
    let report = parse_unified_diff("+++ b\n@@ -1 +1 @@\n+y\n");
    assert_eq!(report.sections[0].old_label, "?");
}

#[test]
fn the_no_newline_marker_is_kept_as_a_note_line() {
    let report =
        parse_unified_diff("--- a\n+++ b\n@@ -1 +1 @@\n-x\n\\ No newline at end of file\n");
    let lines = &report.sections[0].hunks[0].lines;
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[1].kind, DiffLineKind::Note);
}

#[test]
fn diff_command_echo_lines_are_dropped() {
    let report = parse_unified_diff("diff -u a b\n--- a\n+++ b\n@@ -1 +1 @@\n+y\n");
    assert!(report.notes.is_empty());
    assert_eq!(report.sections.len(), 1);
}

#[test]
fn text_outside_any_section_becomes_a_report_note() {
    let report = parse_unified_diff("Binary files a and b differ\n");
    assert_eq!(report.notes, args(&["Binary files a and b differ"]));
    assert!(report.sections.is_empty());
}

#[test]
fn text_inside_a_section_becomes_a_section_note() {
    let report = parse_unified_diff("--- a\n+++ b\nOnly in a: extra\n");
    assert_eq!(report.sections[0].notes, args(&["Only in a: extra"]));
}

#[test]
fn an_empty_diff_produces_an_empty_report() {
    let report = parse_unified_diff("");
    assert!(report.sections.is_empty());
    assert!(report.notes.is_empty());
}

#[test]
fn additions_are_summed_across_every_hunk_of_a_section() {
    let report = parse_unified_diff("--- a\n+++ b\n@@ -1 +1 @@\n+one\n@@ -9 +9 @@\n+two\n-three\n");
    assert_eq!(report.sections[0].hunks.len(), 2);
    assert_eq!(report.sections[0].additions, 2);
    assert_eq!(report.sections[0].deletions, 1);
}
