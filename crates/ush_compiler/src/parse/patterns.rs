//! `glob "…"` and `a | b` match patterns.

use anyhow::{Result, anyhow, bail};

use super::expr::parse_pattern;
use crate::{
    ast::{Expr, Pattern, PatternFields, Statement, StatementKind},
    types::HeapVec as Vec,
    util::parse_string_literal,
};

/// `glob "*.ts"`: the pattern is a string literal checked here, so a
/// malformed bracket expression is a compile error.
pub(super) fn parse_glob(source: &str) -> Result<Pattern> {
    let pattern = parse_string_literal(source.trim())
        .ok_or_else(|| anyhow!("`glob` takes a string literal, as in `glob \"*.ts\"`"))?;
    crate::glob::lower(&pattern)?;
    Ok(Pattern::Glob(pattern))
}

/// `a | b | c`, flattened. Alternatives cannot bind names: which
/// alternative matched is only known at run time.
pub(super) fn parse_alternatives(parts: &[&str]) -> Result<Pattern> {
    let mut alternatives = Vec::with_capacity(parts.len());
    for part in parts {
        if part.trim().is_empty() {
            bail!("empty alternative in a `|` pattern");
        }
        match parse_pattern(part)? {
            Pattern::Or(inner) => alternatives.extend(inner),
            pattern if binds(&pattern) => bail!(
                "`{}`: an alternative in a `|` pattern cannot bind a name",
                part.trim()
            ),
            pattern => alternatives.push(pattern),
        }
    }
    Ok(Pattern::Or(alternatives))
}

fn binds(pattern: &Pattern) -> bool {
    match pattern {
        Pattern::Binding(_) => true,
        Pattern::Or(alternatives) => alternatives.iter().any(binds),
        Pattern::Variant(variant) => match &variant.fields {
            PatternFields::Unit => false,
            PatternFields::Tuple(items) => items.iter().any(binds),
            PatternFields::Struct(fields) => fields.iter().any(|field| binds(&field.pattern)),
        },
        _ => false,
    }
}

/// Split `pattern => body` at the first `=>` outside a string.
pub(super) fn split_arm(source: &str) -> Option<(&str, &str)> {
    let mut state = crate::scan::ScanState::default();
    let mut index = 0usize;
    while index < source.len() {
        if state.top_level() && source[index..].starts_with("=>") {
            return Some((&source[..index], &source[index + 2..]));
        }
        index = crate::scan::advance(source, index, &mut state);
    }
    None
}

/// The loop variable of `for name in …`. `for (a, b) in …` binds a hidden
/// tuple and starts the body with `let a = tuple.0` and `let b = tuple.1`;
/// `_` skips an element.
pub(super) fn parse_for_binding(
    source: &str,
    line_no: usize,
) -> Result<(crate::types::AstString, Vec<Statement>)> {
    let Some(inner) = source
        .strip_prefix('(')
        .and_then(|rest| rest.strip_suffix(')'))
    else {
        return Ok((super::declaration_support::parse_name(source)?, Vec::new()));
    };
    let names = inner.split(',').map(str::trim).collect::<Vec<_>>();
    if names.len() < 2 {
        bail!("`for ({inner}) in …` needs two or more names; write `for {inner} in …` for one");
    }
    let tuple: crate::types::AstString = format!("__ush_for_{line_no}").into();
    let mut lets = Vec::new();
    for (index, name) in names.into_iter().enumerate() {
        if name == "_" {
            continue;
        }
        let name = super::declaration_support::parse_name(name)?;
        lets.push(Statement::new(
            line_no,
            StatementKind::Let {
                name,
                expr: Expr::Field {
                    base: alloc::boxed::Box::new(Expr::Var(tuple.clone())),
                    name: format!("{index}").into(),
                },
            },
        ));
    }
    Ok((tuple, lets))
}
