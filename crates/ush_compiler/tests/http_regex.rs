use std::{fs, process::Command};

use tempfile::tempdir;
use ush_compiler::UshCompiler;

#[test]
fn std_http_helpers_can_read_and_download_file_urls() {
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

    let compiled = UshCompiler::default()
        .compile_file(&script)
        .expect("compile file");
    let shell = dir.path().join("program.sh");
    fs::write(&shell, compiled).expect("write shell");

    let output = Command::new("/bin/sh")
        .arg(&shell)
        .output()
        .expect("run shell");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "hello from http\nhello from http\n"
    );
}

#[test]
fn std_regex_helpers_support_module_and_method_style_calls() {
    let compiled = UshCompiler::default()
        .compile_source(
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
        .expect("compile source");

    let dir = tempdir().expect("tempdir");
    let shell = dir.path().join("program.sh");
    fs::write(&shell, compiled).expect("write shell");

    let output = Command::new("/bin/sh")
        .arg(&shell)
        .output()
        .expect("run shell");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "true\nv0.3.4\n0.3.4\nv0-3-4\ntrue\nv0.3.4\n0.3.4\nv0-3-4\n"
    );
}
