use std::collections::BTreeMap;

use crate::parser::{ParsedLine, parse_line, split_background_job};

#[test]
fn a_trailing_ampersand_marks_a_background_job() {
    assert_eq!(split_background_job("sleep 1 &"), Some("sleep 1"));
    assert_eq!(split_background_job("sleep 1&"), Some("sleep 1"));
}

#[test]
fn an_ampersand_in_the_middle_is_not_a_background_job() {
    assert_eq!(split_background_job("a & b"), None);
    assert_eq!(split_background_job("true && false"), None);
}

#[test]
fn a_bare_ampersand_is_not_a_background_job() {
    assert_eq!(split_background_job("&"), None);
    assert_eq!(split_background_job("  &  "), None);
}

#[test]
fn quoted_ampersands_are_not_background_markers() {
    assert_eq!(split_background_job("echo '&'"), None);
    assert_eq!(split_background_job("echo \"&\""), None);
}

#[test]
fn background_detection_runs_before_the_posix_fallback() {
    let parsed = parse_line("sleep 1 &", &BTreeMap::new()).expect("parse");
    assert!(matches!(parsed, ParsedLine::Background(source) if source == "sleep 1"));
}

#[test]
fn a_comment_after_the_ampersand_still_backgrounds() {
    let parsed = parse_line("sleep 1 & # later", &BTreeMap::new()).expect("parse");
    assert!(matches!(parsed, ParsedLine::Background(source) if source == "sleep 1"));
}
