use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use core::mem;

use crate::types::OutputString;

use super::{CompiledScript, SourceMap, SourceMapLine, SourceMapSection};

#[derive(Debug, Clone)]
pub(crate) struct OutputBuffer {
    text: OutputString,
    mapped: bool,
    line_origins: Vec<Option<usize>>,
    line_sections: Vec<SourceMapSection>,
    verbatim: BTreeSet<usize>,
    current_origin: Option<usize>,
    current_section: SourceMapSection,
}

impl OutputBuffer {
    pub(crate) fn new(mapped: bool) -> Self {
        Self {
            text: OutputString::new(),
            mapped,
            line_origins: Vec::new(),
            line_sections: Vec::new(),
            verbatim: BTreeSet::new(),
            current_origin: None,
            current_section: SourceMapSection::UserCode,
        }
    }

    /// Reserve shell text up front. Only a requested sidecar needs per-line metadata.
    pub(crate) fn for_program(section: SourceMapSection, mapped: bool) -> Self {
        const EXPECTED_BYTES: usize = 2 * 1024;
        const EXPECTED_LINES: usize = 64;

        Self {
            text: OutputString::with_capacity(EXPECTED_BYTES),
            mapped,
            line_origins: Vec::with_capacity(if mapped { EXPECTED_LINES } else { 0 }),
            line_sections: Vec::with_capacity(if mapped { EXPECTED_LINES } else { 0 }),
            verbatim: BTreeSet::new(),
            current_origin: None,
            current_section: section,
        }
    }

    pub(crate) fn push_str(&mut self, value: &str) {
        self.text.push_str(value);
        if !self.mapped {
            return;
        }
        // Codegen pushes multi-kilobyte runtime blobs through here,
        // so count line breaks over the bytes instead of decoding
        // every character.
        for _ in memchr::memchr_iter(b'\n', value.as_bytes()) {
            self.line_origins.push(self.current_origin);
            self.line_sections.push(self.current_section);
        }
    }

    pub(crate) fn push(&mut self, ch: char) {
        self.text.push(ch);
        if self.mapped && ch == '\n' {
            self.line_origins.push(self.current_origin);
            self.line_sections.push(self.current_section);
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.text
    }

    pub(crate) fn append(&mut self, mut other: Self) {
        let base = self.text.len();
        self.verbatim
            .extend(other.verbatim.iter().map(|pos| base + *pos));
        self.text.push_str(&other.text);
        self.line_origins.append(&mut other.line_origins);
        self.line_sections.append(&mut other.line_sections);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub(crate) fn set_origin(&mut self, line: Option<usize>) -> Option<usize> {
        mem::replace(&mut self.current_origin, line)
    }

    pub(crate) fn set_section(&mut self, section: SourceMapSection) -> SourceMapSection {
        mem::replace(&mut self.current_section, section)
    }

    pub(crate) fn append_buffer(&mut self, other: &Self, indent: usize) {
        let mut position = 0;
        for (index, line) in other.text.split_inclusive('\n').enumerate() {
            if other.verbatim.contains(&position) {
                self.verbatim.insert(self.text.len());
            } else {
                self.push_indent(indent);
            }
            self.text.push_str(line);
            if !line.ends_with('\n') {
                self.text.push('\n');
            }
            if self.mapped {
                self.line_origins.push(
                    other
                        .line_origins
                        .get(index)
                        .copied()
                        .unwrap_or(other.current_origin),
                );
                self.line_sections.push(
                    other
                        .line_sections
                        .get(index)
                        .copied()
                        .unwrap_or(other.current_section),
                );
            }
            position += line.len();
        }
    }

    /// Shell source has significant columns (here-doc delimiters and bodies).
    pub(crate) fn push_shell_source(&mut self, source: &str) {
        let origin = self.current_origin;
        for (offset, line) in source.split('\n').enumerate() {
            self.verbatim.insert(self.text.len());
            self.current_origin = origin.map(|n| n + offset);
            self.push_str(line);
            self.push('\n');
        }
        self.current_origin = origin;
    }

    pub(crate) fn finish(&mut self) {
        while self.text.ends_with("\n\n") {
            self.text.pop();
            self.line_origins.pop();
            self.line_sections.pop();
        }
    }

    pub(crate) fn into_shell(self) -> OutputString {
        self.text
    }

    /// Writes `indent` spaces without building a temporary string.
    /// Nested blocks re-indent every line they contain, so this runs
    /// once per generated line.
    pub(crate) fn push_indent(&mut self, indent: usize) {
        const SPACES: &str = "                                ";
        let mut remaining = indent;
        while remaining > 0 {
            let chunk = remaining.min(SPACES.len());
            self.text.push_str(&SPACES[..chunk]);
            remaining -= chunk;
        }
    }

    pub(crate) fn into_compiled(mut self, source: Option<&str>) -> CompiledScript {
        self.finish_partial_line();
        let source_lines: Vec<_> = source.unwrap_or("").lines().collect();
        let sourcemap = SourceMap {
            lines: self
                .text
                .lines()
                .zip(self.line_origins)
                .zip(self.line_sections)
                .enumerate()
                .map(|(index, ((text, source_line), section))| SourceMapLine {
                    generated_line: index + 1,
                    section,
                    source_line,
                    generated_text: text.into(),
                    source_text: source_line
                        .and_then(|n| source_lines.get(n - 1))
                        .map(|s| (*s).into()),
                })
                .collect(),
        };
        CompiledScript {
            shell: self.text,
            sourcemap,
        }
    }

    fn finish_partial_line(&mut self) {
        if !self.text.is_empty() && !self.text.ends_with('\n') {
            self.line_origins.push(self.current_origin);
            self.line_sections.push(self.current_section);
        }
    }
}
