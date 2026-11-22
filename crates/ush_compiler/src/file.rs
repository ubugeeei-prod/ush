use super::{CompiledScript, OutputString, UshCompiler, codegen, sourcemap};
use anyhow::{Context, Result, ensure};
use std::{fs, path::Path};

impl UshCompiler {
    pub fn compile_file(&self, path: &Path) -> Result<OutputString> {
        self.compile_file_as(path, false, |out, _| out.into_shell())
    }

    pub fn compile_file_with_sourcemap(&self, path: &Path) -> Result<CompiledScript> {
        self.compile_file_as(path, true, |out, source| out.into_compiled(Some(source)))
    }

    fn compile_file_as<T>(
        &self,
        path: &Path,
        mapped: bool,
        finish: impl FnOnce(sourcemap::OutputBuffer, &str) -> T,
    ) -> Result<T> {
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
                origin: path.to_str().unwrap_or("<source>"),
            },
            mapped,
        )
        .map(|out| finish(out, &source))
        .with_context(|| format!("failed to compile {}", path.display()))
    }

    /// Emits a relocatable artifact adjacent to its source.
    pub fn compile_checked_in_file(&self, path: &Path, output: &Path) -> Result<OutputString> {
        self.compile_checked_in_as(path, output, false, |out, _| out.into_shell())
    }

    /// Emits a relocatable artifact and its generated-line index.
    pub fn compile_checked_in_file_with_sourcemap(
        &self,
        path: &Path,
        output: &Path,
    ) -> Result<CompiledScript> {
        self.compile_checked_in_as(path, output, true, |out, src| out.into_compiled(Some(src)))
    }

    fn compile_checked_in_as<T>(
        &self,
        path: &Path,
        output: &Path,
        mapped: bool,
        finish: impl FnOnce(sourcemap::OutputBuffer, &str) -> T,
    ) -> Result<T> {
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
            mapped,
        )
        .map(|out| finish(out, &source))
        .with_context(|| format!("failed to compile {}", path.display()))
    }
}

fn canonical_parent(path: &Path) -> Result<std::path::PathBuf> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::canonicalize(parent).with_context(|| format!("failed to canonicalize {}", parent.display()))
}

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
