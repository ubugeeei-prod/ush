#[cfg(test)]
mod tests;

use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

pub(crate) const RESET: &str = "\u{1b}[0m";
pub(crate) const BOLD: &str = "\u{1b}[1m";
pub(crate) const DIM: &str = "\u{1b}[2m";
pub(crate) const BLUE_BOLD: &str = "\u{1b}[1;34m";
pub(crate) const CYAN_BOLD: &str = "\u{1b}[1;36m";
pub(crate) const GREEN_BOLD: &str = "\u{1b}[1;32m";
pub(crate) const YELLOW_BOLD: &str = "\u{1b}[1;33m";
pub(crate) const MAGENTA_BOLD: &str = "\u{1b}[1;35m";
pub(crate) const RED_BOLD: &str = "\u{1b}[1;31m";

pub(crate) fn human_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    if bytes < 1024 {
        return format!("{bytes} B");
    }

    let mut value = bytes as f64;
    let mut unit_index = 0usize;
    while value >= 1024.0 && unit_index < UNITS.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }

    if value >= 10.0 {
        format!("{value:.0} {}", UNITS[unit_index])
    } else {
        format!("{value:.1} {}", UNITS[unit_index])
    }
}

pub(crate) fn pluralize(count: usize, singular: &str, plural: &str) -> String {
    format!("{count} {}", if count == 1 { singular } else { plural })
}

pub(crate) fn paint(style: &str, value: impl Display) -> String {
    format!("{style}{value}{RESET}")
}

pub(crate) fn dim(value: impl Display) -> String {
    paint(DIM, value)
}

pub(crate) fn badge(value: impl Display, style: &str) -> String {
    format!("{style}[{value}]{RESET}")
}

pub(crate) fn normalize_path(cwd: &Path, value: &str) -> PathBuf {
    let path = Path::new(value);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}
