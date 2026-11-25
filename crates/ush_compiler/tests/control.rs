use std::{fs, process::Command};

use tempfile::tempdir;
use ush_compiler::UshCompiler;

fn run_program(source: &str) -> String {
    let compiled = UshCompiler::default()
        .compile_source(source)
        .expect("compile ush program");
    let dir = tempdir().expect("tempdir");
    let script = dir.path().join("program.sh");
    fs::write(&script, compiled).expect("write script");

    let output = Command::new("/bin/sh")
        .arg(&script)
        .output()
        .expect("run compiled script");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn if_blocks_can_return_tail_values() {
    let output = run_program(
        r#"
        fn pick(flag: Bool) -> String {
          if flag {
            "yes"
          }
          else {
            "no"
          }
        }
        let value = pick true
        print value
    "#,
    );

    assert_eq!(output, "yes\n");
}

#[test]
fn for_in_supports_ranges_lists_and_tuples() {
    let output = run_program(
        r#"
        for item in 0..3 {
          print item
        }
        let items = [3, 4]
        for item in items {
          print item
        }
        let pair = (5, 6)
        for item in pair {
          print item
        }
    "#,
    );

    assert_eq!(output, "0\n1\n2\n3\n4\n5\n6\n");
}

#[test]
fn while_and_loop_support_break() {
    let output = run_program(
        r#"
        let mut count = 0
        while count < 3 {
          print count
          count = count + 1
          if count == 3 {
            break
          }
        }
        loop {
          print 9
          break
        }
    "#,
    );

    assert_eq!(output, "0\n1\n2\n9\n");
}

#[test]
fn if_let_conditions_can_bind_and_chain() {
    let output = run_program(
        r#"
        enum Option {
          None,
          Some(Int),
        }
        let maybe = Option::Some(7)
        if let Option::Some(it) = maybe && it == 7 {
          print it
        }
    "#,
    );

    assert_eq!(output, "7\n");
}

#[test]
fn else_branches_may_share_the_closing_brace_line() {
    let output = run_program(
        r#"
fn size(n: Int) -> String {
  if n > 5 {
    "big"
  } else if n > 2 {
    "mid"
  } else {
    "small"
  }
}
print size 9
print size 3
print size 1
if size 3 == "mid" {
  print "same line"
}
# A comment may sit between a branch and its `else`.
else {
  print "unreachable"
}
"#,
    );
    assert_eq!(output, "big\nmid\nsmall\nsame line\n");
}
