use serde_json::json;

use crate::helpers::ValueStream;

#[test]
fn only_the_empty_stream_reports_itself_as_empty() {
    assert!(ValueStream::Empty.is_empty());
    assert!(!ValueStream::Text(String::new()).is_empty());
    assert!(!ValueStream::Lines(Vec::new()).is_empty());
    assert!(!ValueStream::Json(json!(null)).is_empty());
}

#[test]
fn text_round_trips_untouched() {
    let stream = ValueStream::Text("a\nb".to_string());
    assert_eq!(stream.to_text().expect("text"), "a\nb");
}

#[test]
fn lines_are_joined_with_a_trailing_newline() {
    let stream = ValueStream::Lines(vec!["a".to_string(), "b".to_string()]);
    assert_eq!(stream.to_text().expect("text"), "a\nb\n");
}

#[test]
fn an_empty_line_list_renders_as_the_empty_string() {
    assert_eq!(ValueStream::Lines(Vec::new()).to_text().expect("text"), "");
}

#[test]
fn json_is_pretty_printed_and_newline_terminated() {
    let stream = ValueStream::Json(json!({"name": "ush"}));
    assert_eq!(
        stream.to_text().expect("text"),
        "{\n  \"name\": \"ush\"\n}\n"
    );
}

#[test]
fn bytes_match_the_rendered_text() {
    let stream = ValueStream::Lines(vec!["日本語".to_string()]);
    assert_eq!(stream.to_bytes().expect("bytes"), "日本語\n".as_bytes());
}

#[test]
fn text_is_split_into_lines_without_the_terminator() {
    let lines = ValueStream::Text("a\nb\n".to_string())
        .into_lines()
        .expect("lines");
    assert_eq!(lines, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn a_final_line_without_a_newline_is_kept() {
    let lines = ValueStream::Text("a\nb".to_string())
        .into_lines()
        .expect("lines");
    assert_eq!(lines, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn the_empty_stream_has_no_lines() {
    assert!(ValueStream::Empty.into_lines().expect("lines").is_empty());
    assert!(
        ValueStream::Text(String::new())
            .into_lines()
            .expect("lines")
            .is_empty()
    );
}

#[test]
fn a_json_array_becomes_one_line_per_element() {
    let lines = ValueStream::Json(json!(["a", 1, true, null]))
        .into_lines()
        .expect("lines");
    assert_eq!(lines, vec!["a", "1", "true", "null"]);
}

#[test]
fn json_strings_are_unquoted_when_split_into_lines() {
    let lines = ValueStream::Json(json!(["with \"quotes\""]))
        .into_lines()
        .expect("lines");
    assert_eq!(lines, vec!["with \"quotes\""]);
}

#[test]
fn a_non_array_json_value_becomes_a_single_line() {
    let lines = ValueStream::Json(json!({"a": 1}))
        .into_lines()
        .expect("lines");
    assert_eq!(lines, vec!["{\"a\":1}"]);
}

#[test]
fn the_default_stream_is_empty() {
    assert!(ValueStream::default().is_empty());
}
