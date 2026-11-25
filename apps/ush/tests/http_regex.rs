use std::fs;

use tempfile::tempdir;

fn ush() -> std::process::Command {
    std::process::Command::new(env!("CARGO_BIN_EXE_ush"))
}

#[test]
fn ush_scripts_support_std_http_helpers() {
    let dir = tempdir().expect("tempdir");
    let source_dir = dir.path().join("source");
    fs::create_dir_all(&source_dir).expect("create source dir");
    fs::write(source_dir.join("payload.txt"), "hello from http\n").expect("write payload");

    let script = source_dir.join("http.ush");
    fs::write(
        &script,
        r#"
        use std::http::{download, get}
        use std::path::{from_source, tmpfile}
        let source_root = from_source "."
        let payload = source_root.join("payload.txt").resolve()
        let url = "file://" + payload
        let copy = tmpfile()
        print $ get $url
        download $url $copy
        print $ copy.read_text()
        copy.remove()
        "#,
    )
    .expect("write script");

    let output = ush().arg(&script).output().expect("run ush");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "hello from http\nhello from http\n"
    );
}

#[test]
fn ush_scripts_support_std_regex_helpers() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("regex.ush");
    fs::write(
        &script,
        r#"
        use std::regex::{capture, find, is_match, replace as regex_replace}
        print $ is_match "module-ready" "^module"
        print $ find "ush-v0.3.4" "v[0-9.]+"
        print $ capture "release-v0.3.4" "v([0-9.]+)" 1
        print $ regex_replace "v0.3.4" "[.]" "-"
        print $ "feature-ready".is_match("^feature")
        print $ "release-v0.3.4".find("v[0-9.]+")
        print $ "release-v0.3.4".capture("v([0-9.]+)", 1)
        print $ "v0.3.4".replace_regex("[.]", "-")
        "#,
    )
    .expect("write script");

    let output = ush().arg(&script).output().expect("run ush");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "true\nv0.3.4\n0.3.4\nv0-3-4\ntrue\nv0.3.4\n0.3.4\nv0-3-4\n"
    );
}
