use std::{
    fs,
    io::Write,
    process::{Command, Stdio},
};
use tempfile::tempdir;

fn ush() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ush"))
}

#[test]
fn raw_shell_keeps_stdin_and_heredoc_layout_inside_a_function() {
    let dir = tempdir().unwrap();
    let script = dir.path().join("interop.ush");
    let source = r#"fn read_body() -> String / unknown, stdin, stderr, exec, fs, env, net {
  let text = $(
IFS= read -r input
cat <<EOF_BODY
$input
  second line ($input)
EOF_BODY
  )?
  text
}
print read_body()?
"#;
    fs::write(&script, source).unwrap();
    let mut child = ush()
        .arg(&script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"a b\n").unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"a b\n  second line (a b)\n");
}

#[test]
fn checked_in_task_failure_points_to_source_even_after_relocation() {
    let dir = tempdir().unwrap();
    let tmp = dir.path().join("task storage");
    fs::create_dir(&tmp).unwrap();
    let src = dir.path().join("build.ush");
    let sh = dir.path().join("build.sh");
    fs::write(&src,"fn build() -> String / unknown, stderr {\n  fail(23, \"missing input\")\n}\nlet job = async build()\nlet value = job.await\nprint value\n").unwrap();
    let result = ush()
        .arg("compile")
        .arg(&src)
        .arg("--checked-in")
        .arg("-o")
        .arg(&sh)
        .output()
        .unwrap();
    assert!(result.status.success(), "{:?}", result.stderr);
    fs::remove_file(src).unwrap();
    let output = Command::new("sh")
        .arg(sh)
        .env("TMPDIR", &tmp)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(23));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(
        message.contains("build.ush:2: missing input (exit 23)"),
        "{message}"
    );
    assert!(!message.contains("__ush_"), "{message}");
    assert!(fs::read_dir(tmp).unwrap().next().is_none());
}

#[test]
fn imported_commands_preserve_argv_and_parent_shell_state() {
    let dir = tempdir().unwrap();
    fs::create_dir(dir.path().join("two words")).unwrap();
    let src = dir.path().join("commands.ush");
    fs::write(
        &src,
        r#"using $bin::
fn echo(value: String) -> String {
  "local:" + value
}
cd("two words")?
let wd = $bin::pwd()?
print wd
print echo("ok")
let job = async printf("%s", "$(other) ' *")
let first = job.await
let again = job.await
print first
print again
"#,
    )
    .unwrap();
    let output = ush().arg(src).current_dir(dir.path()).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(
        text.ends_with("/two words\nlocal:ok\n$(other) ' *\n$(other) ' *\n"),
        "{text}"
    );
}

#[test]
fn external_bin_arguments_obey_the_same_integer_bounds_as_typed_calls() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("bin.ush");
    fs::write(&src,"unit ms\nfn bin(delay: Int<ms> where delay > 0 && delay < 10) -> () / stdout {\n  print delay\n}\n").unwrap();
    let run = |arg: &str| ush().arg(&src).arg(arg).output().unwrap();
    let output = run("008");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"8\n");
    for invalid in ["0", "10", "ten", "9999999999999999999999"] {
        assert_eq!(run(invalid).status.code(), Some(2));
    }
}
