use super::{
    describe_git_status_row, git_status_conflict, git_status_label, parse_git_status_args,
    parse_git_status_header, parse_git_status_record, render_git_status_row,
};
use crate::style::{common::RED_BOLD, strip_ansi};

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn badges(index: char, worktree: char) -> Vec<String> {
    describe_git_status_row(index, worktree, "file.rs".to_string(), None)
        .badges
        .iter()
        .map(|badge| strip_ansi(badge))
        .collect()
}

#[test]
fn shape_only_flags_are_accepted_and_produce_no_pathspec() {
    let parsed = parse_git_status_args(&args(&["-s", "--short", "-b", "--branch", "--porcelain"]))
        .expect("parse");
    assert!(parsed.is_empty());
}

#[test]
fn positional_arguments_become_pathspecs() {
    assert_eq!(
        parse_git_status_args(&args(&["src", "README.md"])).expect("parse"),
        args(&["src", "README.md"])
    );
}

#[test]
fn a_double_dash_forces_the_rest_to_be_pathspecs() {
    assert_eq!(
        parse_git_status_args(&args(&["--", "-weird-name"])).expect("parse"),
        args(&["-weird-name"])
    );
}

#[test]
fn unknown_flags_bail_out_to_plain_git() {
    assert!(parse_git_status_args(&args(&["--ignored"])).is_none());
    assert!(parse_git_status_args(&args(&["-z"])).is_none());
}

#[test]
fn a_plain_branch_header_has_no_details() {
    let header = parse_git_status_header("main");
    assert_eq!(header.branch, "main");
    assert!(header.details.is_empty());
}

#[test]
fn an_upstream_is_reported_as_a_tracking_detail() {
    let header = parse_git_status_header("main...origin/main");
    assert_eq!(header.branch, "main");
    assert_eq!(header.details, args(&["tracks origin/main"]));
}

#[test]
fn ahead_and_behind_counts_are_split_into_details() {
    let header = parse_git_status_header("main...origin/main [ahead 2, behind 1]");
    assert_eq!(header.branch, "main");
    assert_eq!(
        header.details,
        args(&["tracks origin/main", "ahead 2", "behind 1"])
    );
}

#[test]
fn a_fresh_repository_reports_that_it_has_no_commits() {
    let header = parse_git_status_header("No commits yet on main");
    assert_eq!(header.branch, "main");
    assert_eq!(header.details, args(&["no commits yet"]));
}

#[test]
fn a_detached_head_is_labelled_as_such() {
    let header = parse_git_status_header("HEAD (no branch)");
    assert_eq!(header.branch, "detached HEAD");
    assert_eq!(header.details, args(&["HEAD (no branch)"]));
}

#[test]
fn a_status_record_is_split_into_codes_and_a_path() {
    let mut records = ["ignored"].into_iter();
    let mut rows = Vec::new();
    parse_git_status_record(" M src/main.rs", &mut records, &mut rows);

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].path, "src/main.rs");
    assert!(rows[0].original_path.is_none());
}

#[test]
fn a_rename_record_pulls_the_original_path_from_the_next_record() {
    let mut records = ["old.rs", "next"].into_iter();
    let mut rows = Vec::new();
    parse_git_status_record("R  new.rs", &mut records, &mut rows);

    assert_eq!(rows[0].path, "new.rs");
    assert_eq!(rows[0].original_path.as_deref(), Some("old.rs"));
    assert_eq!(records.next(), Some("next"));
}

#[test]
fn truncated_records_are_ignored() {
    let mut records = std::iter::empty();
    let mut rows = Vec::new();
    parse_git_status_record("M", &mut records, &mut rows);
    parse_git_status_record("", &mut records, &mut rows);

    assert!(rows.is_empty());
}

#[test]
fn staged_and_unstaged_changes_get_separate_badges() {
    assert_eq!(badges('M', ' '), vec!["[staged modified]"]);
    assert_eq!(badges(' ', 'M'), vec!["[unstaged modified]"]);
    assert_eq!(
        badges('M', 'M'),
        vec!["[staged modified]", "[unstaged modified]"]
    );
}

#[test]
fn every_status_code_has_a_label() {
    assert_eq!(git_status_label('M'), Some("modified"));
    assert_eq!(git_status_label('A'), Some("added"));
    assert_eq!(git_status_label('D'), Some("deleted"));
    assert_eq!(git_status_label('R'), Some("renamed"));
    assert_eq!(git_status_label('C'), Some("copied"));
    assert_eq!(git_status_label('U'), Some("updated"));
    assert_eq!(git_status_label(' '), None);
    assert_eq!(git_status_label('?'), None);
}

#[test]
fn untracked_files_get_a_dedicated_badge() {
    assert_eq!(badges('?', '?'), vec!["[untracked]"]);
}

#[test]
fn every_conflict_pair_is_recognized() {
    for (index, worktree) in [
        ('D', 'D'),
        ('A', 'U'),
        ('U', 'D'),
        ('U', 'A'),
        ('D', 'U'),
        ('A', 'A'),
        ('U', 'U'),
    ] {
        assert!(git_status_conflict(index, worktree), "{index}{worktree}");
        assert_eq!(badges(index, worktree), vec!["[conflict]"]);
    }
}

#[test]
fn ordinary_pairs_are_not_conflicts() {
    assert!(!git_status_conflict('M', ' '));
    assert!(!git_status_conflict(' ', 'M'));
    assert!(!git_status_conflict('?', '?'));
}

#[test]
fn renames_add_a_badge_on_top_of_the_change_badges() {
    let row = describe_git_status_row('R', ' ', "new.rs".to_string(), Some("old.rs".to_string()));
    let badges = row
        .badges
        .iter()
        .map(|badge| strip_ansi(badge))
        .collect::<Vec<_>>();

    assert_eq!(badges, vec!["[staged renamed]", "[renamed]"]);
}

#[test]
fn deletions_are_styled_as_errors() {
    assert_eq!(
        describe_git_status_row('D', ' ', "gone.rs".to_string(), None).style,
        RED_BOLD
    );
    assert_eq!(
        describe_git_status_row(' ', 'D', "gone.rs".to_string(), None).style,
        RED_BOLD
    );
}

#[test]
fn a_rendered_row_lists_the_path_then_the_badges() {
    let row = describe_git_status_row('M', ' ', "src/main.rs".to_string(), None);
    let mut out = String::new();
    render_git_status_row(&mut out, &row);

    assert_eq!(strip_ansi(&out), "src/main.rs [staged modified]\n");
}

#[test]
fn a_rendered_rename_shows_where_the_file_came_from() {
    let row = describe_git_status_row('R', ' ', "new.rs".to_string(), Some("old.rs".to_string()));
    let mut out = String::new();
    render_git_status_row(&mut out, &row);

    assert_eq!(
        strip_ansi(&out),
        "new.rs [staged renamed] [renamed]\n  from old.rs\n"
    );
}
