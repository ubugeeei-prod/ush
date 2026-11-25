//! Where module errors are reported, at compile time and at run time,
//! and how module scripts behave when relocated.

mod support;

use std::{fs, path::Path, process::Command};

use support::modules::{Project, shells};
use ush_compiler::UshCompiler;

#[test]
fn import_cycles_name_every_file() {
    let project = Project::new(&[
        ("lib/a.ush", "mod \"b.ush\"\npub fn a() {\n}\n"),
        ("lib/b.ush", "mod \"a.ush\"\npub fn b() {\n}\n"),
        ("main.ush", "mod \"lib/a.ush\"\n"),
        ("again.ush", "mod \"again.ush\"\n"),
    ]);
    let error = project.error("main.ush");
    assert!(
        error.contains(
            "line 1: lib/b.ush: line 1: import cycle: lib/a.ush -> lib/b.ush -> lib/a.ush"
        ),
        "{error}"
    );
    assert!(
        project
            .error("again.ush")
            .contains("import cycle: again.ush -> again.ush")
    );
}

#[test]
fn compile_errors_inside_modules_name_the_module_file_and_line() {
    let project = Project::new(&[
        ("lib/bad.ush", "pub fn ok() {\n}\nprint \"top level\"\n"),
        (
            "lib/typo.ush",
            "pub fn f() -> String {\n  let x = \"a\"\n  x + missing\n}\n",
        ),
        ("statement.ush", "print 1\nmod \"lib/bad.ush\"\n"),
        ("typo.ush", "mod \"lib/typo.ush\"\nprint typo::f\n"),
        ("absent.ush", "mod \"lib/absent.ush\"\n"),
    ]);
    let error = project.error("statement.ush");
    assert!(
        error.contains("line 2: lib/bad.ush: line 3: a module contains only declarations"),
        "{error}"
    );
    let error = project.error("typo.ush");
    assert!(
        error.contains("line 1: lib/typo.ush: line 3: unknown variable: missing"),
        "{error}"
    );
    assert!(
        project
            .error("absent.ush")
            .contains("line 1: module not found: lib/absent.ush")
    );
}

#[test]
fn runtime_failures_inside_modules_report_the_module_file() {
    let project = Project::new(&[
        (
            "lib/boom.ush",
            "pub fn boom() -> () / unknown, io, fs, env, net, exec {\n  $ false\n}\n",
        ),
        ("main.ush", "mod \"lib/boom.ush\"\nboom::boom\n"),
    ]);
    let shell = project.compile("main.ush").unwrap();
    let script = project.root.join("main.sh");
    fs::write(&script, shell).unwrap();
    for shell in shells() {
        let output = Command::new(shell).arg(&script).output().unwrap();
        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("lib/boom.ush:2: command failed"),
            "{shell}: {stderr}"
        );
    }
}

#[test]
fn module_names_must_not_collapse_into_one_shell_function() {
    let project = Project::new(&[
        ("gh.ush", "pub fn run() {\n}\n"),
        ("main.ush", "mod \"gh.ush\"\nfn gh__run() {\n}\ngh::run\n"),
    ]);
    assert!(
        project
            .error("main.ush")
            .contains("compile to the same shell function")
    );
}

#[test]
fn source_compiled_from_memory_cannot_import_modules() {
    let error = UshCompiler.compile_source("mod \"x.ush\"\n").unwrap_err();
    assert!(format!("{error:#}").contains("source compiled from memory has no directory"));
}

#[test]
fn checked_in_output_in_another_directory_runs_from_anywhere() {
    let project = Project::new(&[
        ("lib/data.txt", "lib data\n"),
        ("lib/hello.sh", "hello() { printf 'hello %s' \"$1\"; }\n"),
        (
            "lib/files.ush",
            "use std::path::{from_source}\nextern \"hello.sh\" as sh {\n  fn hello(name: String) -> String\n}\npub fn data() -> String / unknown, fs, env {\n  let path = from_source \"data.txt\"\n  path.read_text()\n}\npub fn greet() -> String / unknown, stdin, stderr, fs, env, net, exec {\n  sh::hello \"module\"\n}\n",
        ),
        ("ci/notes.txt", "ci notes\n"),
        (
            "ci/main.ush",
            "mod \"../lib/files.ush\"\nuse std::path::{from_source}\nprint files::data\nprint files::greet\nlet notes = from_source \"notes.txt\"\nprint notes.read_text()\n",
        ),
    ]);
    let source = project.root.join("ci/main.ush");
    let output = project.root.join("ci/compiled/main.sh");
    fs::create_dir_all(output.parent().unwrap()).unwrap();
    let shell = UshCompiler
        .compile_checked_in_file(&source, &output)
        .unwrap();
    assert!(shell.starts_with(
        "#!/bin/sh\n# Generated from '../main.ush' by ush.\n# DO NOT EDIT MANUALLY.\n# Source: ../main.ush\n# Source map: embedded __ush_at markers; ush explain SOURCE LINE\n# Regenerate with: ush compile '../main.ush' --checked-in -o 'main.sh'\n"
    ));
    assert!(!shell.contains(project.root.to_str().unwrap()));
    fs::write(&output, shell).unwrap();
    for shell in shells() {
        let run = Command::new(shell)
            .arg(&output)
            .current_dir("/")
            .output()
            .unwrap();
        assert!(
            run.status.success(),
            "{shell}: {}",
            String::from_utf8_lossy(&run.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&run.stdout),
            "lib data\nhello module\nci notes\n",
            "{shell}"
        );
    }
}

#[test]
fn module_source_maps_name_the_module_file() {
    let project = Project::new(&[
        ("lib/m.ush", "pub fn f() {\n  print \"x\"\n}\n"),
        ("main.ush", "mod \"lib/m.ush\"\nm::f\n"),
    ]);
    let compiled = UshCompiler
        .compile_file_with_sourcemap(&project.root.join("main.ush"))
        .unwrap();
    let line = compiled
        .sourcemap
        .lines
        .iter()
        .find(|line| line.generated_text.contains("__ush_at 2 'print \"x\"'"))
        .expect("module line");
    assert_eq!(line.source_line, Some(2));
    assert!(
        line.source_file
            .as_deref()
            .is_some_and(|file| file.ends_with("lib/m.ush"))
    );
    assert!(Path::new(line.source_file.as_deref().unwrap()).exists());
}
