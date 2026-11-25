use std::{fs, process::Command};

use tempfile::tempdir;

fn ush() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ush"))
}

#[test]
fn ush_scripts_support_use_imports_and_std_modules() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("modules.ush");
    fs::write(
        &script,
        r#"
        use std::env::{get_or, set}
        use std::path::{exists, tmpfile}
        set "USH_STD_SMOKE" "ok"
        let file = tmpfile()
        print $ exists $file
        print $ get_or "USH_STD_SMOKE" "missing"
        "#,
    )
    .expect("write script");

    let output = ush().arg(&script).output().expect("run ush");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "true\nok\n");
}

#[test]
fn ush_scripts_support_std_fs_and_command_helpers() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("system.ush");
    fs::write(
        &script,
        r#"
        use std::command::{capture, exists, status}
        use std::fs::{read_text, tmpfile, write_text}
        let path = tmpfile()
        write_text $path "ready"
        print $ exists "sh"
        print $ capture "printf '%s\n' cmd"
        print $ read_text $path
        print $ status "exit 3"
        "#,
    )
    .expect("write script");

    let output = ush().arg(&script).output().expect("run ush");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "true\ncmd\nready\n3\n"
    );
}

#[test]
fn ush_scripts_support_source_relative_path_refs() {
    let dir = tempdir().expect("tempdir");
    let source_dir = dir.path().join("source");
    let run_dir = dir.path().join("run");
    fs::create_dir_all(&source_dir).expect("create source dir");
    fs::create_dir_all(&run_dir).expect("create run dir");
    fs::write(source_dir.join("notes.txt"), "source\n").expect("write source file");
    fs::write(run_dir.join("notes.txt"), "cwd\n").expect("write cwd file");

    let script = source_dir.join("paths.ush");
    fs::write(
        &script,
        r#"
        use std::path::{from_cwd, from_source}
        let source_root = from_source "."
        let source_file = source_root.join("notes.txt")
        let cwd_file = from_cwd "notes.txt"
        let source_parent = source_file.dirname()
        let source_copy = source_parent.join("notes.txt")
        let cwd_abs = cwd_file.resolve()
        print $ source_copy.read_text()
        print $ cwd_file.read_text()
        print $ source_file.exists()
        print $ cwd_abs.basename()
        print $ cwd_abs.exists()
        "#,
    )
    .expect("write script");

    let output = ush()
        .current_dir(&run_dir)
        .arg(&script)
        .output()
        .expect("run ush");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "source\ncwd\ntrue\nnotes.txt\ntrue\n"
    );
}
