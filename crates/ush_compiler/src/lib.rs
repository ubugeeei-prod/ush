//! `.ush` to POSIX `sh` compiler core.
//!
//! Builds as `no_std + alloc` when the default `std` feature is
//! disabled, which is the configuration used by host integrations
//! that need to embed the compiler without pulling in the full
//! `std`. The default `std` feature wires up `anyhow/std` and the
//! filesystem helpers (`compile_file`, `compile_file_with_sourcemap`,
//! `describe_file`).
//!
//! ## Pipeline
//!
//! Each `compile_source` / `compile_file` call walks the source
//! through the same stages in order:
//!
//! 1. **Parse** — turns `.ush` text into an AST.
//! 2. **Resolve imports** — flattens `use` paths.
//! 3. **Effects pass** — typed-error / `Problem!T` propagation,
//!    match exhaustiveness, control-flow validation.
//! 4. **Codegen** — lowers to POSIX `sh` and emits a [`SourceMap`].
//!
//! The public surface is intentionally narrow: `UshCompiler`
//! (zero-sized struct + constant alias of the same name) plus the
//! four `compile_*` / `describe_*` methods and the
//! [`CompiledScript`] / [`SourceMap`] types they return.
//! Everything else (parser internals, AST types, codegen helpers,
//! effects-pass internals, etc.) is `pub(crate)` and is not
//! considered part of any external API.

#![cfg_attr(not(feature = "std"), no_std)]

mod ast;
mod codegen;
mod docs;
mod effects;
mod env;
mod errors;
mod imports;
mod matching;
mod parse;
mod scan;
mod sourcemap;
mod string_literal;
mod traits;
mod types;
mod util;

#[macro_use]
extern crate alloc;

use anyhow::Result;
pub use docs::ScriptDocs;
pub use effects::{Effect, EffectSet, FunctionEffects};
pub use sourcemap::{
    CompiledScript, SourceMap, SourceMapLine, SourceMapSection, SourceMapSectionSummary,
    SourceMapSourceLine, SourceMapSummary,
};
use types::OutputString;

#[cfg(feature = "std")]
use anyhow::Context;
#[cfg(feature = "std")]
use std::{fs, path::Path};

#[derive(Debug, Clone, Default)]
pub struct UshCompiler {
    _private: (),
}

#[allow(non_upper_case_globals)]
pub const UshCompiler: UshCompiler = UshCompiler { _private: () };

impl UshCompiler {
    #[cfg(feature = "std")]
    pub fn compile_file(&self, path: &Path) -> Result<OutputString> {
        Ok(self.compile_file_with_sourcemap(path)?.shell)
    }

    #[cfg(feature = "std")]
    pub fn compile_file_with_sourcemap(&self, path: &Path) -> Result<CompiledScript> {
        let source = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let absolute = fs::canonicalize(path)
            .with_context(|| format!("failed to canonicalize {}", path.display()))?;
        let source_dir = absolute.parent().and_then(|item| item.to_str());
        let source_path = absolute.to_str();
        self.compile_with_context(
            &source,
            path.file_name().and_then(|name| name.to_str()),
            source_dir,
            source_path,
        )
        .with_context(|| format!("failed to compile {}", path.display()))
    }

    pub fn compile_source(&self, source: &str) -> Result<OutputString> {
        Ok(self.compile_source_with_sourcemap(source)?.shell)
    }

    pub fn compile_source_with_sourcemap(&self, source: &str) -> Result<CompiledScript> {
        self.compile_with_context(source, None, None, None)
    }

    fn compile_with_context(
        &self,
        source: &str,
        script_name: Option<&str>,
        source_dir: Option<&str>,
        source_path: Option<&str>,
    ) -> Result<CompiledScript> {
        let program = imports::resolve_program(parse::parse_program(source)?)?;
        let docs = ScriptDocs::parse(source);
        codegen::compile_program(
            &program,
            &docs,
            source,
            script_name,
            source_dir,
            source_path,
        )
    }

    #[cfg(feature = "std")]
    pub fn describe_file(&self, path: &Path) -> Result<ScriptDocs> {
        let source = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        Ok(self.describe_source(&source))
    }

