use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};

use crate::Shell;

#[derive(Debug, Clone, Default)]
pub struct SessionStartup {
    /// Load the env file. Unlike the profile and the rc file this is
    /// on for *every* invocation, including `ush -c ...`.
    pub load_env: bool,
    pub load_profile: bool,
    /// Also read `/etc/profile`. Login shells only, and off by
    /// default so embedding callers (and tests) opt in explicitly.
    pub load_system_profile: bool,
    pub load_rc: bool,
    pub env_file: Option<PathBuf>,
    pub profile_file: Option<PathBuf>,
    pub rc_file: Option<PathBuf>,
}

#[derive(Clone, Copy)]
enum StartupKind {
    /// `~/.config/ush/env.sh` — the `.zshenv` tier.
    ///
    /// A login shell reads the profile and an interactive shell reads
    /// the rc file, which leaves `ush -c ...` reading nothing at all.
    /// That is the invocation editor terminals and agent runners use,
    /// and it is why a `PATH` configured for `ush` looked like it
    /// only existed at the prompt.
    Env,
    Profile,
    Rc,
}

struct StartupEntry {
    path: PathBuf,
    required: bool,
    label: &'static str,
}

impl Shell {
    pub fn load_session_startup(&mut self, startup: &SessionStartup) -> Result<()> {
        for entry in self.startup_entries(StartupKind::Env, startup) {
            self.load_startup_entry(entry)?;
        }
        for entry in self.startup_entries(StartupKind::Profile, startup) {
            self.load_startup_entry(entry)?;
        }
        for entry in self.startup_entries(StartupKind::Rc, startup) {
            self.load_startup_entry(entry)?;
        }
        Ok(())
    }

    fn load_startup_entry(&mut self, entry: StartupEntry) -> Result<()> {
        if !entry.path.exists() {
            if entry.required {
                bail!("missing {} file: {}", entry.label, entry.path.display());
            }
            return Ok(());
        }

        self.last_status = self
            .source_path(&entry.path)
            .with_context(|| format!("failed to load {} {}", entry.label, entry.path.display()))?;
        Ok(())
    }

    fn startup_entries(&self, kind: StartupKind, startup: &SessionStartup) -> Vec<StartupEntry> {
        let mut entries = Vec::new();
        let mut seen = BTreeSet::new();

        let (enabled, explicit, config_paths, defaults, label) = match kind {
            StartupKind::Env => (
                startup.load_env || startup.env_file.is_some(),
                startup.env_file.as_ref(),
                &self.config.shell.env_files,
                self.default_env_candidates(),
                "env",
            ),
            StartupKind::Profile => (
                startup.load_profile || startup.profile_file.is_some(),
                startup.profile_file.as_ref(),
                &self.config.shell.profile_files,
                self.default_profile_candidates(),
                "profile",
            ),
            StartupKind::Rc => (
                startup.load_rc || startup.rc_file.is_some(),
                startup.rc_file.as_ref(),
                &self.config.shell.rc_files,
                self.default_rc_candidates(),
                "rc",
            ),
        };

        if !enabled {
            return entries;
        }

        // The system profile runs before anything user-owned so a
        // user file can still override it. On macOS this is what runs
        // `path_helper`, so skipping it left a login `ush` with only
        // whatever `PATH` the terminal happened to inherit.
        if matches!(kind, StartupKind::Profile) && startup.load_system_profile {
            for path in system_profile_candidates() {
                push_unique(
                    &mut entries,
                    &mut seen,
                    StartupEntry {
                        path,
                        required: false,
                        label,
                    },
                );
            }
        }

        if let Some(path) = explicit {
            push_unique(
                &mut entries,
                &mut seen,
                StartupEntry {
                    path: self.resolve_cli_startup_path(path),
                    required: true,
                    label,
                },
            );
        }

        for path in config_paths {
            push_unique(
                &mut entries,
                &mut seen,
                StartupEntry {
                    path: self.resolve_config_startup_path(path),
                    required: true,
                    label,
                },
            );
        }

        for path in defaults {
            push_unique(
                &mut entries,
                &mut seen,
                StartupEntry {
                    path,
                    required: false,
                    label,
                },
            );
        }

        entries
    }

    fn default_env_candidates(&self) -> Vec<PathBuf> {
        let mut paths = vec![self.paths.config_dir.join("env.sh")];
        if let Some(home) = self.home_dir() {
            paths.push(home.join(".ush_env"));
        }
        paths
    }

    fn default_profile_candidates(&self) -> Vec<PathBuf> {
        let mut paths = vec![self.paths.config_dir.join("profile.sh")];
        if let Some(home) = self.home_dir() {
            paths.push(home.join(".ush_profile"));
        }
        paths
    }

    fn default_rc_candidates(&self) -> Vec<PathBuf> {
        let mut paths = vec![self.paths.config_dir.join("rc.sh")];
        if let Some(home) = self.home_dir() {
            paths.push(home.join(".ushrc"));
            paths.push(home.join(".config.ush"));
        }
        paths.push(self.paths.config_dir.join(".config.ush"));
        paths
    }

    fn resolve_config_startup_path(&self, path: &Path) -> PathBuf {
        let expanded = self.expand_startup_path(path);
        if expanded.is_absolute() {
            expanded
        } else {
            self.paths.config_dir.join(expanded)
        }
    }

    fn resolve_cli_startup_path(&self, path: &Path) -> PathBuf {
        let expanded = self.expand_startup_path(path);
        if expanded.is_absolute() {
            expanded
        } else {
            self.cwd.join(expanded)
        }
    }

    fn expand_startup_path(&self, path: &Path) -> PathBuf {
        let value = path.to_string_lossy();
        if value == "~"
            && let Some(home) = self.home_dir()
        {
            return home;
        }
        if let Some(rest) = value.strip_prefix("~/")
            && let Some(home) = self.home_dir()
        {
            return home.join(rest);
        }
        path.to_path_buf()
    }

    fn home_dir(&self) -> Option<PathBuf> {
        self.env.get("HOME").map(PathBuf::from)
    }
}

/// System-wide profile files, loaded for login shells only.
fn system_profile_candidates() -> Vec<PathBuf> {
    vec![
        PathBuf::from("/etc/profile"),
        PathBuf::from("/etc/ush/profile"),
    ]
}

fn push_unique(entries: &mut Vec<StartupEntry>, seen: &mut BTreeSet<PathBuf>, entry: StartupEntry) {
    if seen.insert(entry.path.clone()) {
        entries.push(entry);
    }
}

#[cfg(test)]
mod tests;
