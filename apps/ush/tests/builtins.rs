use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[path = "builtins/git_views.rs"]
mod git_views;
#[path = "builtins/glob_and_rm.rs"]
mod glob_and_rm;
#[path = "builtins/interactive.rs"]
mod interactive;
#[path = "builtins/lookup.rs"]
mod lookup;
#[path = "builtins/stylish_files.rs"]
mod stylish_files;
mod support;
#[path = "builtins/tasks_and_which.rs"]
mod tasks_and_which;

pub(crate) use std::os::unix::fs::symlink;
pub(crate) use support::assert_snapshot;
pub(crate) use tempfile::tempdir;

fn ush() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ush"))
}

fn fixture(name: &str) -> String {
    format!("builtins/{name}.stdout")
}

fn run_with_stdin(args: &[&str], stdin: &str) -> std::process::Output {
    run_with_stdin_in_dir(args, stdin, None)
}

fn run_with_stdin_in_dir(args: &[&str], stdin: &str, dir: Option<&Path>) -> std::process::Output {
    let mut child = ush()
        .args(args)
        .current_dir(dir.unwrap_or_else(|| Path::new(".")))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn ush");
    // A broken pipe here means the child decided it had no question
    // to ask and exited first — which is exactly what the `rm` guard
    // does when stdin is not a terminal. That is an answer, not a
    // test failure.
    let _ = child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(stdin.as_bytes());
    child.wait_with_output().expect("wait ush")
}

fn run_git(dir: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "git {:?} failed\nstdout:\n{}\nstderr:\n{}",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_git_repo(dir: &Path) {
    run_git(dir, &["init", "-q", "-b", "main"]);
    run_git(dir, &["config", "user.name", "ush"]);
    run_git(dir, &["config", "user.email", "ush@example.com"]);
    fs::write(dir.join("tracked.txt"), "hello\n").expect("write tracked");
    run_git(dir, &["add", "tracked.txt"]);
    run_git(dir, &["commit", "-q", "-m", "initial commit"]);
}

fn history_files(home: &Path) -> [PathBuf; 2] {
    [
        home.join("Library/Caches/dev.ubugeeei.ush/history.txt"),
        home.join(".cache/ush/history.txt"),
    ]
}

fn normalize_command_paths(text: &str, names: &[&str]) -> String {
    let mut out = text.to_string();
    for name in names {
        let output = Command::new("/bin/sh")
            .args(["-c", &format!("command -v {name}")])
            .output()
            .expect("resolve command path");
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !path.is_empty() {
            out = out.replace(&path, &format!("<{}_PATH>", name.to_ascii_uppercase()));
        }
    }
    out
}

fn normalize_git_hashes(text: &str) -> String {
    let mut out = String::new();
    let mut token = String::new();

    for ch in text.chars() {
        if ch.is_ascii_hexdigit() {
            token.push(ch);
            continue;
        }
        flush_token(&mut out, &mut token);
        out.push(ch);
    }
    flush_token(&mut out, &mut token);
    out
}

fn flush_token(out: &mut String, token: &mut String) {
    if token.len() >= 7 && token.chars().all(|ch| ch.is_ascii_hexdigit()) {
        out.push_str("<SHA>");
    } else {
        out.push_str(token);
    }
    token.clear();
}

fn normalize_iso_dates(text: &str) -> String {
    let mut out = String::new();
    let chars = text.chars().collect::<Vec<_>>();
    let mut index = 0usize;

    while index < chars.len() {
        if index + 10 <= chars.len()
            && chars[index..index + 10]
                .iter()
                .enumerate()
                .all(|(offset, ch)| match offset {
                    4 | 7 => *ch == '-',
                    _ => ch.is_ascii_digit(),
                })
        {
            out.push_str("<DATE>");
            index += 10;
            continue;
        }

        out.push(chars[index]);
        index += 1;
    }

    out
}

fn normalize_ls_output(text: &str) -> String {
    let normalized = text
        .lines()
        .map(|line| {
            let mut line = line.to_string();
            if let Some(index) = line.find(" items, updated ") {
                let mut start = index;
                while start > 0 && line.as_bytes()[start - 1].is_ascii_digit() {
                    start -= 1;
                }
                line.replace_range(start..index, "<ITEMS>");
            }
            if let Some(index) = line.find("updated ") {
                line.replace_range(index + "updated ".len()..line.len(), "<TIMESTAMP>");
            }
            line
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!("{normalized}\n")
}

fn write_history(home: &Path, entries: &[&str]) {
    let body = if entries.is_empty() {
        String::new()
    } else {
        format!("{}\n", entries.join("\n"))
    };
    for path in history_files(home) {
        fs::create_dir_all(path.parent().expect("history dir")).expect("create history dir");
        fs::write(path, &body).expect("write history");
    }
}