    pub fn describe_source(&self, source: &str) -> ScriptDocs {
        ScriptDocs::parse(source)
    }

    /// The effect row of every function in `path`, plus the effects
    /// of the top-level program.
    #[cfg(feature = "std")]
    pub fn effects_file(&self, path: &Path) -> Result<EffectReport> {
        let source = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        self.effects_source(&source)
            .with_context(|| format!("failed to analyze {}", path.display()))
    }

    pub fn effects_source(&self, source: &str) -> Result<EffectReport> {
        let program = imports::resolve_program(parse::parse_program(source)?)?;
        Ok(EffectReport {
            functions: effects::describe_function_effects(&program)?,
            top_level: effects::top_level_effects(&program)?,
        })
    }
}

/// What a whole program touches, function by function.
#[derive(Debug, Clone)]
pub struct EffectReport {
    pub functions: alloc::vec::Vec<FunctionEffects>,
    pub top_level: EffectSet,
}

#[cfg(test)]
mod tests {
    use super::UshCompiler;

    #[test]
    fn compiles_basic_program() {
        let compiler = UshCompiler;
        let output = compiler
            .compile_source(
                r#"
                let greeting = "hello"
                print greeting + " world"
                $ printf '%s\n' hi
            "#,
            )
            .expect("compile");

        assert!(output.contains("greeting='hello'"));
        assert!(output.contains("printf '%s\\n'"));
        assert!(output.contains("printf '%s\\n' hi"));
    }

    #[test]
    fn concat_is_shell_quoted() {
        let compiler = UshCompiler;
        let output = compiler
            .compile_source(
                r#"
                let greeting = "hello"
                print greeting + " world"
            "#,
            )
            .expect("compile");

        assert!(output.contains("\"$(printf '%s' \"${greeting}\" ' world')\""));
    }

    #[test]
    fn enum_is_lowered_to_tagged_shell_bindings() {
        let compiler = UshCompiler;
        let output = compiler
            .compile_source(
                r#"
                enum Option {
                  None,
                  Some(String),
                }
                let value = Option::Some("hello")
            "#,
            )
            .expect("compile");

        assert!(output.contains("value__tag='Option::Some'"));
        assert!(output.contains("value__0='hello'"));
    }

    #[test]
    fn async_await_lowers_to_task_handle_and_result_read() {
        let compiler = UshCompiler;
        let output = compiler
            .compile_source(
                r#"
                fn worker(message: String) -> String {
                  return message
                }
                let task = async worker "ok"
                let result = task.await
                print result
            "#,
            )
            .expect("compile");

        assert!(output.starts_with("#!/bin/sh\nset -eu\n"));
        assert!(output.contains("ush_fn_worker()"));
        assert!(output.contains("__ush_task_seq='0'"));
        assert!(
            output.contains(
                "__ush_task_0__result=\"${TMPDIR:-/tmp}/__ush_task_0.$$.$__ush_task_seq\""
            )
        );
        assert!(
            output.contains(
                "( __ush_return_path=\"${__ush_task_0__result}\"; ush_fn_worker 'ok' ) &"
            )
        );
        assert!(output.contains("result=\"$(cat \"${__ush_task_0__result}\")\""));
        assert!(output.contains("rm -f \"$__ush_task_file\""));
        assert!(!output.contains("mktemp"));
        assert!(output.contains("wait \"$__ush_job\""));
    }

    #[test]
    fn functional_calls_capture_sync_return_values() {
        let compiler = UshCompiler;
        let output = compiler
            .compile_source(
                r#"
                fn greet(message: String) -> String {
                  return "<" + message + ">"
                }
                let value = greet "ush"
                print $ greet (value)
            "#,
            )
            .expect("compile");

        assert!(
            output.contains("__ush_value_0=\"$(__ush_capture_return='1' ush_fn_greet 'ush')\"")
        );
        assert!(output.contains("value=\"${__ush_value_0}\""));
        assert!(
            output.contains(
                "__ush_value_1=\"$(__ush_capture_return='1' ush_fn_greet \"${value}\")\""
            )
        );
        assert!(output.contains("printf '%s\\n' \"${__ush_value_1}\""));
    }
}
