#[cfg(test)]
mod tests;

use std::{fmt::Write as _, path::Path};

use anyhow::Result;

use crate::helpers::ValueStream;

use super::{
    super::common::{
        BLUE_BOLD, BOLD, CYAN_BOLD, MAGENTA_BOLD, YELLOW_BOLD, badge, dim, paint, pluralize,
    },
    git_capture,
    model::GitLogRow,
};

pub(super) fn render_git_log(cwd: &Path, args: &[String]) -> Result<Option<ValueStream>> {
    let Some((max_count, include_all)) = parse_git_log_args(args) else {
        return Ok(None);
    };

    let mut command_args = vec![
        "log".to_string(),
        "--decorate=short".to_string(),
        "--date=short".to_string(),
        format!("-n{max_count}"),
        "--pretty=format:%h%x1f%ad%x1f%an%x1f%d%x1f%s".to_string(),
    ];
    if include_all {
        command_args.push("--all".to_string());
    }
    let Some(stdout) = git_capture(cwd, &command_args)? else {
        return Ok(None);
    };

    let rows = stdout
        .lines()
        .filter_map(|line| {
            let columns = line.split('\u{1f}').collect::<Vec<_>>();
            (columns.len() >= 5).then(|| GitLogRow {
                commit: columns[0].to_string(),
                date: columns[1].to_string(),
                author: columns[2].to_string(),
                refs: format_git_refs(columns[3]),
                subject: columns[4].to_string(),
            })
        })
        .collect::<Vec<_>>();

    let mut out = String::new();
    let _ = writeln!(
        out,
        "{} {}",
        paint(BLUE_BOLD, "git"),
        paint(CYAN_BOLD, "log")
    );
    let _ = writeln!(out, "{}", dim(pluralize(rows.len(), "commit", "commits")));
    for row in &rows {
        render_git_log_row(&mut out, row);
    }
    Ok(Some(ValueStream::Text(out)))
}

fn parse_git_log_args(args: &[String]) -> Option<(usize, bool)> {
    let mut max_count = 12usize;
    let mut include_all = false;
    let mut pending_count = false;
    for arg in args {
        if pending_count {
            max_count = arg.parse().ok()?;
            pending_count = false;
            continue;
        }
        match arg.as_str() {
            "--oneline" => {}
            "--all" => include_all = true,
            "-n" | "--max-count" => pending_count = true,
            _ if arg.starts_with("--max-count=") => {
                max_count = arg.split_once('=')?.1.parse().ok()?
            }
            _ if arg.starts_with("-n") && arg.len() > 2 => max_count = arg[2..].parse().ok()?,
            _ if arg.starts_with('-') && arg[1..].chars().all(|ch| ch.is_ascii_digit()) => {
                max_count = arg[1..].parse().ok()?
            }
            _ => return None,
        }
    }
    (!pending_count).then_some((max_count.max(1), include_all))
}

fn format_git_refs(raw: &str) -> Vec<String> {
    let trimmed = raw
        .trim()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .trim();
    if trimmed.is_empty() {
        return Vec::new();
    }
    trimmed
        .split(", ")
        .map(|value| {
            if let Some(tag) = value.strip_prefix("tag: ") {
                badge(format!("tag {tag}"), YELLOW_BOLD)
            } else if value.starts_with("HEAD") {
                badge(value, BLUE_BOLD)
            } else if value.contains('/') {
                badge(value, CYAN_BOLD)
            } else {
                badge(value, MAGENTA_BOLD)
            }
        })
        .collect()
}

fn render_git_log_row(out: &mut String, row: &GitLogRow) {
    let mut parts = vec![paint(CYAN_BOLD, &row.commit)];
    parts.extend(row.refs.iter().cloned());
    parts.push(paint(BOLD, &row.subject));
    let _ = writeln!(out, "{}", parts.join(" "));
    let _ = writeln!(out, "  {}", dim(format!("{} · {}", row.date, row.author)));
}
