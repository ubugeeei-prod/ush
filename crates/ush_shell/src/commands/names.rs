#[cfg(test)]
mod tests;

use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::PathBuf,
    sync::Arc,
    time::SystemTime,
};

use super::BUILTIN_COMMANDS;

/// Structured helper pipelines. They are not builtins — the parser
/// routes them separately — but they complete and highlight like one.
pub(crate) const HELPER_COMMANDS: &[&str] = &[
    "any",
    "car",
    "cdr",
    "drop",
    "each",
    "enumerate",
    "fany",
    "ffilter",
    "ffmap",
    "filter",
    "fjoin",
    "flat",
    "fmap",
    "frev",
    "fsome",
    "fsort",
    "fst",
    "funiq",
    "fzip",
    "head",
    "html",
    "json",
    "len",
    "length",
    "lines",
    "map",
    "nth",
    "snd",
    "some",
    "swap",
    "tail",
    "take",
    "xml",
];

/// Every name the REPL will complete or highlight as a command.
///
/// Backed by an `Arc` so the REPL can hand the same set to the line
/// editor on every prompt without copying a few thousand strings.
#[derive(Clone, Default, Debug)]
pub struct CommandNames(Arc<BTreeSet<String>>);

impl CommandNames {
    pub(crate) fn contains(&self, name: &str) -> bool {
        self.0.contains(name)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<String>> for CommandNames {
    fn from(values: Vec<String>) -> Self {
        Self(Arc::new(values.into_iter().collect()))
    }
}

impl FromIterator<String> for CommandNames {
    fn from_iter<I: IntoIterator<Item = String>>(values: I) -> Self {
        Self(Arc::new(values.into_iter().collect()))
    }
}

/// Caches the completion name set between prompts.
///
/// Rebuilding it means reading every directory on `PATH`, which is
/// thousands of directory entries on a normal machine — far too much
/// to redo after every command. Instead each directory is `stat`ed
/// (about ten syscalls) and the cached set is reused unless `PATH`,
/// one of its directories, or the alias table changed.
#[derive(Default)]
pub(crate) struct CommandNameCache {
    path: String,
    directories: Vec<(PathBuf, Option<SystemTime>)>,
    aliases: Vec<String>,
    names: CommandNames,
    populated: bool,
}

impl CommandNameCache {
    pub(crate) fn names(
        &mut self,
        path: Option<&str>,
        aliases: &BTreeMap<String, String>,
    ) -> CommandNames {
        let path = path.unwrap_or_default();
        let directories = directory_signature(path);
        if self.populated
            && self.path == path
            && self.directories == directories
            && self.aliases_match(aliases)
        {
            return self.names.clone();
        }

        self.names = build_names(path, aliases);
        self.path = path.to_string();
        self.directories = directories;
        self.aliases = aliases.keys().cloned().collect();
        self.populated = true;
        self.names.clone()
    }

    fn aliases_match(&self, aliases: &BTreeMap<String, String>) -> bool {
        self.aliases.len() == aliases.len()
            && self
                .aliases
                .iter()
                .zip(aliases.keys())
                .all(|(cached, current)| cached == current)
    }
}

/// One `stat` per `PATH` entry. A newly installed executable bumps
/// its directory's mtime, so this is enough to notice one.
fn directory_signature(path: &str) -> Vec<(PathBuf, Option<SystemTime>)> {
    env::split_paths(path)
        .map(|dir| {
            let modified = fs::metadata(&dir).and_then(|meta| meta.modified()).ok();
            (dir, modified)
        })
        .collect()
}

fn build_names(path: &str, aliases: &BTreeMap<String, String>) -> CommandNames {
    let mut names = BTreeSet::new();
    names.extend(BUILTIN_COMMANDS.iter().copied().map(str::to_string));
    names.extend(HELPER_COMMANDS.iter().copied().map(str::to_string));
    names.extend(aliases.keys().cloned());

    for dir in env::split_paths(path) {
        let Ok(entries) = fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                names.insert(name.to_string());
            }
        }
    }

    CommandNames(Arc::new(names))
}
