use super::{
    super::{
        parse::parse_grep_args,
        render::{parse_grep_output, render_grep_no_matches, render_grep_report},
    },
    args,
};
use crate::style::strip_ansi;

#[test]
fn grep_output_lines_are_split_into_source_line_and_text() {
    let report = parse_grep_output("a.txt:12:let value = 1;\n");

    assert_eq!(report.groups.len(), 1);
    assert_eq!(report.groups[0].source, "a.txt");
    assert_eq!(report.groups[0].rows[0].line_number, 12);
    assert_eq!(report.groups[0].rows[0].text, "let value = 1;");
}

#[test]
fn consecutive_rows_from_one_file_share_a_group() {
    let report = parse_grep_output("a.txt:1:one\na.txt:2:two\nb.txt:3:three\n");

    assert_eq!(report.groups.len(), 2);
    assert_eq!(report.groups[0].rows.len(), 2);
    assert_eq!(report.groups[1].source, "b.txt");
}

#[test]
fn revisiting_a_file_later_starts_a_new_group() {
    let report = parse_grep_output("a.txt:1:one\nb.txt:2:two\na.txt:3:three\n");

    assert_eq!(report.groups.len(), 3);
    assert_eq!(report.groups[2].source, "a.txt");
}

#[test]
fn standard_input_is_relabelled() {
    let report = parse_grep_output("(standard input):1:piped\n");
    assert_eq!(report.groups[0].source, "stdin");
}

#[test]
fn text_containing_colons_survives_intact() {
    let report = parse_grep_output("a.txt:7:http://example.com:8080/path\n");
    assert_eq!(
        report.groups[0].rows[0].text,
        "http://example.com:8080/path"
    );
}

#[test]
fn unparseable_lines_become_notes() {
    let report = parse_grep_output("grep: nope: No such file or directory\nplain\n");

    assert!(report.groups.is_empty());
    assert_eq!(report.notes.len(), 2);
}

#[test]
fn blank_lines_are_dropped_entirely() {
    let report = parse_grep_output("\n\n");
    assert!(report.groups.is_empty());
    assert!(report.notes.is_empty());
}

#[test]
fn a_no_match_run_renders_the_pattern_and_a_badge() {
    let options = parse_grep_args(&args(&["needle", "a.txt"])).expect("parse");
    let rendered = strip_ansi(&render_grep_no_matches(&options));

    assert_eq!(rendered, "grep needle\n[no matches] pattern not found\n");
}

#[test]
fn a_report_counts_matches_and_sources() {
    let options = parse_grep_args(&args(&["needle"])).expect("parse");
    let report = parse_grep_output("a.txt:1:one\nb.txt:2:two\n");
    let rendered = strip_ansi(&render_grep_report(&options, &report));

    assert!(rendered.starts_with("grep needle\n"));
    assert!(rendered.contains("2 matches, 2 sources\n"));
    assert!(rendered.contains("a.txt [1 match]"));
    assert!(rendered.contains("[line 1] one"));
    assert!(rendered.contains("[line 2] two"));
}

#[test]
fn a_single_match_uses_singular_wording() {
    let options = parse_grep_args(&args(&["needle"])).expect("parse");
    let report = parse_grep_output("a.txt:1:one\n");
    let rendered = strip_ansi(&render_grep_report(&options, &report));

    assert!(rendered.contains("1 match, 1 source\n"));
}

#[test]
fn several_patterns_are_summarized_rather_than_echoed() {
    let options = parse_grep_args(&args(&["-e", "one", "-e", "two"])).expect("parse");
    let rendered = strip_ansi(&render_grep_no_matches(&options));

    assert!(rendered.starts_with("grep 2 patterns\n"));
}

#[test]
fn a_lone_pattern_file_is_named_in_the_header() {
    let options = parse_grep_args(&args(&["-f", "patterns.txt"])).expect("parse");
    let rendered = strip_ansi(&render_grep_no_matches(&options));

    assert!(rendered.starts_with("grep patterns from patterns.txt\n"));
}

#[test]
fn notes_are_rendered_before_the_groups() {
    let options = parse_grep_args(&args(&["needle"])).expect("parse");
    let report = parse_grep_output("grep: nope: No such file\na.txt:1:one\n");
    let rendered = strip_ansi(&render_grep_report(&options, &report));

    let note = rendered.find("[note]").expect("note");
    let group = rendered.find("a.txt").expect("group");
    assert!(note < group);
}

#[test]
fn an_empty_report_still_renders_a_header() {
    let options = parse_grep_args(&args(&["needle"])).expect("parse");
    let report = parse_grep_output("");
    let rendered = strip_ansi(&render_grep_report(&options, &report));

    assert_eq!(rendered, "grep needle\n0 matches\n");
}
