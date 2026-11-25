//! The List API: `xs[i]`, `xs + ys` and the methods `len`, `is_empty`,
//! `contains`, `first`, `last`, `join` and `enumerate`.
//!
//! A list lives in shell variables `P__len`, `P__0`, `P__1`, …, so `len`
//! is a read and an element is one `eval` of a variable name built from
//! a checked integer; element values are never evaluated as code.

mod access;
mod build;
mod iterate;
mod methods;

use alloc::boxed::Box;

use anyhow::{Result, bail};

use super::{FunctionRegistry, infer};
use crate::{
    ast::{Expr, MethodCall, Type},
    env::{EnumRegistry, Env},
    traits::TraitImplRegistry,
};

pub(super) use access::hoist_index;
pub(crate) use build::{append_in_place, emit_to_target, handles};
pub(crate) use iterate::compile_enumerate_for;
pub(super) use methods::hoist_method;

/// What lowering a list expression needs from its surroundings.
pub(crate) struct Lowering<'a> {
    pub functions: &'a FunctionRegistry,
    pub impls: &'a TraitImplRegistry,
    pub enums: &'a EnumRegistry,
    pub inside_function: bool,
}

impl<'a> Lowering<'a> {
    pub(crate) fn new(
        functions: &'a FunctionRegistry,
        impls: &'a TraitImplRegistry,
        enums: &'a EnumRegistry,
        inside_function: bool,
    ) -> Self {
        Self {
            functions,
            impls,
            enums,
            inside_function,
        }
    }

    fn infer(&self, expr: &Expr, env: &Env) -> Result<Type> {
        infer(expr, env, self.functions, self.impls, self.enums)
    }

    /// `return 1` inside a function, `exit 1` at top level.
    fn fail(&self) -> &'static str {
        if self.inside_function {
            "return 1"
        } else {
            "exit 1"
        }
    }
}

const METHODS: &[&str] = &[
    "len",
    "is_empty",
    "contains",
    "first",
    "last",
    "join",
    "enumerate",
];

pub(super) fn infer_index(
    base: &Expr,
    index: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    let Type::List(item) = infer(base, env, functions, impls, enums)? else {
        bail!("only a List can be indexed with `[…]`");
    };
    if infer(index, env, functions, impls, enums)? != Type::Int {
        bail!("a list index must be an Int");
    }
    Ok(*item)
}

/// `xs + ys`: `Some` when the parts are lists, which must share one type.
pub(super) fn infer_concat(
    parts: &[Expr],
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Option<Type>> {
    let types = parts
        .iter()
        .map(|part| infer(part, env, functions, impls, enums))
        .collect::<Result<crate::types::HeapVec<_>>>()?;
    if !types.iter().any(|ty| matches!(ty, Type::List(_))) {
        return Ok(None);
    }
    if types.iter().any(|ty| *ty != types[0]) {
        bail!(
            "`+` joins lists of one type only, found {}",
            types
                .iter()
                .map(Type::render)
                .collect::<alloc::vec::Vec<_>>()
                .join(" + ")
        );
    }
    Ok(Some(types[0].clone()))
}

pub(super) fn is_list_method(
    call: &MethodCall,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> bool {
    METHODS.contains(&call.method.as_str())
        && matches!(
            infer(&call.receiver, env, functions, impls, enums),
            Ok(Type::List(_))
        )
}

/// The type of a List method call, `None` when the receiver is not a list.
pub(super) fn infer_method(
    call: &MethodCall,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Option<Type>> {
    let Type::List(item) = infer(&call.receiver, env, functions, impls, enums)? else {
        return Ok(None);
    };
    let args = call
        .args
        .iter()
        .map(|arg| {
            if arg.label.is_some() {
                bail!("`{}` takes positional arguments", call.method);
            }
            infer(&arg.expr, env, functions, impls, enums)
        })
        .collect::<Result<crate::types::HeapVec<_>>>()?;
    let scalar = matches!(*item, Type::String | Type::Int | Type::Bool);
    let ty = match (call.method.as_str(), args.as_slice()) {
        ("len", []) => Type::Int,
        ("is_empty", []) => Type::Bool,
        ("first" | "last", []) => *item,
        ("enumerate", []) => Type::List(Box::new(Type::Tuple(vec![Type::Int, *item]))),
        ("contains", [needle]) if scalar && *needle == *item => Type::Bool,
        ("contains", [_]) if !scalar => {
            bail!(
                "`contains` needs a list of String, Int or Bool, found {}",
                Type::List(item).render()
            )
        }
        ("contains", [needle]) => bail!(
            "`contains` on a {} needs a {} argument, found {}",
            Type::List(item.clone()).render(),
            item.render(),
            needle.render()
        ),
        ("join", [Type::String]) if scalar => Type::String,
        ("join", [_]) if !scalar => {
            bail!(
                "`join` needs a list of String, Int or Bool, found {}",
                Type::List(item).render()
            )
        }
        ("join", _) => bail!("`join` takes one String separator, as in `items.join \", \"`"),
        (method, _) if METHODS.contains(&method) => {
            bail!("wrong arguments for the List method `{method}`")
        }
        _ => return Ok(None),
    };
    Ok(Some(ty))
}

/// Whether the call can fail at run time: `first` and `last` on a list
/// that turns out to be empty.
pub(crate) fn list_method_fails(
    call: &MethodCall,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> bool {
    matches!(call.method.as_str(), "first" | "last")
        && is_list_method(call, env, functions, impls, enums)
}

/// Whether [`hoist`] lowers `expr`: `$env::NAME`, `xs[i]` or a List method.
pub(super) fn hoists(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> bool {
    match expr {
        Expr::Env(_) | Expr::Index { .. } => true,
        Expr::MethodCall(call) => is_list_method(call, env, functions, impls, enums),
        _ => false,
    }
}

/// Evaluate `expr` into a temporary before the statement that uses it.
pub(super) fn hoist(
    expr: &Expr,
    env: &mut Env,
    low: &Lowering<'_>,
    state: &mut crate::env::CodegenState,
    out: &mut crate::sourcemap::OutputBuffer,
) -> Result<Expr> {
    match expr {
        Expr::Env(name) => super::text::hoist_env(name, env, state, low.inside_function, out),
        Expr::Index { base, index } => hoist_index(base, index, env, low, state, out),
        Expr::MethodCall(call) => hoist_method(call, env, low, state, out),
        _ => bail!("not a list or environment expression"),
    }
}
