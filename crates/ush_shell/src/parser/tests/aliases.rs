use std::collections::BTreeMap;

use crate::parser::alias::expand_alias;

fn aliases(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
        .iter()
        .map(|(name, value)| ((*name).to_string(), (*value).to_string()))
        .collect()
}

#[test]
fn a_known_alias_replaces_the_command_word() {
    let aliases = aliases(&[("ll", "ls -la")]);
    assert_eq!(expand_alias("ll", &aliases).expect("expand"), "ls -la");
    assert_eq!(
        expand_alias("ll src", &aliases).expect("expand"),
        "ls -la src"
    );
}

#[test]
fn an_unknown_command_word_is_left_alone() {
    let aliases = aliases(&[("ll", "ls -la")]);
    assert_eq!(expand_alias("ls src", &aliases).expect("expand"), "ls src");
    assert_eq!(expand_alias("", &aliases).expect("expand"), "");
}

#[test]
fn self_referential_aliases_expand_exactly_once() {
    let aliases = aliases(&[("ls", "ls --color=auto")]);
    assert_eq!(
        expand_alias("ls src", &aliases).expect("expand"),
        "ls --color=auto src"
    );
    assert_eq!(
        expand_alias("ls", &aliases).expect("expand"),
        "ls --color=auto"
    );
}

#[test]
fn mutually_recursive_aliases_terminate() {
    let aliases = aliases(&[("a", "b 1"), ("b", "a 2")]);
    assert_eq!(expand_alias("a", &aliases).expect("expand"), "a 2 1");
}

#[test]
fn alias_chains_resolve_through_intermediate_names() {
    let aliases = aliases(&[("a", "b"), ("b", "c"), ("c", "echo done")]);
    assert_eq!(expand_alias("a", &aliases).expect("expand"), "echo done");
}

#[test]
fn expansion_happens_after_leading_assignments() {
    let aliases = aliases(&[("gm", "git commit -m")]);
    assert_eq!(
        expand_alias("EDITOR=vim gm 'msg'", &aliases).expect("expand"),
        "EDITOR=vim git commit -m 'msg'"
    );
}

#[test]
fn quoting_the_command_word_suppresses_expansion() {
    let aliases = aliases(&[("ll", "ls -la")]);
    assert_eq!(expand_alias("'ll'", &aliases).expect("expand"), "'ll'");
    assert_eq!(expand_alias("\"ll\"", &aliases).expect("expand"), "\"ll\"");
    assert_eq!(expand_alias("\\ll", &aliases).expect("expand"), "\\ll");
}

#[test]
fn an_empty_alias_table_is_a_no_op() {
    let empty = BTreeMap::new();
    assert_eq!(
        expand_alias("ls -la src", &empty).expect("expand"),
        "ls -la src"
    );
}

#[test]
fn unterminated_quotes_surface_as_errors() {
    let aliases = aliases(&[("ll", "ls -la")]);
    // The scan only reaches the end of the line while every word so
    // far is a leading assignment, so that is what it takes to hit
    // the unterminated-quote guard.
    let error = expand_alias("A='oops", &aliases).expect_err("unterminated");
    assert!(error.to_string().contains("unterminated"));
}

#[test]
fn an_unterminated_quote_after_the_command_word_is_left_to_the_tokenizer() {
    let aliases = aliases(&[("ll", "ls -la")]);
    assert_eq!(
        expand_alias("echo 'oops", &aliases).expect("expand"),
        "echo 'oops"
    );
}

#[test]
fn alias_values_that_are_pipelines_are_substituted_verbatim() {
    let aliases = aliases(&[("recent", "ls -t | head")]);
    assert_eq!(
        expand_alias("recent", &aliases).expect("expand"),
        "ls -t | head"
    );
}
