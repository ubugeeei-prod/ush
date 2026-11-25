//! Portable relative paths for generated artifacts and module names.

use anyhow::{Context, Result, ensure};
use std::path::Path;

/// `to` relative to `from`, both absolute and canonical, with `/`
/// separators; empty when they are the same directory.
pub(crate) fn relative(from: &Path, to: &Path) -> Result<String> {
    let from = from.components().collect::<Vec<_>>();
    let to = to.components().collect::<Vec<_>>();
    let common = from.iter().zip(&to).take_while(|(a, b)| a == b).count();
    let mut parts = vec![".."; from.len() - common];
    for component in &to[common..] {
        parts.push(portable(component.as_os_str().to_str(), "path")?);
    }
    Ok(parts.join("/"))
}

/// `directory/file`, or `file` when the directory is empty.
pub(crate) fn join(directory: &str, file: &str) -> String {
    match directory {
        "" => file.into(),
        _ => format!("{directory}/{file}"),
    }
}

/// Removes `.` and folds `name/..` without touching the filesystem.
/// Leading `..` and a leading `/` are kept.
pub(crate) fn normalize(path: &str) -> String {
    let absolute = path.starts_with('/');
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." if parts.last().is_some_and(|last| *last != "..") => {
                parts.pop();
            }
            ".." if absolute => {}
            _ => parts.push(part),
        }
    }
    let joined = parts.join("/");
    if absolute {
        format!("/{joined}")
    } else {
        joined
    }
}

/// The parent of a path as written, `/`-separated; empty for a bare name.
pub(crate) fn written_parent(path: &Path) -> String {
    path.parent()
        .and_then(Path::to_str)
        .unwrap_or("")
        .replace(std::path::MAIN_SEPARATOR, "/")
}

/// A UTF-8 name without line breaks, which generated shell can quote.
pub(crate) fn portable<'a>(name: Option<&'a str>, kind: &str) -> Result<&'a str> {
    let name = name.with_context(|| format!("{kind} must be valid UTF-8"))?;
    ensure!(
        !name.contains(['\n', '\r']),
        "{kind} must not contain a line break: {name:?}"
    );
    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_paths_climb_then_descend() {
        let rel = |a: &str, b: &str| relative(Path::new(a), Path::new(b)).unwrap();
        assert_eq!(rel("/repo/ci/compiled", "/repo/ci"), "..");
        assert_eq!(rel("/repo/ci", "/repo/ci"), "");
        assert_eq!(rel("/repo/a/b", "/repo/lib"), "../../lib");
        assert_eq!(rel("/repo", "/repo/lib/x"), "lib/x");
    }

    #[test]
    fn normalization_is_lexical() {
        assert_eq!(normalize("scripts/../lib/gh.ush"), "lib/gh.ush");
        assert_eq!(normalize("./a/./b"), "a/b");
        assert_eq!(normalize("../../lib/x.ush"), "../../lib/x.ush");
        assert_eq!(normalize("a/../../b"), "../b");
        assert_eq!(normalize("/repo/../lib"), "/lib");
        assert_eq!(join("", "x.ush"), "x.ush");
        assert_eq!(join("..", "x.ush"), "../x.ush");
    }
}
