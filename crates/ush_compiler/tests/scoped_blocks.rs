//! `cd` and `export` blocks under every installed shell: the change
//! applies to the body only, nested blocks compose, and early exits
//! keep their status and source location.

mod support;

use support::{each_shell, run_with};
use tempfile::tempdir;

fn each_run(body: &str, mut check: impl FnMut(&str, (i32, String, String))) {
    let source = format!("use $bin::*\nuse std::env::get_or\nuse std::path::basename\n{body}");
    each_shell(|shell, target| {
        let dir = tempdir().unwrap();
        std::fs::create_dir(dir.path().join("a b")).unwrap();
        check(shell, run_with(shell, target, dir.path(), &source));
    });
}

fn succeeds(body: &str, expected: &str) {
    each_run(body, |shell, (status, stdout, stderr)| {
        assert_eq!(status, 0, "{shell}: {stderr}");
        assert_eq!(stdout, expected, "{shell}");
    });
}

#[test]
fn cd_blocks_restore_the_directory_and_keep_outer_assignments() {
    succeeds(
        r#"let start = pwd
let mut inside = ""
cd "a b" {
  inside = basename pwd
}
print inside
print (pwd) == start
cd "a b"
print basename pwd
"#,
        "a b\ntrue\na b\n",
    );
}

#[test]
fn export_blocks_restore_values_and_the_unset_state() {
    succeeds(
        r#"export KEEP = "old"
export KEEP = "new", FRESH = "f" {
  sh "-c" "printf '%s %s\n' \"$KEEP\" \"$FRESH\""
}
sh "-c" "printf '%s %s\n' \"${KEEP-unset}\" \"${FRESH-unset}\""
print get_or "FRESH" "unset"
"#,
        "new f\nold unset\nunset\n",
    );
}

#[test]
fn blocks_nest_and_values_are_evaluated_before_any_binding() {
    succeeds(
        r#"let start = pwd
export A = "1" {
  cd "a b" {
    export A = "2", B = std::env::get "A" {
      sh "-c" "printf '%s %s %s\n' \"${PWD##*/}\" \"$A\" \"$B\""
    }
    print get_or "B" "unset"
  }
  print get_or "A" "unset"
}
print get_or "A" "unset"
print (pwd) == start
"#,
        "a b 2 1\nunset\n1\nunset\ntrue\n",
    );
}

#[test]
fn break_and_continue_restore_the_blocks_they_leave() {
    succeeds(
        r#"let start = pwd
for name in ["a", "b", "c"] {
  cd "a b" {
    if name == "b" {
      continue
    }
    export STAGE = name {
      if name == "c" {
        break
      }
      print basename pwd + " " + get_or "STAGE" "unset"
    }
  }
}
print get_or "STAGE" "unset"
print (pwd) == start
"#,
        "a b a\nunset\ntrue\n",
    );
}

#[test]
fn failures_inside_blocks_keep_their_status_and_line() {
    for (body, status, line) in [
        (
            "cd \"a b\" {\n  fail 3 \"stop\"\n}\n",
            Some(3),
            "<memory>:5: stop (exit 3)",
        ),
        (
            "export X = \"1\" {\n  sh \"-c\" \"exit 6\"?\n}\n",
            Some(6),
            "<memory>:5: command failed (exit 6)",
        ),
        (
            "fn f() -> () / unknown, env, fs, stderr {\n  cd \"a b\" {\n    fail 5 \"nope\"\n  }\n}\nf\n",
            Some(5),
            "<memory>:6: nope (exit 5)",
        ),
        (
            // `cd` itself chooses the status: 2 in dash, 1 elsewhere.
            "cd \"missing\" {\n  print \"unreachable\"\n}\n",
            None,
            "<memory>:4: command failed",
        ),
    ] {
        each_run(body, |shell, (actual, stdout, stderr)| {
            assert_ne!(actual, 0, "{shell}: {body}");
            assert!(
                status.is_none_or(|s| s == actual),
                "{shell}: {body}\n{stderr}"
            );
            assert!(!stdout.contains("unreachable"), "{shell}");
            assert!(stderr.contains(line), "{shell}: {stderr}");
        });
    }
}

#[test]
fn export_reaches_child_processes_for_the_rest_of_the_script() {
    succeeds(
        "export GREETING = \"hi\"\nsh \"-c\" \"echo $GREETING\"\n",
        "hi\n",
    );
}
