use anyhow::{Result, anyhow};

use super::{
    super::{
        ast::{StatementKind, UseItem},
        util::{parse_brace_body, split_top_level},
    },
    path::{default_import_alias, parse_identifier, parse_import_path},
};
use crate::types::HeapVec as Vec;

pub(super) fn parse_use(source: &str) -> Result<StatementKind> {
    let trimmed = source.trim();
    if let Some(prefix) = trimmed.strip_suffix("::*") {
        let path = if prefix == "$bin" {
            "$bin".into()
        } else {
            parse_import_path(prefix)?
        };
        return Ok(StatementKind::Use(vec![UseItem {
            path,
            alias: "*".into(),
        }]));
    }
    if trimmed.ends_with("::") {
        anyhow::bail!("wildcard imports must use `use module::*`");
    }
    if let Some((head, inner)) = parse_brace_body(trimmed) {
        let head = head
            .trim()
            .strip_suffix("::")
            .ok_or_else(|| anyhow!("grouped imports must use `module::{{item}}`"))?;
        let prefix = if head == "$bin" {
            head.into()
        } else {
            parse_import_path(head)?
        };
        let mut items = Vec::new();
        for item in split_top_level(inner, ',') {
            if item.is_empty() {
                continue;
            }
            items.push(parse_group_item(&prefix, item)?);
        }
        return Ok(StatementKind::Use(items));
    }
    Ok(StatementKind::Use(vec![parse_item(trimmed)?]))
}

fn parse_group_item(prefix: &str, source: &str) -> Result<UseItem> {
    let (name, alias) = split_alias(source)?;
    if prefix == "$bin" {
        return parse_program(name, alias);
    }
    let name = parse_identifier(name)?;
    Ok(UseItem {
        path: format!("{prefix}::{name}").into(),
        alias: alias.map_or_else(|| name.clone(), Into::into),
    })
}

fn parse_item(source: &str) -> Result<UseItem> {
    let (path, alias) = split_alias(source)?;
    let path = parse_import_path(path)?;
    Ok(UseItem {
        alias: alias.map_or_else(|| default_import_alias(&path), Ok)?,
        path,
    })
}

/// `vp` or `"apt-get" as apt_get`: a quoted name must be given a ush alias.
fn parse_program(name: &str, alias: Option<crate::types::AstString>) -> Result<UseItem> {
    let (program, quoted) = match name
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
    {
        Some(program) => (program, true),
        None => (name, false),
    };
    if !crate::commands::is_program_name(program) {
        anyhow::bail!("invalid program name: {name}");
    }
    let alias = match alias {
        Some(alias) => alias,
        None if !quoted => parse_identifier(program)?,
        None => anyhow::bail!("give the program {name} a ush name: `{name} as name`"),
    };
    Ok(UseItem {
        path: format!("$bin::{program}").into(),
        alias,
    })
}

fn split_alias(source: &str) -> Result<(&str, Option<crate::types::AstString>)> {
    if let Some((path, alias)) = source.split_once(" as ") {
        return Ok((path.trim(), Some(parse_identifier(alias)?)));
    }
    Ok((source.trim(), None))
}
