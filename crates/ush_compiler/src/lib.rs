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
use anyhow::{Context, ensure};
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
            codegen::SourceContext::Absolute {
                directory: source_dir.unwrap_or(""),
                path: source_path.unwrap_or(""),
            },
        )
        .with_context(|| format!("failed to compile {}", path.display()))
    }

    /// Compiles a script into a deterministic artifact intended to be checked in.
    ///
    /// The source and output must be adjacent. The generated shell resolves
    /// source-relative paths from its own location instead of embedding the
    /// build machine's absolute path.
    #[cfg(feature = "std")]
    pub fn compile_checked_in_file_with_sourcemap(
        &self,
        path: &Path,
        output: &Path,
    ) -> Result<CompiledScript> {
        let source_parent = canonical_parent(path)?;
        let output_parent = canonical_parent(output)?;
        ensure!(
            source_parent == output_parent,
            "checked-in source and output must be in the same directory"
        );

        let source_name = portable_file_name(path, "source")?;
        let output_name = portable_file_name(output, "output")?;
        let source = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        self.compile_with_context(
            &source,
            Some(source_name),
            codegen::SourceContext::CheckedIn {
                source_name,
                output_name,
            },
        )
        .with_context(|| format!("failed to compile {}", path.display()))
    }

    pub fn compile_source(&self, source: &str) -> Result<OutputString> {
        Ok(self.compile_source_with_sourcemap(source)?.shell)
    }

    pub fn compile_source_with_sourcemap(&self, source: &str) -> Result<CompiledScript> {
        self.compile_with_context(source, None, codegen::SourceContext::none())
    }

    fn compile_with_context(
        &self,
        source: &str,
        script_name: Option<&str>,
        source_context: codegen::SourceContext<'_>,
    ) -> Result<CompiledScript> {
        let program = imports::resolve_program(parse::parse_program(source)?)?;
        let docs = ScriptDocs::parse(source);
        codegen::compile_program(&program, &docs, source, script_name, source_context)
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

#[cfg(feature = "std")]
fn canonical_parent(path: &Path) -> Result<std::path::PathBuf> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::canonicalize(parent).with_context(|| format!("failed to canonicalize {}", parent.display()))
}

#[cfg(feature = "std")]
fn portable_file_name<'a>(path: &'a Path, kind: &str) -> Result<&'a str> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .with_context(|| format!("checked-in {kind} must have a UTF-8 file name"))?;
    ensure!(
        !name.contains(['\n', '\r']),
        "checked-in {kind} file name must not contain a newline"
    );
    Ok(name)
}

/// What a whole program touches, function by function.
#[derive(Debug, Clone)]
pub struct EffectReport {
    pub functions: alloc::vec::Vec<FunctionEffects>,
    pub top_level: EffectSet,
}

#[cfg(test)]
mod tests;
