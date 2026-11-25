use std::{
    fs,
    io::Write,
    process::{Command, Output, Stdio},
};
use tempfile::tempdir;
use ush_compiler::{ShellTarget, UshCompiler};

fn each_shell(mut check: impl FnMut(&str, ShellTarget)) {
    let mut ran = 0;
    for (shell, target) in [
        ("sh", ShellTarget::Sh),
        ("dash", ShellTarget::Sh),
        ("bash", ShellTarget::Bash),
        ("zsh", ShellTarget::Zsh),
    ] {
        if Command::new(shell).arg("-c").arg(":").status().is_ok() {
            ran += 1;
            check(shell, target);
        }
    }
    assert!(ran > 0);
}

fn run(shell: &str, target: ShellTarget, source: &str, input: &[u8]) -> Output {
    let dir = tempdir().unwrap();
    let script = dir.path().join("program.sh");
    let compiler = UshCompiler::for_target(target);
    let cases = compiler.test_cases_source(source).unwrap();
    let generated = match cases.as_slice() {
        [] => compiler.compile_source(source),
        [case] => {
            let input = dir.path().join("test.ush");
            fs::write(&input, source).unwrap();
            compiler.compile_test_file(&input, case.line)
        }
        _ => panic!("fixture runner expects one named test per source"),
    }
    .unwrap_or_else(|error| panic!("{error:#}"));
    assert!(
        !generated.contains("__runtime"),
        "portable code must stand alone"
    );
    fs::write(&script, generated).unwrap();
    let mut child = Command::new(shell)
        .arg(script)
        .current_dir(dir.path())
        .env("USH_RUNTIME", "/missing/runtime")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn portable_apis_work_across_shells_without_a_runtime() {
    for source in [
        include_str!("fixtures/portable/strings.ush"),
        include_str!("fixtures/portable/filesystem.ush"),
        include_str!("fixtures/portable/paths.ush"),
    ] {
        assert_eq!(
            UshCompiler::default()
                .test_cases_source(source)
                .unwrap()
                .len(),
            1
        );
        each_shell(|shell, target| {
            let output = run(shell, target, source, b"");
            assert!(
                output.status.success(),
                "{shell}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert_eq!(output.stdout, b"passed\n", "{shell}");
        });
    }
}

#[test]
fn stdin_readers_preserve_empty_lines_and_unterminated_records() {
    each_shell(|shell, target| {
        for (source, input) in [
            (
                r#"std::test::assert_eq std::io::read_lines ["a\r", "", "b\\c"]"#,
                b"a\r\n\nb\\c".as_slice(),
            ),
            (
                r#"std::test::assert_eq std::io::read_all "a\n\nb""#,
                b"a\n\nb\n\n".as_slice(),
            ),
            (
                r#"std::test::assert_eq (std::string::join std::io::read_lines ",") """#,
                b"".as_slice(),
            ),
        ] {
            let output = run(shell, target, source, input);
            assert!(
                output.status.success(),
                "{shell}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    });
}

#[test]
fn invalid_arguments_stop_at_the_call_with_a_useful_diagnostic() {
    each_shell(|shell, target| {
        for (call, message) in [
            (r#"std::string::repeat "x" -1"#, "count must be nonnegative"),
            (r#"std::string::count "x" """#, "part must not be empty"),
            ("std::int::abs -9223372036854775808", "integer overflow"),
            ("std::int::clamp 1 10 0", "min must not exceed max"),
            ("std::terminal::is_tty -1", "descriptor must be nonnegative"),
            ("std::process::sleep -1", "seconds must be nonnegative"),
            (r#"std::fs::read_dir """#, "path must not be empty"),
            (r#"std::fs::read_dir "missing""#, "cannot read directory"),
            (r#"std::fs::chmod "missing" "u+x""#, "octal digits"),
        ] {
            let output = run(
                shell,
                target,
                &format!("{call}\nprint \"unreachable\"\n"),
                b"",
            );
            assert!(!output.status.success(), "{shell}: {call}");
            assert!(!String::from_utf8_lossy(&output.stdout).contains("unreachable"));
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(stderr.contains(message), "{shell}: {stderr}");
            assert!(stderr.contains("<memory>:1:"), "{shell}: {stderr}");
        }
    });
}
