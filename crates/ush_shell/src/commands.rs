mod names;

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};
use which::{Error as WhichError, which_in, which_in_all};

pub(crate) use self::names::{CommandNameCache, CommandNames};

/// The builtin names in display order. Completion and `command_names`
/// iterate this; [`is_builtin`] answers lookups through
/// [`BUILTIN_COMMAND_SET`] instead, and a test keeps the two in sync.
pub(crate) const BUILTIN_COMMANDS: &[&str] = &[
    ":", ".", "[", "alias", "bg", "cd", "command", "confirm", "disown", "echo", "env", "exit",
    "export", "false", "fg", "fsam", "glob", "help", "history", "input", "jobs", "port", "pwd",
    "rm", "sammary", "select", "source", "stop", "tasks", "test", "true", "type", "unalias",
    "unset", "wait", "which",
];

/// Perfect-hash lookup for [`is_builtin`], which the parser calls
/// once per pipeline stage. A linear scan over `BUILTIN_COMMANDS`
/// costs up to 36 string comparisons for every miss — every external
/// command, in other words.
static BUILTIN_COMMAND_SET: phf::Set<&'static str> = phf::phf_set! {
    ":", ".", "[", "alias", "bg", "cd", "command", "confirm", "disown", "echo", "env", "exit",
    "export", "false", "fg", "fsam", "glob", "help", "history", "input", "jobs", "port", "pwd",
    "rm", "sammary", "select", "source", "stop", "tasks", "test", "true", "type", "unalias",
    "unset", "wait", "which",
};

/// Where to look for external commands.
///
/// `PATH` has to come from the shell's own environment rather than
/// from the process environment: a `PATH` exported by an rc file, a
/// `source`d script, or a plain `export` at the prompt only ever
/// lands in [`Shell::env`], and `std::env` still holds whatever the
/// terminal handed us at startup. Resolving through the process
/// environment is why `export PATH=...` looked like it did nothing.
///
/// [`Shell::env`]: crate::Shell
#[derive(Debug, Clone)]
pub(crate) struct CommandSearch {
    path: Option<String>,
    cwd: PathBuf,
}

impl CommandSearch {
    pub(crate) fn new(path: Option<&str>, cwd: &Path) -> Self {
        Self {
            path: path.map(str::to_string),
            cwd: cwd.to_path_buf(),
        }
    }

    /// Lookup context for callers that have no `Shell` to ask, such
    /// as the helper that picks a browser opener.
    pub(crate) fn from_process_env() -> Self {
        Self {
            path: std::env::var("PATH").ok(),
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CommandLookup {
    Alias(String),
    Builtin,
    External(PathBuf),
}

pub(crate) fn is_builtin(command: &str) -> bool {
    BUILTIN_COMMAND_SET.contains(command)
}

pub(crate) fn lookup_command(
    command: &str,
    aliases: &BTreeMap<String, String>,
    search: &CommandSearch,
) -> Option<CommandLookup> {
    lookup_all_commands(command, aliases, search)
        .into_iter()
        .next()
}

pub(crate) fn lookup_all_commands(
    command: &str,
    aliases: &BTreeMap<String, String>,
    search: &CommandSearch,
) -> Vec<CommandLookup> {
    let mut lookups = Vec::new();

    if let Some(alias) = aliases.get(command) {
        lookups.push(CommandLookup::Alias(alias.clone()));
    }
    if is_builtin(command) {
        lookups.push(CommandLookup::Builtin);
    }
    lookups.extend(
        find_all_external_commands(command, search)
            .into_iter()
            .map(CommandLookup::External),
    );

    lookups
}

pub(crate) fn find_external_command(command: &str, search: &CommandSearch) -> Option<PathBuf> {
    which_in(command, search.path.as_deref(), &search.cwd).ok()
}

pub(crate) fn find_all_external_commands(command: &str, search: &CommandSearch) -> Vec<PathBuf> {
    match which_in_all(command, search.path.as_deref(), &search.cwd) {
        Ok(paths) => {
            let mut unique = Vec::new();
            for path in paths {
                if !unique.contains(&path) {
                    unique.push(path);
                }
            }
            unique
        }
        Err(WhichError::CannotFindBinaryPath) => Vec::new(),
        Err(_) => Vec::new(),
    }
}

/// Resolves `command` to the absolute program `ush` will execute.
///
/// The absolute path matters as much as the lookup does: on Unix
/// `Command::spawn` resolves a bare program name through the *parent*
/// process' `PATH`, ignoring anything passed to `Command::env`. Handing
/// it a resolved path is the only way a `PATH` that lives in the
/// shell's own environment can decide which binary runs.
pub(crate) fn resolve_external_command(command: &str, search: &CommandSearch) -> Result<PathBuf> {
    find_external_command(command, search).ok_or_else(|| anyhow!("command not found: {command}"))
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, path::Path};

    use super::{
        BUILTIN_COMMAND_SET, BUILTIN_COMMANDS, CommandLookup, CommandSearch, is_builtin,
        lookup_command,
    };

    fn search() -> CommandSearch {
        CommandSearch::new(std::env::var("PATH").ok().as_deref(), Path::new("."))
    }

    #[test]
    fn the_lookup_set_matches_the_display_list() {
        assert_eq!(BUILTIN_COMMAND_SET.len(), BUILTIN_COMMANDS.len());
        for builtin in BUILTIN_COMMANDS {
            assert!(is_builtin(builtin), "{builtin} is missing from the set");
        }
    }

    #[test]
    fn the_display_list_has_no_duplicates() {
        let mut sorted = BUILTIN_COMMANDS.to_vec();
        sorted.sort_unstable();
        let before = sorted.len();
        sorted.dedup();
        assert_eq!(sorted.len(), before);
    }

    #[test]
    fn recognizes_builtin_names() {
        assert!(is_builtin("echo"));
        assert!(is_builtin("test"));
        assert!(!is_builtin("definitely-not-a-real-command"));
    }

    #[test]
    fn aliases_take_priority_in_lookup() {
        let mut aliases = BTreeMap::new();
        aliases.insert("ll".to_string(), "ls -la".to_string());

        assert_eq!(
            lookup_command("ll", &aliases, &search()),
            Some(CommandLookup::Alias("ls -la".to_string()))
        );
    }

    #[test]
    fn resolves_through_the_supplied_path_rather_than_the_process_path() {
        let dir = tempfile::tempdir().expect("tempdir");
        let program = dir.path().join("ush-test-only-tool");
        std::fs::write(&program, "#!/bin/sh\n").expect("write");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755))
                .expect("chmod");
        }

        let empty = CommandSearch::new(Some(""), dir.path());
        assert!(super::find_external_command("ush-test-only-tool", &empty).is_none());

        let scoped = CommandSearch::new(Some(dir.path().to_str().expect("utf-8")), dir.path());
        assert_eq!(
            super::find_external_command("ush-test-only-tool", &scoped),
            Some(program)
        );
    }
}
