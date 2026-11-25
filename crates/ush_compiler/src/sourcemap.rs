mod output_buffer;

use alloc::{collections::BTreeMap, vec::Vec};
use core::fmt::Write as _;

use crate::types::OutputString;

pub(crate) use output_buffer::OutputBuffer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledScript {
    pub shell: OutputString,
    pub sourcemap: SourceMap,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SourceMap {
    pub lines: Vec<SourceMapLine>,
}

impl SourceMap {
    pub fn line(&self, generated_line: usize) -> Option<&SourceMapLine> {
        self.lines.get(generated_line.checked_sub(1)?)
    }

    pub fn source_line(&self, generated_line: usize) -> Option<usize> {
        self.line(generated_line).and_then(|line| line.source_line)
    }

    /// Generated lines for a line of the root script.
    pub fn generated_lines_for_source(&self, source_line: usize) -> Vec<usize> {
        self.lines
            .iter()
            .filter_map(|line| {
                (line.source_file.is_none() && line.source_line == Some(source_line))
                    .then_some(line.generated_line)
            })
            .collect()
    }

    /// Source lines with their generated lines: the root script's first,
    /// then each module's, by file and line.
    pub fn source_index(&self) -> Vec<SourceMapSourceLine> {
        let mut grouped = BTreeMap::<(Option<&str>, usize), SourceMapSourceLine>::new();
        for line in &self.lines {
            let Some(source_line) = line.source_line else {
                continue;
            };
            let entry = grouped
                .entry((line.source_file.as_deref(), source_line))
                .or_insert_with(|| SourceMapSourceLine {
                    source_file: line.source_file.clone(),
                    source_line,
                    source_text: line.source_text.clone(),
                    generated_lines: Vec::new(),
                });
            if entry.source_text.is_none() {
                entry.source_text = line.source_text.clone();
            }
            entry.generated_lines.push(line.generated_line);
        }
        grouped.into_values().collect()
    }

    pub fn summary(&self) -> SourceMapSummary {
        let mut mapped_line_count = 0usize;
        let mut first_mapped_generated_line = None::<usize>;
        let mut last_mapped_generated_line = None::<usize>;
        let mut section_counts = BTreeMap::<SourceMapSection, SourceMapSectionSummary>::new();

        for line in &self.lines {
            let entry =
                section_counts
                    .entry(line.section)
                    .or_insert_with(|| SourceMapSectionSummary {
                        section: line.section,
                        generated_line_count: 0,
                        mapped_line_count: 0,
                    });
            entry.generated_line_count += 1;

            if line.source_line.is_some() {
                mapped_line_count += 1;
                entry.mapped_line_count += 1;
                first_mapped_generated_line.get_or_insert(line.generated_line);
                last_mapped_generated_line = Some(line.generated_line);
            }
        }

        let generated_line_count = self.lines.len();
        let source_line_count = self.source_index().len();

        SourceMapSummary {
            generated_line_count,
            mapped_line_count,
            unmapped_line_count: generated_line_count.saturating_sub(mapped_line_count),
            source_line_count,
            first_mapped_generated_line,
            last_mapped_generated_line,
            sections: SourceMapSection::all()
                .into_iter()
                .filter_map(|section| section_counts.remove(&section))
                .collect(),
        }
    }

    pub fn render_listing(&self) -> OutputString {
        self.render_listing_inner(false)
    }

    pub fn render_mapped_listing(&self) -> OutputString {
        self.render_listing_inner(true)
    }

    fn render_listing_inner(&self, mapped_only: bool) -> OutputString {
        let mut out = OutputString::new();
        let summary = self.summary();
        if mapped_only {
            let _ = writeln!(
                out,
                "mapped {} / {} generated lines across {} source lines",
                summary.mapped_line_count, summary.generated_line_count, summary.source_line_count
            );
        } else {
            let _ = writeln!(
                out,
                "generated {} lines, mapped {}, unmapped {}, source {}",
                summary.generated_line_count,
                summary.mapped_line_count,
                summary.unmapped_line_count,
                summary.source_line_count
            );
        }
        if let (Some(first), Some(last)) = (
            summary.first_mapped_generated_line,
            summary.last_mapped_generated_line,
        ) {
            let _ = writeln!(out, "mapped span G{first:04}..G{last:04}");
        }

        let mut current_section = None::<SourceMapSection>;
        let mut rendered_any = false;
        for line in &self.lines {
            if mapped_only && line.source_line.is_none() {
                continue;
            }
            if current_section != Some(line.section) {
                if rendered_any {
                    out.push('\n');
                }
                let _ = writeln!(out, "-- {} --", line.section.label());
                current_section = Some(line.section);
            }
            let source = match (&line.source_file, line.source_line) {
                (Some(file), Some(value)) => format!("{file}:S{value:04}"),
                (None, Some(value)) => format!("S{value:04}"),
                (_, None) => OutputString::from("----"),
            };
            let _ = writeln!(
                out,
                "G{:04} -> {} | {}",
                line.generated_line, source, line.generated_text
            );
            if let Some(source_text) = &line.source_text {
                let _ = writeln!(out, "             <= {}", source_text);
            }
            rendered_any = true;
        }
        if !rendered_any {
            out.push_str("(no mapped lines)\n");
        }
        out
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum SourceMapSection {
    RuntimeSupport,
    DocSupport,
    #[default]
    UserCode,
}

impl SourceMapSection {
    pub fn all() -> [Self; 3] {
        [Self::RuntimeSupport, Self::DocSupport, Self::UserCode]
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::RuntimeSupport => "runtime-support",
            Self::DocSupport => "doc-support",
            Self::UserCode => "user-code",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceMapSourceLine {
    /// The module file, or `None` for the root script.
    pub source_file: Option<OutputString>,
    pub source_line: usize,
    pub source_text: Option<OutputString>,
    pub generated_lines: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceMapSummary {
    pub generated_line_count: usize,
    pub mapped_line_count: usize,
    pub unmapped_line_count: usize,
    pub source_line_count: usize,
    pub first_mapped_generated_line: Option<usize>,
    pub last_mapped_generated_line: Option<usize>,
    pub sections: Vec<SourceMapSectionSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceMapSectionSummary {
    pub section: SourceMapSection,
    pub generated_line_count: usize,
    pub mapped_line_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceMapLine {
    pub generated_line: usize,
    pub section: SourceMapSection,
    /// The module file `source_line` belongs to; `None` for the root script.
    pub source_file: Option<OutputString>,
    pub source_line: Option<usize>,
    pub generated_text: OutputString,
    pub source_text: Option<OutputString>,
}
