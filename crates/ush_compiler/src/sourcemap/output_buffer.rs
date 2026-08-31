use alloc::vec::Vec;
use core::{iter, mem};

use crate::types::OutputString;

use super::{CompiledScript, SourceMap, SourceMapLine, SourceMapSection};

#[derive(Debug, Clone)]
pub(crate) struct OutputBuffer {
    text: OutputString,
    line_origins: Vec<Option<usize>>,
    line_sections: Vec<SourceMapSection>,
    current_origin: Option<usize>,
    current_section: SourceMapSection,
}

impl Default for OutputBuffer {
    fn default() -> Self {
        Self {
            text: OutputString::new(),
            line_origins: Vec::new(),
            line_sections: Vec::new(),
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
        const EXPECTED_BYTES: usize = 24 * 1024;
        const EXPECTED_LINES: usize = 640;

        Self {
            text: OutputString::with_capacity(EXPECTED_BYTES),
            line_origins: Vec::with_capacity(EXPECTED_LINES),
            line_sections: Vec::with_capacity(EXPECTED_LINES),
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
        for (line, (origin, section)) in other.text.lines().zip(other.completed_line_metadata()) {
            let previous_origin = self.set_origin(origin);
            let previous_section = self.set_section(section);
            self.push_indent(indent);
            self.push_str(line);
            self.push('\n');
            self.set_origin(previous_origin);
            self.set_section(previous_section);
        }
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
        let generated_lines = self
            .text
            .lines()
            .map(OutputString::from)
            .collect::<Vec<_>>();
        debug_assert_eq!(generated_lines.len(), self.line_origins.len());
        debug_assert_eq!(generated_lines.len(), self.line_sections.len());
        let source_lines = source
            .map(|value| value.lines().map(OutputString::from).collect::<Vec<_>>())
            .unwrap_or_default();
        let sourcemap = SourceMap {
            lines: self
                .line_origins
                .into_iter()
                .zip(self.line_sections)
                .zip(
                    generated_lines
                        .into_iter()
                        .chain(iter::repeat_with(OutputString::default)),
                )
                .enumerate()
                .map(|(index, ((source_line, section), generated_text))| {
                    let source_text = source_line
                        .and_then(|line| source_lines.get(line.saturating_sub(1)))
                        .cloned();
                    SourceMapLine {
                        generated_line: index + 1,
                        section,
                        source_line,
                        generated_text,
                        source_text,
                    }
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
