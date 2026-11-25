use super::{
    SourceLine, declaration_support::finish_block, path::parse_identifier,
    signature::parse_function_header,
};
use crate::{
    ast::{Call, CallArg, Expr, FunctionDef, Statement, StatementKind as S, Type},
    types::HeapVec as Vec,
    util::{parse_string_literal, shell_quote},
};
use alloc::boxed::Box;
use anyhow::{Result, anyhow, ensure};

pub(super) fn parse(
    line: usize,
    header: &str,
    lines: &[SourceLine<'_>],
    cursor: &mut usize,
) -> Result<Vec<Statement>> {
    let head = header
        .strip_prefix("extern ")
        .and_then(|s| s.strip_suffix('{'))
        .ok_or_else(|| anyhow!("expected `extern \"file.sh\" as module {{`"))?
        .trim();
    let words = crate::util::split_top_level_whitespace(head);
    ensure!(
        words.len() == 1 || (words.len() == 3 && words[1] == "as"),
        "expected a .sh path and optional `as module`"
    );
    let path = parse_string_literal(words[0])
        .ok_or_else(|| anyhow!("extern path must be a string literal"))?;
    ensure!(
        path.ends_with(".sh") && !path.contains(['\n', '\r']),
        "extern requires a .sh path"
    );
    let module = if words.len() == 3 {
        Some(parse_identifier(words[2])?)
    } else {
        None
    };
    let quoted = shell_quote(&path);
    let source = if path.starts_with('/') {
        format!(". {quoted}")
    } else {
        format!(". \"${{__ush_source_dir:-.}}\"/{quoted}")
    };
    let mut result = vec![Statement::new(line, S::Shell(Expr::String(source.into())))];
    *cursor += 1;
    while *cursor < lines.len() {
        let (line, text) = lines[*cursor];
        let text = text.trim().trim_end_matches(';').trim();
        if text == "}" {
            break;
        }
        if text.is_empty() || text.starts_with('#') {
            *cursor += 1;
            continue;
        }
        let signature = text
            .strip_prefix("fn ")
            .ok_or_else(|| anyhow!("extern blocks contain typed `fn` declarations"))?;
        let (name, params, return_type, declared_effects) = parse_function_header(signature)?;
        ensure!(
            params
                .iter()
                .all(|p| p.ty.is_scalar() && p.default.is_none()),
            "extern arguments must have scalar types without defaults"
        );
        let ty = return_type.as_ref().unwrap();
        ensure!(
            ty.is_scalar(),
            "extern return type must be String, Int, Bool or ()"
        );
        let command = format!(
            "{name}{}",
            params
                .iter()
                .map(|p| format!(" \"${{{}}}\"", p.name))
                .collect::<Vec<_>>()
                .join("")
        );
        let body = if *ty == Type::Unit {
            vec![Statement::new(line, S::Shell(Expr::String(command.into())))]
        } else {
            let capture = Expr::Try(Box::new(Expr::ShellCapture(command.into())));
            let expr = match ty {
                Type::Int | Type::Bool => Expr::Call(Call {
                    name: if *ty == Type::Int {
                        "std::__extern::int"
                    } else {
                        "std::__extern::bool"
                    }
                    .into(),
                    args: vec![CallArg {
                        label: None,
                        expr: capture,
                    }],
                    asynchronous: false,
                }),
                _ => capture,
            };
            vec![Statement::new(line, S::Return(expr))]
        };
        let declared_errors = if declared_effects.is_none() {
            let mut errors = crate::errors::ErrorSet::default();
            errors.insert(crate::errors::ErrorType::Unknown);
            Some(errors)
        } else {
            None
        };
        result.push(Statement::new(
            line,
            S::Function(FunctionDef {
                name: module
                    .as_ref()
                    .map_or(name.clone(), |module| format!("{module}::{name}").into()),
                params,
                return_type,
                declared_effects,
                body,
                receiver: None,
                attrs: vec![],
                scheme: None,
                declared_errors,
            }),
        ));
        *cursor += 1;
    }
    finish_block(lines, cursor, "extern block")?;
    Ok(result)
}
