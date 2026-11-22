use anyhow::{Result, anyhow, bail};

use super::{
    super::ast::{Call, Expr, FunctionDef},
    FunctionRegistry,
};
use crate::types::AstVec as Vec;

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
    functions
        .get(name)
        .ok_or_else(|| anyhow!("unknown function: {name}"))
}

pub(super) fn capture_fds(name: &str) -> &'static str {
    if name.starts_with("std::") || name.starts_with("$bin::") {
        ""
    } else {
        " 4>&1 >&3"
    }
}
