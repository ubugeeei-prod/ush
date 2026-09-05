use anyhow::{Result, anyhow, bail};

use super::{
    super::{
        ast::{Attribute, Call, CallArg, Expr, FunctionParam},
        errors::ErrorSet,
        util::{split_once_top_level, split_top_level, split_top_level_whitespace},
    },
    expr::parse_expr,
    params::parse_params,
    path::{looks_like_call_target, parse_call_target, parse_identifier},
    returns::parse_function_return,
};
use crate::scan::{ScanState, advance};
use crate::types::{AstString as String, HeapVec as Vec};

type ParsedFunctionHeader = (
    String,
    Vec<FunctionParam>,
    Option<super::super::ast::Type>,
    Option<ErrorSet>,
    Option<Vec<String>>,
);

pub(super) fn parse_function_header(header: &str) -> Result<ParsedFunctionHeader> {
    let (name, inner, tail) =
        split_paren_form(header).ok_or_else(|| anyhow!("functions must use `fn name(args)`"))?;
    let (return_type, declared_errors, declared_effects) = parse_function_return(tail)?;
    Ok((
        parse_identifier(name)?,
        parse_params(inner)?,
        return_type,
        declared_errors,
        declared_effects,
    ))
}

pub(super) fn parse_call(source: &str, asynchronous: bool) -> Result<Call> {
    parse_call_inner(source, asynchronous, true)
}

pub(super) fn looks_like_call(source: &str) -> bool {
    looks_like_call_inner(source, true)
}

pub(super) fn parse_expr_call(source: &str) -> Result<Option<Call>> {
    Ok(parse_call_parts(source, false)?.map(|(name, args)| Call {
        name,
        args,
        asynchronous: false,
    }))
}

pub(super) fn parse_dollar_call(source: &str, arg: super::super::ast::Expr) -> Result<Call> {
    let mut call = parse_call_inner(source, false, true)?;
    call.args.push(CallArg {
        label: None,
        expr: arg,
    });
    Ok(call)
}

pub(super) fn parse_call_args(source: &str) -> Result<Vec<CallArg>> {
    parse_args(source)
}

fn parse_call_inner(source: &str, asynchronous: bool, allow_bare_name: bool) -> Result<Call> {
    let (name, args) = parse_call_parts(source, allow_bare_name)?
        .ok_or_else(|| anyhow!("invalid function call: {source}"))?;
    Ok(Call {
        name,
        args,
        asynchronous,
    })
}

fn looks_like_call_inner(source: &str, allow_bare_name: bool) -> bool {
    split_paren_form(source)
        .map(|(name, _, tail)| looks_like_call_target(name) && tail.is_empty())
        .unwrap_or_else(|| {
            parse_call_parts(source, allow_bare_name)
                .ok()
                .flatten()
                .is_some()
        })
}

fn parse_call_parts(source: &str, allow_bare_name: bool) -> Result<Option<(String, Vec<CallArg>)>> {
    if let Some((name, inner, tail)) = split_paren_form(source) {
        if !tail.is_empty() {
            bail!("invalid function call: {source}");
        }
        return Ok(Some((parse_call_target(name)?, parse_args(inner)?)));
    }

    let parts = split_top_level_whitespace(source.trim());
    if parts.is_empty() || !looks_like_call_target(parts[0]) {
        return Ok(None);
    }
    if parts.len() == 1 && !allow_bare_name {
        return Ok(None);
    }

    let args = if parts.len() == 2 && parts[1] == "()" {
        Vec::new()
    } else {
        parse_call_tokens(&parts[1..])?
    };
    Ok(Some((parse_call_target(parts[0])?, args)))
}

fn parse_args(source: &str) -> Result<Vec<CallArg>> {
    split_top_level(source, ',')
        .into_iter()
        .filter(|part| !part.is_empty())
        .map(parse_call_arg)
        .collect()
}

pub(super) fn parse_await_task(source: &str) -> Result<Option<String>> {
    let trimmed = source.trim();
    if let Some(task) = trimmed.strip_suffix(".await") {
        return Ok(Some(parse_identifier(task.trim())?));
    }
    if let Some(task) = trimmed.strip_prefix("await ") {
        return Ok(Some(parse_identifier(task.trim())?));
    }
    Ok(None)
}

pub(super) fn split_paren_form(source: &str) -> Option<(&str, &str, &str)> {
    let mut state = ScanState::default();
    let mut open = None;
    let mut index = 0usize;
    while index < source.len() {
        if !state.in_string() && source.as_bytes()[index] == b'(' {
            open = Some(index);
            state.paren = 1;
            index += 1;
            break;
        }
        index = advance(source, index, &mut state);
    }
    let open = open?;
    while index < source.len() {
        if !state.in_string() && source.as_bytes()[index] == b'(' {
            state.paren += 1;
            index += 1;
            continue;
        }
        if !state.in_string() && source.as_bytes()[index] == b')' {
            state.paren = state.paren.saturating_sub(1);
            if state.paren == 0 {
                return Some((
                    source[..open].trim(),
                    source[open + 1..index].trim(),
                    source[index + 1..].trim(),
                ));
            }
            index += 1;
            continue;
        }
        index = advance(source, index, &mut state);
    }
    None
}

fn parse_call_tokens(parts: &[&str]) -> Result<Vec<CallArg>> {
    let mut args = Vec::new();
    let mut index = 0usize;
    while index < parts.len() {
        let current = parts[index];
        if let Some(label) = current.strip_suffix(':') {
            args.push(CallArg {
                label: Some(parse_identifier(label)?),
                expr: parse_expr(
                    parts
                        .get(index + 1)
                        .ok_or_else(|| anyhow!("missing value for `{label}`"))?,
                )?,
            });
            index += 2;
            continue;
        }
        args.push(parse_call_arg(current)?);
        index += 1;
    }
    Ok(args)
}

fn parse_call_arg(source: &str) -> Result<CallArg> {
    if let Some((name, expr)) = split_once_top_level(source, ':')
        && super::path::looks_like_call_target(name)
    {
        return Ok(CallArg {
            label: Some(parse_identifier(name)?),
            expr: parse_expr(expr)?,
        });
    }
    Ok(CallArg {
        label: None,
        expr: parse_expr(source)?,
    })
}

pub(super) fn attr_expr(attrs: &[Attribute], name: &str) -> Result<Option<Expr>> {
    let mut value = None;
    for attr in attrs {
        match attr.name.as_str() {
            other if other == name => {
                if value.is_some() {
                    bail!("duplicate attribute: {name}");
                }
                value = Some(
                    attr.value
                        .clone()
                        .ok_or_else(|| anyhow!("attribute `{name}` requires a value"))?,
                );
            }
            "alias" | "default" => {}
            other => bail!("unsupported parameter attribute: {other}"),
        }
    }
    Ok(value)
}

pub(super) fn attr_string(attrs: &[Attribute], name: &str) -> Result<Option<String>> {
    match attr_expr(attrs, name)? {
        None => Ok(None),
        Some(Expr::String(value)) => Ok(Some(value)),
        Some(_) => bail!("attribute `{name}` expects a string literal"),
    }
}
