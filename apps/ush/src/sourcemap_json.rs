use std::path::Path;

use anyhow::{Context, Result};
use serde::Serialize;
use ush_compiler::SourceMap;

#[derive(Serialize)]
struct JsonSourceMap {
    version: u32,
    source: String,
    generated: Option<String>,
    summary: JsonSourceMapSummary,
    sources: Vec<JsonSourceMapSourceLine>,
    lines: Vec<JsonSourceMapLine>,
}

#[derive(Serialize)]
struct JsonSourceMapLine {
    generated_line: usize,
    section: String,
    /// The module file; absent for the root script.
    #[serde(skip_serializing_if = "Option::is_none")]
    source_file: Option<String>,
    source_line: Option<usize>,
    generated_text: String,
    source_text: Option<String>,
}

#[derive(Serialize)]
struct JsonSourceMapSummary {
    generated_line_count: usize,
    mapped_line_count: usize,
    unmapped_line_count: usize,
    source_line_count: usize,
    first_mapped_generated_line: Option<usize>,
    last_mapped_generated_line: Option<usize>,
    sections: Vec<JsonSourceMapSectionSummary>,
}

#[derive(Serialize)]
struct JsonSourceMapSectionSummary {
    section: String,
    generated_line_count: usize,
    mapped_line_count: usize,
}

#[derive(Serialize)]
struct JsonSourceMapSourceLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_file: Option<String>,
    source_line: usize,
    source_text: Option<String>,
    generated_lines: Vec<usize>,
}

pub(super) fn write_sourcemap_file(
    path: &Path,
    input: &Path,
    output: Option<&Path>,
    sourcemap: &SourceMap,
) -> Result<()> {
    let summary = sourcemap.summary();
    let payload = JsonSourceMap {
        version: 2,
        source: input.display().to_string(),
        generated: output.map(|item| item.display().to_string()),
        summary: JsonSourceMapSummary {
            generated_line_count: summary.generated_line_count,
            mapped_line_count: summary.mapped_line_count,
            unmapped_line_count: summary.unmapped_line_count,
            source_line_count: summary.source_line_count,
            first_mapped_generated_line: summary.first_mapped_generated_line,
            last_mapped_generated_line: summary.last_mapped_generated_line,
            sections: summary
                .sections
                .into_iter()
                .map(|section| JsonSourceMapSectionSummary {
                    section: section.section.label().to_string(),
                    generated_line_count: section.generated_line_count,
                    mapped_line_count: section.mapped_line_count,
                })
                .collect(),
        },
        sources: sourcemap
            .source_index()
            .into_iter()
            .map(|line| JsonSourceMapSourceLine {
                source_file: line.source_file,
                source_line: line.source_line,
                source_text: line.source_text,
                generated_lines: line.generated_lines,
            })
            .collect(),
        lines: sourcemap
            .lines
            .iter()
            .map(|line| JsonSourceMapLine {
                generated_line: line.generated_line,
                section: line.section.label().to_string(),
                source_file: line.source_file.clone(),
                source_line: line.source_line,
                generated_text: line.generated_text.clone(),
                source_text: line.source_text.clone(),
            })
            .collect(),
    };
    let json = serde_json::to_string_pretty(&payload).context("failed to serialize sourcemap")?;
    std::fs::write(path, format!("{json}\n"))
        .with_context(|| format!("failed to write {}", path.display()))
}
