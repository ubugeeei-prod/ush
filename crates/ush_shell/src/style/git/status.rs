#[cfg(test)]
mod tests;

use std::{fmt::Write as _, path::Path};

use anyhow::Result;

use crate::helpers::ValueStream;

use super::{
    super::common::{
        BLUE_BOLD, CYAN_BOLD, GREEN_BOLD, MAGENTA_BOLD, RED_BOLD, YELLOW_BOLD, badge, dim, paint,
        pluralize,
    },
    git_capture,
    model::{GitStatusHeader, GitStatusRow},
};

pub(super) fn render_git_status(cwd: &Path, args: &[String]) -> Result<Option<ValueStream>> {
    let Some(pathspecs) = parse_git_status_args(args) else {
        return Ok(None);
    };

    let mut command_args = vec![
        "status".to_string(),
        "--porcelain=v1".to_string(),
        "--branch".to_string(),
        "-z".to_string(),
        "--untracked-files=all".to_string(),
    ];
    if !pathspecs.is_empty() {
        command_args.push("--".to_string());
        command_args.extend(pathspecs);
    }

    let Some(stdout) = git_capture(cwd, &command_args)? else {
        return Ok(None);
    };

    let mut records = stdout.split('\0').filter(|record| !record.is_empty());
    let mut header = None;
    let mut rows = Vec::new();
    if let Some(record) = records.next() {
        if let Some(branch) = record.strip_prefix("## ") {
            header = Some(parse_git_status_header(branch));
        } else {
            parse_git_status_record(record, &mut records, &mut rows);
        }
    }
    while let Some(record) = records.next() {
        parse_git_status_record(record, &mut records, &mut rows);
    }

    let branch = header.unwrap_or_else(|| GitStatusHeader {
        branch: "repository".to_string(),
        details: Vec::new(),
    });
    let mut meta = branch.details;
    meta.push(if rows.is_empty() {
        "working tree clean".to_string()
    } else {
        pluralize(rows.len(), "changed path", "changed paths")
    });

    let mut out = String::new();
    let _ = writeln!(
        out,
        "{} {} {}",
        paint(BLUE_BOLD, "git"),
        paint(CYAN_BOLD, "status"),
        paint(MAGENTA_BOLD, &branch.branch)
    );
    let _ = writeln!(out, "{}", dim(meta.join(", ")));
    if rows.is_empty() {
        let _ = writeln!(
            out,
            "{} {}",
            badge("clean", GREEN_BOLD),
            dim("nothing to commit")
        );
    } else {
        for row in &rows {
            render_git_status_row(&mut out, row);
        }
    }
    Ok(Some(ValueStream::Text(out)))
}

fn parse_git_status_args(args: &[String]) -> Option<Vec<String>> {
    let mut pathspecs = Vec::new();
    let mut force_paths = false;
    for arg in args {
        match arg.as_str() {
            "-s" | "--short" | "-b" | "--branch" | "--porcelain" | "--porcelain=v1" => {}
            "--" => force_paths = true,
            _ if force_paths || !arg.starts_with('-') => pathspecs.push(arg.clone()),
            _ => return None,
        }
    }
    Some(pathspecs)
}

fn parse_git_status_header(value: &str) -> GitStatusHeader {
    if let Some(branch) = value.strip_prefix("No commits yet on ") {
        return GitStatusHeader {
            branch: branch.to_string(),
            details: vec!["no commits yet".to_string()],
        };
    }
    if value.starts_with("HEAD ") {
        return GitStatusHeader {
            branch: "detached HEAD".to_string(),
            details: vec![value.to_string()],
        };
    }

    let (head, tracking) = value
        .split_once(" [")
        .map(|(head, rest)| (head, Some(rest.trim_end_matches(']'))))
        .unwrap_or((value, None));
    let (branch, upstream) = head
        .split_once("...")
        .map(|(branch, upstream)| (branch, Some(upstream)))
        .unwrap_or((head, None));
    let mut details = Vec::new();
    if let Some(upstream) = upstream {
        details.push(format!("tracks {upstream}"));
    }
    if let Some(tracking) = tracking {
        details.extend(tracking.split(", ").map(str::to_string));
    }
    GitStatusHeader {
        branch: branch.to_string(),
        details,
    }
}

fn parse_git_status_record<'a, I>(record: &str, records: &mut I, rows: &mut Vec<GitStatusRow>)
where
    I: Iterator<Item = &'a str>,
{
    if record.len() < 3 {
        return;
    }
    let mut chars = record.chars();
    let index = chars.next().unwrap_or(' ');
    let worktree = chars.next().unwrap_or(' ');
    let path = record[3..].to_string();
    let original_path = if matches!(index, 'R' | 'C') {
        records.next().map(str::to_string)
    } else {
        None
    };
    rows.push(describe_git_status_row(
        index,
        worktree,
        path,
        original_path,
    ));
}

fn describe_git_status_row(
    index: char,
    worktree: char,
    path: String,
    original_path: Option<String>,
) -> GitStatusRow {
    let mut badges = Vec::new();
    let conflict = git_status_conflict(index, worktree);
    if conflict {
        badges.push(badge("conflict", RED_BOLD));
    } else if index == '?' && worktree == '?' {
        badges.push(badge("untracked", MAGENTA_BOLD));
    } else {
        if let Some(label) = git_status_label(index) {
            badges.push(badge(format!("staged {label}"), GREEN_BOLD));
        }
        if let Some(label) = git_status_label(worktree) {
            badges.push(badge(format!("unstaged {label}"), YELLOW_BOLD));
        }
    }
    if original_path.is_some() {
        badges.push(badge("renamed", MAGENTA_BOLD));
    }

    let style = if conflict || matches!(index, 'D') || matches!(worktree, 'D') {
        RED_BOLD
    } else if original_path.is_some() {
        MAGENTA_BOLD
    } else if index == '?' && worktree == '?' || matches!(index, 'A') || matches!(worktree, 'A') {
        GREEN_BOLD
    } else {
        CYAN_BOLD
    };
    GitStatusRow {
        path,
        original_path,
        badges,
        style,
    }
}

fn git_status_conflict(index: char, worktree: char) -> bool {
    matches!(
        (index, worktree),
        ('D', 'D') | ('A', 'U') | ('U', 'D') | ('U', 'A') | ('D', 'U') | ('A', 'A') | ('U', 'U')
    )
}

fn git_status_label(code: char) -> Option<&'static str> {
    match code {
        'M' => Some("modified"),
        'A' => Some("added"),
        'D' => Some("deleted"),
        'R' => Some("renamed"),
        'C' => Some("copied"),
        'U' => Some("updated"),
        _ => None,
    }
}

fn render_git_status_row(out: &mut String, row: &GitStatusRow) {
    let mut parts = vec![paint(row.style, &row.path)];
    parts.extend(row.badges.iter().cloned());
    let _ = writeln!(out, "{}", parts.join(" "));
    if let Some(original_path) = &row.original_path {
        let _ = writeln!(out, "  {}", dim(format!("from {original_path}")));
    }
}
