//! Session-shape tests: how `ush` behaves as *the* shell an editor
//! terminal, a terminal emulator, or an agent runner starts.
//!
//! Every case here is a shape that used to fail outright — `-i` was
//! rejected, a login shell started with `-ush` never read a profile,
//! `PATH` exported from a startup file was ignored by command
//! lookup, and `a && b` ran in a child `/bin/sh` where `ush`
//! builtins do not exist.

use std::fs;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn ush() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ush"));
    // Keep the developer's own startup files out of the picture.
    command.args(["--no-env", "--no-rc"]);
    command
}

fn write_tool(dir: &Path, name: &str) {
    let path = dir.join(name);
    fs::write(&path, "#!/bin/sh\necho tool-ran\n").expect("write tool");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("chmod");
    }
}

fn stdout_of(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn accepts_the_posix_interactive_flag() {
    let output = ush()
        .args(["-i", "-c", "echo ok"])
        .output()
        .expect("run ush");

    assert!(
        output.status.success(),
        "ush -i -c should run, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(stdout_of(&output), "ok\n");
}

#[test]
fn a_dash_prefixed_argv0_starts_a_login_shell() {
    let dir = tempdir().expect("tempdir");
    let profile = dir.path().join("profile.sh");
    fs::write(&profile, "export LOGIN_MARKER=yes\n").expect("write profile");

    let output = Command::new(env!("CARGO_BIN_EXE_ush"))
        .arg0("-ush")
        .args([
            "--no-env",
            "--no-rc",
            "--profile-file",
            profile.to_str().expect("utf-8"),
            "-c",
            "echo $LOGIN_MARKER",
        ])
        .output()
        .expect("run ush");

    assert_eq!(stdout_of(&output), "yes\n");
}

#[test]
fn the_env_file_is_read_even_for_a_bare_dash_c() {
    let dir = tempdir().expect("tempdir");
    let bin = dir.path().join("bin");
    fs::create_dir_all(&bin).expect("mkdir bin");
    write_tool(&bin, "ush-session-test-tool");

    let env_file = dir.path().join("env.sh");
    fs::write(
        &env_file,
        format!("export PATH=\"{}:$PATH\"\n", bin.display()),
    )
    .expect("write env file");

    let output = Command::new(env!("CARGO_BIN_EXE_ush"))
        .args([
            "--no-rc",
            "--env-file",
            env_file.to_str().expect("utf-8"),
            "-c",
            "ush-session-test-tool",
        ])
        .output()
        .expect("run ush");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(stdout_of(&output), "tool-ran\n");
}

#[test]
fn command_lookup_follows_a_path_exported_at_runtime() {
    let dir = tempdir().expect("tempdir");
    let bin = dir.path().join("bin");
    fs::create_dir_all(&bin).expect("mkdir bin");
    write_tool(&bin, "ush-runtime-path-tool");

    let output = ush()
        .args([
            "-c",
            &format!(
                "export PATH=\"{}:$PATH\"\nush-runtime-path-tool",
                bin.display()
            ),
        ])
        .output()
        .expect("run ush");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(stdout_of(&output), "tool-ran\n");
}

#[test]
fn and_or_lists_still_reach_ush_builtins() {
    let dir = tempdir().expect("tempdir");
    let nested = dir.path().join("nested");
    fs::create_dir_all(&nested).expect("mkdir nested");

    // `cd` is a builtin: run through `/bin/sh` it changes a child's
    // directory and the following `pwd` reports the old one.
    let output = ush()
        .args(["-c", "cd nested && pwd"])
        .current_dir(dir.path())
        .output()
        .expect("run ush");

    assert!(output.status.success());
    // The process starts in the resolved directory, so compare
    // against the resolved one — `/tmp` is a symlink on macOS.
    let expected = fs::canonicalize(&nested).expect("canonicalize");
    assert_eq!(
        stdout_of(&output).trim_end(),
        expected.display().to_string()
    );
}

#[test]
fn and_or_lists_short_circuit_like_posix() {
    let output = ush()
        .args(["-c", "false && echo no || echo yes"])
        .output()
        .expect("run ush");

    assert!(output.status.success());
    assert_eq!(stdout_of(&output), "yes\n");
}

#[test]
fn a_multiline_dash_c_runs_every_line() {
    let output = ush()
        .args(["-c", "echo one\necho two\necho three"])
        .output()
        .expect("run ush");

    assert!(output.status.success());
    assert_eq!(stdout_of(&output), "one\ntwo\nthree\n");
}

#[test]
fn cd_keeps_the_logical_path() {
    // macOS resolves `/tmp` to `/private/tmp`. POSIX `cd` is logical,
    // so `pwd` has to answer with the path that was asked for.
    let output = ush()
        .args(["-c", "cd /tmp\npwd"])
        .output()
        .expect("run ush");

    assert!(output.status.success());
    assert_eq!(stdout_of(&output), "/tmp\n");
}
