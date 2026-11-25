use std::{fs, process::Command};

use tempfile::tempdir;
use ush_compiler::UshCompiler;

fn run_program(source: &str) -> String {
    let compiled = UshCompiler::default()
        .compile_source(source)
        .expect("compile ush program");
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("program.sh");
    fs::write(&script, compiled).expect("write script");

    let output = Command::new("/bin/sh")
        .arg(&script)
        .output()
        .expect("run compiled script");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn std_modules_support_fully_qualified_calls() {
    let output = run_program(
        r#"
        let joined = std::path::join "/tmp/" "/ush"
        print joined
        print $ std::string::trim_prefix "prefix-value" "prefix-"
    "#,
    );

    assert_eq!(output, "/tmp/ush\nvalue\n");
}

#[test]
fn use_imports_can_bring_std_functions_into_scope() {
    let output = run_program(
        r#"
        use std::env::{get_or, set as env_set}
        use std::string::{replace, starts_with}
        env_set "USH_STD_IMPORT" "ready"
        print $ get_or "USH_STD_IMPORT" "fallback"
        print $ replace "hello world" "world" "ush"
        print $ starts_with "ush" "u"
    "#,
    );

    assert_eq!(output, "ready\nhello ush\ntrue\n");
}

#[test]
fn std_path_helpers_cover_temp_files_and_metadata() {
    let output = run_program(
        r#"
        use std::path::{tmpfile, exists, is_file, basename}
        let path = tmpfile()
        print $ exists $path
        print $ is_file $path
        print $ basename $path
    "#,
    );

    let mut lines = output.lines();
    assert_eq!(lines.next(), Some("true"));
    assert_eq!(lines.next(), Some("true"));
    assert!(lines.next().is_some_and(|line| line.ends_with(".tmp")));
}

#[test]
fn prepend_env_updates_the_current_shell_session() {
    let output = run_program(
        r#"
        use std::path::prepend_env
        use std::env::get_or
        prepend_env "USH_PATH_TEST" "/first"
        prepend_env "USH_PATH_TEST" "/second"
        print $ get_or "USH_PATH_TEST" ""
    "#,
    );

    assert_eq!(output, "/second:/first\n");
}

#[test]
fn std_fs_helpers_cover_basic_file_operations() {
    let sandbox = tempdir().expect("tempdir");
    let moved = sandbox.path().join("moved.txt");
    let directory = sandbox.path().join("dir");

    let output = run_program(&format!(
        r#"
        use std::fs::{{append_text, copy, exists, is_dir, is_file, mkdir_p, move, read_text, remove, tmpfile, write_text}}
        let path = tmpfile()
        write_text $path "alpha"
        append_text $path ":beta"
        let moved = "{moved}"
        copy $path $moved
        print $ read_text $moved
        print $ exists $moved
        print $ is_file $moved
        mkdir_p "{directory}"
        print $ is_dir "{directory}"
        move $moved $path
        remove $path
        print $ exists $path
        $ rmdir "{directory}"
      "#,
        moved = moved.display(),
        directory = directory.display(),
    ));

    assert_eq!(output, "alpha:beta\ntrue\ntrue\ntrue\nfalse\n");
}

#[test]
fn std_command_helpers_capture_output_and_status() {
    let output = run_program(
        r#"
        use std::command::{capture, capture_stderr, exists, run, status}
        print $ exists "sh"
        print $ capture "printf '%s\n' hello"
        print $ capture_stderr "printf '%s\n' warn >&2"
        print $ status "exit 7"
        run "printf '%s\n' done"
      "#,
    );

    assert_eq!(output, "true\nhello\nwarn\n7\ndone\n");
}

#[test]
fn path_refs_support_source_relative_and_cwd_relative_flows() {
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
        use std::fs::read_text
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

    let compiled = UshCompiler::default()
        .compile_file(&script)
        .expect("compile file");
    let shell = dir.path().join("program.sh");
    fs::write(&shell, compiled).expect("write shell");

    let output = Command::new("/bin/sh")
        .arg(&shell)
        .current_dir(&run_dir)
        .output()
        .expect("run compiled script");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "source\ncwd\ntrue\nnotes.txt\ntrue\n"
    );
}

#[test]
fn path_methods_can_mutate_and_read_files() {
    let output = run_program(
        r#"
        use std::path::tmpfile
        let path = tmpfile()
        path.write_text("alpha")
        path.append_text(":beta")
        print $ path.read_text()
        print $ path.sha256() != ""
        print $ path.mime_type().starts_with("text/")
        path.remove()
        print $ path.exists()
      "#,
    );

    assert_eq!(output, "alpha:beta\ntrue\ntrue\nfalse\n");
}
