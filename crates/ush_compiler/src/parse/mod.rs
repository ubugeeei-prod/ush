mod argument;
mod atoms;
mod attr;
mod compare;
mod comprehension;
mod condition;
mod control;
mod declaration;
mod declaration_support;
mod effect_decl;
mod expr;
mod expr_support;
mod external;
mod handle;
mod impls;
mod inline;
mod items;
mod module_decl;
mod params;
mod path;
mod patterns;
mod pipeline;
mod process;
mod record;
mod returns;
mod scoped;
mod signature;
mod statement;
mod statement_support;
mod testing;
mod use_decl;

use anyhow::{Result, bail};
use bumpalo::{
    Bump,
    collections::{String as BumpString, Vec as BumpVec},
};

use super::ast::Statement;
use crate::scan::{continues_pipeline, triple_quote_toggles};
use crate::types::HeapVec as Vec;

pub(super) type SourceLine<'a> = (usize, &'a str);
pub(crate) use returns::resolve_rows;

/// How the top level of a file is read.
#[derive(Clone, Copy)]
pub(crate) enum FileRole<'a> {
    /// A script: declarations and statements.
    Script,
    /// A module: declarations only. `dir` is the module's directory
    /// relative to the root script's, so its `extern` libraries are
    /// sourced from beside the module.
    Module { dir: &'a str },
}

pub(crate) fn parse_program(source: &str) -> Result<Vec<Statement>> {
    parse_file(source, FileRole::Script)
}

pub(crate) fn parse_file(source: &str, role: FileRole<'_>) -> Result<Vec<Statement>> {
    let arena = Bump::new();
    let lines = join_pipeline_lines(collect_source_lines(source, &arena)?, &arena)?;
    let mut cursor = 0usize;
    let program = statement::parse_block(&lines, &mut cursor, Some(role), false)?;

    if cursor < lines.len() {
        let (line_no, line) = &lines[cursor];
        bail!("line {line_no}: unexpected closing brace: {}", line.trim());
    }

    Ok(program)
}

fn collect_source_lines<'a>(
    source: &'a str,
    arena: &'a Bump,
) -> Result<BumpVec<'a, SourceLine<'a>>> {
    let mut lines = BumpVec::new_in(arena);
    let mut block = BumpString::new_in(arena);
    let mut block_start = 0usize;
    let mut in_multiline = false;

    let mut source_lines = source.lines().enumerate();
    while let Some((index, line)) = source_lines.next() {
        let line_no = index + 1;
        if in_multiline {
            block.push('\n');
            block.push_str(line);
            if triple_quote_toggles(line) % 2 == 1 {
                let joined = arena.alloc_str(block.as_str());
                lines.push((block_start, &*joined));
                block.clear();
                in_multiline = false;
            }
            continue;
        }
        if super::scan::opens_shell_capture(line) {
            block.push_str(line);
            let indent = &line[..line.len() - line.trim_start().len()];
            loop {
                let (_, next) = source_lines
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("line {line_no}: unterminated shell capture"))?;
                block.push('\n');
                block.push_str(next);
                if super::scan::closes_shell_capture(next, indent) {
                    break;
                }
            }
            lines.push((line_no, &*arena.alloc_str(block.as_str())));
            block.clear();
            continue;
        }
        if triple_quote_toggles(line) % 2 == 1 {
            block_start = line_no;
            block.push_str(line);
            in_multiline = true;
            continue;
        }
        if let Some(branch) = same_line_else(line) {
            lines.push((line_no, "}"));
            lines.push((line_no, branch));
            continue;
        }
        lines.push((line_no, line));
    }

    if in_multiline {
        bail!("line {block_start}: unterminated multiline string");
    }
    Ok(lines)
}

/// `} else {` and `} else if … {` close one branch and open the next on one line;
/// the branch is read as if it started its own line.
fn same_line_else(line: &str) -> Option<&str> {
    let branch = line.trim_start().strip_prefix('}')?.trim_start();
    (branch.starts_with("else {") || branch.starts_with("else if ")).then_some(branch)
}

/// Join a line that ends with `|` or `|&` with the line holding the next
/// stage. Blank lines and comments between the stages are skipped.
fn join_pipeline_lines<'a>(
    lines: BumpVec<'a, SourceLine<'a>>,
    arena: &'a Bump,
) -> Result<BumpVec<'a, SourceLine<'a>>> {
    let mut joined = BumpVec::with_capacity_in(lines.len(), arena);
    let mut pending: Option<(usize, BumpString<'a>)> = None;
    for (line_no, line) in lines {
        let Some((start, text)) = &mut pending else {
            if continues_pipeline(line) {
                pending = Some((line_no, BumpString::from_str_in(line, arena)));
            } else {
                joined.push((line_no, line));
            }
            continue;
        };
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        text.push('\n');
        text.push_str(line);
        if !continues_pipeline(line) {
            joined.push((*start, &*arena.alloc_str(text)));
            pending = None;
        }
    }
    if let Some((start, _)) = pending {
        bail!("line {start}: the pipeline continues past the end of the file");
    }
    Ok(joined)
}
