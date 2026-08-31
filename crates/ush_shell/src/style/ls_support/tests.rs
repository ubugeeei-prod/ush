use std::{fs, os::unix::fs::PermissionsExt};

use tempfile::tempdir;

use super::{
    EntryKind, HiddenMode, LsSummary, describe_ls_entry, render_ls_row, render_ls_section,
};
use crate::style::strip_ansi;

#[test]
fn hidden_modes_merge_towards_the_widest_setting() {
    assert!(
        !HiddenMode::Default
            .include(HiddenMode::Default)
            .shows_hidden()
    );
    assert!(
        HiddenMode::Default
            .include(HiddenMode::AlmostAll)
            .shows_hidden()
    );
    assert!(
        HiddenMode::AlmostAll
            .include(HiddenMode::All)
            .shows_dot_entries()
    );
    assert!(
        HiddenMode::All
            .include(HiddenMode::AlmostAll)
            .shows_dot_entries()
    );
}

#[test]
fn a_regular_file_reports_its_size() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("file.txt");
    fs::write(&path, "hello").expect("write");

    let row = describe_ls_entry("file.txt", &path, HiddenMode::Default).expect("describe");
    let mut out = String::new();
    render_ls_row(&mut out, &row);
    let text = strip_ansi(&out);

    assert!(matches!(row.kind, EntryKind::File));
    assert!(text.starts_with("file.txt [file] 5 B, updated "));
}

#[test]
fn an_executable_bit_promotes_a_file_to_exec() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("run.sh");
    fs::write(&path, "#!/bin/sh\n").expect("write");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("chmod");

    let row = describe_ls_entry("run.sh", &path, HiddenMode::Default).expect("describe");
    assert!(matches!(row.kind, EntryKind::Exec));
}

#[test]
fn a_directory_is_suffixed_with_a_slash_and_counts_children() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("sub");
    fs::create_dir(&path).expect("mkdir");
    fs::write(path.join("a.txt"), "x").expect("write");
    fs::write(path.join(".hidden"), "x").expect("write");

    let row = describe_ls_entry("sub", &path, HiddenMode::Default).expect("describe");
    let mut out = String::new();
    render_ls_row(&mut out, &row);

    assert!(matches!(row.kind, EntryKind::Dir));
    assert!(strip_ansi(&out).starts_with("sub/ [dir] 1 item,"));
}

#[test]
fn showing_hidden_entries_changes_the_child_count() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("sub");
    fs::create_dir(&path).expect("mkdir");
    fs::write(path.join("a.txt"), "x").expect("write");
    fs::write(path.join(".hidden"), "x").expect("write");

    let row = describe_ls_entry("sub", &path, HiddenMode::AlmostAll).expect("describe");
    let mut out = String::new();
    render_ls_row(&mut out, &row);
    assert!(strip_ansi(&out).contains("2 items"));

    let row = describe_ls_entry("sub", &path, HiddenMode::All).expect("describe");
    let mut out = String::new();
    render_ls_row(&mut out, &row);
    assert!(strip_ansi(&out).contains("4 items"));
}

#[test]
fn a_symlink_reports_its_target_without_following_it() {
    let dir = tempdir().expect("tempdir");
    let target = dir.path().join("target.txt");
    let link = dir.path().join("link.txt");
    fs::write(&target, "hello").expect("write");
    std::os::unix::fs::symlink(&target, &link).expect("symlink");

    let row = describe_ls_entry("link.txt", &link, HiddenMode::Default).expect("describe");
    let mut out = String::new();
    render_ls_row(&mut out, &row);

    assert!(matches!(row.kind, EntryKind::Link));
    assert!(strip_ansi(&out).contains(&format!("-> {}", target.display())));
}

#[test]
fn a_broken_symlink_still_describes_itself() {
    let dir = tempdir().expect("tempdir");
    let link = dir.path().join("broken");
    std::os::unix::fs::symlink(dir.path().join("nowhere"), &link).expect("symlink");

    let row = describe_ls_entry("broken", &link, HiddenMode::Default).expect("describe");
    assert!(matches!(row.kind, EntryKind::Link));
}

#[test]
fn a_missing_path_surfaces_the_io_error() {
    let dir = tempdir().expect("tempdir");
    assert!(describe_ls_entry("gone", &dir.path().join("gone"), HiddenMode::Default).is_err());
}

#[test]
fn a_section_counts_every_entry_kind_it_saw() {
    let mut summary = LsSummary::default();
    summary.observe(EntryKind::Dir);
    summary.observe(EntryKind::File);
    summary.observe(EntryKind::File);
    summary.observe(EntryKind::Exec);
    summary.observe(EntryKind::Link);

    let rendered = strip_ansi(&render_ls_section("src", &summary, "body\n"));
    assert_eq!(
        rendered,
        "ls src\n5 entries, 1 dir, 1 exec, 2 files, 1 link\nbody\n"
    );
}

#[test]
fn an_empty_section_still_reports_a_zero_count() {
    let rendered = strip_ansi(&render_ls_section(".", &LsSummary::default(), ""));
    assert_eq!(rendered, "ls .\n0 entries\n");
}
