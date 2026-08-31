#[cfg(test)]
mod tests;

use std::{fmt::Write as _, fs, path::Path};

use anyhow::{Context, Result};

use crate::helpers::ValueStream;

use super::common::{BLUE_BOLD, CYAN_BOLD, dim, human_bytes, normalize_path, paint, pluralize};

pub fn render_cat(cwd: &Path, args: &[String], input: &ValueStream) -> Result<Option<ValueStream>> {
    let mut numbered = true;
    let mut targets = Vec::new();
    for arg in args {
        match arg.as_str() {
            "-n" => numbered = true,
            _ if arg.starts_with('-') => return Ok(None),
            _ => targets.push(arg.clone()),
        }
    }

    let mut buffer = String::new();
    if targets.is_empty() {
        append_numbered(&mut buffer, None, &input.to_text()?, numbered);
        return Ok(Some(ValueStream::Text(buffer)));
    }

    for (index, target) in targets.into_iter().enumerate() {
        if index > 0 {
            buffer.push('\n');
        }
        let path = normalize_path(cwd, &target);
        let text = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        append_numbered(&mut buffer, Some(&path), &text, numbered);
    }

    Ok(Some(ValueStream::Text(buffer)))
}

fn append_numbered(buffer: &mut String, path: Option<&Path>, text: &str, numbered: bool) {
    if let Some(path) = path {
        let _ = writeln!(
            buffer,
            "{} {}",
            paint(BLUE_BOLD, "cat"),
            paint(CYAN_BOLD, path.display())
        );
        let _ = writeln!(
            buffer,
            "{}",
            dim(format!(
                "{}, {}",
                pluralize(count_display_lines(text), "line", "lines"),
                human_bytes(text.len() as u64)
            ))
        );
    }

    if text.is_empty() {
        if path.is_some() {
            let _ = writeln!(buffer, "{}", dim("(empty)"));
        }
        return;
    }

    let width = count_display_lines(text).to_string().len();
    for (index, chunk) in text.split_inclusive('\n').enumerate() {
        let line = chunk.strip_suffix('\n').unwrap_or(chunk);
        if numbered {
            let _ = write!(
                buffer,
                "{} {} {}",
                dim(format!("{:>width$}", index + 1, width = width)),
                paint(CYAN_BOLD, "|"),
                line
            );
        } else {
            buffer.push_str(line);
        }
        buffer.push('\n');
    }
}

fn count_display_lines(text: &str) -> usize {
    text.split_inclusive('\n').count()
}
