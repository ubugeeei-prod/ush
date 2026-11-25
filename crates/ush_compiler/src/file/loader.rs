//! Reads modules from disk, relative to the file that imports them.

use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::paths::{join, normalize, portable, relative, written_parent};
use crate::modules::{ModuleLoader, ModuleSource};

pub(crate) struct FsLoader {
    root_key: String,
    root_display: String,
    /// The root script's canonical directory.
    root_dir: PathBuf,
    /// Directories that diagnostics and failure reports name modules in.
    display_dir: String,
    origin_dir: String,
}

impl FsLoader {
    /// `path` is the root script as given; `origin` is how the generated
    /// script names it at run time.
    pub(crate) fn new(path: &Path, origin: &str) -> Result<Self> {
        let canonical = fs::canonicalize(path)
            .with_context(|| format!("failed to canonicalize {}", path.display()))?;
        Ok(Self {
            root_key: canonical.to_string_lossy().into_owned(),
            root_display: path.display().to_string(),
            root_dir: canonical
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_default(),
            display_dir: written_parent(path),
            origin_dir: written_parent(Path::new(origin)),
        })
    }
}

impl ModuleLoader for FsLoader {
    fn root(&self) -> (&str, &str) {
        (&self.root_key, &self.root_display)
    }

    fn load(&mut self, importer: &str, path: &str) -> Result<ModuleSource> {
        let wanted = Path::new(importer)
            .parent()
            .unwrap_or(Path::new("."))
            .join(path);
        let canonical = fs::canonicalize(&wanted).with_context(|| {
            format!("module not found: {path} (looked for {})", wanted.display())
        })?;
        let text = fs::read_to_string(&canonical)
            .with_context(|| format!("failed to read module {}", canonical.display()))?;
        let dir = relative(&self.root_dir, canonical.parent().unwrap_or(Path::new("/")))?;
        let file = portable(
            canonical.file_name().and_then(|name| name.to_str()),
            "module file name",
        )?;
        let from_root = join(&dir, file);
        Ok(ModuleSource {
            key: canonical.to_string_lossy().into_owned(),
            display: normalize(&join(&self.display_dir, &from_root)),
            origin: normalize(&join(&self.origin_dir, &from_root)),
            dir,
            text,
        })
    }
}
