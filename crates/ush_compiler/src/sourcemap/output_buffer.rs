use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use core::mem;

use crate::types::OutputString;

use super::{CompiledScript, SourceMap, SourceMapLine, SourceMapSection};

#[derive(Debug, Clone)]
pub(crate) struct OutputBuffer {
    text: OutputString,
    line_origins: Vec<Option<usize>>,
    line_sections: Vec<SourceMapSection>,
    verbatim: BTreeSet<usize>,
    current_origin: Option<usize>,
    current_section: SourceMapSection,
}

impl Default for OutputBuffer {
    fn default() -> Self {
        Self {
            text: OutputString::new(),
            line_origins: Vec::new(),
            line_sections: Vec::new(),
            verbatim: BTreeSet::new(),
            current_origin: None,
            current_section: SourceMapSection::UserCode,
        }
    }
}

impl OutputBuffer {
    /// Root buffer for a whole program. Every script carries the
    /// shared runtime prelude, so the final text is always at least
    /// this large — reserving up front avoids a dozen reallocations
    /// and copies of a growing buffer.
    pub(crate) fn for_program(section: SourceMapSection) -> Self {
        const EXPECTED_BYTES: usize = 2 * 1024;
        const EXPECTED_LINES: usize = 64;

        Self {
            text: OutputString::with_capacity(EXPECTED_BYTES),
            line_origins: Vec::with_capacity(EXPECTED_LINES),
            line_sections: Vec::with_capacity(EXPECTED_LINES),
            verbatim: BTreeSet::new(),
            current_origin: None,
            current_section: section,
        }
    }

    pub(crate) fn push_str(&mut self, value: &str) {
        self.text.push_str(value);
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
        if ch == '\n' {
            self.line_origins.push(self.current_origin);
            self.line_sections.push(self.current_section);
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.text
    }

    pub(crate) fn append(&mut self, mut other: Self) {
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
        for (index, (line, (origin, section))) in other
            .text
            .lines()
            .zip(other.completed_line_metadata())
            .enumerate()
        {
            let previous_origin = self.set_origin(origin);
            let previous_section = self.set_section(section);
            if other.verbatim.contains(&index) {
                self.verbatim.insert(self.line_origins.len());
            } else {
                self.push_indent(indent);
            }
            self.push_str(line);
            self.push('\n');
            self.set_origin(previous_origin);
            self.set_section(previous_section);
        }
    }

    /// Shell source has significant columns (here-doc delimiters and bodies).
    pub(crate) fn push_shell_source(&mut self, source: &str) {
        let origin = self.current_origin;
        for (offset, line) in source.split('\n').enumerate() {
            self.verbatim.insert(self.line_origins.len());
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

    fn completed_line_metadata(
        &self,
    ) -> impl Iterator<Item = (Option<usize>, SourceMapSection)> + '_ {
        let trailing = (!self.text.is_empty() && !self.text.ends_with('\n'))
            .then_some((self.current_origin, self.current_section));
        self.line_origins
            .iter()
            .copied()
            .zip(self.line_sections.iter().copied())
            .chain(trailing)
    }

    fn finish_partial_line(&mut self) {
        if !self.text.is_empty() && !self.text.ends_with('\n') {
            self.line_origins.push(self.current_origin);
            self.line_sections.push(self.current_section);
        }
    }
}
