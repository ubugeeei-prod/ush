//! Structured results: an element that is itself a list, tuple or
//! record, `xs + ys`, `xs.enumerate()` and record or tuple fields, written
//! into the storage of a binding.

use anyhow::{Result, bail};

use super::{
    Lowering,
    access::{copy_element, element_index, read_list},
};
use crate::{
    ast::{Expr, Type},
    codegen::wire,
    env::{CodegenState, Env, Storage},
    matching::read_expr,
    sourcemap::OutputBuffer,
    types::{AstString as Name, HeapVec as Vec},
};

/// Whether [`emit_to_target`] lowers `expr`.
pub(crate) fn handles(expr: &Expr, env: &Env, low: &Lowering<'_>) -> bool {
    match expr {
        Expr::Index { .. } => true,
        Expr::Field { name, .. } => name != "await",
        Expr::Add(parts) => parts
            .first()
            .is_some_and(|part| matches!(low.infer(part, env), Ok(Type::List(_)))),
        Expr::MethodCall(call) => {
            matches!(call.method.as_str(), "first" | "last" | "enumerate")
                && super::is_list_method(call, env, low.functions, low.impls, low.enums)
        }
        _ => false,
    }
}

pub(crate) fn emit_to_target(
    target: &str,
    expr: &Expr,
    expected: &Type,
    env: &Env,
    low: &Lowering<'_>,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<()> {
    match expr {
        Expr::Index { base, index } => {
            let (prefix, item) = read_list(base, env, low, state, out)?;
            let position = element_index(&prefix, Some(index), "", env, low, state, out)?;
            copy_element(&prefix, &position, &item, target, state, out);
        }
        Expr::MethodCall(call) if call.method != "enumerate" => {
            let (prefix, item) = read_list(&call.receiver, env, low, state, out)?;
            let position = element_index(&prefix, None, &call.method, env, low, state, out)?;
            copy_element(&prefix, &position, &item, target, state, out);
        }
        Expr::MethodCall(call) => {
            let (prefix, item) = read_list(&call.receiver, env, low, state, out)?;
            into_fresh(
                target,
                &[&prefix],
                expected,
                state,
                out,
                |dest, state, out| {
                    enumerate(dest, &prefix, &item, state, out);
                },
            );
        }
        Expr::Add(parts) => {
            let Type::List(item) = expected else {
                bail!("`+` builds a List here, not a {}", expected.render());
            };
            let mut prefixes = Vec::with_capacity(parts.len());
            for part in parts {
                prefixes.push(read_list(part, env, low, state, out)?.0);
            }
            let sources = prefixes.iter().map(Name::as_str).collect::<Vec<_>>();
            into_fresh(
                target,
                &sources,
                expected,
                state,
                out,
                |dest, state, out| {
                    concat(dest, &sources, item, "0", state, out);
                },
            );
        }
        Expr::Field { base, name } => {
            let binding = read_expr(
                base,
                env,
                low.functions,
                low.impls,
                low.enums,
                state,
                low.inside_function,
                out,
            )?;
            let field = super::super::method_fields::field_binding(&binding, name, low.enums)?;
            let (Storage::Adt(source) | Storage::Aggregate(source)) = &field.storage else {
                bail!("`.{name}` is not a structured value");
            };
            wire::copy(&field.ty, source, target, state, out);
        }
        _ => bail!("expected a structured value for {target}"),
    }
    Ok(())
}

/// Build into `target`, or into a temporary copied over it afterwards when
/// a source is the target itself, as in `let xs = ["a"] + xs`.
fn into_fresh(
    target: &str,
    sources: &[&str],
    ty: &Type,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
    build: impl FnOnce(&str, &mut CodegenState, &mut OutputBuffer),
) {
    if !sources.contains(&target) {
        return build(target, state, out);
    }
    let temp = state.temp_var("list");
    build(&temp, state, out);
    wire::copy(ty, &temp, target, state, out);
}

/// `xs = xs + ys + …` on a `let mut` list: the new items are read first,
/// then written after the existing ones, without copying the list.
pub(crate) fn append_in_place(
    target: &str,
    parts: &[Expr],
    expected: &Type,
    env: &Env,
    low: &Lowering<'_>,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<()> {
    let Type::List(item) = expected else {
        bail!("`+` builds a List here, not a {}", expected.render());
    };
    let mut prefixes = Vec::with_capacity(parts.len());
    for part in parts {
        prefixes.push(read_list(part, env, low, state, out)?.0);
    }
    let sources = prefixes.iter().map(Name::as_str).collect::<Vec<_>>();
    concat(
        target,
        &sources,
        item,
        &format!("${{{target}__len}}"),
        state,
        out,
    );
    Ok(())
}

/// Copy the items of `sources` into `target` from position `start` on.
fn concat(
    target: &str,
    sources: &[&str],
    item: &Type,
    start: &str,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) {
    let count = state.temp_var("count");
    out.push_str(&format!("{count}={start}\n"));
    for source in sources {
        let index = state.temp_var("index");
        out.push_str(&format!(
            "{index}=0\nwhile [ \"${{{index}}}\" -lt \"${{{source}__len}}\" ]; do\n"
        ));
        copy_element(
            source,
            &index,
            item,
            &format!("{target}__${{{count}}}"),
            state,
            out,
        );
        out.push_str(&format!(
            "  {index}=$(({index} + 1))\n  {count}=$(({count} + 1))\ndone\n"
        ));
    }
    out.push_str(&format!("{target}__len=${{{count}}}\n"));
}

fn enumerate(
    target: &str,
    source: &str,
    item: &Type,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) {
    let index = state.temp_var("index");
    out.push_str(&format!(
        "{index}=0\nwhile [ \"${{{index}}}\" -lt \"${{{source}__len}}\" ]; do\n  eval \"{target}__${{{index}}}__len=2 {target}__${{{index}}}__0=${{{index}}}\"\n"
    ));
    copy_element(
        source,
        &index,
        item,
        &format!("{target}__${{{index}}}__1"),
        state,
        out,
    );
    out.push_str(&format!(
        "  {index}=$(({index} + 1))\ndone\n{target}__len=${{{source}__len}}\n"
    ));
}
