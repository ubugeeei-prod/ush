//! End-to-end checks for the `std::path`, `std::fs`, `std::env`,
//! and `std::command` helpers.

mod support;

use std::fs;

use tempfile::tempdir;

use support::{run, run_in};

#[test]
fn joining_paths_normalizes_the_separator() {
    let output = run(r#"
        use std::path::join
        print $ join "/tmp" "ush"
        print $ join "/tmp/" "ush"
        print $ join "/tmp" "/ush"
        print $ join "/" "ush"
    "#);

    assert_eq!(output, "/tmp/ush\n/tmp/ush\n/tmp/ush\n/ush\n");
}

#[test]
fn joining_with_an_empty_side_returns_the_other_side() {
    let output = run(r#"
        use std::path::join
        print $ join "" "/tmp"
        print $ join "/tmp" ""
    "#);

    assert_eq!(output, "/tmp\n/tmp\n");
}

#[test]
fn dirname_and_basename_split_a_path() {
    let output = run(r#"
        use std::path::{dirname, basename}
        print $ dirname "/tmp/ush/file.txt"
        print $ basename "/tmp/ush/file.txt"
    "#);

    assert_eq!(output, "/tmp/ush\nfile.txt\n");
}

#[test]
fn path_predicates_report_what_is_on_disk() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("file.txt"), "hello").expect("write");
    fs::create_dir(dir.path().join("sub")).expect("mkdir");

    let output = run_in(
        &dir,
        r#"
        use std::path::{exists, is_file, is_dir, from_cwd}
        print $ exists (from_cwd "file.txt")
        print $ exists (from_cwd "missing.txt")
        print $ is_file (from_cwd "file.txt")
        print $ is_file (from_cwd "sub")
        print $ is_dir (from_cwd "sub")
        print $ is_dir (from_cwd "file.txt")
    "#,
    );

    assert_eq!(output, "true\nfalse\ntrue\nfalse\ntrue\nfalse\n");
}

#[test]
fn mkdir_p_creates_nested_directories() {
    let dir = tempdir().expect("tempdir");
    let output = run_in(
        &dir,
        r#"
        use std::path::{mkdir_p, is_dir, from_cwd}
        let target = from_cwd "a/b/c"
        mkdir_p target
        print $ is_dir target
    "#,
    );

    assert_eq!(output, "true\n");
    assert!(dir.path().join("a/b/c").is_dir());
}

#[test]
fn cwd_relative_paths_resolve_against_the_working_directory() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("file.txt"), "hello").expect("write");

    let output = run_in(
        &dir,
        r#"
        use std::path::{from_cwd, resolve, basename}
        let file = from_cwd "file.txt"
        print $ basename (resolve file)
    "#,
    );

    assert_eq!(output, "file.txt\n");
}

#[test]
fn a_temporary_file_is_created_and_writable() {
    let output = run(r#"
        use std::path::tmpfile
        use std::fs::{write_text, read_text}
        let file = tmpfile()
        write_text file "from-fs"
        print $ read_text file
    "#);

    assert_eq!(output, "from-fs\n");
}

#[test]
fn files_round_trip_through_read_and_write() {
    let dir = tempdir().expect("tempdir");
    let output = run_in(
        &dir,
        r#"
        use std::path::from_cwd
        use std::fs::{write_text, read_text}
        let file = from_cwd "out.txt"
        write_text file "first"
        write_text file "second"
        print $ read_text file
    "#,
    );

    assert_eq!(output, "second\n");
    assert_eq!(
        fs::read_to_string(dir.path().join("out.txt")).expect("read"),
        "second"
    );
}

#[test]
fn environment_variables_can_be_set_and_read_back() {
    let output = run(r#"
        use std::env::{set, get, get_or}
        set "USH_TEST_VALUE" "ready"
        print $ get "USH_TEST_VALUE"
        print $ get_or "USH_TEST_VALUE" "fallback"
        print $ get_or "USH_TEST_MISSING" "fallback"
    "#);

    assert_eq!(output, "ready\nready\nfallback\n");
}

#[test]
fn command_lookup_and_capture_reach_the_real_shell() {
    let output = run(r#"
        use std::command::{capture, exists}
        print $ exists "sh"
        print $ exists "definitely-not-a-real-command"
        print $ capture "printf '%s' from-command"
    "#);

    assert_eq!(output, "true\nfalse\nfrom-command\n");
}

#[test]
fn std_helpers_compose_through_method_call_syntax() {
    let output = run(r#"
        use std::path::join
        let nested = join "/tmp" "ush"
        print nested.dirname()
        print nested.basename()
    "#);

    assert_eq!(output, "/tmp\nush\n");
}
