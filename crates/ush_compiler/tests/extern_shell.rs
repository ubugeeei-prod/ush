use std::{fs, process::Command};
use tempfile::tempdir;
use ush_compiler::UshCompiler;

#[test]
fn extern_modules_are_source_relative_typed_and_relocatable() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("helper's library.sh"),
        r#"
greet() { printf 'hello %s' "$1"; }
answer() { printf 42; }
truth() { printf true; }
action() { printf 'action:%s\n' "$1"; }
"#,
    )
    .unwrap();
    let path = dir.path().join("main.ush");
    fs::write(
        &path,
        r#"
extern "helper's library.sh" as helpers {
  fn greet(name: String) -> String
  fn answer() -> Int
  fn truth() -> Bool
  fn action(value: String)
}
use helpers::*
print greet "a b'$;*"
print helpers::answer + 1
print truth
helpers::action "done"
"#,
    )
    .unwrap();
    let generated = dir.path().join("main.sh");
    let shell = UshCompiler
        .compile_checked_in_file(&path, &generated)
        .unwrap();
    assert!(!shell.contains(&dir.path().display().to_string()));
    fs::write(&generated, shell).unwrap();
    let output = Command::new("sh")
        .arg(&generated)
        .current_dir("/")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "hello a b'$;*\n43\ntrue\naction:done\n"
    );
}

#[test]
fn extern_failures_and_invalid_values_fail_at_the_call_site() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("main.ush");
    for (body, ty, expected) in [
        ("return 7", "String", 7),
        ("printf invalid", "Int", 1),
        ("printf 1", "Bool", 1),
    ] {
        fs::write(
            dir.path().join("helpers.sh"),
            format!("value() {{ {body}; }}\n"),
        )
        .unwrap();
        fs::write(&path, format!("extern \"helpers.sh\" {{\n fn value() -> {ty}\n}}\nprint value\nprint \"unreachable\"\n")).unwrap();
        let shell = UshCompiler.compile_file(&path).unwrap();
        let generated = dir.path().join("main.sh");
        fs::write(&generated, shell).unwrap();
        let output = Command::new("sh").arg(&generated).output().unwrap();
        assert_eq!(
            output.status.code(),
            Some(expected),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(!String::from_utf8_lossy(&output.stdout).contains("unreachable"));
    }
}
