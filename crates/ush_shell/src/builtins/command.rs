use anyhow::{Result, bail};

use super::introspection::{LookupStyle, describe_commands, describe_which};
use crate::{Shell, ValueStream, commands, parser::CommandSpec, style};

impl Shell {
    pub(super) fn handle_env(
        &mut self,
        args: &[String],
        input: ValueStream,
    ) -> Result<(ValueStream, i32)> {
        let mut overrides = Vec::new();
        let mut index = 0usize;
        while let Some(arg) = args.get(index) {
            if let Some((name, value)) = arg.split_once('=') {
                overrides.push((name.to_string(), value.to_string()));
                index += 1;
                continue;
            }
            break;
        }

        let rest = &args[index..];
        if rest.is_empty() {
            let mut vars = self.env.clone();
            vars.extend(overrides);
            let text = if self.options.stylish {
                style::render_env_map(&vars)
            } else {
                render_env(&vars)
            };
            return Ok((ValueStream::Text(text), 0));
        }

        let spec = make_spec(rest);
        if commands::is_builtin(&spec.command) {
            return self
                .with_env_overrides(&overrides, |shell| shell.execute_builtin(&spec, input));
        }
        self.execute_external(
            &CommandSpec {
                assignments: overrides,
                ..spec
            },
            input,
            true,
        )
    }

    pub(super) fn handle_command_builtin(
        &mut self,
        args: &[String],
        input: ValueStream,
    ) -> Result<(ValueStream, i32)> {
        match args.first().map(String::as_str) {
            Some("-v") => self.handle_lookup(&args[1..], LookupStyle::Path, "command -v"),
            Some("-V") => self.handle_lookup(&args[1..], LookupStyle::Verbose, "command -V"),
            Some(option) if option.starts_with('-') => {
                bail!("unsupported command option: {option}")
            }
            _ => self.execute_ignoring_aliases(args, input),
        }
    }

    pub(super) fn handle_lookup(
        &self,
        args: &[String],
        style: LookupStyle,
        name: &str,
    ) -> Result<(ValueStream, i32)> {
        if args.is_empty() {
            bail!("{name} requires at least one command name");
        }

        if self.options.stylish && name == "which" {
            let mut rows = Vec::new();
            let mut status = 0;
            for arg in args {
                let matches =
                    commands::lookup_all_commands(arg, &self.aliases, &self.command_search());
                if matches.is_empty() {
                    status = 1;
                }
                rows.push((arg.clone(), matches));
            }
            return Ok((ValueStream::Text(style::render_which(name, &rows)), status));
        }

        if self.options.stylish && matches!(name, "type" | "command -v" | "command -V") {
            let mut rows = Vec::new();
            let mut status = 0;
            for arg in args {
                let result = commands::lookup_command(arg, &self.aliases, &self.command_search());
                if result.is_none() {
                    status = 1;
                }
                rows.push((arg.clone(), result));
            }
            return Ok((ValueStream::Text(style::render_lookup(name, &rows)), status));
        }

        if name == "which" {
            let (text, status) = describe_which(&self.aliases, args, &self.command_search());
            return Ok((ValueStream::Text(text), status));
        }

        let (text, status) = describe_commands(&self.aliases, args, style, &self.command_search());
        Ok((ValueStream::Text(text), status))
    }

    fn execute_ignoring_aliases(
        &mut self,
        args: &[String],
        input: ValueStream,
    ) -> Result<(ValueStream, i32)> {
        let spec = make_spec(args);
        if commands::is_builtin(&spec.command) {
            return self.execute_builtin(&spec, input);
        }
        self.execute_external(&spec, input, true)
    }

    fn with_env_overrides<T>(
        &mut self,
        overrides: &[(String, String)],
        apply: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<T> {
        let previous = overrides
            .iter()
            .map(|(name, _)| (name.clone(), self.env.get(name).cloned()))
            .collect::<Vec<_>>();
        for (name, value) in overrides {
            self.env.insert(name.clone(), value.clone());
        }
        let result = apply(self);
        for (name, value) in previous {
            match value {
                Some(value) => {
                    self.env.insert(name, value);
                }
                None => {
                    self.env.remove(&name);
                }
            }
        }
        result
    }
}

fn make_spec(args: &[String]) -> CommandSpec {
    let Some(command) = args.first() else {
        return CommandSpec {
            raw: String::new(),
            command: String::new(),
            args: Vec::new(),
            assignments: Vec::new(),
        };
    };
    CommandSpec {
        raw: args.join(" "),
        command: command.clone(),
        args: args[1..].to_vec(),
        assignments: Vec::new(),
    }
}

fn render_env(env: &std::collections::HashMap<String, String>) -> String {
    let mut entries = env.iter().collect::<Vec<_>>();
    entries.sort_by_key(|(name, _)| *name);
    let mut text = entries
        .into_iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<_>>()
        .join("\n");
    if !text.is_empty() {
        text.push('\n');
    }
    text
}
