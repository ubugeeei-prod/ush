use super::{format_git_refs, parse_git_log_args, render_git_log_row};
use crate::style::{git::model::GitLogRow, strip_ansi};

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn refs(raw: &str) -> Vec<String> {
    format_git_refs(raw)
        .iter()
        .map(|value| strip_ansi(value))
        .collect()
}

#[test]
fn the_default_shows_a_dozen_commits() {
    assert_eq!(parse_git_log_args(&args(&[])), Some((12, false)));
    assert_eq!(parse_git_log_args(&args(&["--oneline"])), Some((12, false)));
}

#[test]
fn a_count_can_arrive_in_every_git_spelling() {
    assert_eq!(parse_git_log_args(&args(&["-n", "5"])), Some((5, false)));
    assert_eq!(parse_git_log_args(&args(&["-n5"])), Some((5, false)));
    assert_eq!(parse_git_log_args(&args(&["-5"])), Some((5, false)));
    assert_eq!(
        parse_git_log_args(&args(&["--max-count", "5"])),
        Some((5, false))
    );
    assert_eq!(
        parse_git_log_args(&args(&["--max-count=5"])),
        Some((5, false))
    );
}

#[test]
fn a_zero_count_is_clamped_to_one_commit() {
    assert_eq!(parse_git_log_args(&args(&["-0"])), Some((1, false)));
}

#[test]
fn all_widens_the_history_to_every_ref() {
    assert_eq!(parse_git_log_args(&args(&["--all"])), Some((12, true)));
    assert_eq!(parse_git_log_args(&args(&["--all", "-3"])), Some((3, true)));
}

#[test]
fn a_dangling_or_unparseable_count_is_rejected() {
    assert!(parse_git_log_args(&args(&["-n"])).is_none());
    assert!(parse_git_log_args(&args(&["-n", "many"])).is_none());
    assert!(parse_git_log_args(&args(&["--max-count=many"])).is_none());
    assert!(parse_git_log_args(&args(&["-nmany"])).is_none());
}

#[test]
fn unknown_flags_and_revisions_bail_out_to_plain_git() {
    assert!(parse_git_log_args(&args(&["--graph"])).is_none());
    assert!(parse_git_log_args(&args(&["main..topic"])).is_none());
}

#[test]
fn an_empty_decoration_yields_no_badges() {
    assert!(refs("").is_empty());
    assert!(refs("  ").is_empty());
    assert!(refs("()").is_empty());
}

#[test]
fn decorations_are_unwrapped_and_split() {
    assert_eq!(
        refs(" (HEAD -> main, origin/main, tag: v1.0)"),
        vec!["[HEAD -> main]", "[origin/main]", "[tag v1.0]"]
    );
}

#[test]
fn a_plain_branch_name_still_becomes_a_badge() {
    assert_eq!(refs("(topic)"), vec!["[topic]"]);
}

#[test]
fn a_log_row_renders_the_commit_subject_then_metadata() {
    let row = GitLogRow {
        commit: "abc1234".to_string(),
        date: "2026-08-31".to_string(),
        author: "Ubugeeei".to_string(),
        refs: Vec::new(),
        subject: "add tests".to_string(),
    };
    let mut out = String::new();
    render_git_log_row(&mut out, &row);

    assert_eq!(
        strip_ansi(&out),
        "abc1234 add tests\n  2026-08-31 · Ubugeeei\n"
    );
}

#[test]
fn refs_are_rendered_between_the_commit_and_the_subject() {
    let row = GitLogRow {
        commit: "abc1234".to_string(),
        date: "2026-08-31".to_string(),
        author: "Ubugeeei".to_string(),
        refs: format_git_refs("(HEAD -> main)"),
        subject: "add tests".to_string(),
    };
    let mut out = String::new();
    render_git_log_row(&mut out, &row);

    assert!(strip_ansi(&out).starts_with("abc1234 [HEAD -> main] add tests\n"));
}
