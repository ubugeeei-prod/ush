use super::*;

#[test]
fn ush_script_executes_via_sh() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("hello.ush");
    fs::write(
        &script,
        r#"
        let greeting = "hello"
        print greeting + " world"
        "#,
    )
    .expect("write script");

    let output = ush().arg(&script).output().expect("run ush");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "hello world\n");
}

#[test]
fn ush_script_supports_nested_enums() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("result.ush");
    fs::write(
        &script,
        r#"
        enum Result {
          Ok(String),
          Err(String),
        }
        enum Envelope {
          Wrap(Result),
          Missing,
        }
        let payload = Envelope::Wrap(Result::Ok("done"))
        match payload {
          Envelope::Wrap(Result::Ok(message)) => print message
          _ => print "fallback"
        }
        "#,
    )
    .expect("write script");

    let output = ush().arg(&script).output().expect("run ush");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "done\n");
}

#[test]
fn ush_script_supports_async_functions() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("async.ush");
    fs::write(
        &script,
        r#"
        fn worker(message: String) -> String {
          message
        }
        print "main"
        let task = async worker "worker"
        print "after"
        let result = task.await
        print result
        "#,
    )
    .expect("write script");

    let output = ush().arg(&script).output().expect("run ush");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "main\nafter\nworker\n"
    );
}

#[test]
fn ush_script_supports_functional_calls() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("functional.ush");
    fs::write(
        &script,
        r#"
        fn greet(name: String) -> String {
          "hi " + name
        }
        fn wrap(message: String) -> String {
          "<" + message + ">"
        }
        fn label() -> String {
          "ush"
        }
        print wrap (greet (label ()))
        "#,
    )
    .expect("write script");

    let output = ush().arg(&script).output().expect("run ush");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "<hi ush>\n");
}

#[test]
fn ush_script_supports_unit_and_trait_declarations() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("traits.ush");
    fs::write(
        &script,
        r#"
        trait Named {}
        impl Eq for () {}
        impl Add for Int {}
        fn noop() -> () {
          ()
        }
        let value = noop ()
        print value == ()
        print 1 + 2
        "#,
    )
    .expect("write script");

    let output = ush().arg(&script).output().expect("run ush");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "true\n3\n");
}

#[test]
fn ush_script_exposes_generated_docs() {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("docs.ush");
    fs::write(
        &script,
        r#"
        #| Demo script.
        #| @usage docs.ush --man greet
        #| Greet a user.
        #| @param name target user
        #| @return greeting text
        fn greet(name: String) -> String {
          "hi " + name
        }
        print $ greet "ush"
        "#,
    )
    .expect("write script");

    let help = ush()
        .args([script.to_str().unwrap(), "--help"])
        .output()
        .expect("run help");
    let man = ush()
        .args([script.to_str().unwrap(), "--man", "greet"])
        .output()
        .expect("run man");
    let complete = ush()
        .args([script.to_str().unwrap(), "--complete", "gr"])
        .output()
        .expect("run completion");

    assert!(help.status.success());
    assert!(help.stderr.is_empty());
    assert_snapshot(
        &fixture("script_help"),
        &String::from_utf8_lossy(&help.stdout),
    );
    assert!(man.status.success());
    assert!(man.stderr.is_empty());
    assert_snapshot(
        &fixture("script_man"),
        &String::from_utf8_lossy(&man.stdout),
    );
    assert!(complete.status.success());
    assert_eq!(String::from_utf8_lossy(&complete.stdout), "greet\n");
}
