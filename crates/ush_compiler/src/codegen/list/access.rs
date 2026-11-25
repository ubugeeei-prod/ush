//! Reading one element: `xs[i]`, `xs.first()` and `xs.last()`.

use anyhow::{Result, bail};

use super::Lowering;
use crate::{
    ast::{Expr, Type},
    codegen::{binding_for_name, compile_runtime_primitive_expr, wire},
    env::{Binding, CodegenState, Env, Storage},
    matching::read_expr,
    sourcemap::OutputBuffer,
    types::AstString as Name,
};

/// Read `list` into its shell storage: the variable prefix and the item type.
pub(super) fn read_list(
    list: &Expr,
    env: &Env,
    low: &Lowering<'_>,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<(Name, Type)> {
    let binding = read_expr(
        list,
        env,
        low.functions,
        low.impls,
        low.enums,
        state,
        low.inside_function,
        out,
    )?;
    match (binding.ty, binding.storage) {
        (Type::List(item), Storage::Aggregate(prefix)) => Ok((prefix, *item)),
        (other, _) => bail!("expected a List, found {}", other.render()),
    }
}

pub(crate) fn hoist_index(
    base: &Expr,
    index: &Expr,
    env: &mut Env,
    low: &Lowering<'_>,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<Expr> {
    let (prefix, item) = read_list(base, env, low, state, out)?;
    let position = element_index(&prefix, Some(index), "", env, low, state, out)?;
    Ok(bind_element(&prefix, &position, &item, env, state, out))
}

/// Put the position of an element into a temporary, failing the way
/// `fail` does when there is no such element. `index` is `None` for
/// `first` and `last`, which `method` names.
pub(super) fn element_index(
    prefix: &str,
    index: Option<&Expr>,
    method: &str,
    env: &Env,
    low: &Lowering<'_>,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<Name> {
    let position = state.temp_var("index");
    let Some(index) = index else {
        out.push_str(&format!(
            "if [ \"${{{prefix}__len}}\" -eq 0 ]; then __ush_report 1 '{method}() needs a nonempty list'; {}; fi\n",
            low.fail()
        ));
        let value = if method == "first" {
            "0".into()
        } else {
            format!("$(({prefix}__len - 1))")
        };
        out.push_str(&format!("{position}={value}\n"));
        return Ok(position);
    };
    let value = compile_runtime_primitive_expr(
        index,
        env,
        low.functions,
        low.impls,
        low.enums,
        state,
        low.inside_function,
        out,
    )?;
    out.push_str(&format!(
        "{position}={value}\nif [ \"${{{position}}}\" -lt 0 ] || [ \"${{{position}}}\" -ge \"${{{prefix}__len}}\" ]; then __ush_report 1 \"index ${{{position}}} is out of range for a list of length ${{{prefix}__len}}\"; {}; fi\n",
        low.fail()
    ));
    Ok(position)
}

/// Copy the element at `position` into fresh storage and bind it.
pub(super) fn bind_element(
    prefix: &str,
    position: &str,
    item: &Type,
    env: &mut Env,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Expr {
    let temp = state.temp_var("item");
    copy_element(prefix, position, item, &temp, state, out);
    env.insert(temp.clone(), binding_for_name(&temp, item.clone()));
    Expr::Var(temp)
}

/// `target = prefix[position]` for any item type.
pub(super) fn copy_element(
    prefix: &str,
    position: &str,
    item: &Type,
    target: &str,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) {
    let source = format!("{prefix}__${{{position}}}");
    if item.is_scalar() {
        // A target with a computed index is only assignable through eval.
        if !target.contains('$') {
            out.push_str(&format!("{target}=''\n"));
        }
        out.push_str(&format!("eval \"{target}=\\\"\\${{{source}}}\\\"\"\n"));
    } else {
        wire::copy(item, &source, target, state, out);
    }
}

/// The binding a scalar temporary gets.
pub(super) fn scalar(name: &str, ty: Type) -> Binding {
    Binding {
        ty,
        storage: Storage::Primitive(name.into()),
    }
}
