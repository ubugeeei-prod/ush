//! Config loading through the public surface: an explicit path wins
//! over the discovery chain, and every documented alias is accepted.

use std::fs;

use tempfile::tempdir;
use ush_config::{ShellKeymap, UshConfig};

fn load(contents: &str, extension: &str) -> UshConfig {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join(format!("config.{extension}"));
    fs::write(&path, contents).expect("write config");
    UshConfig::load(Some(&path)).expect("load config")
}

#[test]
fn an_explicit_json_config_is_loaded() {
    let config = load(
        r#"{ "shell": { "stylish_default": true, "history_size": 42 } }"#,
        "json",
    );

    assert!(config.shell.stylish_default);
    assert_eq!(config.shell.history_size, 42);
}

#[test]
fn camel_case_aliases_are_accepted() {
    let config = load(
        r#"
        {
          "shell": {
            "stylishDefault": true,
            "historySize": 7,
            "editMode": "vi",
            "profileFiles": ["p.sh"],
            "rcFiles": ["rc.sh"]
          }
        }
        "#,
        "json",
    );

    assert!(config.shell.stylish_default);
    assert_eq!(config.shell.history_size, 7);
    assert_eq!(config.shell.keymap, ShellKeymap::Vi);
    assert_eq!(config.shell.profile_files.len(), 1);
    assert_eq!(config.shell.rc_files.len(), 1);
}

#[test]
fn an_empty_config_keeps_every_default() {
    let config = load("{}", "json");
    let defaults = UshConfig::default();

    assert_eq!(config.shell.stylish_default, defaults.shell.stylish_default);
    assert_eq!(config.shell.interaction, defaults.shell.interaction);
    assert_eq!(config.shell.history_size, defaults.shell.history_size);
    assert_eq!(config.shell.keymap, defaults.shell.keymap);
    assert!(config.shell.prompt.is_none());
    assert!(config.aliases.is_empty());
}

#[test]
fn interaction_defaults_to_enabled_and_can_be_turned_off() {
    assert!(UshConfig::default().shell.interaction);
    assert!(
        !load(r#"{ "shell": { "interaction": false } }"#, "json")
            .shell
            .interaction
    );
}

#[test]
fn the_default_history_size_is_five_thousand() {
    assert_eq!(UshConfig::default().shell.history_size, 5_000);
}

#[test]
fn aliases_are_loaded_in_sorted_order() {
    let config = load(
        r#"{ "aliases": { "ll": "ls -la", "gs": "git status" } }"#,
        "json",
    );

    assert_eq!(
        config.aliases.keys().cloned().collect::<Vec<_>>(),
        vec!["gs".to_string(), "ll".to_string()]
    );
    assert_eq!(config.aliases["ll"], "ls -la");
}

#[test]
fn a_custom_prompt_is_carried_through() {
    let config = load(r#"{ "shell": { "prompt": "ush> " } }"#, "json");
    assert_eq!(config.shell.prompt.as_deref(), Some("ush> "));
}

#[test]
fn a_missing_explicit_config_falls_back_to_defaults() {
    let dir = tempdir().expect("tempdir");
    let config = UshConfig::load(Some(&dir.path().join("nope.json"))).expect("load");

    assert_eq!(config.shell.history_size, 5_000);
}

#[test]
fn malformed_json_names_the_file_it_could_not_load() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("config.json");
    fs::write(&path, "{ not json").expect("write");

    let error = UshConfig::load(Some(&path)).expect_err("malformed config");
    let message = format!("{error:#}");

    assert!(message.contains("failed to load config"), "{message}");
    assert!(message.contains("config.json"), "{message}");
}

#[test]
fn an_unknown_extension_is_rejected() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("config.yaml");
    fs::write(&path, "shell: {}").expect("write");

    let error = UshConfig::load(Some(&path)).expect_err("unsupported format");
    assert!(format!("{error:#}").contains("unsupported config format"));
}

#[test]
fn runtime_paths_point_at_an_existing_config_and_cache_directory() {
    let paths = UshConfig::runtime_paths().expect("runtime paths");

    assert!(paths.config_dir.is_dir());
    assert!(paths.cache_dir.is_dir());
    assert_eq!(paths.history_file.parent(), Some(paths.cache_dir.as_path()));
    assert_eq!(
        paths
            .history_file
            .file_name()
            .and_then(|name| name.to_str()),
        Some("history.txt")
    );
}
