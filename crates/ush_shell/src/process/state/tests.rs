use std::{
    collections::BTreeMap,
    process::{Command, Stdio},
};

use tempfile::tempdir;
use ush_config::UshConfig;

use crate::ShellOptions;

use super::{Shell, StatefulShellRun, render_alias_prelude, shell_quote};

#[test]
fn fallback_updates_current_directory_environment_and_aliases() {
    let mut shell = Shell::new(
        UshConfig::default(),
        ShellOptions {
            stylish: false,
            interaction: false,
            print_ast: false,
        },
    )
    .expect("shell");
    let dir = tempdir().expect("tempdir");
    let path = dir.path().to_str().expect("utf8 path");
    let state = StatefulShellRun::new(
        &format!(
            "cd {} && export FOO=bar && alias ll='ls -la' && true",
            shell_quote(path)
        ),
        &shell.aliases,
    )
    .expect("state");
    let mut command = Command::new("/bin/sh");
    command
        .arg(state.runner_path())
        .current_dir(&shell.cwd)
        .envs(&shell.env)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit());
    state.populate_command_env(&mut command);

    let status = command.status().expect("run fallback");
    assert!(status.success());
    state.apply(&mut shell).expect("apply state");

    assert_eq!(shell.cwd, dir.path());
    assert_eq!(shell.env.get("FOO"), Some(&"bar".to_string()));
    assert_eq!(shell.aliases.get("ll"), Some(&"ls -la".to_string()));
}

#[test]
fn alias_values_are_single_quoted_for_the_fallback_shell() {
    assert_eq!(shell_quote("ls -la"), "'ls -la'");
    assert_eq!(shell_quote(""), "''");
}

#[test]
fn embedded_single_quotes_are_escaped() {
    assert_eq!(shell_quote("echo 'hi'"), r#"'echo '\''hi'\'''"#);
}

#[test]
fn shell_metacharacters_are_kept_literal() {
    assert_eq!(shell_quote("a $HOME `id` b"), "'a $HOME `id` b'");
}

#[test]
fn the_alias_prelude_declares_one_alias_per_line() {
    let aliases = BTreeMap::from([
        ("gs".to_string(), "git status".to_string()),
        ("ll".to_string(), "ls -la".to_string()),
    ]);

    assert_eq!(
        render_alias_prelude(&aliases),
        "alias gs='git status'\nalias ll='ls -la'\n"
    );
}

#[test]
fn an_empty_alias_table_renders_an_empty_prelude() {
    assert_eq!(render_alias_prelude(&BTreeMap::new()), "");
}

#[test]
fn an_incomplete_run_leaves_the_shell_untouched() {
    let mut shell = Shell::new(
        UshConfig::default(),
        ShellOptions {
            stylish: false,
            interaction: false,
            print_ast: false,
        },
    )
    .expect("shell");
    let before = shell.cwd.clone();
    // The runner was never executed, so no snapshot marker exists.
    let state = StatefulShellRun::new("true", &shell.aliases).expect("state");
    state.apply(&mut shell).expect("apply");

    assert_eq!(shell.cwd, before);
}
