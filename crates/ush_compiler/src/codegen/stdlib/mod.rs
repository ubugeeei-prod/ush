mod env;
mod fs;
mod http;
mod path;
mod process;
mod regex;
mod string;

use anyhow::{Result, bail};

use super::functions::FunctionRegistry;
use crate::{
    ast::{FunctionDef, FunctionParam, Type},
    sourcemap::OutputBuffer,
    types::HeapVec as Vec,
};

pub(super) fn register_builtins(functions: &mut FunctionRegistry) -> Result<()> {
    for def in env::definitions()
        .into_iter()
        .chain(fs::definitions())
        .chain(http::definitions())
        .chain(path::definitions())
        .chain(process::definitions())
        .chain(regex::definitions())
        .chain(string::definitions())
    {
        if functions.insert(def.name.clone(), def).is_some() {
            bail!("duplicate function");
        }
    }
    Ok(())
}

pub(super) fn emit_builtins(out: &mut OutputBuffer) {
    env::emit(out);
    fs::emit(out);
    http::emit(out);
    path::emit(out);
    process::emit(out);
    regex::emit(out);
    string::emit(out);
}

pub(super) fn runs_in_parent(name: &str) -> bool {
    env::runs_in_parent(name) || path::runs_in_parent(name) || fs::runs_in_parent(name)
}

pub(super) fn builtin(
    name: &str,
    params: Vec<FunctionParam>,
    return_type: Option<Type>,
) -> FunctionDef {
    FunctionDef {
        declared_effects: None,
        attrs: Vec::new(),
        name: name.into(),
        receiver: None,
        params,
        return_type,
        declared_errors: None,
        body: Vec::new(),
    }
}

pub(super) fn param(name: &str, ty: Type) -> FunctionParam {
    FunctionParam {
        name: name.into(),
        ty,
        default: None,
        cli_alias: None,
    }
}

pub(super) fn emit_fn(out: &mut OutputBuffer, name: &str, body: &str) {
    out.push_str(&super::shared::shell_function_name(name));
    out.push_str("() {\n");
    out.push_str(body);
    if !body.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("}\n\n");
}
