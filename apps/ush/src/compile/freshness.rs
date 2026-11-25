//! `ush compile --check`: is the committed output what the compiler
//! produces now?

use std::{fs, io::ErrorKind, path::Path};

use anyhow::{Context, Result, bail};

/// Lines shown on each side of a stale hunk before it is cut short.
const HUNK_LINES: usize = 12;

/// A missing output directory means a missing output, not a compile error.
pub fn ensure_dir(output: &Path) -> Result<()> {
    match output.parent().filter(|dir| !dir.as_os_str().is_empty()) {
        Some(dir) if !dir.is_dir() => bail!("{} is missing", output.display()),
        _ => Ok(()),
    }
}

pub fn check(expected: &str, output: &Path) -> Result<()> {
    let current = match fs::read_to_string(output) {
        Ok(current) => current,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            bail!("{} is missing", output.display())
        }
        Err(error) => {
            return Err(error).with_context(|| format!("failed to read {}", output.display()));
        }
    };
    if current == expected {
        return Ok(());
    }
    bail!(
        "{} is stale\n{}",
        output.display(),
        hunk(output, &current, expected)
    )
}

/// The first and last differing lines as one unified-diff hunk; the
/// matching prefix and suffix are left out.
fn hunk(path: &Path, current: &str, expected: &str) -> String {
    let old = current.lines().collect::<Vec<_>>();
    let new = expected.lines().collect::<Vec<_>>();
    let prefix = old.iter().zip(&new).take_while(|(a, b)| a == b).count();
    let suffix = old[prefix..]
        .iter()
        .rev()
        .zip(new[prefix..].iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    let removed = &old[prefix..old.len() - suffix];
    let added = &new[prefix..new.len() - suffix];
    let mut out = format!(
        "--- {0} (committed)\n+++ {0} (generated)\n@@ -{1},{2} +{1},{3} @@\n",
        path.display(),
        prefix + 1,
        removed.len(),
        added.len()
    );
    for (sign, lines) in [('-', removed), ('+', added)] {
        for line in lines.iter().take(HUNK_LINES) {
            out.push_str(&format!("{sign}{line}\n"));
        }
        if lines.len() > HUNK_LINES {
            out.push_str(&format!(
                "{sign}… {} more lines\n",
                lines.len() - HUNK_LINES
            ));
        }
    }
    if current.lines().eq(expected.lines()) {
        out.push_str("(line endings or the final newline differ)\n");
    }
    out.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::hunk;
    use std::path::Path;

    #[test]
    fn a_hunk_shows_only_the_changed_lines() {
        let rendered = hunk(Path::new("x.sh"), "a\nb\nc\nd\n", "a\nB\nc\nd\n");
        assert_eq!(
            rendered,
            "--- x.sh (committed)\n+++ x.sh (generated)\n@@ -2,1 +2,1 @@\n-b\n+B"
        );
    }

    #[test]
    fn insertions_and_newline_changes_are_described() {
        let rendered = hunk(Path::new("x.sh"), "a\nc\n", "a\nb\nc\n");
        assert!(rendered.ends_with("@@ -2,0 +2,1 @@\n+b"), "{rendered}");
        let rendered = hunk(Path::new("x.sh"), "a\n", "a");
        assert!(rendered.contains("final newline"), "{rendered}");
    }
}
