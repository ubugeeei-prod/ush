//! `"…${expr}…"` templates and `$NAME` environment reads.
//!
//! A template lowers to shell concatenation: literal pieces are quoted
//! words and each hole is a parameter expansion of a variable that already
//! holds its value, so a value is never parsed as shell code.

use anyhow::{Result, bail};

use super::{FunctionRegistry, compile_primitive_expr, infer};
use crate::{
    ast::{Expr, Type},
    env::{Binding, CodegenState, EnumRegistry, Env, Storage},
    sourcemap::OutputBuffer,
    traits::TraitImplRegistry,
    types::OutputString as String,
    util::shell_quote,
};

pub(super) fn infer_template(
    parts: &[Expr],
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    for part in parts {
        match infer(part, env, functions, impls, enums)? {
            Type::String | Type::Int | Type::Bool => {}
            Type::List(_) => {
                bail!("cannot interpolate a list: join it first, as in `${{items.join \", \"}}`")
            }
            other => bail!(
                "cannot interpolate a {}: only String, Int and Bool values can appear in `${{…}}`",
                other.render()
            ),
        }
    }
    Ok(Type::String)
}

/// The shell word for a template whose holes were already hoisted.
pub(super) fn compile_template(
    parts: &[Expr],
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<String> {
    let mut out = String::new();
    for part in parts {
        match part {
            Expr::String(text) => quote_text(text, &mut out),
            _ => {
                let word = compile_primitive_expr(part, env, functions, impls, enums)?;
                // Arithmetic is unquoted in other positions; here it joins a word.
                if word.starts_with("$((") {
                    out.push_str(&format!("\"{word}\""));
                } else {
                    out.push_str(&word);
                }
            }
        }
    }
    if out.is_empty() {
        out.push_str("''");
    }
    Ok(out)
}

/// Quote template text, with each line break as `"${__ush_lf}"` and no
/// empty `''` words around it.
fn quote_text(text: &str, out: &mut String) {
    for (index, line) in text.split('\n').enumerate() {
        if index > 0 {
            out.push_str("\"${__ush_lf}\"");
        }
        if !line.is_empty() {
            out.push_str(&shell_quote(line));
        }
    }
}

/// `$NAME` reads a variable of the process environment. A scalar ush
/// binding of the same name is also written to that shell variable, for
/// raw shell code, so the read would see the binding: it is rejected.
pub(super) fn infer_env(name: &str, env: &Env) -> Result<Type> {
    if env.contains_key(name) {
        bail!(
            "`$env::{name}` reads the environment, but `{name}` is also a ush binding here; rename the binding, or read it with `std::env::get \"{name}\"`"
        );
    }
    Ok(Type::String)
}

/// Copy `$NAME` into a temporary, failing with the variable's name when it
/// is unset or empty.
pub(super) fn hoist_env(
    name: &str,
    env: &mut Env,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<Expr> {
    infer_env(name, env)?;
    let temp = state.temp_var("env");
    out.push_str(&format!(
        "{temp}=${{{name}-}}\nif [ -z \"${{{temp}}}\" ]; then __ush_report 1 'environment variable {name} is not set or is empty'; {} 1; fi\n",
        if inside_function { "return" } else { "exit" }
    ));
    env.insert(
        temp.clone(),
        Binding {
            ty: Type::String,
            storage: Storage::Primitive(temp.clone()),
        },
    );
    Ok(Expr::Var(temp))
}
