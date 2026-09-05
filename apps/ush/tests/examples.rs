use std::{fs, path::PathBuf, process::Command};

use ush_compiler::UshCompiler;

fn example_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples")
}

fn ush() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ush"))
}

fn example_path(name: &str) -> PathBuf {
    example_dir().join(name)
}

fn ush_examples() -> Vec<PathBuf> {
    let mut paths = fs::read_dir(example_dir())
        .expect("read examples")
        .filter_map(|entry| entry.ok().map(|item| item.path()))
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("ush"))
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

#[test]
fn all_example_scripts_compile() {
    let compiler = UshCompiler;

    for path in ush_examples() {
        compiler
            .compile_file(&path)
            .unwrap_or_else(|error| panic!("failed to compile {}: {error:#}", path.display()));
    }
}

#[test]
fn runnable_examples_exit_successfully() {
    let cases: &[(&str, &[&str])] = &[
        ("adt.ush", &[]),
        ("alias.ush", &[]),
        ("async.ush", &[]),
        ("async_block.ush", &[]),
        ("bin.ush", &["--name", "ush", "--verbose"]),
        ("bin_defaults.ush", &["--target", "prod", "--verbose"]),
        ("control_flow.ush", &[]),
        ("docs.ush", &[]),
        ("effects.ush", &[]),
        ("functional.ush", &[]),
        ("hello.ush", &[]),
        ("http_regex.ush", &[]),
        ("literal_match.ush", &[]),
        ("multiline_string.ush", &[]),
        ("named_args.ush", &[]),
        ("option.ush", &[]),
        ("primitives.ush", &[]),
        ("response.ush", &[]),
        ("shell_string.ush", &[]),
        ("smoke_test.ush", &[]),
        ("std_modules.ush", &[]),
        ("task_fanout.ush", &[]),
        ("task_math.ush", &[]),
        ("traits.ush", &[]),
        ("type.ush", &[]),
        ("unit.ush", &[]),
        ("zero_arg.ush", &[]),
    ];

    for (name, args) in cases {
        assert_success(name, args);
    }
}

fn assert_success(name: &str, args: &[&str]) {
    let path = example_path(name);
    let output = ush()
        .arg(&path)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("failed to run {}: {error}", path.display()));

    assert!(
        output.status.success(),
        "{} failed\nstdout:\n{}\nstderr:\n{}",
        path.display(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn example_catalog_mentions_new_scripts() {
    let catalog = fs::read_to_string(example_path("README.md")).expect("read catalog");

    for name in [
        "async_block.ush",
        "control_flow.ush",
        "http_regex.ush",
        "multiline_string.ush",
        "named_args.ush",
        "response.ush",
        "smoke_test.ush",
        "std_modules.ush",
        "task_fanout.ush",
        "bin_defaults.ush",
        "sample.json",
    ] {
        assert!(catalog.contains(name), "catalog should mention {name}");
    }
}
