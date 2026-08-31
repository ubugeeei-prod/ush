#[cfg(test)]
mod tests;

use std::{
    collections::{BTreeMap, HashMap},
    fs,
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result};
use tempfile::NamedTempFile;

use crate::Shell;

use super::{STATE_ALIAS_FILE, STATE_CHUNK_FILE, STATE_CWD_FILE, STATE_DONE_FILE, STATE_ENV_FILE};

pub(super) struct StatefulShellRun {
    chunk: NamedTempFile,
    runner: NamedTempFile,
    cwd: NamedTempFile,
    env: NamedTempFile,
    aliases: NamedTempFile,
    done: NamedTempFile,
}

impl StatefulShellRun {
    pub(super) fn new(source: &str, aliases: &BTreeMap<String, String>) -> Result<Self> {
        let mut chunk = NamedTempFile::new().context("failed to create fallback chunk file")?;
        chunk
            .write_all(source.as_bytes())
            .context("failed to write fallback chunk")?;

        let cwd = NamedTempFile::new().context("failed to create cwd snapshot file")?;
        let env = NamedTempFile::new().context("failed to create env snapshot file")?;
        let aliases_snapshot =
            NamedTempFile::new().context("failed to create alias snapshot file")?;
        let done = NamedTempFile::new().context("failed to create done snapshot file")?;
        let mut runner = NamedTempFile::new().context("failed to create fallback runner file")?;
        runner
            .write_all(render_runner_script(aliases).as_bytes())
            .context("failed to write fallback runner")?;

        Ok(Self {
            chunk,
            runner,
            cwd,
            env,
            aliases: aliases_snapshot,
            done,
        })
    }

    pub(super) fn runner_path(&self) -> &Path {
        self.runner.path()
    }

    pub(super) fn populate_command_env(&self, command: &mut Command) {
        command
            .env(STATE_CHUNK_FILE, self.chunk.path())
            .env(STATE_CWD_FILE, self.cwd.path())
            .env(STATE_ENV_FILE, self.env.path())
            .env(STATE_ALIAS_FILE, self.aliases.path())
            .env(STATE_DONE_FILE, self.done.path());
    }

    pub(super) fn apply(&self, shell: &mut Shell) -> Result<()> {
        if !snapshot_completed(self.done.path())? {
            return Ok(());
        }

        let env = read_env_snapshot(self.env.path())?;
        if !env.is_empty() {
            shell.env = env;
        }

        if let Some(cwd) = read_cwd_snapshot(self.cwd.path())? {
            shell.cwd = cwd;
            shell
                .env
                .insert("PWD".to_string(), shell.cwd.display().to_string());
        }

        shell.aliases = read_alias_snapshot(self.aliases.path())?;
        Ok(())
    }
}

pub(super) fn render_alias_prelude(aliases: &BTreeMap<String, String>) -> String {
    let mut prelude = String::new();
    for (name, value) in aliases {
        prelude.push_str("alias ");
        prelude.push_str(name);
        prelude.push('=');
        prelude.push_str(&shell_quote(value));
        prelude.push('\n');
    }
    prelude
}

fn render_runner_script(aliases: &BTreeMap<String, String>) -> String {
    let mut script = String::from(
        "#!/bin/sh\n\
__ush_dump_state() {\n\
  __ush_status=$?\n\
  trap - 0\n\
  __ush_state_cwd=$USH_INTERNAL_STATE_CWD\n\
  __ush_state_env=$USH_INTERNAL_STATE_ENV\n\
  __ush_state_alias=$USH_INTERNAL_STATE_ALIAS\n\
  __ush_state_done=$USH_INTERNAL_STATE_DONE\n\
  unset USH_INTERNAL_CHUNK_FILE USH_INTERNAL_STATE_CWD USH_INTERNAL_STATE_ENV USH_INTERNAL_STATE_ALIAS USH_INTERNAL_STATE_DONE\n\
  pwd > \"$__ush_state_cwd\" 2>/dev/null || :\n\
  env > \"$__ush_state_env\" 2>/dev/null || :\n\
  alias > \"$__ush_state_alias\" 2>/dev/null || :\n\
  printf 'ok\\n' > \"$__ush_state_done\" 2>/dev/null || :\n\
  exit \"$__ush_status\"\n\
}\n\
trap '__ush_dump_state' 0\n",
    );

    script.push_str(&render_alias_prelude(aliases));
    script.push_str("eval \"$(cat \"$USH_INTERNAL_CHUNK_FILE\")\"\n");
    script
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', r#"'\''"#))
}

fn snapshot_completed(path: &Path) -> Result<bool> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text.trim() == "ok"),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error)
            .with_context(|| format!("failed to read done snapshot from {}", path.display())),
    }
}

fn read_cwd_snapshot(path: &Path) -> Result<Option<PathBuf>> {
    let value = fs::read_to_string(path)
        .with_context(|| format!("failed to read cwd snapshot from {}", path.display()))?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    Ok(Some(trimmed.into()))
}

fn read_env_snapshot(path: &Path) -> Result<HashMap<String, String>> {
    let value = fs::read_to_string(path)
        .with_context(|| format!("failed to read env snapshot from {}", path.display()))?;
    let mut env = HashMap::new();
    for line in value.lines() {
        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        env.insert(name.to_string(), value.to_string());
    }
    Ok(env)
}

fn read_alias_snapshot(path: &Path) -> Result<BTreeMap<String, String>> {
    let value = fs::read_to_string(path)
        .with_context(|| format!("failed to read alias snapshot from {}", path.display()))?;
    let mut aliases = BTreeMap::new();
    for line in value.lines() {
        let tokens = shell_words::split(line)
            .with_context(|| format!("failed to parse alias snapshot line: {line}"))?;
        let Some(binding) = tokens.first() else {
            continue;
        };
        let Some((name, value)) = binding.split_once('=') else {
            continue;
        };
        aliases.insert(name.to_string(), value.to_string());
    }
    Ok(aliases)
}
