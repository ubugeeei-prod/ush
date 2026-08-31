#[cfg(test)]
mod tests;

use std::{fmt::Write as _, path::Path};

use anyhow::Result;

use crate::helpers::ValueStream;

use super::{
    super::common::{BLUE_BOLD, BOLD, CYAN_BOLD, MAGENTA_BOLD, badge, dim, paint, pluralize},
    git_capture,
    model::{GitBranchRow, GitRefScope},
};

pub(super) fn render_git_branch(cwd: &Path, args: &[String]) -> Result<Option<ValueStream>> {
    let Some((include_local, include_remote)) = parse_git_branch_args(args) else {
        return Ok(None);
    };

    let mut command_args = vec![
        "for-each-ref".to_string(),
        "--format=%(HEAD)\t%(refname)\t%(refname:short)\t%(upstream:short)\t%(objectname:short)\t%(committerdate:short)\t%(subject)".to_string(),
    ];
    if include_local {
        command_args.push("refs/heads".to_string());
    }
    if include_remote {
        command_args.push("refs/remotes".to_string());
    }

    let Some(stdout) = git_capture(cwd, &command_args)? else {
        return Ok(None);
    };

    let mut local = Vec::new();
    let mut remote = Vec::new();
    for line in stdout.lines() {
        let columns = line.splitn(7, '\t').collect::<Vec<_>>();
        if columns.len() < 7 {
            continue;
        }
        let full_ref = columns[1];
        let name = columns[2].trim();
        if name.is_empty() || name.ends_with("/HEAD") {
            continue;
        }
        let row = GitBranchRow {
            scope: if full_ref.starts_with("refs/remotes/") {
                GitRefScope::Remote
            } else {
                GitRefScope::Local
            },
            name: name.to_string(),
            current: columns[0].trim() == "*",
            upstream: (!columns[3].is_empty()).then_some(columns[3].to_string()),
            commit: columns[4].to_string(),
            date: columns[5].to_string(),
            subject: columns[6].to_string(),
        };
        match row.scope {
            GitRefScope::Local => local.push(row),
            GitRefScope::Remote => remote.push(row),
        }
    }

    let mut out = String::new();
    let _ = writeln!(
        out,
        "{} {}",
        paint(BLUE_BOLD, "git"),
        paint(CYAN_BOLD, "branch")
    );
    let _ = writeln!(
        out,
        "{}",
        dim(pluralize(local.len() + remote.len(), "branch", "branches"))
    );

    let grouped = !local.is_empty() && !remote.is_empty();
    if !local.is_empty() {
        if grouped {
            let _ = writeln!(out, "{}", dim("local"));
        }
        for row in &local {
            render_git_branch_row(&mut out, row);
        }
    }
    if !remote.is_empty() {
        if !local.is_empty() {
            out.push('\n');
        }
        let _ = writeln!(out, "{}", dim("remote"));
        for row in &remote {
            render_git_branch_row(&mut out, row);
        }
    }
    Ok(Some(ValueStream::Text(out)))
}

fn parse_git_branch_args(args: &[String]) -> Option<(bool, bool)> {
    let mut include_local = true;
    let mut include_remote = false;
    for arg in args {
        match arg.as_str() {
            "-a" | "--all" => include_remote = true,
            "-r" | "--remotes" => {
                include_local = false;
                include_remote = true;
            }
            "--list" => {}
            _ => return None,
        }
    }
    Some((include_local, include_remote))
}

fn render_git_branch_row(out: &mut String, row: &GitBranchRow) {
    let style = if row.current {
        BLUE_BOLD
    } else if row.scope == GitRefScope::Remote {
        CYAN_BOLD
    } else {
        BOLD
    };
    let mut parts = vec![paint(style, &row.name)];
    if row.current {
        parts.push(badge("current", BLUE_BOLD));
    }
    if row.scope == GitRefScope::Remote {
        parts.push(badge("remote", CYAN_BOLD));
    }
    if let Some(upstream) = &row.upstream {
        parts.push(badge(upstream, MAGENTA_BOLD));
    }
    parts.push(dim(&row.commit));
    let _ = writeln!(out, "{}", parts.join(" "));

    let details = [
        (!row.date.is_empty()).then_some(row.date.clone()),
        (!row.subject.is_empty()).then_some(row.subject.clone()),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    if !details.is_empty() {
        let _ = writeln!(out, "  {}", dim(details.join(" · ")));
    }
}
