use anyhow::{Result, anyhow, bail};

use super::{
    super::{
        ast::{FunctionDef, StatementKind, TraitDef, TraitImpl, Type},
        util::{parse_type, split_top_level, split_top_level_whitespace},
    },
    SourceLine,
    declaration_support::finish_block,
    signature,
};
use crate::types::{AstString as String, HeapVec as Vec};

pub(super) fn parse_trait(
    header: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<StatementKind> {
    let name = parse_empty_item(header, lines, cursor, "trait declaration")?;
    Ok(StatementKind::Trait(TraitDef { name }))
}

pub(super) fn parse_impl(
    header: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<StatementKind> {
    if let Some(inner) = header.trim().strip_suffix("{}") {
        *cursor += 1;
        return parse_impl_header(inner.trim(), Vec::new());
    }
    let source = header
        .trim()
        .strip_suffix('{')
        .ok_or_else(|| anyhow!("expected `{{` after impl header"))?
        .trim();
    let (trait_name, ty) = parse_impl_target(source)?;
    let methods = parse_methods(&ty, lines, cursor)?;
    Ok(StatementKind::Impl(TraitImpl {
        trait_name,
        ty,
        methods,
    }))
}

fn parse_methods(
    receiver: &Type,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<Vec<FunctionDef>> {
    let mut methods = Vec::new();
    *cursor += 1;
    while *cursor < lines.len() {
        let (line_no, line) = lines[*cursor];
        let trimmed = line.trim();
        if trimmed == "}" {
            break;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') {
            *cursor += 1;
            continue;
        }
        let Some(rest) = trimmed.strip_prefix("fn ") else {
            bail!("only `fn` items are supported inside impl blocks");
        };
        methods.push(parse_method(line_no, rest, lines, cursor, receiver)?);
    }
    let _ = finish_block(lines, cursor, "impl block")?;
    Ok(methods)
}

fn parse_method(
    _line_no: usize,
    header: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
    receiver: &Type,
) -> Result<FunctionDef> {
    let head = header
        .strip_suffix('{')
        .ok_or_else(|| anyhow!("expected `{{` after method signature"))?
        .trim();
    let rewritten = rewrite_receiver(head, receiver)?;
    let (name, mut params, return_type, declared_errors, declared_effects) =
        signature::parse_function_header(&rewritten)?;
    let Some(first) = params.first() else {
        bail!("methods must declare `self` as the first parameter");
    };
    if first.name != "self" || first.ty != *receiver {
        bail!("methods must declare `self` as the first parameter");
    }
    params.remove(0);
    *cursor += 1;
    let body = super::statement::parse_block(lines, cursor, false, return_type.is_some())?;
    let _ = finish_block(lines, cursor, "method body")?;
    Ok(FunctionDef {
        declared_effects,
        attrs: Vec::new(),
        name,
        receiver: Some(receiver.clone()),
        params,
        return_type,
        declared_errors,
        body,
    })
}

fn rewrite_receiver(header: &str, receiver: &Type) -> Result<String> {
    let open = header
        .find('(')
        .ok_or_else(|| anyhow!("methods must use `fn name(self, ...)`"))?;
    let close = header
        .rfind(')')
        .ok_or_else(|| anyhow!("methods must use `fn name(self, ...)`"))?;
    let name = header[..open].trim();
    let inner = header[open + 1..close].trim();
    let tail = header[close + 1..].trim();
    let params = split_top_level(inner, ',');
    let mut rewritten: Vec<String> = Vec::new();
    for (index, param) in params
        .into_iter()
        .filter(|part| !part.is_empty())
        .enumerate()
    {
        if index == 0 && param.trim() == "self" {
            rewritten.push(format!("self: {}", receiver.render()).into());
        } else {
            rewritten.push(param.trim().into());
        }
    }
    let args = rewritten.join(", ");
    Ok(format!("{name}({args}) {tail}").trim().into())
}

fn parse_impl_header(source: &str, methods: Vec<FunctionDef>) -> Result<StatementKind> {
    let (trait_name, ty) = parse_impl_target(source)?;
    Ok(StatementKind::Impl(TraitImpl {
        trait_name,
        ty,
        methods,
    }))
}

fn parse_impl_target(source: &str) -> Result<(Option<String>, Type)> {
    let parts = split_top_level_whitespace(source);
    match parts.as_slice() {
        [ty] => Ok((
            None,
            parse_type(ty).ok_or_else(|| anyhow!("invalid type: {ty}"))?,
        )),
        [trait_name, "for", ty] => Ok((
            Some((*trait_name).trim().into()),
            parse_type(ty).ok_or_else(|| anyhow!("invalid type: {ty}"))?,
        )),
        _ => bail!("expected `impl Type {{}}` or `impl Trait for Type {{}}`"),
    }
}

fn parse_empty_item(
    header: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
    kind: &str,
) -> Result<String> {
    let trimmed = header.trim();
    if let Some(inner) = trimmed.strip_suffix("{}") {
        *cursor += 1;
        return Ok(inner.trim().into());
    }
    let head = trimmed
        .strip_suffix('{')
        .ok_or_else(|| anyhow!("expected empty body for {kind}"))?
        .trim();
    *cursor += 1;
    let _ = finish_block(lines, cursor, kind)?;
    Ok(head.into())
}
