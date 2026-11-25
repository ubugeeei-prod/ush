//! `defer` under every installed shell: deferred blocks run last
//! registered first when their function returns or the script exits,
//! including on failure and on a handled signal, before unfinished tasks
//! are joined and temporary directories removed.

mod support;

use std::{
    fs, os::unix::process::CommandExt, path::Path, process::Command, thread, time::Duration,
};

use support::each_program;
use ush_compiler::UshCompiler;

const PRELUDE: &str = "use $bin::*\n";

fn each_run(body: &str, check: impl FnMut(&str, &Path, (i32, String, String))) {
    each_program(&format!("{PRELUDE}{body}"), check);
}

#[test]
fn deferred_blocks_run_last_first_at_exit() {
    each_run(
        r#"defer print "first registered"
defer {
  print "second registered"
}
if true {
  defer print "conditional"
}
print "body"
"#,
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(
                stdout, "body\nconditional\nsecond registered\nfirst registered\n",
                "{shell}"
            );
        },
    );
}

#[test]
fn a_failing_script_runs_its_deferred_blocks_and_keeps_its_status() {
    each_run(
        r#"defer print "cleanup"
sh "-c" "exit 3"
print "unreachable"
"#,
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 3, "{shell}: {stderr}");
            assert_eq!(stdout, "cleanup\n", "{shell}");
            assert!(
                stderr.contains("<memory>:3: command failed (exit 3)"),
                "{shell}: {stderr}"
            );
        },
    );
}

#[test]
fn a_failing_deferred_block_reports_itself_and_the_others_still_run() {
    each_run(
        r#"defer print "still runs"
defer {
  sh "-c" "exit 5"
  print "unreachable"
}
print "body"
"#,
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 5, "{shell}: {stderr}");
            assert_eq!(stdout, "body\nstill runs\n", "{shell}");
            assert!(
                stderr.contains("<memory>:4: command failed (exit 5)"),
                "{shell}: {stderr}"
            );
        },
    );
    // The script's own failure wins over a later deferred failure.
    each_run(
        "defer sh \"-c\" \"exit 5\"\nsh \"-c\" \"exit 2\"\n",
        |shell, _, (status, _, stderr)| assert_eq!(status, 2, "{shell}: {stderr}"),
    );
}

#[test]
fn function_defers_run_when_the_function_returns() {
    each_run(
        r#"fn step(name: String) -> () / stdout {
  defer print "leave " + name
  defer {
    print "inner " + name
  }
  print "in " + name
}
fn value() -> String {
  defer print "value cleanup"
  "computed"
}
fn failing() -> () / unknown, exec, io, fs, env, net {
  defer print "failing cleanup"
  sh "-c" "exit 4"
}
defer print "script exit"
step "a"
step "b"
print value
print "failing " + (status_of failing)
"#,
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(
                stdout,
                "in a\ninner a\nleave a\nin b\ninner b\nleave b\nvalue cleanup\ncomputed\n\
                 failing cleanup\nfailing 4\nscript exit\n",
                "{shell}"
            );
        },
    );
}

#[test]
fn defers_run_before_tasks_are_joined_and_temporary_directories_removed() {
    each_run(
        r#"let work = std::fs::temp_dir
let server = spawn sleep "300"
defer {
  server.kill()
  if test "-d" work {
    print "temp dir still there"
  }
}
print work
"#,
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            let mut lines = stdout.lines();
            let work = lines.next().unwrap();
            assert_eq!(lines.next(), Some("temp dir still there"), "{shell}");
            assert!(!Path::new(work).exists(), "{shell}: {work} was not removed");
        },
    );
}

#[test]
fn a_handled_signal_runs_the_deferred_blocks() {
    support::each_shell(|shell, target| {
        let dir = tempfile::tempdir().unwrap();
        let source = format!(
            "{PRELUDE}defer {{\n  touch \"cleaned\"\n}}\ntouch \"started\"\nsleep \"30\"\n"
        );
        let compiled = UshCompiler::for_target(target)
            .compile_source(&source)
            .unwrap();
        fs::write(dir.path().join("program.sh"), compiled).unwrap();
        // A terminal sends Ctrl-C to the whole process group; do the same
        // with TERM, so the running `sleep` ends too.
        let mut child = Command::new(shell)
            .arg("program.sh")
            .current_dir(dir.path())
            .process_group(0)
            .spawn()
            .unwrap();
        while !dir.path().join("started").exists() {
            thread::sleep(Duration::from_millis(20));
        }
        thread::sleep(Duration::from_millis(100));
        Command::new("kill")
            .args(["-TERM", "--", &format!("-{}", child.id())])
            .status()
            .unwrap();
        let status = child.wait().unwrap();
        assert_eq!(status.code(), Some(143), "{shell}");
        assert!(dir.path().join("cleaned").exists(), "{shell}");
    });
}

#[test]
fn defer_is_rejected_where_it_cannot_run_as_written() {
    for (body, message) in [
        ("for x in 0..2 {\n  defer print \"x\"\n}", "inside a loop"),
        (
            "cd \"/\" {\n  defer print \"x\"\n}",
            "inside a `cd` or `export` block",
        ),
        (
            "let t = async {\n  defer print \"x\"\n  1\n}",
            "inside an `async` block",
        ),
        ("defer {\n  defer print \"x\"\n}", "inside a deferred block"),
        (
            "fn f() -> Int {\n  defer {\n    return 1\n  }\n  2\n}",
            "cannot `return`",
        ),
        ("defer {\n  break\n}", "need a loop inside it"),
        ("defer", "needs a statement or a block"),
    ] {
        let error = UshCompiler::default()
            .compile_source(&format!("{PRELUDE}{body}\n"))
            .expect_err(body);
        let error = format!("{error:#}");
        assert!(error.contains(message), "{body}\n=> {error}");
    }
}
