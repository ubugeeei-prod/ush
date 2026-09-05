mod history;
mod process_control;
mod source;

use std::{
    env,
    path::{Component, Path, PathBuf},
};

use anyhow::{Context, Result, anyhow, bail};

use super::test_eval;
use crate::{Shell, ValueStream, expand::strip_outer_quotes, style};

/// Resolves `.` and `..` textually, without touching the filesystem.
///
/// This is what keeps `cd` logical: `..` moves up the path the user
/// typed, and a symlinked directory keeps the name it was reached by.
fn lexically_normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() {
                    out.push("..");
                }
            }
            other => out.push(other.as_os_str()),
        }
    }
    if out.as_os_str().is_empty() {
        out.push(".");
    }
    out
}

impl Shell {
    pub(super) fn change_directory(&mut self, args: &[String]) -> Result<(ValueStream, i32)> {
        let target = match args.first().map(String::as_str) {
            None => self
                .env
                .get("HOME")
                .cloned()
                .unwrap_or_else(|| ".".to_string()),
            // `cd -` returns to the previous directory and echoes it,
            // the way every other shell does.
            Some("-") => self
                .env
                .get("OLDPWD")
                .cloned()
                .ok_or_else(|| anyhow!("OLDPWD not set"))?,
            Some(value) => value.to_string(),
        };
        let echo_target = args.first().map(String::as_str) == Some("-");

        // Keep the path the user asked for rather than the one the
        // kernel resolves to. POSIX `cd` is logical unless you ask
        // for `cd -P`, and on macOS resolving symlinks silently turns
        // `cd /tmp` into `/private/tmp` — in `pwd`, in `$PWD`, and in
        // the prompt.
        let path = lexically_normalize(&self.normalize_path(&target));
        env::set_current_dir(&path)
            .with_context(|| format!("failed to change directory to {}", path.display()))?;

        let previous = std::mem::replace(&mut self.cwd, path);
        self.env
            .insert("OLDPWD".to_string(), previous.display().to_string());
        self.env
            .insert("PWD".to_string(), self.cwd.display().to_string());

        if echo_target {
            return Ok((ValueStream::Text(format!("{}\n", self.cwd.display())), 0));
        }
        Ok((ValueStream::Empty, 0))
    }

    pub(super) fn render_pwd(&self) -> String {
        if self.options.stylish {
            format!(
                "{} {}\n",
                style::paint("\u{1b}[1;34m", "cwd"),
                style::paint("\u{1b}[1;36m", self.cwd.display())
            )
        } else {
            format!("{}\n", self.cwd.display())
        }
    }

    pub(super) fn handle_alias(&mut self, args: &[String]) -> Result<(ValueStream, i32)> {
        if args.is_empty() {
            if self.options.stylish {
                return Ok((ValueStream::Text(style::render_aliases(&self.aliases)), 0));
            }

            let text = self
                .aliases
                .iter()
                .map(|(name, value)| format!("alias {name}='{}'", value.replace('\'', r#"'\''"#)))
                .collect::<Vec<_>>()
                .join("\n");
            return Ok((ValueStream::Text(with_trailing_newline(text)), 0));
        }

        for arg in args {
            let (name, value) = arg
                .split_once('=')
                .ok_or_else(|| anyhow!("alias syntax must be name=value"))?;
            self.aliases
                .insert(name.to_string(), strip_outer_quotes(value).to_string());
        }
        Ok((ValueStream::Empty, 0))
    }

    pub(super) fn handle_unalias(&mut self, args: &[String]) -> Result<(ValueStream, i32)> {
        for arg in args {
            self.aliases.remove(arg);
        }
        Ok((ValueStream::Empty, 0))
    }

    pub(super) fn handle_export(&mut self, args: &[String]) -> Result<(ValueStream, i32)> {
        for arg in args {
            let (name, value) = arg
                .split_once('=')
                .ok_or_else(|| anyhow!("export syntax must be NAME=value"))?;
            self.env.insert(name.to_string(), self.expand_value(value)?);
        }
        Ok((ValueStream::Empty, 0))
    }

    pub(super) fn handle_unset(&mut self, args: &[String]) -> Result<(ValueStream, i32)> {
        for arg in args {
            if arg.starts_with('-') && arg != "-v" {
                bail!("unsupported unset option: {arg}");
            }
            if arg != "-v" {
                self.env.remove(arg);
            }
        }
        Ok((ValueStream::Empty, 0))
    }

    pub(super) fn handle_exit(&mut self, args: &[String]) -> Result<(ValueStream, i32)> {
        let status = args
            .first()
            .and_then(|value| value.parse::<i32>().ok())
            .unwrap_or(self.last_status);
        std::process::exit(status);
    }

    pub(super) fn handle_test(&self, command: &str, args: &[String]) -> Result<(ValueStream, i32)> {
        let parts = if command == "[" {
            let Some(last) = args.last() else {
                bail!("[ requires an expression");
            };
            if last != "]" {
                bail!("[ requires a closing `]`");
            }
            &args[..args.len() - 1]
        } else {
            args
        };
        let matched = test_eval::evaluate(self, parts)?;
        Ok((ValueStream::Empty, if matched { 0 } else { 1 }))
    }
}

pub(super) fn render_echo(args: &[String]) -> String {
    let mut index = 0usize;
    while args.get(index).is_some_and(|arg| arg == "-n") {
        index += 1;
    }
    let mut text = args[index..].join(" ");
    if index == 0 {
        text.push('\n');
    }
    text
}

fn with_trailing_newline(text: String) -> String {
    if text.is_empty() {
        text
    } else {
        format!("{text}\n")
    }
}
