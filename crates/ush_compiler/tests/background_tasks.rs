//! `spawn` and task handles under every installed shell: background
//! commands stream their output, `.await` propagates their status,
//! `status_of` and `allow_fail` observe it, and `kill` signals the whole
//! process tree.

mod support;

use std::{fs, path::Path, process::Command, thread, time::Duration};

use support::each_program;
use ush_compiler::UshCompiler;

const PRELUDE: &str = "use $bin::*\nuse std::io::to_file\n";

fn each_run(body: &str, check: impl FnMut(&str, &Path, (i32, String, String))) {
    each_program(&format!("{PRELUDE}{body}"), check);
}

fn alive(pid: &str) -> bool {
    Command::new("kill")
        .args(["-0", pid])
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

/// Wait up to five seconds for `pid` to go away; a killed process can
/// linger briefly as a zombie until its new parent reaps it.
fn gone(pid: &str) -> bool {
    (0..100).any(|_| {
        let gone = !alive(pid);
        if !gone {
            thread::sleep(Duration::from_millis(50));
        }
        gone
    })
}

#[test]
fn a_spawned_command_streams_and_await_propagates_its_failure() {
    each_run(
        "let job = spawn sh \"-c\" \"echo from job; exit 4\"\njob.await\nprint \"unreachable\"\n",
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 4, "{shell}: {stderr}");
            assert_eq!(stdout, "from job\n", "{shell}");
        },
    );
}

#[test]
fn status_of_and_allow_fail_observe_a_failed_job_every_time() {
    each_run(
        r#"let job = spawn sh "-c" "exit 4"
let first = status_of job.await
let second = status_of job.await
allow_fail job.await
let ok = spawn sh "-c" "echo ok"
ok.await
print "statuses " + first + " " + second + " " + (status_of ok.await)
"#,
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(stdout, "ok\nstatuses 4 4 0\n", "{shell}");
        },
    );
}

#[test]
fn kill_stops_the_task_and_every_process_it_started() {
    each_run(
        r#"let server = spawn sh "-c" "sleep 300 & echo $! > child.pid; echo $$ > shell.pid; wait"
while ! test "-s" "shell.pid" {
  sleep "1"
}
if server.pid > 0 {
  print "running"
}
server.kill()
print "killed " + (status_of server.await)
server.kill()
"#,
        |shell, dir, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(stdout, "running\nkilled 143\n", "{shell}");
            for file in ["child.pid", "shell.pid"] {
                let pid = fs::read_to_string(dir.join(file)).unwrap();
                assert!(gone(pid.trim()), "{shell}: {file} {pid} survived");
            }
        },
    );
}

#[test]
fn a_killed_task_does_not_fail_the_script_when_it_is_joined_at_exit() {
    each_run(
        r#"let server = spawn sleep "300"
let other = spawn sleep "300"
server.kill()
other.kill("KILL")
print "done"
"#,
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(stdout, "done\n", "{shell}");
            // bash prints its own notice for a job a signal ended.
            assert!(!stderr.contains("<memory>"), "{shell}: {stderr}");
        },
    );
}

#[test]
fn an_unkilled_failed_task_still_fails_the_script_at_exit() {
    each_run(
        "let job = spawn sh \"-c\" \"exit 6\"\nprint \"done\"\n",
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 6, "{shell}: {stderr}");
            assert_eq!(stdout, "done\n", "{shell}");
        },
    );
}

#[test]
fn spawn_runs_pipelines_and_unit_functions() {
    each_run(
        r#"fn worker(name: String) -> () / stdout {
  print "worker " + name
}
let log = spawn sh "-c" "echo out; echo err >&2" |& to_file "job.log"
let work = spawn worker "a"
log.await
work.await
"#,
        |shell, dir, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(stdout, "worker a\n", "{shell}");
            let log = fs::read_to_string(dir.join("job.log")).unwrap();
            assert_eq!(log.lines().count(), 2, "{shell}: {log}");
        },
    );
}

#[test]
fn a_signal_name_is_checked() {
    each_run(
        "let job = spawn sleep \"1\"\nlet signal = \"BOGUS\"\njob.kill(signal)\n",
        |shell, _, (status, _, stderr)| {
            assert_eq!(status, 2, "{shell}: {stderr}");
            assert!(
                stderr.contains("unknown signal: BOGUS"),
                "{shell}: {stderr}"
            );
        },
    );
    for (body, message) in [
        (
            r#"let job = spawn sleep "1"
job.kill("BOGUS")"#,
            "unknown signal `BOGUS`",
        ),
        (
            r#"let job = spawn sleep "1"
job.kill(9)"#,
            "takes a signal name",
        ),
        (r#"let job = spawn print "x""#, "`spawn` runs a command"),
    ] {
        let error = UshCompiler::default()
            .compile_source(&format!("{PRELUDE}{body}"))
            .expect_err(body);
        let error = format!("{error:#}");
        assert!(error.contains(message), "{body}\n=> {error}");
    }
}
