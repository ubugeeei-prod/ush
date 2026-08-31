use std::collections::HashMap;

use super::{contains_glob, expand_home, expand_vars, strip_outer_quotes};

fn env() -> HashMap<String, String> {
    HashMap::from([
        ("HOME".to_string(), "/home/ubugeeei".to_string()),
        ("NAME".to_string(), "ush".to_string()),
        ("EMPTY".to_string(), String::new()),
        ("PATH_LIKE".to_string(), "/usr/bin:/bin".to_string()),
    ])
}

#[test]
fn bare_tilde_expands_to_home() {
    assert_eq!(expand_home("~", &env()), "/home/ubugeeei");
}

#[test]
fn tilde_slash_prefix_expands_but_bare_prefix_does_not() {
    assert_eq!(expand_home("~/src", &env()), "/home/ubugeeei/src");
    assert_eq!(expand_home("~other/src", &env()), "~other/src");
    assert_eq!(expand_home("a~/src", &env()), "a~/src");
}

#[test]
fn tilde_is_left_alone_when_home_is_unset() {
    let empty = HashMap::new();
    assert_eq!(expand_home("~", &empty), "~");
    assert_eq!(expand_home("~/src", &empty), "~/src");
}

#[test]
fn bare_variable_names_expand() {
    let env = env();
    assert_eq!(expand_vars("$NAME", &env, 0).expect("expand"), "ush");
    assert_eq!(
        expand_vars("hello $NAME!", &env, 0).expect("expand"),
        "hello ush!"
    );
}

#[test]
fn variable_names_stop_at_non_identifier_characters() {
    let env = env();
    assert_eq!(expand_vars("$NAME-x", &env, 0).expect("expand"), "ush-x");
    assert_eq!(
        expand_vars("$NAME/bin", &env, 0).expect("expand"),
        "ush/bin"
    );
    assert_eq!(expand_vars("$NAME.", &env, 0).expect("expand"), "ush.");
}

#[test]
fn braced_expansion_reads_the_whole_name() {
    let env = env();
    assert_eq!(expand_vars("${NAME}", &env, 0).expect("expand"), "ush");
    assert_eq!(
        expand_vars("${NAME}${NAME}", &env, 0).expect("expand"),
        "ushush"
    );
    assert_eq!(expand_vars("x${NAME}y", &env, 0).expect("expand"), "xushy");
}

#[test]
fn unknown_variables_expand_to_the_empty_string() {
    let env = env();
    assert_eq!(expand_vars("[$MISSING]", &env, 0).expect("expand"), "[]");
    assert_eq!(expand_vars("[${MISSING}]", &env, 0).expect("expand"), "[]");
    assert_eq!(expand_vars("[$EMPTY]", &env, 0).expect("expand"), "[]");
}

#[test]
fn last_status_expands_through_dollar_question() {
    let env = env();
    assert_eq!(expand_vars("$?", &env, 0).expect("expand"), "0");
    assert_eq!(expand_vars("code=$?", &env, 42).expect("expand"), "code=42");
    assert_eq!(expand_vars("$?", &env, -1).expect("expand"), "-1");
}

#[test]
fn unterminated_brace_expansion_is_an_error() {
    let error = expand_vars("${NAME", &env(), 0).expect_err("unterminated");
    assert!(error.to_string().contains("unterminated"));
}

#[test]
fn a_lone_trailing_dollar_is_kept_verbatim() {
    let env = env();
    assert_eq!(expand_vars("$", &env, 0).expect("expand"), "$");
    assert_eq!(
        expand_vars("cost: 5$", &env, 0).expect("expand"),
        "cost: 5$"
    );
}

#[test]
fn dollar_before_an_unsupported_character_is_kept_verbatim() {
    let env = env();
    assert_eq!(expand_vars("$1", &env, 0).expect("expand"), "$1");
    assert_eq!(expand_vars("$ ", &env, 0).expect("expand"), "$ ");
    assert_eq!(expand_vars("$$", &env, 0).expect("expand"), "$$");
}

#[test]
fn underscore_prefixed_names_are_valid_identifiers() {
    let env = HashMap::from([("_HIDDEN".to_string(), "yes".to_string())]);
    assert_eq!(expand_vars("$_HIDDEN", &env, 0).expect("expand"), "yes");
}

#[test]
fn multibyte_text_survives_expansion() {
    let env = HashMap::from([("GREETING".to_string(), "こんにちは".to_string())]);
    assert_eq!(
        expand_vars("→ $GREETING 🎉", &env, 0).expect("expand"),
        "→ こんにちは 🎉"
    );
}

#[test]
fn text_without_a_dollar_is_returned_unchanged() {
    let env = env();
    for value in ["", "plain", "a/b/c", "日本語のテキスト", "{}[]()"] {
        assert_eq!(expand_vars(value, &env, 0).expect("expand"), value);
    }
}

#[test]
fn glob_detection_covers_star_question_and_bracket() {
    assert!(contains_glob("*.rs"));
    assert!(contains_glob("src/?.rs"));
    assert!(contains_glob("src/[ab].rs"));
    assert!(!contains_glob("src/main.rs"));
    assert!(!contains_glob(""));
}

#[test]
fn outer_quotes_are_stripped_only_when_they_match() {
    assert_eq!(strip_outer_quotes("\"value\""), "value");
    assert_eq!(strip_outer_quotes("'value'"), "value");
    assert_eq!(strip_outer_quotes("value"), "value");
    assert_eq!(strip_outer_quotes("\"value'"), "\"value'");
    assert_eq!(strip_outer_quotes("\"outer 'inner'\""), "outer 'inner'");
}

#[test]
fn stripping_quotes_leaves_a_single_quote_character_alone() {
    assert_eq!(strip_outer_quotes("\""), "\"");
    assert_eq!(strip_outer_quotes("'"), "'");
    assert_eq!(strip_outer_quotes("\"\""), "");
}
