use std::{fs, process::Command};
use tempfile::tempdir;
use ush_compiler::{ShellTarget, UshCompiler};

#[test]
fn terminal_queries_see_stdout_before_capturing_the_result() {
    let mut ran = 0;
    for (shell, target) in [
        ("sh", ShellTarget::Sh),
        ("bash", ShellTarget::Bash),
        ("zsh", ShellTarget::Zsh),
    ] {
        if Command::new(shell).arg("-c").arg(":").status().is_err() {
            continue;
        }
        ran += 1;
        let dir = tempdir().unwrap();
        let script = dir.path().join("terminal.sh");
        let source = r#"
use std::terminal::is_tty
use std::test::{assert, assert_eq}
fn nested() -> Bool / unknown, env {
  is_tty 1
}
fn default_value(#[default(std::terminal::is_tty(1))] value: Bool) -> Bool / unknown, env {
  value
}
assert (is_tty 1)
assert nested
assert default_value
assert_eq (is_tty 0) false
assert_eq (is_tty 2) false
print "passed"
"#;
        fs::write(
            &script,
            UshCompiler::for_target(target)
                .compile_source(source)
                .unwrap(),
        )
        .unwrap();
        let output = Command::new("python3")
            .arg("-c")
            .arg(
                r#"
import os, pty, select, subprocess, sys
master, slave = pty.openpty()
try:
    result = subprocess.run([sys.argv[1], sys.argv[2]], stdin=subprocess.DEVNULL,
                            stdout=slave, stderr=subprocess.PIPE, timeout=20)
    assert result.returncode == 0, result.stderr.decode()
    assert select.select([master], [], [], 5)[0], "no terminal output"
    output = os.read(master, 4096)
    assert output.replace(b'\r\n', b'\n') == b'passed\n', output
finally:
    if slave is not None: os.close(slave)
    os.close(master)
"#,
            )
            .arg(shell)
            .arg(&script)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{shell}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert!(ran > 0);
}
