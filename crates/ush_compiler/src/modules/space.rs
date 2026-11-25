//! One line numbering for every file of a linked program.
//!
//! The root script keeps its own line numbers and each module follows
//! it at an offset, the way a compiler's source map gives every file a
//! range of one position space. A statement's `line` therefore names
//! its file as well, and survives every pass that copies it.

use alloc::string::String;
use anyhow::{Error, anyhow};

use crate::types::HeapVec as Vec;

#[derive(Debug, Clone)]
pub(crate) struct SourceFile {
    /// Lines of all files before this one.
    pub base: usize,
    pub lines: usize,
    /// How compile-time diagnostics name the file.
    pub display: String,
    /// How runtime failure reports name the file.
    pub origin: String,
    /// The root-script line whose `mod` first brought this file in.
    pub root_line: usize,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SourceSpace {
    files: Vec<SourceFile>,
    text: String,
}

impl SourceSpace {
    /// Adds a file at the end of the space and returns its first position.
    pub(crate) fn push(&mut self, text: &str, display: String, origin: String, root_line: usize) {
        let base = self.files.last().map_or(0, |file| file.base + file.lines);
        let lines = text.lines().count();
        self.text.push_str(text);
        if !text.is_empty() && !text.ends_with('\n') {
            self.text.push('\n');
        }
        self.files.push(SourceFile {
            base,
            lines,
            display,
            origin,
            root_line,
        });
    }

    pub(crate) fn file(&self, index: usize) -> &SourceFile {
        &self.files[index]
    }

    /// Every file's lines, root first, in position order.
    pub(crate) fn text(&self) -> &str {
        &self.text
    }

    pub(crate) fn has_modules(&self) -> bool {
        self.files.len() > 1
    }

    /// The module file and its own line for a position; `None` in the root.
    pub(crate) fn locate(&self, line: usize) -> Option<(&SourceFile, usize)> {
        locate(self.modules(), line)
    }

    /// The module files, for passes that locate positions without the text.
    pub(crate) fn modules(&self) -> &[SourceFile] {
        &self.files[1.min(self.files.len())..]
    }

    /// `line R: lib/gh.ush: line N` — the root `mod` line first, so an
    /// editor showing the root script has a line to underline.
    pub(crate) fn describe(file: &SourceFile, line: usize) -> String {
        format!("line {}: {}: line {line}", file.root_line, file.display)
    }

    /// Rewrites the innermost position of a compile error into the module
    /// file and line it came from. Errors located in the root keep their
    /// text.
    pub(crate) fn render_error(&self, error: Error) -> Error {
        relocate(error, |line| {
            self.locate(line)
                .map(|(file, local)| Self::describe(file, local))
        })
    }
}

/// The module file and its own line for a position among `modules`, the
/// files after the root script.
pub(crate) fn locate(modules: &[SourceFile], line: usize) -> Option<(&SourceFile, usize)> {
    let index = modules
        .partition_point(|file| file.base < line)
        .checked_sub(1)?;
    let file = &modules[index];
    (line <= file.base + file.lines).then(|| (file, line - file.base))
}

/// Replaces `line N` segments of an error with `locate(N)`'s rendering,
/// using the innermost one that `locate` claims and dropping the rest.
/// The result is one message, so `Display` shows the whole chain.
pub(crate) fn relocate(error: Error, locate: impl Fn(usize) -> Option<String>) -> Error {
    let rendered = format!("{error:#}");
    let segments = rendered.split(": ").collect::<Vec<_>>();
    let position = |segment: &str| segment.strip_prefix("line ")?.parse::<usize>().ok();
    let Some(location) = segments
        .iter()
        .rev()
        .find_map(|segment| locate(position(segment)?))
    else {
        return anyhow!("{rendered}");
    };
    let rest = segments
        .iter()
        .filter(|segment| position(segment).is_none())
        .copied()
        .collect::<Vec<_>>()
        .join(": ");
    anyhow!("{location}: {rest}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn space() -> SourceSpace {
        let mut space = SourceSpace::default();
        space.push(
            "mod \"lib/a.ush\"\nprint 1\n",
            "main.ush".into(),
            "main.ush".into(),
            0,
        );
        space.push(
            "fn f() {\n  g\n}\n",
            "lib/a.ush".into(),
            "lib/a.ush".into(),
            1,
        );
        space
    }

    #[test]
    fn positions_after_the_root_belong_to_modules() {
        let space = space();
        assert!(space.locate(2).is_none());
        let (file, line) = space.locate(4).unwrap();
        assert_eq!((file.display.as_str(), line), ("lib/a.ush", 2));
        assert!(space.locate(6).is_none());
        assert_eq!(space.text().lines().nth(3), Some("  g"));
    }

    #[test]
    fn the_innermost_module_position_names_the_file() {
        let space = space();
        let error = anyhow!("unknown function: g")
            .context("line 4")
            .context("line 3");
        assert_eq!(
            format!("{:#}", space.render_error(error)),
            "line 1: lib/a.ush: line 2: unknown function: g"
        );
        let root = anyhow!("unknown function: g").context("line 2");
        assert_eq!(
            format!("{:#}", space.render_error(root)),
            "line 2: unknown function: g"
        );
    }
}
