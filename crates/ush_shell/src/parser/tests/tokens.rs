use crate::parser::{
    is_assignment, is_identifier, split_assignments, split_unquoted, strip_comment,
};

fn owned(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn segments<'a>(values: &[&'a str]) -> Vec<&'a str> {
    values.to_vec()
}

#[test]
fn comments_are_stripped_from_the_first_unquoted_hash() {
    assert_eq!(strip_comment("echo hi # trailing"), "echo hi ");
    assert_eq!(strip_comment("# whole line"), "");
    assert_eq!(strip_comment("echo hi"), "echo hi");
}

#[test]
fn a_hash_glued_to_a_word_is_not_a_comment() {
    assert_eq!(strip_comment("echo a#b"), "echo a#b");
    assert_eq!(strip_comment("git show HEAD#1"), "git show HEAD#1");
}

#[test]
fn quoted_hashes_are_not_comments() {
    assert_eq!(strip_comment("echo \"a # b\""), "echo \"a # b\"");
    assert_eq!(strip_comment("echo 'a # b'"), "echo 'a # b'");
    assert_eq!(strip_comment("echo \"keep\" # drop"), "echo \"keep\" ");
}

#[test]
fn splitting_respects_quotes() {
    assert_eq!(
        split_unquoted("ls | len", '|').expect("split"),
        segments(&["ls", "len"])
    );
    assert_eq!(
        split_unquoted("echo 'a|b'", '|').expect("split"),
        segments(&["echo 'a|b'"])
    );
    assert_eq!(
        split_unquoted("echo \"a|b\" | len", '|').expect("split"),
        segments(&["echo \"a|b\"", "len"])
    );
}

#[test]
fn splitting_keeps_empty_segments() {
    assert_eq!(
        split_unquoted("ls ||", '|').expect("split"),
        segments(&["ls", "", ""])
    );
    assert_eq!(split_unquoted("", '|').expect("split"), segments(&[""]));
}

#[test]
fn escaped_separators_are_not_split_points() {
    assert_eq!(
        split_unquoted("echo a\\|b", '|').expect("split"),
        segments(&["echo a\\|b"])
    );
}

#[test]
fn unbalanced_quotes_are_rejected() {
    let error = split_unquoted("echo 'unterminated", '|').expect_err("unterminated");
    assert!(error.to_string().contains("unterminated"));
    let error = split_unquoted("echo \"unterminated", '|').expect_err("unterminated");
    assert!(error.to_string().contains("unterminated"));
}

#[test]
fn multibyte_segments_split_on_char_boundaries() {
    assert_eq!(
        split_unquoted("echo 日本語 | len", '|').expect("split"),
        segments(&["echo 日本語", "len"])
    );
}

#[test]
fn leading_assignments_are_separated_from_the_command() {
    let (assignments, rest) = split_assignments(owned(&["A=1", "B=2", "echo", "hi"]));
    assert_eq!(
        assignments,
        vec![
            ("A".to_string(), "1".to_string()),
            ("B".to_string(), "2".to_string())
        ]
    );
    assert_eq!(rest, owned(&["echo", "hi"]));
}

#[test]
fn assignments_after_the_command_word_are_plain_arguments() {
    let (assignments, rest) = split_assignments(owned(&["echo", "A=1"]));
    assert!(assignments.is_empty());
    assert_eq!(rest, owned(&["echo", "A=1"]));
}

#[test]
fn assignment_only_lines_have_no_command() {
    let (assignments, rest) = split_assignments(owned(&["A=1"]));
    assert_eq!(assignments, vec![("A".to_string(), "1".to_string())]);
    assert!(rest.is_empty());
}

#[test]
fn assignment_values_may_be_empty_or_contain_equals() {
    let (assignments, rest) = split_assignments(owned(&["A=", "B=x=y"]));
    assert_eq!(
        assignments,
        vec![
            ("A".to_string(), String::new()),
            ("B".to_string(), "x=y".to_string())
        ]
    );
    assert!(rest.is_empty());
}

#[test]
fn assignment_detection_requires_an_identifier_name() {
    assert!(is_assignment("A=1"));
    assert!(is_assignment("_a1=1"));
    assert!(!is_assignment("1A=1"));
    assert!(!is_assignment("a-b=1"));
    assert!(!is_assignment("=1"));
    assert!(!is_assignment("plain"));
}

#[test]
fn identifier_rules_match_posix_names() {
    assert!(is_identifier("PATH"));
    assert!(is_identifier("_"));
    assert!(is_identifier("a1"));
    assert!(!is_identifier(""));
    assert!(!is_identifier("1a"));
    assert!(!is_identifier("a.b"));
    assert!(!is_identifier("日本語"));
}
