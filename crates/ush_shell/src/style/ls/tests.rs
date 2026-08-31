use std::fs;

use tempfile::tempdir;

use super::{ls_entries, parse_ls_args};
use crate::style::{ls_support::HiddenMode, render_ls, strip_ansi};

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn names(entries: &[(String, std::path::PathBuf)]) -> Vec<String> {
    let mut names = entries
        .iter()
        .map(|(name, _)| name.clone())
        .collect::<Vec<_>>();
    names.sort();
    names
}

#[test]
fn no_arguments_lists_the_current_directory_without_hidden_files() {
    let (hidden_mode, targets) = parse_ls_args(&args(&[])).expect("parse");
    assert!(!hidden_mode.shows_hidden());
    assert!(targets.is_empty());
}

#[test]
fn all_and_almost_all_control_hidden_entries() {
    let (hidden_mode, _) = parse_ls_args(&args(&["-a"])).expect("parse");
    assert!(hidden_mode.shows_hidden());
    assert!(hidden_mode.shows_dot_entries());

    let (hidden_mode, _) = parse_ls_args(&args(&["-A"])).expect("parse");
    assert!(hidden_mode.shows_hidden());
    assert!(!hidden_mode.shows_dot_entries());

    let (hidden_mode, _) = parse_ls_args(&args(&["--all"])).expect("parse");
    assert!(hidden_mode.shows_dot_entries());
    let (hidden_mode, _) = parse_ls_args(&args(&["--almost-all"])).expect("parse");
    assert!(!hidden_mode.shows_dot_entries());
}

#[test]
fn the_widest_hidden_mode_wins_when_flags_are_combined() {
    let (hidden_mode, _) = parse_ls_args(&args(&["-A", "-a"])).expect("parse");
    assert!(hidden_mode.shows_dot_entries());
    let (hidden_mode, _) = parse_ls_args(&args(&["-a", "-A"])).expect("parse");
    assert!(hidden_mode.shows_dot_entries());
    let (hidden_mode, _) = parse_ls_args(&args(&["-aA"])).expect("parse");
    assert!(hidden_mode.shows_dot_entries());
}

#[test]
fn presentation_only_flags_are_accepted() {
    for flag in [
        "-l",
        "-1",
        "-C",
        "-F",
        "-G",
        "-h",
        "-m",
        "-p",
        "-x",
        "--long",
        "--human-readable",
        "--classify",
        "--file-type",
        "--color",
        "--color=auto",
        "--indicator-style=slash",
    ] {
        assert!(parse_ls_args(&args(&[flag])).is_some(), "{flag}");
    }
}

#[test]
fn unsupported_flags_bail_out_to_plain_ls() {
    assert!(parse_ls_args(&args(&["-R"])).is_none());
    assert!(parse_ls_args(&args(&["--recursive"])).is_none());
    assert!(parse_ls_args(&args(&["--indicator-style=none"])).is_none());
}

#[test]
fn a_double_dash_forces_the_rest_to_be_paths() {
    let (_, targets) = parse_ls_args(&args(&["--", "-weird"])).expect("parse");
    assert_eq!(targets, args(&["-weird"]));
}

#[test]
fn hidden_entries_are_filtered_out_by_default() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("visible.txt"), "x").expect("write");
    fs::write(dir.path().join(".hidden"), "x").expect("write");

    let entries = ls_entries(dir.path(), HiddenMode::Default).expect("entries");
    assert_eq!(names(&entries), vec!["visible.txt"]);
}

#[test]
fn almost_all_shows_hidden_files_but_not_dot_entries() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("visible.txt"), "x").expect("write");
    fs::write(dir.path().join(".hidden"), "x").expect("write");

    let entries = ls_entries(dir.path(), HiddenMode::AlmostAll).expect("entries");
    assert_eq!(names(&entries), vec![".hidden", "visible.txt"]);
}

#[test]
fn all_adds_the_dot_and_dotdot_entries() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("visible.txt"), "x").expect("write");

    let entries = ls_entries(dir.path(), HiddenMode::All).expect("entries");
    assert_eq!(names(&entries), vec![".", "..", "visible.txt"]);
}

#[test]
fn listing_a_file_yields_that_single_entry() {
    let dir = tempdir().expect("tempdir");
    let file = dir.path().join("only.txt");
    fs::write(&file, "x").expect("write");

    let entries = ls_entries(&file, HiddenMode::Default).expect("entries");
    assert_eq!(names(&entries), vec!["only.txt"]);
}

#[test]
fn a_rendered_listing_summarizes_and_sorts_its_entries() {
    let dir = tempdir().expect("tempdir");
    fs::create_dir(dir.path().join("sub")).expect("mkdir");
    fs::write(dir.path().join("b.txt"), "hello").expect("write");
    fs::write(dir.path().join("a.txt"), "hi").expect("write");

    let rendered = render_ls(dir.path(), &args(&["."]))
        .expect("render")
        .expect("stylish");
    let text = strip_ansi(&rendered.to_text().expect("text"));

    assert!(text.starts_with("ls .\n"));
    assert!(text.contains("3 entries, 1 dir, 2 files\n"));
    let first = text.find("a.txt").expect("a.txt");
    let second = text.find("b.txt").expect("b.txt");
    let third = text.find("sub/").expect("sub");
    assert!(first < second && second < third);
    assert!(text.contains("sub/ [dir] 0 items"));
    assert!(text.contains("a.txt [file] 2 B"));
}

#[test]
fn several_targets_render_as_several_sections() {
    let dir = tempdir().expect("tempdir");
    fs::create_dir(dir.path().join("one")).expect("mkdir");
    fs::create_dir(dir.path().join("two")).expect("mkdir");

    let rendered = render_ls(dir.path(), &args(&["one", "two"]))
        .expect("render")
        .expect("stylish");
    let text = strip_ansi(&rendered.to_text().expect("text"));

    assert!(text.contains("ls one\n"));
    assert!(text.contains("ls two\n"));
}

#[test]
fn an_unreadable_target_names_the_path_it_could_not_read() {
    let dir = tempdir().expect("tempdir");
    let error = render_ls(dir.path(), &args(&["missing"])).expect_err("missing");
    let message = format!("{error:#}");

    assert!(message.contains("failed to read"), "{message}");
    assert!(message.contains("missing"), "{message}");
}

#[test]
fn unsupported_flags_leave_the_stylish_renderer_alone() {
    let dir = tempdir().expect("tempdir");
    assert!(
        render_ls(dir.path(), &args(&["-R"]))
            .expect("render")
            .is_none()
    );
}
