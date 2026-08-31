mod extra;

use super::{command_position, env_query, has_trailing_escape, tokenize, word_start};

#[test]
fn tokenizes_keywords_and_operators() {
    assert_eq!(
        tokenize("FOO=1 echo hi | grep h && printf ok"),
        [
            "FOO=1", "echo", "hi", "|", "grep", "h", "&&", "printf", "ok"
        ]
    );
}

#[test]
fn tracks_command_position_after_assignments() {
    assert!(command_position("FOO=1 ec", 6));
    assert!(command_position("echo hi | gr", 10));
    assert!(!command_position("echo fi", 5));
}

#[test]
fn finds_word_and_env_prefixes() {
    assert_eq!(word_start("echo $PA", 8), 5);
    assert_eq!(env_query("${PAT"), Some((2, "PAT".to_string(), true)));
    assert_eq!(env_query("$PAT"), Some((1, "PAT".to_string(), false)));
    assert!(has_trailing_escape("echo hi \\"));
}
