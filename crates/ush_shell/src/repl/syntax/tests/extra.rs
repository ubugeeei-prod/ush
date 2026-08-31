use super::super::{
    has_trailing_escape, has_unclosed_quotes, is_assignment, is_keyword, keywords, needs_refresh,
    previous_token, tokenize, word_start,
};

fn tokens(line: &str) -> Vec<String> {
    tokenize(line)
}

#[test]
fn every_posix_keyword_is_recognized() {
    for keyword in keywords() {
        assert!(is_keyword(keyword), "{keyword}");
    }
    assert!(!is_keyword("echo"));
    assert!(!is_keyword(""));
}

#[test]
fn assignments_need_an_identifier_on_the_left() {
    assert!(is_assignment("FOO=1"));
    assert!(is_assignment("_a=1"));
    assert!(is_assignment("A="));
    assert!(!is_assignment("1A=1"));
    assert!(!is_assignment("a-b=1"));
    assert!(!is_assignment("plain"));
}

#[test]
fn quoted_words_stay_whole_with_their_quotes() {
    assert_eq!(tokens("echo 'a b'"), vec!["echo", "'a b'"]);
    assert_eq!(tokens("echo \"a b\""), vec!["echo", "\"a b\""]);
    assert_eq!(tokens("echo \"a \\\" b\""), vec!["echo", "\"a \\\" b\""]);
}

#[test]
fn escaped_spaces_do_not_split_a_word() {
    assert_eq!(tokens("cat a\\ b"), vec!["cat", "a\\ b"]);
}

#[test]
fn operators_become_their_own_tokens() {
    assert_eq!(tokens("a|b"), vec!["a", "|", "b"]);
    assert_eq!(tokens("a||b"), vec!["a", "||", "b"]);
    assert_eq!(tokens("a&&b"), vec!["a", "&&", "b"]);
    assert_eq!(tokens("a>b"), vec!["a", ">", "b"]);
    assert_eq!(tokens("a>>b"), vec!["a", ">>", "b"]);
    assert_eq!(tokens("a<b"), vec!["a", "<", "b"]);
    assert_eq!(tokens("a;b"), vec!["a", ";", "b"]);
    assert_eq!(tokens("(a)"), vec!["(", "a", ")"]);
    assert_eq!(tokens("{a}"), vec!["{", "a", "}"]);
}

#[test]
fn a_comment_ends_the_token_stream() {
    assert_eq!(tokens("echo hi # note"), vec!["echo", "hi"]);
    assert_eq!(tokens("# note"), Vec::<String>::new());
}

#[test]
fn a_hash_inside_a_word_is_kept() {
    assert_eq!(tokens("git show HEAD#1"), vec!["git", "show", "HEAD#1"]);
}

#[test]
fn an_empty_line_has_no_tokens() {
    assert!(tokens("").is_empty());
    assert!(tokens("   ").is_empty());
}

#[test]
fn the_previous_token_is_the_last_complete_word() {
    assert_eq!(
        previous_token("git checkout ", 13),
        Some("checkout".to_string())
    );
    assert_eq!(previous_token("git ch", 4), Some("git".to_string()));
    assert_eq!(previous_token("", 0), None);
}

#[test]
fn word_start_stops_at_separators() {
    assert_eq!(word_start("echo hi", 7), 5);
    assert_eq!(word_start("echo", 4), 0);
    assert_eq!(word_start("a|b", 3), 2);
    assert_eq!(word_start("a;b", 3), 2);
    assert_eq!(word_start("a(b", 3), 2);
}

#[test]
fn word_start_handles_multibyte_prefixes() {
    let line = "echo 日本語";
    assert_eq!(word_start(line, line.len()), 5);
}

#[test]
fn unclosed_quotes_are_detected_in_both_styles() {
    assert!(has_unclosed_quotes("echo 'open"));
    assert!(has_unclosed_quotes("echo \"open"));
    assert!(!has_unclosed_quotes("echo 'closed'"));
    assert!(!has_unclosed_quotes("echo \"closed\""));
    assert!(!has_unclosed_quotes(""));
}

#[test]
fn a_quote_inside_the_other_quote_style_is_not_an_opener() {
    assert!(!has_unclosed_quotes("echo \"it's fine\""));
    assert!(!has_unclosed_quotes("echo 'a \" b'"));
}

#[test]
fn an_escaped_double_quote_does_not_close_the_string() {
    assert!(has_unclosed_quotes("echo \"a \\\""));
}

#[test]
fn trailing_escapes_are_counted_in_pairs() {
    assert!(has_trailing_escape("echo hi \\"));
    assert!(!has_trailing_escape("echo hi \\\\"));
    assert!(has_trailing_escape("echo hi \\\\\\"));
    assert!(!has_trailing_escape("echo hi"));
    assert!(!has_trailing_escape(""));
}

#[test]
fn a_refresh_is_needed_at_the_end_of_a_line_or_around_special_characters() {
    assert!(needs_refresh("echo", 4));
    assert!(needs_refresh("echo $HOME x", 5));
    assert!(needs_refresh("a | b", 1));
    assert!(!needs_refresh("echo hi", 2));
}
