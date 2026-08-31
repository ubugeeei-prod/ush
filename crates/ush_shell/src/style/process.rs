#[cfg(test)]
mod tests;

use std::{fmt::Write as _, path::Path, process::Command};

use anyhow::{Context, Result};

use crate::helpers::ValueStream;

use super::common::{BLUE_BOLD, CYAN_BOLD, YELLOW_BOLD, badge, dim, paint, pluralize};

struct PsRow {
    pid: String,
    ppid: String,
    stat: String,
    cpu: String,
    mem: String,
    command: String,
}

pub fn render_ps(args: &[String]) -> Result<Option<ValueStream>> {
    if !args.is_empty() {
        return Ok(None);
    }

    let output = Command::new("ps")
        .args(["-eo", "pid,ppid,stat,%cpu,%mem,comm"])
        .output()
        .context("failed to run ps")?;
    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8(output.stdout)?;
    let rows = stdout
        .lines()
        .skip(1)
        .filter_map(|line| {
            let columns = line.split_whitespace().collect::<Vec<_>>();
            (columns.len() >= 6).then(|| PsRow {
                pid: columns[0].to_string(),
                ppid: columns[1].to_string(),
                stat: columns[2].to_string(),
                cpu: columns[3].to_string(),
                mem: columns[4].to_string(),
                command: columns[5..].join(" "),
            })
        })
        .collect::<Vec<_>>();

    let mut out = String::new();
    let _ = writeln!(
        out,
        "{} {}",
        paint(BLUE_BOLD, "ps"),
        dim(pluralize(rows.len(), "process", "processes"))
    );
    for row in &rows {
        render_ps_row(&mut out, row);
    }
    Ok(Some(ValueStream::Text(out)))
}

pub fn render_kill(args: &[String]) -> Result<Option<ValueStream>> {
    if args.is_empty() {
        return Ok(None);
    }

    let mut signal = "TERM".to_string();
    let mut pid = None::<String>;
    for arg in args {
        if let Some(value) = arg.strip_prefix('-') {
            signal = value.to_string();
        } else {
            pid = Some(arg.clone());
        }
    }
    let Some(pid) = pid else {
        return Ok(None);
    };

    let output = Command::new("kill")
        .args([format!("-{signal}"), pid.clone()])
        .output()
        .context("failed to run kill")?;
    if !output.status.success() {
        return Ok(None);
    }

    Ok(Some(ValueStream::Text(format!(
        "{} {} {}\n",
        paint(BLUE_BOLD, "kill"),
        badge(format!("SIG{}", signal.to_uppercase()), YELLOW_BOLD),
        paint(CYAN_BOLD, pid)
    ))))
}

fn render_ps_row(out: &mut String, row: &PsRow) {
    let display_name = Path::new(&row.command)
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or(&row.command);
    let _ = writeln!(
        out,
        "{} {} {} {}",
        paint(CYAN_BOLD, display_name),
        badge(format!("pid {}", row.pid), BLUE_BOLD),
        badge(row.stat.as_str(), YELLOW_BOLD),
        dim(format!(
            "ppid {}, cpu {}%, mem {}%",
            row.ppid, row.cpu, row.mem
        ))
    );
    if display_name != row.command {
        let _ = writeln!(out, "  {}", dim(&row.command));
    }
}
