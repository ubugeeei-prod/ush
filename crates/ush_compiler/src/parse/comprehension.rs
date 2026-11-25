use super::{expr::parse_expr, path::parse_identifier};
use crate::{
    ast::{Comprehension, ComprehensionClause},
    util::split_top_level_whitespace,
};
use anyhow::{Result, anyhow, ensure};

pub(super) fn parse(source: &str) -> Result<Option<Comprehension>> {
    let parts = split_top_level_whitespace(source);
    let Some(first) = parts.iter().position(|p| *p == "for") else {
        return Ok(None);
    };
    ensure!(
        first > 0,
        "list comprehension requires a value before `for`"
    );
    let value = parse_expr(&parts[..first].join(" "))?;
    let mut clauses = alloc::vec::Vec::new();
    let mut i = first;
    while i < parts.len() {
        let keyword = parts[i];
        i += 1;
        let name = if keyword == "for" {
            let name = parse_identifier(
                parts
                    .get(i)
                    .ok_or_else(|| anyhow!("expected comprehension binding"))?,
            )?;
            ensure!(
                parts.get(i + 1) == Some(&"in"),
                "expected `in` after comprehension binding"
            );
            i += 2;
            Some(name)
        } else {
            ensure!(keyword == "if", "expected `for` or `if` in comprehension");
            None
        };
        let start = i;
        while i < parts.len() && !matches!(parts[i], "for" | "if") {
            i += 1;
        }
        ensure!(
            i > start,
            "missing expression after comprehension `{keyword}`"
        );
        let expr = parse_expr(&parts[start..i].join(" "))?;
        clauses.push(match name {
            Some(name) => ComprehensionClause::For {
                name,
                iterable: expr,
            },
            None => ComprehensionClause::If(expr),
        });
    }
    Ok(Some(Comprehension { value, clauses }))
}
