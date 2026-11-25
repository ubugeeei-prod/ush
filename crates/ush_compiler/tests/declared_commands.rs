//! Programs outside the command catalog, named explicitly with `use $bin::{…}`.

mod support;

use std::{fs, os::unix::fs::PermissionsExt};

use tempfile::tempdir;
use ush_compiler::UshCompiler;

/// Puts an executable `name` that echoes its argv on the program's PATH.
fn stub(dir: &tempfile::TempDir, name: &str) {
    let bin = dir.path().join("bin");
    fs::create_dir_all(&bin).expect("create bin");
    let path = bin.join(name);
    fs::write(
        &path,
        format!("#!/bin/sh\nprintf '{name}'\nprintf ' <%s>' \"$@\"\necho\n"),
    )
    .expect("write stub");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("chmod stub");
}

const ON_PATH: &str =
    "use std::path::cwd\nexport PATH = cwd + \"/bin:\" + std::env::get \"PATH\"\n";

fn compile_error(source: &str) -> String {
    match UshCompiler::default().compile_source(source) {
        Ok(_) => panic!("expected a compile error for:\n{source}"),
        Err(error) => format!("{error:#}"),
    }
}

#[test]
fn a_named_program_runs_with_typed_argv() {
    let dir = tempdir().expect("tempdir");
    stub(&dir, "vp");
    let output = support::run_in(
        &dir,
        &format!("use $bin::{{vp}}\n{ON_PATH}vp \"run\" \"two words\" \"\"\n"),
    );
    assert_eq!(output, "vp <run> <two words> <>\n");
}

#[test]
fn a_quoted_program_name_takes_a_ush_alias() {
    let dir = tempdir().expect("tempdir");
    stub(&dir, "fc-match");
    stub(&dir, "apt-get");
    let source = format!(
        "use $bin::*\nuse $bin::{{\"fc-match\" as fc_match, \"apt-get\" as apt_get}}\n{ON_PATH}\
         let family = fc_match \":lang=ja\"\nprint family\napt_get \"update\" | tr \"a-z\" \"A-Z\"\n"
    );
    assert_eq!(
        support::run_in(&dir, &source),
        "fc-match <:lang=ja>\nAPT-GET <UPDATE>\n"
    );
}

#[test]
fn the_wildcard_imports_only_the_catalog() {
    let error = compile_error("use $bin::*\nvp \"run\"\n");
    assert!(error.contains("unknown function: vp"), "{error}");
}

#[test]
fn program_names_are_single_safe_words() {
    for (source, message) in [
        (
            "use $bin::{\"fc-match\"}",
            "give the program \"fc-match\" a ush name",
        ),
        ("use $bin::{\"-rf\" as rf}", "invalid program name: \"-rf\""),
        ("use $bin::{\"a b\" as ab}", "invalid program name: \"a b\""),
        ("use $bin::{\"a;b\" as ab}", "invalid program name: \"a;b\""),
    ] {
        let error = compile_error(source);
        assert!(error.contains(message), "{source}\n=> {error}");
    }
}
