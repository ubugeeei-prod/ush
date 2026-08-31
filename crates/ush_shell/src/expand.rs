#[cfg(test)]
mod tests;

use std::{
    borrow::Cow,
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{Result, bail};

use super::{Shell, commands::CommandNames};
use crate::prompt::render_prompt;

impl Shell {
    pub(crate) fn command_names(&self) -> CommandNames {
        self.command_names
            .borrow_mut()
            .names(self.env.get("PATH").map(String::as_str), &self.aliases)
    }

    pub(crate) fn prompt(&self) -> String {
        if let Some(prompt) = &self.config.shell.prompt {
            return prompt.clone();
        }
        render_prompt(
            &self.cwd,
            self.env.get("HOME").map(String::as_str),
            self.last_status,
            self.config.shell.starship.as_ref(),
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
        if contains_glob(&expanded) {
            let matches = glob::glob(&expanded)?
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
