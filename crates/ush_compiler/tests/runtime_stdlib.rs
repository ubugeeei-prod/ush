//! End-to-end checks for the `std::string` helpers: each program
//! is compiled to POSIX `sh` and then executed, so a regression in
//! the emitted shell shows up as a wrong value rather than a diff.

mod support;

use support::run;

#[test]
fn string_prefix_and_suffix_predicates_agree_with_the_shell() {
    let output = run(r#"
        use std::string::{starts_with, ends_with}
        print $ starts_with "hello.txt" "hello"
        print $ starts_with "hello.txt" "world"
        print $ ends_with "hello.txt" ".txt"
        print $ ends_with "hello.txt" ".ush"
    "#);

    assert_eq!(output, "true\nfalse\ntrue\nfalse\n");
}

#[test]
fn an_empty_needle_matches_every_string() {
    let output = run(r#"
        use std::string::{starts_with, ends_with}
        print $ starts_with "value" ""
        print $ ends_with "value" ""
    "#);

    assert_eq!(output, "true\ntrue\n");
}

#[test]
fn a_needle_longer_than_the_value_never_matches() {
    let output = run(r#"
        use std::string::{starts_with, ends_with}
        print $ starts_with "ab" "abc"
        print $ ends_with "ab" "abc"
    "#);

    assert_eq!(output, "false\nfalse\n");
}

#[test]
fn replace_rewrites_every_occurrence() {
    let output = run(r#"
        use std::string::replace
        print $ replace "banana" "a" "o"
        print $ replace "hello.txt" ".txt" ".ush"
        print $ replace "aaa" "aa" "b"
    "#);

    assert_eq!(output, "bonono\nhello.ush\nba\n");
}

#[test]
fn replacing_an_empty_needle_leaves_the_value_alone() {
    let output = run(r#"
        use std::string::replace
        print $ replace "value" "" "x"
    "#);

    assert_eq!(output, "value\n");
}

#[test]
fn trimming_only_removes_a_matching_edge() {
    let output = run(r#"
        use std::string::{trim_prefix, trim_suffix}
        print $ trim_prefix "module.ush" "module"
        print $ trim_prefix "module.ush" "other"
        print $ trim_suffix "module.ush" ".ush"
        print $ trim_suffix "module.ush" ".rs"
        print $ trim_suffix "module.ush" ""
    "#);

    assert_eq!(output, ".ush\nmodule.ush\nmodule\nmodule.ush\nmodule.ush\n");
}

#[test]
fn strings_with_shell_metacharacters_survive_the_round_trip() {
    let output = run(r#"
        use std::string::replace
        print $ replace "a $HOME b" "$HOME" "*"
        print $ replace "a 'quoted' b" "'quoted'" "ok"
    "#);

    assert_eq!(output, "a * b\na ok b\n");
}
