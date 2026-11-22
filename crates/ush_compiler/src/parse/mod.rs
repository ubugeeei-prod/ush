mod attr;
mod compare;
mod condition;
mod control;
mod declaration;
mod declaration_support;
mod effect_decl;
mod expr;
mod expr_support;
mod handle;
mod impls;
mod inline;
mod params;
mod path;
mod record;
mod returns;
mod signature;
mod statement;
mod statement_support;
mod use_decl;

use anyhow::{Result, bail};
use bumpalo::{
    Bump,
    collections::{String as BumpString, Vec as BumpVec},
};

use super::ast::Statement;
use crate::scan::triple_quote_toggles;
use crate::types::HeapVec as Vec;

pub(super) type SourceLine<'a> = (usize, &'a str);

pub(crate) fn parse_program(source: &str) -> Result<Vec<Statement>> {
    let arena = Bump::new();
    let lines = collect_source_lines(source, &arena)?;
    let mut cursor = 0usize;
    let mut program = statement::parse_block(&lines, &mut cursor, true, false)?;

    if cursor < lines.len() {
        let (line_no, line) = &lines[cursor];
        bail!("line {line_no}: unexpected closing brace: {}", line.trim());
    }

    returns::resolve_rows(&mut program)?;
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
        lines.push((line_no, line));
    }

    if in_multiline {
        bail!("line {block_start}: unterminated multiline string");
    }
    Ok(lines)
}
