use super::{parse_git_branch_args, render_git_branch_row};
use crate::style::{
    git::model::{GitBranchRow, GitRefScope},
    strip_ansi,
};

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn row(name: &str, scope: GitRefScope, current: bool) -> GitBranchRow {
    GitBranchRow {
        scope,
        name: name.to_string(),
        current,
        upstream: None,
        commit: "abc1234".to_string(),
        date: "2026-08-31".to_string(),
        subject: "initial commit".to_string(),
    }
}

#[test]
fn no_arguments_lists_local_branches_only() {
    assert_eq!(parse_git_branch_args(&args(&[])), Some((true, false)));
    assert_eq!(
        parse_git_branch_args(&args(&["--list"])),
        Some((true, false))
    );
}

#[test]
fn all_adds_remotes_to_the_local_list() {
    assert_eq!(parse_git_branch_args(&args(&["-a"])), Some((true, true)));
    assert_eq!(parse_git_branch_args(&args(&["--all"])), Some((true, true)));
}

#[test]
fn remotes_only_drops_the_local_list() {
    assert_eq!(parse_git_branch_args(&args(&["-r"])), Some((false, true)));
    assert_eq!(
        parse_git_branch_args(&args(&["--remotes"])),
        Some((false, true))
    );
}

#[test]
fn unknown_flags_and_branch_names_bail_out_to_plain_git() {
    assert!(parse_git_branch_args(&args(&["-d", "topic"])).is_none());
    assert!(parse_git_branch_args(&args(&["new-branch"])).is_none());
}

#[test]
fn a_local_row_renders_its_name_commit_and_details() {
    let mut out = String::new();
    render_git_branch_row(&mut out, &row("main", GitRefScope::Local, false));

    assert_eq!(
        strip_ansi(&out),
        "main abc1234\n  2026-08-31 · initial commit\n"
    );
}

#[test]
fn the_checked_out_branch_is_badged_as_current() {
    let mut out = String::new();
    render_git_branch_row(&mut out, &row("main", GitRefScope::Local, true));

    assert!(strip_ansi(&out).starts_with("main [current] abc1234\n"));
}

#[test]
fn remote_rows_are_badged_as_remote() {
    let mut out = String::new();
    render_git_branch_row(&mut out, &row("origin/main", GitRefScope::Remote, false));

    assert!(strip_ansi(&out).starts_with("origin/main [remote] abc1234\n"));
}

#[test]
fn an_upstream_is_rendered_as_its_own_badge() {
    let mut branch = row("topic", GitRefScope::Local, false);
    branch.upstream = Some("origin/topic".to_string());
    let mut out = String::new();
    render_git_branch_row(&mut out, &branch);

    assert!(strip_ansi(&out).starts_with("topic [origin/topic] abc1234\n"));
}

#[test]
fn a_row_without_date_or_subject_skips_the_detail_line() {
    let mut branch = row("topic", GitRefScope::Local, false);
    branch.date = String::new();
    branch.subject = String::new();
    let mut out = String::new();
    render_git_branch_row(&mut out, &branch);

    assert_eq!(strip_ansi(&out), "topic abc1234\n");
}

#[test]
fn a_row_with_only_a_date_still_renders_a_detail_line() {
    let mut branch = row("topic", GitRefScope::Local, false);
    branch.subject = String::new();
    let mut out = String::new();
    render_git_branch_row(&mut out, &branch);

    assert_eq!(strip_ansi(&out), "topic abc1234\n  2026-08-31\n");
}
