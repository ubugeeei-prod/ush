#[cfg(test)]
mod tests;

use std::{
    borrow::Cow,
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{Result, bail};

use super::{
    Shell,
    commands::{CommandNames, CommandSearch},
};
use crate::prompt::{
    PromptContext, current_git_branch, render_prompt, render_template, wants_git_branch,
};

impl Shell {
    pub(crate) fn command_names(&self) -> CommandNames {
        self.command_names
            .borrow_mut()
            .names(self.env.get("PATH").map(String::as_str), &self.aliases)
    }

    /// Where external commands are looked up: the shell's own
    /// `PATH` plus its own working directory, never `std::env`.
    pub(crate) fn command_search(&self) -> CommandSearch {
        CommandSearch::new(self.env.get("PATH").map(String::as_str), &self.cwd)
    }

    pub(crate) fn prompt(&self) -> String {
        if let Some(template) = self.prompt_template() {
            return self.render_prompt_template(&template);
        }
        render_prompt(
            &self.cwd,
            self.env.get("HOME").map(String::as_str),
            self.last_status,
            self.config.shell.starship.as_ref(),
        )
    }

    /// The prompt template in effect, most specific source first.
    ///
    /// `USH_PROMPT` and `PS1` come from the shell's own environment,
    /// so an rc file can set the prompt with a plain `export` — which
    /// is what people reach for first, and what silently did nothing
    /// while `shell.prompt` was the only supported source.
    fn prompt_template(&self) -> Option<String> {
        for key in ["USH_PROMPT", "PS1"] {
            if let Some(value) = self.env.get(key)
                && !value.is_empty()
            {
                return Some(value.clone());
            }
        }
        self.config.shell.prompt.clone()
    }

    fn render_prompt_template(&self, template: &str) -> String {
        let expanded = self
            .expand_value(template)
            .unwrap_or_else(|_| template.to_string());
        let branch = wants_git_branch(&expanded)
            .then(|| current_git_branch(&self.cwd))
            .flatten();

        render_template(
            &expanded,
            &PromptContext {
                cwd: &self.cwd,
                home: self.env.get("HOME").map(String::as_str),
                user: self.env.get("USER").map(String::as_str),
                host: self.env.get("HOSTNAME").map(String::as_str),
                last_status: self.last_status,
                git_branch: branch.as_deref(),
            },
        )
    }

    pub(crate) fn expand_args(&self, args: &[String]) -> Result<Vec<String>> {
        let mut expanded = Vec::new();
        for arg in args {
            expanded.extend(self.expand_arg(arg)?);
        }
        Ok(expanded)
    }

    pub(crate) fn expand_value(&self, value: &str) -> Result<String> {
        let value = expand_home(value, &self.env);
        Ok(expand_vars(&value, &self.env, self.last_status)?.into_owned())
    }

    pub(crate) fn normalize_path(&self, value: &str) -> PathBuf {
        let path = Path::new(value);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.cwd.join(path)
        }
    }

    fn expand_arg(&self, arg: &str) -> Result<Vec<String>> {
        let expanded = self.expand_value(arg)?;
        // A word that merely *looks* like a glob is still a word.
        // `echo [$?]` is not a character class, and failing the whole
        // command over it is worse than passing the text through.
        if contains_glob(&expanded)
            && let Ok(paths) = glob::glob(&expanded)
        {
            let matches = paths
                .filter_map(Result::ok)
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>();
            if !matches.is_empty() {
                return Ok(matches);
            }
        }
        Ok(vec![expanded])
    }
}

pub(crate) fn strip_outer_quotes(value: &str) -> &str {
    value
        .strip_prefix('"')
        .and_then(|inner| inner.strip_suffix('"'))
        .or_else(|| {
            value
                .strip_prefix('\'')
                .and_then(|inner| inner.strip_suffix('\''))
        })
        .unwrap_or(value)
}

fn expand_home<'a>(value: &'a str, env: &HashMap<String, String>) -> Cow<'a, str> {
    if value == "~" {
        return match env.get("HOME") {
            Some(home) => Cow::Owned(home.clone()),
            None => Cow::Borrowed("~"),
        };
    }
    if let Some(rest) = value.strip_prefix("~/")
        && let Some(home) = env.get("HOME")
    {
        return Cow::Owned(format!("{home}/{rest}"));
    }
    Cow::Borrowed(value)
}

/// Expands `$NAME`, `${NAME}`, and `$?` against `env`.
///
/// Runs on every argument of every command, so it walks the bytes
/// directly and borrows the input untouched when there is nothing to
/// expand. Every character it reacts to is ASCII, which keeps the
/// byte offsets on `char` boundaries.
fn expand_vars<'a>(
    value: &'a str,
    env: &HashMap<String, String>,
    last_status: i32,
) -> Result<Cow<'a, str>> {
    let bytes = value.as_bytes();
    let Some(first) = memchr::memchr(b'$', bytes) else {
        return Ok(Cow::Borrowed(value));
    };

    let mut result = String::with_capacity(value.len());
    result.push_str(&value[..first]);
    let mut index = first;

    while index < bytes.len() {
        if bytes[index] != b'$' {
            let rest = &bytes[index..];
            let run = memchr::memchr(b'$', rest).unwrap_or(rest.len());
            result.push_str(&value[index..index + run]);
            index += run;
            continue;
        }
        if index + 1 >= bytes.len() {
            result.push('$');
            break;
        }

        match bytes[index + 1] {
            b'?' => {
                result.push_str(&last_status.to_string());
                index += 2;
            }
            b'{' => {
                let Some(offset) = memchr::memchr(b'}', &bytes[index + 2..]) else {
                    bail!("unterminated variable expansion");
                };
                let end = index + 2 + offset;
                push_var(&mut result, &value[index + 2..end], env);
                index = end + 1;
            }
            next if next == b'_' || next.is_ascii_alphabetic() => {
                let mut end = index + 1;
                while end < bytes.len()
                    && (bytes[end] == b'_' || bytes[end].is_ascii_alphanumeric())
                {
                    end += 1;
                }
                push_var(&mut result, &value[index + 1..end], env);
                index = end;
            }
            _ => {
                result.push('$');
                index += 1;
            }
        }
    }

    Ok(Cow::Owned(result))
}

fn push_var(out: &mut String, name: &str, env: &HashMap<String, String>) {
    if let Some(value) = env.get(name) {
        out.push_str(value);
    }
}

fn contains_glob(value: &str) -> bool {
    value.contains('*') || value.contains('?') || value.contains('[')
}
