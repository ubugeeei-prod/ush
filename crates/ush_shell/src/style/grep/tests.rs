mod render_report;

use super::parse::{build_grep_command_args, parse_grep_args};

pub(super) fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

#[test]
fn the_first_positional_argument_becomes_the_pattern() {
    let options = parse_grep_args(&args(&["needle", "a.txt", "b.txt"])).expect("parse");
    assert_eq!(options.patterns, args(&["needle"]));
    assert_eq!(options.targets, args(&["a.txt", "b.txt"]));
}

#[test]
fn a_missing_pattern_bails_out_to_the_plain_renderer() {
    assert!(parse_grep_args(&args(&[])).is_none());
    assert!(parse_grep_args(&args(&["-i"])).is_none());
}

#[test]
fn a_dangling_value_flag_is_rejected() {
    assert!(parse_grep_args(&args(&["-e"])).is_none());
    assert!(parse_grep_args(&args(&["needle", "-m"])).is_none());
}

#[test]
fn long_flags_are_recognized() {
    let options = parse_grep_args(&args(&[
        "--ignore-case",
        "--invert-match",
        "--word-regexp",
        "--line-regexp",
        "--fixed-strings",
        "--extended-regexp",
        "--recursive",
        "--no-messages",
        "--text",
        "needle",
    ]))
    .expect("parse");

    assert!(options.ignore_case);
    assert!(options.invert_match);
    assert!(options.word_regexp);
    assert!(options.line_regexp);
    assert!(options.fixed_strings);
    assert!(options.extended_regexp);
    assert!(options.recursive);
    assert!(options.no_messages);
    assert!(options.text);
}

#[test]
fn output_shape_flags_are_accepted_and_ignored() {
    let options = parse_grep_args(&args(&["-n", "-H", "-h", "--color", "needle"])).expect("parse");
    assert_eq!(options.patterns, args(&["needle"]));

    let options = parse_grep_args(&args(&["--color=always", "needle"])).expect("parse");
    assert_eq!(options.patterns, args(&["needle"]));
}

#[test]
fn short_flags_can_be_bundled() {
    let options = parse_grep_args(&args(&["-inw", "needle"])).expect("parse");
    assert!(options.ignore_case);
    assert!(options.word_regexp);
    assert!(!options.invert_match);
}

#[test]
fn a_bundled_value_flag_consumes_the_rest_of_the_bundle() {
    let options = parse_grep_args(&args(&["-ieneedle"])).expect("parse");
    assert!(options.ignore_case);
    assert_eq!(options.patterns, args(&["needle"]));

    let options = parse_grep_args(&args(&["-m3", "needle"])).expect("parse");
    assert_eq!(options.max_count, Some(3));
}

#[test]
fn patterns_can_arrive_through_every_supported_spelling() {
    assert_eq!(
        parse_grep_args(&args(&["-e", "needle"]))
            .expect("parse")
            .patterns,
        args(&["needle"])
    );
    assert_eq!(
        parse_grep_args(&args(&["--regexp=needle"]))
            .expect("parse")
            .patterns,
        args(&["needle"])
    );
    assert_eq!(
        parse_grep_args(&args(&["--file=patterns.txt"]))
            .expect("parse")
            .pattern_files,
        args(&["patterns.txt"])
    );
    assert_eq!(
        parse_grep_args(&args(&["-f", "patterns.txt"]))
            .expect("parse")
            .pattern_files,
        args(&["patterns.txt"])
    );
}

#[test]
fn an_explicit_pattern_makes_every_positional_a_target() {
    let options = parse_grep_args(&args(&["-e", "needle", "a.txt"])).expect("parse");
    assert_eq!(options.patterns, args(&["needle"]));
    assert_eq!(options.targets, args(&["a.txt"]));
}

#[test]
fn max_count_accepts_both_spellings_and_rejects_garbage() {
    assert_eq!(
        parse_grep_args(&args(&["-m", "5", "needle"]))
            .expect("parse")
            .max_count,
        Some(5)
    );
    assert_eq!(
        parse_grep_args(&args(&["--max-count=5", "needle"]))
            .expect("parse")
            .max_count,
        Some(5)
    );
    assert!(parse_grep_args(&args(&["-m", "many", "needle"])).is_none());
}

#[test]
fn binary_file_handling_only_accepts_text_mode() {
    assert!(
        parse_grep_args(&args(&["--binary-files=text", "needle"]))
            .expect("parse")
            .text
    );
    assert!(parse_grep_args(&args(&["--binary-files=without-match", "needle"])).is_none());
}

#[test]
fn unknown_flags_bail_out_to_the_plain_renderer() {
    assert!(parse_grep_args(&args(&["--perl-regexp", "needle"])).is_none());
    assert!(parse_grep_args(&args(&["-Z", "needle"])).is_none());
}

#[test]
fn a_double_dash_forces_the_rest_to_be_positional() {
    let options = parse_grep_args(&args(&["--", "-i", "-v"])).expect("parse");
    assert_eq!(options.patterns, args(&["-i"]));
    assert_eq!(options.targets, args(&["-v"]));
}

#[test]
fn command_args_always_ask_grep_for_line_numbers_and_filenames() {
    let options = parse_grep_args(&args(&["needle", "a.txt"])).expect("parse");
    assert_eq!(
        build_grep_command_args(&options),
        args(&["-nH", "-e", "needle", "--", "a.txt"])
    );
}

#[test]
fn command_args_forward_every_enabled_flag() {
    let options = parse_grep_args(&args(&["-ivwxFEsa", "-r", "-m", "2", "needle"])).expect("parse");
    let built = build_grep_command_args(&options);

    for flag in ["-i", "-v", "-w", "-x", "-F", "-E", "-R", "-s", "-a"] {
        assert!(built.contains(&flag.to_string()), "missing {flag}");
    }
    let max = built.iter().position(|arg| arg == "-m").expect("max count");
    assert_eq!(built[max + 1], "2");
}

#[test]
fn command_args_forward_pattern_files_after_patterns() {
    let options =
        parse_grep_args(&args(&["-e", "one", "-f", "patterns.txt", "a.txt"])).expect("parse");
    let built = build_grep_command_args(&options);

    let pattern = built.iter().position(|arg| arg == "-e").expect("pattern");
    let file = built.iter().position(|arg| arg == "-f").expect("file");
    assert!(pattern < file);
    assert_eq!(built.last().expect("target"), "a.txt");
}

#[test]
fn has_pattern_source_reflects_both_pattern_kinds() {
    assert!(
        parse_grep_args(&args(&["needle"]))
            .expect("parse")
            .has_pattern_source()
    );
    assert!(
        parse_grep_args(&args(&["-f", "patterns.txt"]))
            .expect("parse")
            .has_pattern_source()
    );
}
