#[cfg(test)]
mod tests;

use std::{collections::BTreeMap, fmt::Write as _, path::Path};

use crate::style::{badge, dim, human_bytes, paint, pluralize};

#[derive(Clone)]
pub(super) struct SummaryRow {
    pub(super) path: String,
    pub(super) kind: String,
    pub(super) lines: usize,
    pub(super) bytes: u64,
}

#[derive(Clone)]
struct TypeSummaryRow {
    kind: String,
    files: usize,
    lines: usize,
    bytes: u64,
}

pub(super) fn type_name(path: &Path) -> String {
    if is_lock_file(path) {
        return "lock".to_string();
    }
    path.extension()
        .and_then(|ext| ext.to_str())
        .filter(|ext| !ext.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| "(none)".to_string())
}

pub(super) fn is_lock_file(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };
    matches!(
        name,
        "Cargo.lock"
            | "Gemfile.lock"
            | "Podfile.lock"
            | "Pipfile.lock"
            | "poetry.lock"
            | "composer.lock"
            | "package-lock.json"
            | "pnpm-lock.yaml"
            | "yarn.lock"
            | "bun.lock"
            | "bun.lockb"
            | "uv.lock"
    ) || name.ends_with(".lock")
}

pub(super) fn summarize_types(rows: &[SummaryRow]) -> Vec<(String, usize, usize, u64)> {
    let mut grouped = BTreeMap::<String, TypeSummaryRow>::new();
    for row in rows {
        let entry = grouped.entry(row.kind.clone()).or_insert(TypeSummaryRow {
            kind: row.kind.clone(),
            files: 0,
            lines: 0,
            bytes: 0,
        });
        entry.files += 1;
        entry.lines += row.lines;
        entry.bytes += row.bytes;
    }
    grouped
        .into_values()
        .map(|row| (row.kind, row.files, row.lines, row.bytes))
        .collect()
}

pub(super) fn count_lines(bytes: &[u8]) -> usize {
    if bytes.is_empty() {
        return 0;
    }
    let line_breaks = bytes.iter().filter(|byte| **byte == b'\n').count();
    line_breaks + usize::from(*bytes.last().unwrap_or(&b'\n') != b'\n')
}

pub(super) fn render_plain(
    rows: &[SummaryRow],
    types: &[(String, usize, usize, u64)],
    total_lines: usize,
    total_bytes: u64,
) -> String {
    let mut out = String::from("lines\tbytes\tpath\n");
    for row in rows {
        out.push_str(&format!("{}\t{}\t{}\n", row.lines, row.bytes, row.path));
    }
    out.push_str(&format!(
        "{}\t{}\tTOTAL ({} files)\n\ntype\tfiles\tlines\tbytes\n",
        total_lines,
        total_bytes,
        rows.len()
    ));
    for (kind, files, lines, bytes) in types {
        out.push_str(&format!("{kind}\t{files}\t{lines}\t{bytes}\n"));
    }
    out.push_str(&format!(
        "TOTAL\t{}\t{}\t{}\n",
        rows.len(),
        total_lines,
        total_bytes
    ));
    out
}

pub(super) fn render_stylish(
    rows: &[SummaryRow],
    types: &[(String, usize, usize, u64)],
    total_lines: usize,
    total_bytes: u64,
) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{} {}",
        paint("\u{1b}[1;35m", "sammary"),
        dim(format!(
            "{}, {}, {}",
            pluralize(rows.len(), "file", "files"),
            pluralize(total_lines, "line", "lines"),
            human_bytes(total_bytes)
        ))
    );
    let _ = writeln!(out);
    let _ = writeln!(out, "{}", paint("\u{1b}[1;34m", "files"));
    for row in rows {
        let _ = writeln!(
            out,
            "{} {} {}",
            paint("\u{1b}[1;36m", &row.path),
            badge(&row.kind, "\u{1b}[1;34m"),
            dim(format!(
                "{}, {}",
                pluralize(row.lines, "line", "lines"),
                human_bytes(row.bytes)
            ))
        );
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "{}", paint("\u{1b}[1;34m", "types"));
    for (kind, files_count, lines, bytes) in types {
        let _ = writeln!(
            out,
            "{} {}",
            paint("\u{1b}[1;36m", kind),
            dim(format!(
                "{}, {}, {}",
                pluralize(*files_count, "file", "files"),
                pluralize(*lines, "line", "lines"),
                human_bytes(*bytes)
            ))
        );
    }
    out
}
