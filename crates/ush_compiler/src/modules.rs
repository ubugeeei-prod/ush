//! Local modules: `mod "lib/gh.ush" as gh`.
//!
//! The linker loads every module a script imports, resolves each file
//! against its own imports, moves a module's items into a namespace of
//! their own (`gh::build`), and inlines the result into one program.
//! Later passes see a single file whose functions happen to have
//! qualified names, so the emitted script stays standalone.
//!
//! Files come from a [`ModuleLoader`], which keeps this core free of
//! filesystem access: the `std` build supplies one that reads beside
//! the importing file.

mod checks;
mod exports;
mod link;
mod reach;
mod relocate;
mod rename;
mod scope;
mod space;
#[cfg(test)]
mod tests;
mod visit;

use alloc::string::String;
use anyhow::{Result, bail};

pub(crate) use link::{Linked, link};
pub(crate) use scope::Scope;
pub(crate) use space::{SourceFile, SourceSpace, locate};

/// A module file supplied by a [`ModuleLoader`].
pub(crate) struct ModuleSource {
    /// Identity: imports with the same key share one module.
    pub key: String,
    /// How compile-time diagnostics name the file.
    pub display: String,
    /// How runtime failure reports name the file.
    pub origin: String,
    /// The module's directory relative to the root script's directory,
    /// empty when they are the same.
    pub dir: String,
    pub text: String,
}

pub(crate) trait ModuleLoader {
    /// The root script's key and display name.
    fn root(&self) -> (&str, &str);
    /// Loads `path` as written in the file whose key is `importer`.
    fn load(&mut self, importer: &str, path: &str) -> Result<ModuleSource>;
}

/// For source compiled from memory, which has no directory to read
/// modules from.
pub(crate) struct NoModules;

impl ModuleLoader for NoModules {
    fn root(&self) -> (&str, &str) {
        ("<memory>", "<memory>")
    }

    fn load(&mut self, _: &str, path: &str) -> Result<ModuleSource> {
        bail!("cannot load module {path:?}: source compiled from memory has no directory")
    }
}
