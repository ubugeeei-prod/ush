use std::{fs, path::PathBuf};

use tempfile::tempdir;
use ush_config::UshConfig;

use crate::{SessionStartup, Shell, ShellOptions};

fn new_shell() -> Shell {
    Shell::new(
        UshConfig::default(),
        ShellOptions {
            stylish: false,
            interaction: false,
            print_ast: false,
        },
    )
    .expect("shell")
}

#[test]
fn explicit_startup_files_load_before_command_execution() {
    let dir = tempdir().expect("tempdir");
    let profile = dir.path().join("profile.sh");
    let rc = dir.path().join("rc.sh");
    fs::write(&profile, "export PROFILE_LOADED=profile\n").expect("write profile");
    fs::write(&rc, "alias ll='echo rc-loaded'\n").expect("write rc");

    let mut shell = new_shell();
    shell
        .load_session_startup(&SessionStartup {
            load_env: false,
            env_file: None,
            load_system_profile: false,
            load_profile: true,
            load_rc: true,
            profile_file: Some(profile),
            rc_file: Some(rc),
        })
        .expect("load startup");

    assert_eq!(
        shell.env.get("PROFILE_LOADED"),
        Some(&"profile".to_string())
    );
    assert_eq!(shell.aliases.get("ll"), Some(&"echo rc-loaded".to_string()));
}

#[test]
fn config_relative_startup_paths_resolve_from_config_dir() {
    let dir = tempdir().expect("tempdir");
    let config_dir = dir.path().join("config");
    fs::create_dir_all(&config_dir).expect("mkdir config");
    fs::write(config_dir.join("team.rc"), "export TEAM_RC=loaded\n").expect("write rc");

    let mut config = UshConfig::default();
    config.shell.rc_files.push(PathBuf::from("team.rc"));
    let mut shell = Shell::new(
        config,
        ShellOptions {
            stylish: false,
            interaction: false,
            print_ast: false,
        },
    )
    .expect("shell");
    shell.paths.config_dir = config_dir;

    shell
        .load_session_startup(&SessionStartup {
            load_env: false,
            env_file: None,
            load_system_profile: false,
            load_profile: false,
            load_rc: true,
            profile_file: None,
            rc_file: None,
        })
        .expect("load rc");

    assert_eq!(shell.env.get("TEAM_RC"), Some(&"loaded".to_string()));
}

#[test]
fn default_candidates_load_from_config_dir_and_home() {
    let dir = tempdir().expect("tempdir");
    let config_dir = dir.path().join("config");
    let home_dir = dir.path().join("home");
    fs::create_dir_all(&config_dir).expect("mkdir config");
    fs::create_dir_all(&home_dir).expect("mkdir home");
    fs::write(
        config_dir.join("profile.sh"),
        "export PROFILE_ORDER=config\n",
    )
    .expect("write config profile");
    fs::write(home_dir.join(".ush_profile"), "export PROFILE_ORDER=home\n")
        .expect("write home profile");
    fs::write(config_dir.join("rc.sh"), "alias ll='echo legacy-config'\n")
        .expect("write legacy config rc");
    fs::write(home_dir.join(".ushrc"), "alias ll='echo legacy-home'\n")
        .expect("write legacy home rc");
    fs::write(
        config_dir.join(".config.ush"),
        "export RC_ORDER=config-dotfile\nalias ll='echo config-dotfile'\n",
    )
    .expect("write config dotfile");
    fs::write(
        home_dir.join(".config.ush"),
        "export RC_ORDER=home-dotfile\nalias ll='echo home-dotfile'\n",
    )
    .expect("write home dotfile");

    let mut shell = new_shell();
    shell.paths.config_dir = config_dir;
    shell
        .env
        .insert("HOME".to_string(), home_dir.display().to_string());

    shell
        .load_session_startup(&SessionStartup {
            load_env: false,
            env_file: None,
            load_system_profile: false,
            load_profile: true,
            load_rc: true,
            profile_file: None,
            rc_file: None,
        })
        .expect("load startup");

    assert_eq!(shell.env.get("PROFILE_ORDER"), Some(&"home".to_string()));
    assert_eq!(
        shell.env.get("RC_ORDER"),
        Some(&"config-dotfile".to_string())
    );
    assert_eq!(
        shell.aliases.get("ll"),
        Some(&"echo config-dotfile".to_string())
    );
}

#[test]
fn missing_explicit_startup_file_is_reported() {
    let mut shell = new_shell();
    let error = shell
        .load_session_startup(&SessionStartup {
            load_env: false,
            env_file: None,
            load_system_profile: false,
            load_profile: true,
            load_rc: false,
            profile_file: Some(PathBuf::from("missing-profile.sh")),
            rc_file: None,
        })
        .expect_err("missing file should fail");

    assert!(error.to_string().contains("missing profile file"));
}
