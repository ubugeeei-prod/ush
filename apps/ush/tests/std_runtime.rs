use std::{
    fs,
    os::unix::net::UnixListener,
    path::Path,
    process::{Command, Output},
};
use tempfile::tempdir;

const USH: &str = env!("CARGO_BIN_EXE_ush");

fn compile(root: &Path, source: &str, shell: &str) -> std::path::PathBuf {
    let input = root.join("runtime.ush");
    let output = root.join("runtime.sh");
    fs::write(&input, source).unwrap();
    let result = Command::new(USH)
        .args(["compile", "--target", shell])
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    output
}

fn run(shell: &str, source: &str, host: &str) -> Output {
    let dir = tempdir().unwrap();
    let _socket = UnixListener::bind(dir.path().join("socket")).unwrap();
    let script = compile(dir.path(), source, shell);
    Command::new(shell)
        .arg(script)
        .current_dir(dir.path())
        .env("USH_RUNTIME", host)
        .env("TZ", "Asia/Tokyo")
        .output()
        .unwrap()
}

fn each_shell(mut check: impl FnMut(&str)) {
    let mut ran = 0;
    for shell in ["sh", "bash", "zsh"] {
        if Command::new(shell).arg("-c").arg(":").status().is_ok() {
            ran += 1;
            check(shell);
        }
    }
    assert!(ran > 0);
}

#[test]
fn runtime_backed_values_and_filesystem_operations_work_on_every_shell() {
    each_shell(|shell| {
        for source in [
            include_str!("fixtures/runtime/values.ush"),
            include_str!("fixtures/runtime/filesystem.ush"),
        ] {
            let output = run(shell, source, USH);
            assert!(
                output.status.success(),
                "{shell}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert_eq!(output.stdout, b"passed\n", "{shell}");
        }
    });
}

#[test]
fn runtime_errors_are_readable_and_abort_at_the_source_call() {
    each_shell(|shell| {
        for (call, message) in [
            (r#"std::string::slice "abc" 2 1"#, "slice range"),
            (r#"std::bytes::from_hex "0""#, "hexadecimal"),
            (r#"std::bytes::decode_base64 "%%%""#, "invalid base64"),
            (r#"std::bytes::decode_base64 "Zh==""#, "invalid base64"),
            ("std::bytes::to_text [0]", "cannot contain NUL"),
            ("std::bytes::to_text [255]", "valid UTF-8"),
            ("std::bytes::to_hex [256]", "too large"),
            ("std::int::add 9223372036854775807 1", "integer overflow"),
            ("std::int::div 1 0", "division by zero"),
            ("std::int::pow 2 -1", "exponent"),
            ("std::random::int 2 1", "min must not exceed max"),
            ("std::random::bytes -1", "nonnegative index"),
            ("std::time::sleep_millis -1", "nonnegative"),
            (r#"std::time::format_utc 0 "%s""#, "unsupported time format"),
            (r#"std::fs::read_bytes "missing""#, "No such file"),
            (r#"std::fs::write_atomic "." "bad""#, "persist"),
        ] {
            let output = run(shell, &format!("{call}\nprint \"unreachable\"\n"), USH);
            assert!(!output.status.success(), "{shell}: {call}");
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(stderr.contains(message), "{shell}: {call}: {stderr}");
            assert!(stderr.contains("runtime.ush:1:"), "{shell}: {stderr}");
            assert!(!String::from_utf8_lossy(&output.stdout).contains("unreachable"));
            assert!(!stderr.contains("stack backtrace"), "{stderr}");
        }
    });
}

#[test]
fn missing_runtime_does_not_affect_portable_programs() {
    each_shell(|shell| {
        let output = run(
            shell,
            r#"print std::string::to_upper_ascii "abc""#,
            "/missing/ush",
        );
        assert!(output.status.success());
        assert_eq!(output.stdout, b"ABC\n");
        let output = run(shell, "print std::runtime::available", "/missing/ush");
        assert!(output.status.success());
        assert_eq!(output.stdout, b"false\n");
        let output = run(shell, r#"print std::string::length "abc""#, "/missing/ush");
        assert_eq!(output.status.code(), Some(127));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("std::string::length requires the ush runtime"));
        assert!(stderr.contains("USH_RUNTIME"));
        assert!(stderr.contains("runtime.ush:1:"));
    });
}

#[test]
fn runtime_protocol_rejects_invalid_versions_arities_and_operations() {
    for args in [
        vec!["v2", "ping"],
        vec!["v1", "ping", "extra"],
        vec!["v1", "std::missing::operation"],
        vec!["v1", "std::bytes::from_hex"],
    ] {
        let output = Command::new(USH)
            .arg("__runtime")
            .args(args)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(!String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }
}

#[test]
fn configured_runtime_paths_can_contain_spaces() {
    let dir = tempdir().unwrap();
    let host = dir.path().join("ush runtime");
    std::os::unix::fs::symlink(USH, &host).unwrap();
    let output = run(
        "sh",
        r#"print std::string::length "😀""#,
        host.to_str().unwrap(),
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"1\n");
}
