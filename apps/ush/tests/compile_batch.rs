//! `ush compile` with several inputs, `--out-dir` and `--check`.

use std::{fs, path::Path, process::Command};

use tempfile::tempdir;

fn ush(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ush"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run ush compile")
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Two workflows in their own directories sharing one library module.
fn project() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    for (path, text) in [
        (
            "lib/gh.ush",
            "pub fn output(name: String) -> String {\n  \"out:\" + name\n}\n",
        ),
        (
            "w1/scripts/a.ush",
            "mod \"../../lib/gh.ush\"\nprint gh::output \"a\"\n",
        ),
        ("w2/scripts/b.ush", "print \"b\"\n"),
    ] {
        let path = dir.path().join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    dir
}

const INPUTS: [&str; 2] = ["w1/scripts/a.ush", "w2/scripts/b.ush"];

fn batch(dir: &Path, extra: &[&str]) -> std::process::Output {
    let mut args = vec!["compile", "--checked-in", "--out-dir", "compiled"];
    args.extend_from_slice(extra);
    args.extend(INPUTS);
    ush(dir, &args)
}

#[test]
fn out_dir_places_each_output_beside_its_input() {
    let dir = project();
    let result = batch(dir.path(), &[]);
    assert!(result.status.success(), "{}", stderr(&result));
    for (script, expected) in [
        ("w1/scripts/compiled/a.sh", "out:a\n"),
        ("w2/scripts/compiled/b.sh", "b\n"),
    ] {
        let run = Command::new("sh")
            .arg(dir.path().join(script))
            .current_dir("/")
            .output()
            .unwrap();
        assert_eq!(
            String::from_utf8_lossy(&run.stdout),
            expected,
            "{}",
            stderr(&run)
        );
    }
    // A per-file command writes the bytes the batch expects.
    let result = ush(
        dir.path(),
        &[
            "compile",
            "--checked-in",
            "-o",
            "w1/scripts/compiled/a.sh",
            "w1/scripts/a.ush",
        ],
    );
    assert!(result.status.success(), "{}", stderr(&result));
    assert!(batch(dir.path(), &["--check"]).status.success());
}

#[test]
fn check_reports_stale_and_missing_outputs_without_writing() {
    let dir = project();
    assert!(batch(dir.path(), &[]).status.success());
    let fresh = batch(dir.path(), &["--check"]);
    assert!(fresh.status.success(), "{}", stderr(&fresh));
    assert!(fresh.stderr.is_empty());

    fs::write(
        dir.path().join("lib/gh.ush"),
        "pub fn output(name: String) -> String {\n  \"OUT:\" + name\n}\n",
    )
    .unwrap();
    fs::remove_file(dir.path().join("w2/scripts/compiled/b.sh")).unwrap();
    let before = fs::read(dir.path().join("w1/scripts/compiled/a.sh")).unwrap();
    let stale = batch(dir.path(), &["--check"]);
    assert_eq!(stale.status.code(), Some(1));
    let message = stderr(&stale);
    assert!(
        message.contains("ush: w1/scripts/compiled/a.sh is stale"),
        "{message}"
    );
    assert!(
        message.contains("--- w1/scripts/compiled/a.sh (committed)"),
        "{message}"
    );
    assert!(
        message.contains("-  __ush_at 2 '\"out:\" + name'"),
        "{message}"
    );
    assert!(
        message.contains("+  __ush_at 2 '\"OUT:\" + name'"),
        "{message}"
    );
    assert!(
        message.contains("ush: w2/scripts/compiled/b.sh is missing"),
        "{message}"
    );
    assert!(message.contains("2 output(s) out of date"), "{message}");
    assert_eq!(
        fs::read(dir.path().join("w1/scripts/compiled/a.sh")).unwrap(),
        before
    );
    assert!(!dir.path().join("w2/scripts/compiled/b.sh").exists());
}

#[test]
fn one_broken_input_does_not_stop_the_others() {
    let dir = project();
    fs::write(dir.path().join("w2/scripts/b.ush"), "print missing\n").unwrap();
    let result = batch(dir.path(), &[]);
    assert_eq!(result.status.code(), Some(1));
    assert!(stderr(&result).contains("unknown variable: missing"));
    assert!(dir.path().join("w1/scripts/compiled/a.sh").exists());
}

#[test]
fn unchanged_outputs_are_not_rewritten() {
    let dir = project();
    assert!(batch(dir.path(), &[]).status.success());
    let output = dir.path().join("w2/scripts/compiled/b.sh");
    let stamp = fs::metadata(&output).unwrap().modified().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(20));
    assert!(batch(dir.path(), &[]).status.success());
    assert_eq!(fs::metadata(&output).unwrap().modified().unwrap(), stamp);
}

#[test]
fn incompatible_flags_are_explained() {
    let dir = project();
    for (args, message) in [
        (
            vec!["compile", "w1/scripts/a.ush", "w2/scripts/b.ush"],
            "several inputs needs --out-dir",
        ),
        (
            vec!["compile", "--check", "w2/scripts/b.ush"],
            "--check needs -o FILE or --out-dir",
        ),
        (
            vec!["compile", "--checked-in", "w2/scripts/b.ush"],
            "--checked-in needs -o FILE or --out-dir",
        ),
        (
            vec![
                "compile",
                "--out-dir",
                "/tmp/one",
                "w1/scripts/a.ush",
                "w1/scripts/a.ush",
            ],
            "would both write",
        ),
    ] {
        let result = ush(dir.path(), &args);
        assert!(!result.status.success());
        assert!(
            stderr(&result).contains(message),
            "{args:?}: {}",
            stderr(&result)
        );
    }
}
