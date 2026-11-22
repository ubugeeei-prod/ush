use super::types::{Index, Ty};
use crate::{
    types::{AstString as Name, HeapVec as Vec, Map},
    util::is_identifier,
};
use anyhow::{Result, anyhow, bail, ensure};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Kind {
    Type(usize),
    Const,
}

#[derive(Clone, Debug)]
pub(crate) struct Scheme {
    pub variables: Map<Name, Kind>,
    pub params: Vec<(Name, Ty)>,
    pub result: Option<Ty>,
}

impl Scheme {
    pub fn inferred_result(params: &[crate::ast::FunctionParam]) -> Self {
        Self {
            variables: Map::default(),
            params: params
                .iter()
                .map(|p| (p.name.clone(), Ty::from_type(&p.ty)))
                .collect(),
            result: None,
        }
    }
    pub fn integer_bounds(&self, name: &str) -> Option<(i64, i64)> {
        self.params
            .iter()
            .find(|p| p.0 == name && p.1.name == "Int")
            .map(|p| (p.1.min, p.1.max))
    }
}

pub(crate) fn header(
    head: &str,
    name: &str,
    params: &str,
    tail: &str,
) -> Result<Option<(Name, Scheme)>> {
    let untyped = !params.trim().is_empty() && split(params).iter().any(|p| !p.contains(':'));
    let advanced = untyped
        || name.contains('<')
        || params.contains(" where ")
        || params.contains('<')
        || params.contains(';')
        || tail.contains('<')
        || tail.contains(';')
        || tail.contains(" where ");
    if !advanced {
        return Ok(None);
    }
    let (name, generics) = application(name)?;
    ensure!(is_identifier(name), "invalid function name: {name}");
    let mut variables = Map::default();
    for item in generics {
        let (name, kind) = if let Some(decl) = item.strip_prefix("const ") {
            let (name, ty) = decl
                .split_once(':')
                .ok_or_else(|| anyhow!("expected const N: Int"))?;
            ensure!(ty.trim() == "Int", "index parameters must have type Int");
            (name.trim(), Kind::Const)
        } else {
            let (name, args) = application(item)?;
            ensure!(
                args.iter().all(|arg| *arg == "_"),
                "constructor parameters are written F<_>"
            );
            (name, Kind::Type(args.len()))
        };
        ensure!(
            is_identifier(name) && variables.insert(name.into(), kind).is_none(),
            "invalid or duplicate type parameter: {name}"
        );
    }
    let mut args = Vec::new();
    for (index, arg) in split(params).into_iter().enumerate() {
        let (name, ty) = match arg.split_once(':') {
            Some((name, ty)) => (name.trim(), parse_type(ty.trim(), name.trim())?),
            None => {
                let parameter = format!("__T{index}");
                variables.insert(parameter.as_str().into(), Kind::Type(0));
                (arg, Ty::named(&parameter))
            }
        };
        ensure!(is_identifier(name), "invalid parameter: {name}");
        args.push((Name::from(name), ty));
    }
    let (value, row) = tail
        .split_once('/')
        .map_or((tail, ""), |(value, _)| (value, &tail[value.len()..]));
    let result = if value.trim().is_empty() {
        None
    } else {
        Some(parse_type(
            value
                .trim()
                .strip_prefix("->")
                .ok_or_else(|| anyhow!("invalid function header: {head}"))?
                .trim(),
            "result",
        )?)
    };
    let erased = args
        .iter()
        .map(|(name, ty)| format!("{name}: {}", ty.erase().render()))
        .collect::<Vec<_>>()
        .join(", ");
    let signature = format!(
        "{name}({erased}) -> {} {row}",
        result
            .as_ref()
            .map(|t| t.erase().render())
            .unwrap_or_else(|| "()".into())
    );
    Ok(Some((
        signature.into(),
        Scheme {
            variables,
            params: args,
            result,
        },
    )))
}

pub fn parse_type(source: &str, binder: &str) -> Result<Ty> {
    if let Some((base, expr)) = source.split_once(" where ") {
        let mut ty = parse_type(base.trim(), binder)?;
        ensure!(ty.name == "Int", "integer refinements require Int");
        for term in expr.split("&&") {
            let predicate = term
                .trim()
                .strip_prefix(binder)
                .ok_or_else(|| anyhow!("predicate must refine `{binder}`"))?
                .trim();
            let op = [">=", "<=", "==", ">", "<"]
                .into_iter()
                .find(|op| predicate.starts_with(op))
                .ok_or_else(|| anyhow!("expected integer comparison"))?;
            let v = predicate[op.len()..].trim().parse::<i64>()?;
            match op {
                ">" => {
                    ty.min = ty.min.max(
                        v.checked_add(1)
                            .ok_or_else(|| anyhow!("empty refinement"))?,
                    )
                }
                ">=" => ty.min = ty.min.max(v),
                "<" => {
                    ty.max = ty.max.min(
                        v.checked_sub(1)
                            .ok_or_else(|| anyhow!("empty refinement"))?,
                    )
                }
                "<=" => ty.max = ty.max.min(v),
                _ => {
                    ty.min = ty.min.max(v);
                    ty.max = ty.max.min(v);
                }
            }
        }
        ensure!(ty.min <= ty.max, "empty refinement");
        return Ok(ty);
    }
    if let Some(inner) = source.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        let (item, length) = inner
            .split_once(';')
            .map_or((inner, None), |(t, n)| (t, Some(n.trim())));
        let mut ty = Ty::app("List", vec![parse_type(item.trim(), binder)?]);
        if let Some(n) = length {
            ty.length = Some(match n.parse::<i64>() {
                Ok(n) if n >= 0 => Index::Literal(n),
                _ if is_identifier(n) => Index::Name(n.into()),
                _ => bail!("invalid array length: {n}"),
            });
        }
        return Ok(ty);
    }
    if source == "()" || source == "Unit" {
        return Ok(Ty::named("()"));
    }
    if let Some(inner) = source.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
        return Ok(Ty::app(
            "Tuple",
            split(inner)
                .into_iter()
                .map(|s| parse_type(s, binder))
                .collect::<Result<_>>()?,
        ));
    }
    let (name, args) = application(source)?;
    ensure!(is_identifier(name), "invalid type: {source}");
    Ok(Ty::app(
        name,
        args.into_iter()
            .map(|s| parse_type(s, binder))
            .collect::<Result<_>>()?,
    ))
}

fn application(source: &str) -> Result<(&str, Vec<&str>)> {
    let Some((name, args)) = source.split_once('<') else {
        return Ok((source.trim(), Vec::new()));
    };
    let inner = args
        .strip_suffix('>')
        .ok_or_else(|| anyhow!("unterminated type application: {source}"))?;
    ensure!(!inner.trim().is_empty(), "empty type argument list");
    Ok((name.trim(), split(inner)))
}

/// Commas inside a constructor, tuple or array belong to that nested type.
/// A refinement comparison is outside the type-argument grammar.
fn split(text: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    if text.trim().is_empty() {
        return parts;
    }
    let (mut start, mut depth, mut predicate) = (0, 0usize, false);
    for (i, c) in text.char_indices() {
        if text[i..].starts_with(" where ") {
            predicate = true;
        }
        match c {
            '[' | '(' => depth += 1,
            '<' if !predicate => depth += 1,
            ']' | ')' => depth = depth.saturating_sub(1),
            '>' if !predicate => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(text[start..i].trim());
                start = i + 1;
                predicate = false;
            }
            _ => {}
        }
    }
    parts.push(text[start..].trim());
    parts
}
