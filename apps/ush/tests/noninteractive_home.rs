use std::{fs, process::Command};
use tempfile::tempdir;

#[test]
fn noninteractive_commands_do_not_need_writable_user_directories() {
    let dir = tempdir().unwrap();
    let home = dir.path().join("not-a-directory");
    fs::write(&home, "unchanged").unwrap();
    for command in ["echo hello", "env HELLO=ush", "history"] {
        let output = Command::new(env!("CARGO_BIN_EXE_ush"))
            .env_clear()
            .env("HOME", &home)
            .args(["-s", "-c", command])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{command}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert_eq!(fs::read_to_string(home).unwrap(), "unchanged");
}

#[test]
fn reading_configuration_does_not_create_user_directories() {
    let dir = tempdir().unwrap();
    let home = dir.path().join("new-home");
    fs::create_dir(&home).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ush"))
        .env_clear()
        .env("HOME", &home)
        .args(["-c", "echo hello"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"hello\n");
    assert_eq!(fs::read_dir(home).unwrap().count(), 0);
}
