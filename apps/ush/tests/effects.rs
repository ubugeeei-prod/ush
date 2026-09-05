//! `ush effects`, effect rows, and the handlers that discharge them.

use std::fs;
use std::process::Command;

use tempfile::tempdir;

fn ush() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ush"))
}

fn write(source: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("script.ush");
    fs::write(&script, source).expect("write script");
    (dir, script)
}

#[test]
fn effects_lists_every_function_and_the_top_level() {
    let (_dir, script) = write(concat!(
        "fn slug(name: String) -> String {\n  name + \"-x\"\n}\n",
        "fn fetch(url: String) -> String {\n  std::http::get(url)\n}\n",
        "print fetch(\"u\") + slug(\"y\")\n",
    ));

    let output = ush()
        .args(["effects", script.to_str().expect("utf-8")])
        .output()
        .expect("run ush effects");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success(), "{stdout}");
    assert!(stdout.contains("fetch"), "{stdout}");
    assert!(stdout.contains("net"), "{stdout}");
    assert!(stdout.contains("slug"), "{stdout}");
    assert!(stdout.contains("pure"), "{stdout}");
    assert!(stdout.contains("(top level)"), "{stdout}");
}

#[test]
fn undeclared_hides_the_functions_that_already_have_a_row() {
    let (_dir, script) = write(concat!(
        "fn fetch(url: String) -> String / { net } {\n  std::http::get(url)\n}\n",
        "fn slug(name: String) -> String {\n  name + \"-x\"\n}\n",
        "print fetch(\"u\") + slug(\"y\")\n",
    ));

    let output = ush()
        .args(["effects", "--undeclared", script.to_str().expect("utf-8")])
        .output()
        .expect("run ush effects");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success(), "{stdout}");
    assert!(stdout.contains("slug"), "{stdout}");
    assert!(!stdout.contains("fetch"), "{stdout}");
}

#[test]
fn a_row_that_misses_an_effect_fails_ush_check() {
    let (_dir, script) = write(
        "fn fetch(url: String) -> String / { fs } {\n  std::http::get(url)\n}\nprint fetch(\"u\")\n",
    );

    let output = ush()
        .args(["check", script.to_str().expect("utf-8")])
        .output()
        .expect("run ush check");
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr.contains("needs `net`"), "{stderr}");
    assert!(stderr.contains("{ net }"), "{stderr}");
}

#[test]
fn an_empty_row_also_refuses_to_run() {
    let (_dir, script) =
        write("fn shout(name: String) -> () / {} {\n  print name\n}\nshout(\"x\")\n");

    let output = ush()
        .arg(script.to_str().expect("utf-8"))
        .output()
        .expect("run ush");

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("needs `io`"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

// --- user effects -----------------------------------------------------

const LOGGED: &str = concat!(
    "effect log(message: String) -> ()\n",
    "fn greet(name: String) -> String / { log } {\n",
    "  do log(\"greeting \" + name)\n",
    "  \"hello \" + name\n",
    "}\n",
);

#[test]
fn a_handler_runs_and_hands_control_back_to_the_body() {
    let (_dir, script) = write(&format!(
        "{LOGGED}try {{\n  print greet(\"ubu\")\n}} with log {{ (message) =>\n  print \"[log] \" + message\n}}\n"
    ));

    let output = ush()
        .arg(script.to_str().expect("utf-8"))
        .output()
        .expect("run ush");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    // The handler ran first, and `greet` then finished its own work.
    assert_eq!(stdout, "[log] greeting ubu\nhello ubu\n");
}

#[test]
fn a_handler_answers_with_a_value() {
    let (_dir, script) = write(concat!(
        "effect ask(question: String) -> String\n",
        "fn greet() -> String / { ask } {\n",
        "  let name = do ask(\"your name?\")\n",
        "  \"hello \" + name\n",
        "}\n",
        "try {\n  print greet()\n} with ask { (question) =>\n  \"ubu\"\n}\n",
    ));

    let output = ush()
        .arg(script.to_str().expect("utf-8"))
        .output()
        .expect("run ush");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "hello ubu\n");
}

#[test]
fn a_nested_handler_shadows_the_one_around_it() {
    let (_dir, script) = write(concat!(
        "effect log(message: String) -> ()\n",
        "fn work() -> () / { log } {\n  do log(\"inner\")\n}\n",
        "try {\n",
        "  do log(\"outer-a\")\n",
        "  try {\n    work()\n  } with log { (m) =>\n    print \"[inner] \" + m\n  }\n",
        "  do log(\"outer-b\")\n",
        "} with log { (m) =>\n  print \"[outer] \" + m\n}\n",
    ));

    let output = ush()
        .arg(script.to_str().expect("utf-8"))
        .output()
        .expect("run ush");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "[outer] outer-a\n[inner] inner\n[outer] outer-b\n"
    );
}

#[test]
fn an_effect_that_is_never_handled_is_a_compile_error() {
    let (_dir, script) = write(&format!("{LOGGED}print greet(\"ubu\")\n"));

    let output = ush()
        .args(["check", script.to_str().expect("utf-8")])
        .output()
        .expect("run ush check");
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr.contains("never handled"), "{stderr}");
}

#[test]
fn user_effects_show_up_in_the_report() {
    let (_dir, script) = write(&format!(
        "{LOGGED}try {{\n  print greet(\"ubu\")\n}} with log {{ (message) =>\n  print message\n}}\n"
    ));

    let output = ush()
        .args(["effects", script.to_str().expect("utf-8")])
        .output()
        .expect("run ush effects");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("greet"), "{stdout}");
    assert!(stdout.contains("log"), "{stdout}");
    // The handler discharged it, so the program itself does not need it.
    let top = stdout
        .lines()
        .find(|line| line.contains("(top level)"))
        .expect("top level row");
    assert!(!top.contains("log"), "{top}");
}
