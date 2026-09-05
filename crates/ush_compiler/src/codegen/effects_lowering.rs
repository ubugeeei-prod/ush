//! Lowering for `effect` declarations and `try … with` handlers.
//!
//! An effect operation becomes an ordinary shell function that
//! forwards to whichever handler is installed:
//!
//! ```sh
//! ush_fn_log() {
//!   [ -n "${__ush_handler_log:-}" ] || { …unhandled…; }
//!   "$__ush_handler_log" "$@"
//! }
//! ```
//!
//! A `try … with` block compiles its handler to a normal function,
//! points the indirection at it for the duration of the body, and
//! puts back whatever was there before. Nesting and shadowing fall
//! out of the save/restore, and because the handler is a plain call
//! its return value is what `do` evaluates to — which makes these
//! *tail-resumptive* handlers: the body continues where it left off,
//! but a handler cannot capture the continuation and run it twice or
//! not at all.

use alloc::string::ToString;

use anyhow::{Result, bail};

use super::{
    super::{
        ast::{EffectDef, EffectHandler, FunctionDef, FunctionParam, Type},
        effects::FunctionErrorRegistry,
        env::{CodegenState, EnumRegistry, Env},
    },
    functions::{FunctionRegistry, compile_function},
    shared::shell_function_name,
    statement::compile_statement,
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;
use crate::types::{AstString as NameString, HeapVec as Vec};

/// The shell variable that holds the current handler for an effect.
fn handler_variable(effect: &str) -> NameString {
    let mut name = NameString::from("__ush_handler_");
    name.push_str(effect);
    name
}

/// Turns an `effect` declaration into a function the rest of the
/// compiler can treat as any other call target.
pub(crate) fn effect_dispatcher(def: &EffectDef) -> FunctionDef {
    FunctionDef {
        attrs: Vec::new(),
        name: def.name.clone(),
        receiver: None,
        params: def.params.clone(),
        return_type: def.return_type.clone().or(Some(Type::Unit)),
        declared_errors: None,
        declared_effects: None,
        body: Vec::new(),
    }
}

pub(crate) fn compile_effect(def: &EffectDef, out: &mut OutputBuffer) {
    let variable = handler_variable(&def.name);
    let function = shell_function_name(&def.name);

    out.push_str(&variable);
    out.push_str("=''\n");
    out.push_str(&function);
    out.push_str("() {\n");
    out.push_str("  if [ -z \"${");
    out.push_str(&variable);
    out.push_str(":-}\" ]; then\n    printf '%s\\n' 'ush: unhandled effect: ");
    out.push_str(&def.name);
    out.push_str("' >&2\n    exit 1\n  fi\n  \"$");
    out.push_str(&variable);
    out.push_str("\" \"$@\"\n}\n\n");
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_handle(
    body: &[crate::ast::Statement],
    handlers: &[EffectHandler],
    env: &mut Env,
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    let mut installed = Vec::new();
    for handler in handlers {
        let operation = functions.get(&handler.effect).ok_or_else(|| {
            anyhow::anyhow!(
                "no effect `{}` is declared; add `effect {}`",
                handler.effect,
                handler.effect
            )
        })?;
        if handler.params.len() != operation.params.len() {
            bail!(
                "handler for `{}` binds {} argument(s) but the effect takes {}",
                handler.effect,
                handler.params.len(),
                operation.params.len()
            );
        }

        let def = handler_function(handler, operation, state);
        compile_function(
            &def,
            globals,
            functions,
            impls,
            enums,
            function_errors,
            state,
            out,
        )?;

        let variable = handler_variable(&handler.effect);
        let saved = saved_variable(&variable, state);
        out.push_str(&saved);
        out.push_str("=\"${");
        out.push_str(&variable);
        out.push_str(":-}\"\n");
        out.push_str(&variable);
        out.push('=');
        out.push_str(&crate::util::shell_quote(&shell_function_name(&def.name)));
        out.push('\n');
        installed.push((variable, saved));
    }

    for statement in body {
        compile_statement(
            statement,
            env,
            globals,
            functions,
            impls,
            enums,
            function_errors,
            state,
            None,
            inside_function,
            false,
            out,
        )?;
    }

    // Innermost first, so a shadowed handler comes back before the
    // one it shadowed.
    for (variable, saved) in installed.into_iter().rev() {
        out.push_str(&variable);
        out.push_str("=\"$");
        out.push_str(&saved);
        out.push_str("\"\n");
    }
    Ok(())
}

/// A handler body compiled as an ordinary function, so it gets the
/// same argument binding and return-value protocol as everything
/// else.
fn handler_function(
    handler: &EffectHandler,
    operation: &FunctionDef,
    state: &mut CodegenState,
) -> FunctionDef {
    let mut params = Vec::new();
    for (name, declared) in handler.params.iter().zip(&operation.params) {
        params.push(FunctionParam {
            name: name.clone(),
            ty: declared.ty.clone(),
            default: None,
            cli_alias: None,
        });
    }

    let mut name = NameString::from("__ush_handle_");
    name.push_str(&handler.effect);
    name.push('_');
    name.push_str(&state.next_handler_id().to_string());

    FunctionDef {
        attrs: Vec::new(),
        name,
        receiver: None,
        params,
        return_type: operation.return_type.clone(),
        declared_errors: None,
        declared_effects: None,
        body: handler.body.clone(),
    }
}

fn saved_variable(variable: &str, state: &mut CodegenState) -> NameString {
    let mut name = NameString::from("__ush_saved");
    name.push_str(variable);
    name.push('_');
    name.push_str(&state.next_handler_id().to_string());
    name
}
