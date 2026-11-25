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
fn unit_values_can_be_returned_and_compared() {
    let output = run_program(
        r#"
        fn noop() -> () {
          ()
        }
        let value = noop ()
        print value == ()
        print value <= ()
    "#,
    );

    assert_eq!(output, "true\ntrue\n");
}

#[test]
fn primitive_comparisons_cover_eq_and_ord() {
    let output = run_program(
        r#"
        print 1 == 1
        print 1 < 2
        print "ant" < "bee"
        print true > false
    "#,
    );

    assert_eq!(output, "true\ntrue\ntrue\ntrue\n");
}

#[test]
fn trait_and_impl_declarations_compile_with_builtin_traits() {
    let output = run_program(
        r#"
        trait Named {}
        enum Token {
          Value(String),
          Empty,
        }
        impl Named for Token {}
        impl Eq for () {}
        impl Ord for String {}
        impl Add for Int {}
        print 1 + 2
    "#,
    );

    assert_eq!(output, "3\n");
}

#[test]
fn display_trait_and_format_work_for_type_structs() {
    let output = run_program(
        r#"
        type User {
          name: String,
          age: Int,
        }
        impl Display for User {
          fn fmt(self) -> String {
            self.name + ":" + self.age
          }
        }
        let user = User { name: "ush", age: 7 }
        print user
        print format $user
    "#,
    );

    assert_eq!(output, "ush:7\nush:7\n");
}

#[test]
fn inherent_methods_can_use_self_and_arguments() {
    let output = run_program(
        r#"
        type User {
          name: String,
        }
        impl User {
          fn prefixed(self, prefix: String) -> String {
            prefix + ":" + self.name
          }
        }
        let user = User { name: "ush" }
        print user.prefixed("id")
    "#,
    );

    assert_eq!(output, "id:ush\n");
}
