#[cfg(test)]
mod tests;

use std::{fmt::Write as _, fs, os::unix::fs::PermissionsExt, path::Path};

use anyhow::Result;
use chrono::{DateTime, Local};

use super::common::{
    BLUE_BOLD, BOLD, CYAN_BOLD, GREEN_BOLD, MAGENTA_BOLD, badge, dim, human_bytes, paint, pluralize,
};

#[derive(Clone, Copy, Default)]
pub(super) enum HiddenMode {
    #[default]
    Default,
    AlmostAll,
    All,
}

#[derive(Clone, Copy)]
pub(super) enum EntryKind {
    Dir,
    Exec,
    File,
    Link,
}

#[derive(Default)]
pub(super) struct LsSummary {
    dirs: usize,
    execs: usize,
    files: usize,
    links: usize,
}

pub(super) struct LsRow {
    display_name: String,
    pub(super) kind: EntryKind,
    details: Vec<String>,
}

impl HiddenMode {
    pub(super) fn include(self, other: Self) -> Self {
        match (self, other) {
            (Self::All, _) | (_, Self::All) => Self::All,
            (Self::AlmostAll, _) | (_, Self::AlmostAll) => Self::AlmostAll,
            _ => Self::Default,
        }
    }

    pub(super) fn shows_hidden(self) -> bool {
        !matches!(self, Self::Default)
    }

    pub(super) fn shows_dot_entries(self) -> bool {
        matches!(self, Self::All)
    }
}

impl EntryKind {
    fn label(self) -> &'static str {
        match self {
            Self::Dir => "dir",
            Self::Exec => "exec",
            Self::File => "file",
            Self::Link => "link",
        }
    }

    fn style(self) -> &'static str {
        match self {
            Self::Dir => BLUE_BOLD,
            Self::Exec => GREEN_BOLD,
            Self::File => BOLD,
            Self::Link => MAGENTA_BOLD,
        }
    }
}

impl LsSummary {
    pub(super) fn observe(&mut self, kind: EntryKind) {
        match kind {
            EntryKind::Dir => self.dirs += 1,
            EntryKind::Exec => self.execs += 1,
            EntryKind::File => self.files += 1,
            EntryKind::Link => self.links += 1,
        }
    }
}

pub(super) fn describe_ls_entry(
    file_name: &str,
    entry_path: &Path,
    hidden_mode: HiddenMode,
) -> Result<LsRow> {
    let metadata = fs::symlink_metadata(entry_path)?;
    let modified = metadata
        .modified()
        .ok()
        .map(|time| {
            DateTime::<Local>::from(time)
                .format("%Y-%m-%d %H:%M")
                .to_string()
        })
        .unwrap_or_else(|| "-".to_string());
    let (kind, display_name) = if metadata.file_type().is_symlink() {
        (EntryKind::Link, file_name.to_string())
    } else if metadata.is_dir() {
        (EntryKind::Dir, format!("{file_name}/"))
    } else if metadata.permissions().mode() & 0o111 != 0 {
        (EntryKind::Exec, file_name.to_string())
    } else {
        (EntryKind::File, file_name.to_string())
    };

    let mut details = Vec::new();
    match kind {
        EntryKind::Dir => details.push(pluralize(
            count_visible_children(entry_path, hidden_mode)?,
            "item",
            "items",
        )),
        EntryKind::Link => {
            let target = fs::read_link(entry_path)
                .map(|path| path.display().to_string())
                .unwrap_or_else(|_| "?".to_string());
            details.push(format!("-> {target}"));
        }
        EntryKind::Exec | EntryKind::File => details.push(human_bytes(metadata.len())),
    }
    details.push(format!("updated {modified}"));
    Ok(LsRow {
        display_name,
        kind,
        details,
    })
}

pub(super) fn render_ls_section(target: &str, summary: &LsSummary, body: &str) -> String {
    let total = summary.dirs + summary.execs + summary.files + summary.links;
    let mut meta = vec![pluralize(total, "entry", "entries")];
    for (count, singular, plural) in [
        (summary.dirs, "dir", "dirs"),
        (summary.execs, "exec", "execs"),
        (summary.files, "file", "files"),
        (summary.links, "link", "links"),
    ] {
        if count > 0 {
            meta.push(pluralize(count, singular, plural));
        }
    }
    format!(
        "{} {}\n{}\n{}",
        paint(BLUE_BOLD, "ls"),
        paint(CYAN_BOLD, target),
        dim(meta.join(", ")),
        body
    )
}

pub(super) fn render_ls_row(out: &mut String, row: &LsRow) {
    let _ = writeln!(
        out,
        "{} {} {}",
        paint(row.kind.style(), &row.display_name),
        badge(row.kind.label(), row.kind.style()),
        dim(row.details.join(", "))
    );
}

fn count_visible_children(path: &Path, hidden_mode: HiddenMode) -> Result<usize> {
    let mut count = fs::read_dir(path)?
        .collect::<std::result::Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|entry| {
            hidden_mode.shows_hidden() || !entry.file_name().to_string_lossy().starts_with('.')
        })
        .count();
    if hidden_mode.shows_dot_entries() {
        count += 2;
    }
    Ok(count)
}
