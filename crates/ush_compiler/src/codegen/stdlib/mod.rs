mod env;
mod external;
mod fs;
mod host;
mod http;
mod path;
pub(crate) mod pipe;
mod portable;
mod process;
mod regex;
mod scripting;
mod string;
pub(crate) mod testing;
pub(crate) use host::required as host_required;

use anyhow::{Result, bail};

use super::functions::FunctionRegistry;
use crate::{
    ast::{FunctionDef, FunctionParam, Type},
    sourcemap::OutputBuffer,
    types::HeapVec as Vec,
};

pub(crate) fn register_builtins(functions: &mut FunctionRegistry) -> Result<()> {
    for def in env::definitions()
        .into_iter()
        .chain(fs::definitions())
        .chain(external::definitions())
        .chain(http::definitions())
        .chain(host::definitions())
        .chain(path::definitions())
        .chain(pipe::definitions())
        .chain(portable::definitions())
        .chain(process::definitions())
        .chain(regex::definitions())
        .chain(scripting::definitions())
        .chain(string::definitions())
        .chain(testing::definitions())
    {
        if functions.insert(def.name.clone(), def).is_some() {
            bail!("duplicate function");
        }
    }
    Ok(())
}

/// Link the transitive closure in library order for reproducible output.
pub(super) fn emit_builtins(out: &mut OutputBuffer, body: &str) {
    if !body.contains("ush_fn_std__") && !body.contains("__ush_std_") {
        return;
    }
    let runtime = concat!(
        include_str!("runtime.sh"),
        "\n",
        include_str!("scripting_runtime.sh"),
        "\n",
        include_str!("portable_runtime.sh"),
        "\n",
        include_str!("host_runtime.sh"),
        "\n",
        include_str!("../list/runtime.sh")
    );
    let definitions: crate::types::Map<_, _> = runtime
        .split_terminator("\n}\n")
        .filter_map(|definition| definition.trim_start().split_once("() {\n"))
        .collect();
    let mut used = crate::types::Set::default();
    let mut pending = vec![body];
    while let Some(text) = pending.pop() {
        for name in text.split(|c: char| !c.is_ascii_alphanumeric() && c != '_') {
            if let Some(&body) = definitions.get(name)
                && used.insert(name)
            {
                pending.push(body);
            }
        }
    }
    for definition in runtime.split_terminator("\n}\n") {
        let definition = definition.trim_start();
        if let Some((name, _)) = definition.split_once("() {")
            && used.contains(name)
        {
            out.push_str(definition);
            out.push_str("\n}\n\n");
        }
    }
}

pub(super) fn runs_in_parent(name: &str) -> bool {
    matches!(name, "std::process::exit" | "std::process::chdir")
        || env::runs_in_parent(name)
        || path::runs_in_parent(name)
        || fs::runs_in_parent(name)
}

pub(super) fn builtin(
    name: &str,
    params: Vec<FunctionParam>,
    return_type: Option<Type>,
) -> FunctionDef {
    FunctionDef {
        scheme: None,
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
