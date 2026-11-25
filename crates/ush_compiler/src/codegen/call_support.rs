use anyhow::{Result, anyhow, bail};

use super::{
    super::ast::{Call, Expr, FunctionDef},
    FunctionRegistry,
    calls::ensure_signature,
    compile_primitive_expr, infer,
};
use crate::{
    env::{EnumRegistry, Env},
    traits::TraitImplRegistry,
    types::{AstVec as Vec, OutputString as String},
};

pub(super) fn resolve_call_args<'a>(call: &'a Call, def: &'a FunctionDef) -> Result<Vec<&'a Expr>> {
    let mut resolved = vec![None; def.params.len()];
    let mut next = 0usize;

    for arg in &call.args {
        let index = match &arg.label {
            Some(label) => def
                .params
                .iter()
                .position(|param| param.name == *label)
                .ok_or_else(|| anyhow!("unknown argument label `{label}` for `{}`", call.name))?,
            None => {
                while next < resolved.len() && resolved[next].is_some() {
                    next += 1;
                }
                if next >= resolved.len() {
                    bail!(
                        "function `{}` expects at most {} arguments",
                        call.name,
                        def.params.len()
                    );
                }
                next
            }
        };
        if resolved[index].is_some() {
            bail!(
                "duplicate argument for `{}`: {}",
                call.name,
                def.params[index].name
            );
        }
        resolved[index] = Some(&arg.expr);
        if arg.label.is_none() {
            next += 1;
        }
    }

    def.params
        .iter()
        .enumerate()
        .map(|(index, param)| {
            resolved[index]
                .or(param.default.as_ref())
                .ok_or_else(|| anyhow!("missing argument for `{}`: {}", call.name, param.name))
        })
        .collect()
}

pub(super) fn function_for_call<'a>(
    name: &str,
    functions: &'a FunctionRegistry,
) -> Result<&'a FunctionDef> {
    if let Some(message) = super::stdlib::pipe::misuse(name) {
        bail!("{message}");
    }
    functions
        .get(name)
        .ok_or_else(|| anyhow!("unknown function: {name}"))
}

pub(super) fn capture_fds(name: &str) -> &'static str {
    if name == "std::io::echo"
        || (name != "std::terminal::is_tty"
            && (name.starts_with("std::") || name.starts_with("$bin::")))
    {
        ""
    } else {
        " 4>&1 >&3"
    }
}

pub(super) fn rendered_args(
    call: &Call,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    def: &FunctionDef,
) -> Result<Vec<String>> {
    ensure_signature(def)?;
    def.params
        .iter()
        .zip(resolve_call_args(call, def)?)
        .map(|(param, arg)| {
            let actual = infer(arg, env, functions, impls, enums)?;
            if actual != param.ty {
                bail!(
                    "type mismatch for `{}`: expected {:?}, found {:?}",
                    param.name,
                    param.ty,
                    actual
                );
            }
            if let crate::ast::Expr::Var(name) = arg
                && let Some(crate::env::Binding {
                    storage:
                        crate::env::Storage::Aggregate(prefix) | crate::env::Storage::Adt(prefix),
                    ..
                }) = env.get(name)
            {
                return Ok(super::wire::encode(&actual, prefix));
            }
            compile_primitive_expr(arg, env, functions, impls, enums)
        })
        .collect()
}
