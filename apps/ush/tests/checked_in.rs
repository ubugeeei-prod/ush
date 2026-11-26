use std::{fs, path::Path, process::Command};

use tempfile::tempdir;

fn ush() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ush"))
}

fn compile(source: &Path, output: &Path) -> std::process::Output {
    ush()
        .args([
            "compile",
            source.to_str().unwrap(),
            "--checked-in",
            "-o",
            output.to_str().unwrap(),
        ])
        .output()
        .expect("compile checked-in script")
}

#[test]
fn output_is_deterministic_and_documents_regeneration() {
    let first = tempdir().expect("first tempdir");
    let second = tempdir().expect("second tempdir");
    let source = "let greeting = \"hello\"\nprint greeting\n";

    for directory in [first.path(), second.path()] {
        fs::write(directory.join("greet.ush"), source).expect("write source");
        let result = compile(&directory.join("greet.ush"), &directory.join("greet.sh"));
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }

    let first_shell = fs::read_to_string(first.path().join("greet.sh")).expect("read output");
    let second_shell = fs::read_to_string(second.path().join("greet.sh")).expect("read output");

    assert_eq!(first_shell, second_shell);
    assert!(first_shell.starts_with(
        "#!/bin/sh\n# Generated from 'greet.ush' by ush.\n# DO NOT EDIT MANUALLY.\n# Source: greet.ush\n# Source map: embedded __ush_at markers; ush explain SOURCE LINE\n# Regenerate with: ush compile 'greet.ush' --checked-in -o 'greet.sh'\n"
    ));
    assert!(!first_shell.contains(first.path().to_str().unwrap()));
    assert!(!first_shell.contains(second.path().to_str().unwrap()));
}

#[test]
fn source_relative_paths_follow_the_generated_script() {
    let directory = tempdir().expect("script tempdir");
    let other_directory = tempdir().expect("working tempdir");
    let source_path = directory.path().join("locate.ush");
    let output_path = directory.path().join("locate.sh");
    fs::write(
        &source_path,
        "use std::path::{from_source}\nlet payload = from_source \"payload.txt\"\nprint payload.resolve()\n",
    )
    .expect("write source");

    let result = compile(&source_path, &output_path);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );

    let output = Command::new("sh")
        .arg(&output_path)
        .current_dir(other_directory.path())
        .output()
        .expect("run generated shell");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!(
            "{}/payload.txt\n",
            fs::canonicalize(directory.path()).unwrap().display()
        )
    );
}

#[test]
fn output_in_another_directory_finds_its_source() {
    let directory = tempdir().expect("tempdir");
    let root = fs::canonicalize(directory.path()).unwrap();
    let source = root.join("scripts/locate.ush");
    let output = root.join("scripts/compiled/locate.sh");
    fs::create_dir_all(output.parent().unwrap()).unwrap();
    fs::write(root.join("scripts/payload.txt"), "payload\n").unwrap();
    fs::write(
        &source,
        "use std::path::{from_source}\nlet payload = from_source \"payload.txt\"\nprint payload.read_text()\n",
    )
    .unwrap();

    let result = compile(&source, &output);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let shell = fs::read_to_string(&output).unwrap();
    assert!(shell.contains("# Source: ../locate.ush\n"));
    assert!(
        shell.contains(
            "# Regenerate with: ush compile '../locate.ush' --checked-in -o 'locate.sh'\n"
        )
    );
    assert!(!shell.contains(root.to_str().unwrap()));

    let other = tempdir().unwrap();
    let run = Command::new("sh")
        .arg(&output)
        .current_dir(other.path())
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(run.stdout, b"payload\n");
}

#[test]
fn relative_script_filename_does_not_require_current_directory_on_path() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("local.ush"), "print \"local\"\n").unwrap();
    assert!(
        compile(&dir.path().join("local.ush"), &dir.path().join("local.sh"))
            .status
            .success()
    );
    let output = Command::new("sh")
        .arg("local.sh")
        .current_dir(dir.path())
        .env("PATH", "/usr/bin:/bin")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"local\n");
}

#[test]
fn a_source_relative_path_reaches_external_programs_as_a_real_path() {
    let directory = tempdir().expect("tempdir");
    let root = fs::canonicalize(directory.path()).unwrap();
    let source = root.join("scripts/read.ush");
    let output = root.join("scripts/compiled/read.sh");
    fs::create_dir_all(output.parent().unwrap()).unwrap();
    fs::write(root.join("scripts/payload.txt"), "payload\n").unwrap();
    fs::write(
        &source,
        "use $bin::{cat}\nuse std::path::from_source\nlet payload = from_source payload.txt\ncat $payload\nprint $payload\n",
    )
    .unwrap();
    let result = compile(&source, &output);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );

    let other = tempdir().unwrap();
    let run = Command::new("sh")
        .arg(&output)
        .current_dir(other.path())
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let payload = root.join("scripts/payload.txt");
    assert_eq!(
        String::from_utf8_lossy(&run.stdout),
        format!("payload\n{}\n", payload.display())
    );
}

#[test]
fn script_names_the_running_file_even_after_a_copy() {
    let directory = tempdir().expect("tempdir");
    let root = fs::canonicalize(directory.path()).unwrap();
    let source = root.join("scripts/where.ush");
    let output = root.join("scripts/compiled/where.sh");
    fs::create_dir_all(output.parent().unwrap()).unwrap();
    fs::create_dir_all(root.join("hooks")).unwrap();
    fs::write(
        &source,
        "use std::path::{dirname, script}\nprint (script)\nprint (dirname (script))\n",
    )
    .unwrap();
    let result = compile(&source, &output);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let copy = root.join("hooks/where-hook");
    fs::copy(&output, &copy).unwrap();

    let other = tempdir().unwrap();
    for (script, directory) in [(&output, "scripts/compiled"), (&copy, "hooks")] {
        let run = Command::new("sh")
            .arg(script)
            .current_dir(other.path())
            .output()
            .unwrap();
        assert!(
            run.status.success(),
            "{}",
            String::from_utf8_lossy(&run.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&run.stdout),
            format!("{}\n{}\n", script.display(), root.join(directory).display())
        );
    }
}
