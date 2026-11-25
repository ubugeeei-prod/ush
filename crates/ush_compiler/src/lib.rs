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
//! 3. **Effects pass** — typed-error / `T / Problem` propagation,
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
mod commands;
mod contracts;
pub use commands::command_names;
mod codegen;
mod docs;
mod effects;
mod env;
mod errors;
#[cfg(feature = "std")]
mod file;
mod imports;
mod matching;
mod parse;
mod scan;
mod sourcemap;
mod string_literal;
mod target;
mod testing;
pub use testing::TestCase;
mod traits;
mod types;
pub use target::ShellTarget;
mod util;

#[macro_use]
extern crate alloc;

use anyhow::Result;
pub use docs::ScriptDocs;
pub use effects::{Effect, EffectSet, FunctionEffects};
pub use scan::{closes_shell_capture, opens_shell_capture};
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
    target: ShellTarget,
}

#[allow(non_upper_case_globals)]
pub const UshCompiler: UshCompiler = UshCompiler {
    target: ShellTarget::Sh,
};

impl UshCompiler {
    pub const fn for_target(target: ShellTarget) -> Self {
        Self { target }
    }

    pub fn compile_source(&self, source: &str) -> Result<OutputString> {
        self.compile_with_context(source, None, codegen::SourceContext::none(), false)
            .map(|out| out.into_shell())
    }

    pub fn compile_source_with_sourcemap(&self, source: &str) -> Result<CompiledScript> {
        self.compile_with_context(source, None, codegen::SourceContext::none(), true)
            .map(|out| out.into_compiled(Some(source)))
    }

    fn compile_with_context(
        &self,
        source: &str,
        script_name: Option<&str>,
        source_context: codegen::SourceContext<'_>,
        mapped: bool,
    ) -> Result<sourcemap::OutputBuffer> {
        let program = imports::resolve_program(parse::parse_program(source)?)?;
        if script_name == Some("bin.ush") && program.iter().any(|s| matches!(&s.kind, ast::StatementKind::Function(f) if f.name=="bin" && f.scheme.as_ref().is_some_and(|scheme| !scheme.variables.is_empty()))) {
            anyhow::bail!("bin.ush requires concrete parameter types for its shell entrypoint");
        }
        let program = contracts::elaborate(program)?;
        let docs = ScriptDocs::parse(source);
        codegen::compile_program(
            &program,
            &docs,
            source,
            script_name,
            source_context,
            self.target,
            mapped,
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
        let mut functions = effects::describe_function_effects(&program)?;
        let lowered = contracts::elaborate(program)?;
        for actual in effects::describe_function_effects(&lowered)? {
            let name = actual
                .name
                .strip_prefix("__ush_inst_")
                .and_then(|name| name.rsplit_once('_').map(|(name, _)| name))
                .unwrap_or(&actual.name);
            if let Some(original) = functions.iter_mut().find(|f| f.name == name) {
                original.inferred.merge(&actual.inferred);
            }
        }
        Ok(EffectReport {
            functions,
            top_level: effects::top_level_effects(&lowered)?,
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
mod tests;
