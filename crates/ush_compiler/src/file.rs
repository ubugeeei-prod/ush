//! Compiling scripts that live on disk, with their modules.

mod loader;
mod paths;

pub(crate) use loader::FsLoader;

use super::{Compiled, CompiledScript, OutputString, UshCompiler, codegen};
use anyhow::{Context, Result};
use std::{fs, path::Path};

impl UshCompiler {
    pub fn compile_file(&self, path: &Path) -> Result<OutputString> {
        let source = read(path)?;
        self.compile_source_at(&source, path)
            .with_context(|| format!("failed to compile {}", path.display()))
    }

    pub fn compile_file_with_sourcemap(&self, path: &Path) -> Result<CompiledScript> {
        let source = read(path)?;
        self.compile_at(&source, path, file_name(path), true)
            .map(Compiled::into_script)
            .with_context(|| format!("failed to compile {}", path.display()))
    }

    /// Compiles `source` as the contents of `path`: its modules are read
    /// beside `path` and source-relative lookups resolve from there. Editors
    /// use this to check a buffer that differs from the file on disk.
    pub fn compile_source_at(&self, source: &str, path: &Path) -> Result<OutputString> {
        self.compile_at(source, path, file_name(path), false)
            .map(Compiled::into_shell)
    }

    /// `script_name` selects `bin.ush` entrypoint handling.
    pub(crate) fn compile_at(
        &self,
        source: &str,
        path: &Path,
        script_name: Option<&str>,
        mapped: bool,
    ) -> Result<Compiled> {
        let origin = path.to_str().unwrap_or("<source>");
        let absolute = fs::canonicalize(path)
            .with_context(|| format!("failed to canonicalize {}", path.display()))?;
        let mut loader = FsLoader::new(path, origin)?;
        self.compile_with_context(
            source,
            script_name,
            codegen::SourceContext::Absolute {
                directory: absolute
                    .parent()
                    .and_then(|item| item.to_str())
                    .unwrap_or(""),
                path: absolute.to_str().unwrap_or(""),
                origin,
            },
            &mut loader,
            mapped,
        )
    }

    /// Emits a relocatable artifact meant to be committed. `output` may
    /// sit anywhere: the script locates its source through the relative
    /// path recorded in it.
    pub fn compile_checked_in_file(&self, path: &Path, output: &Path) -> Result<OutputString> {
        self.compile_checked_in(path, output, false)
            .map(Compiled::into_shell)
    }

    /// Emits a relocatable artifact and its generated-line index.
    pub fn compile_checked_in_file_with_sourcemap(
        &self,
        path: &Path,
        output: &Path,
    ) -> Result<CompiledScript> {
        self.compile_checked_in(path, output, true)
            .map(Compiled::into_script)
    }

    fn compile_checked_in(&self, path: &Path, output: &Path, mapped: bool) -> Result<Compiled> {
        let source_dir = canonical_parent(path)?;
        let output_dir = canonical_parent(output)?;
        let file = paths::portable(
            path.file_name().and_then(|n| n.to_str()),
            "checked-in source file name",
        )?;
        let output_name = paths::portable(
            output.file_name().and_then(|n| n.to_str()),
            "checked-in output file name",
        )?;
        let source_name = paths::join(&paths::relative(&output_dir, &source_dir)?, file);
        let source = read(path)?;
        let mut loader = FsLoader::new(path, &source_name)?;
        self.compile_with_context(
            &source,
            Some(file),
            codegen::SourceContext::CheckedIn {
                source_name: &source_name,
                output_name,
            },
            &mut loader,
            mapped,
        )
        .with_context(|| format!("failed to compile {}", path.display()))
    }
}

fn file_name(path: &Path) -> Option<&str> {
    path.file_name().and_then(|name| name.to_str())
}

fn read(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))
}

fn canonical_parent(path: &Path) -> Result<std::path::PathBuf> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::canonicalize(parent).with_context(|| format!("failed to canonicalize {}", parent.display()))
}
