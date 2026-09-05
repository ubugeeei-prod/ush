use super::*;

#[test]
fn glob_builtin_expands_and_sorts_matches() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("b.txt"), "b\n").expect("write b");
    fs::write(dir.path().join("a.txt"), "a\n").expect("write a");
    fs::write(dir.path().join("note.md"), "md\n").expect("write md");

    let output = ush()
        .args(["-c", "glob '*.txt'"])
        .current_dir(dir.path())
        .output()
        .expect("run ush");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "a.txt\nb.txt\n");
}

#[test]
fn glob_builtin_reads_patterns_from_stdin() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("a.txt"), "a\n").expect("write a");
    fs::write(dir.path().join("note.md"), "md\n").expect("write md");

    let mut child = ush()
        .args(["-c", "glob"])
        .current_dir(dir.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn ush");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(b"*.txt\n*.md\n")
        .expect("write stdin");
    let output = child.wait_with_output().expect("wait ush");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "a.txt\nnote.md\n");
}

#[test]
fn glob_builtin_returns_one_when_nothing_matches() {
    let dir = tempdir().expect("tempdir");

    let output = ush()
        .args(["-c", "glob '*.txt'"])
        .current_dir(dir.path())
        .output()
        .expect("run ush");

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stdout).is_empty());
}

#[test]
fn bracket_test_builtin_returns_zero_for_true_expression() {
    let output = ush().args(["-c", "[ -d . ]"]).output().expect("run ush");
    assert!(output.status.success());
}

#[test]
fn rm_guard_runs_unattended_when_interaction_is_disabled() {
    let dir = tempdir().expect("tempdir");
    let target = dir.path().join("target");
    fs::create_dir_all(target.join("nested")).expect("mkdir target");
    fs::write(target.join("nested/file.txt"), "remove\n").expect("write target");

    let output = ush()
        .args(["-c", "rm -rf target"])
        .env("USH_INTERACTION", "false")
        .current_dir(dir.path())
        .output()
        .expect("run ush");

    assert!(output.status.success());
    assert!(!target.exists());
}

#[test]
fn rm_guard_rejects_split_recursive_short_flags_without_yes() {
    let dir = tempdir().expect("tempdir");
    let target = dir.path().join("target");
    fs::create_dir_all(target.join("nested")).expect("mkdir target");
    fs::write(target.join("nested/file.txt"), "keep\n").expect("write target");

    let output = run_with_stdin_in_dir(&["-c", "rm -r -f target"], "n\n", Some(dir.path()));

    assert_eq!(output.status.code(), Some(130));
    assert!(target.exists());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_snapshot("builtins/rm_guard_reject.stderr", &stderr);
}

#[test]
fn rm_guard_allows_recursive_delete_with_yes() {
    let dir = tempdir().expect("tempdir");
    let target = dir.path().join("target");
    fs::create_dir_all(target.join("nested")).expect("mkdir target");
    fs::write(target.join("nested/file.txt"), "remove\n").expect("write target");

    let output = ush()
        .args(["-c", "rm --yes -r -f target"])
        .current_dir(dir.path())
        .output()
        .expect("run ush");

    assert!(output.status.success());
    assert!(!target.exists());
}
