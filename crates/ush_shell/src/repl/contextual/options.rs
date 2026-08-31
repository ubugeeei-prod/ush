#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

use compact_str::CompactString;

use super::types::Tokens;

#[derive(Clone, Copy)]
pub(crate) struct OptionSpec {
    pub(crate) names: &'static [&'static str],
    pub(crate) values: usize,
    pub(crate) path_value: bool,
    pub(crate) short_inline_value: bool,
}

pub(crate) const fn option_spec(
    names: &'static [&'static str],
    values: usize,
    path_value: bool,
    short_inline_value: bool,
) -> OptionSpec {
    OptionSpec {
        names,
        values,
        path_value,
        short_inline_value,
    }
}

pub(crate) fn pending_value_kind(args: &[CompactString], specs: &[OptionSpec]) -> Option<bool> {
    let mut pending = None;

    for arg in args {
        if let Some((remaining, path_value)) = pending {
            pending = (remaining > 1).then_some((remaining - 1, path_value));
            continue;
        }
        if let Some((spec, inline)) = match_option(arg, specs)
            && spec.values > 0
            && !inline
        {
            pending = Some((spec.values, spec.path_value));
        }
    }

    pending.map(|(_, path_value)| path_value)
}

pub(crate) fn positional_args(args: &[CompactString], specs: &[OptionSpec]) -> Tokens {
    let mut values = Tokens::new();
    let mut pending = 0usize;

    for arg in args {
        if pending > 0 {
            pending -= 1;
            continue;
        }
        if let Some((spec, inline)) = match_option(arg, specs) {
            if spec.values > 0 && !inline {
                pending = spec.values;
            }
            continue;
        }
        if !arg.starts_with('-') {
            values.push(arg.clone());
        }
    }

    values
}

pub(crate) fn explicit_makefile_path(cwd: &Path, args: &[CompactString]) -> Option<PathBuf> {
    let mut index = 0usize;
    let mut path = None;

    while index < args.len() {
        let arg = args[index].as_str();
        match arg {
            "-f" | "--file" | "--makefile" => {
                if let Some(next) = args.get(index + 1) {
                    path = Some(resolve_relative(cwd, next));
                }
                index += 2;
                continue;
            }
            _ => {
                if let Some(value) = inline_value(arg, "--file")
                    .or_else(|| inline_value(arg, "--makefile"))
                    .or_else(|| short_inline_value(arg, "-f"))
                {
                    path = Some(resolve_relative(cwd, value));
                }
            }
        }
        index += 1;
    }

    path
}

pub(crate) fn match_option<'a>(
    arg: &str,
    specs: &'a [OptionSpec],
) -> Option<(&'a OptionSpec, bool)> {
    for spec in specs {
        for name in spec.names {
            if arg == *name {
                return Some((spec, false));
            }
            if inline_value(arg, name).is_some() {
                return Some((spec, true));
            }
            if spec.short_inline_value && short_inline_value(arg, name).is_some() {
                return Some((spec, true));
            }
        }
    }
    None
}

fn inline_value<'a>(arg: &'a str, name: &str) -> Option<&'a str> {
    name.strip_prefix("--")
        .filter(|_| arg.starts_with(name))
        .and_then(|_| {
            arg.as_bytes()
                .get(name.len())
                .is_some_and(|byte| *byte == b'=')
                .then_some(())
        })
        .map(|_| &arg[name.len() + 1..])
}

fn short_inline_value<'a>(arg: &'a str, name: &str) -> Option<&'a str> {
    if !name.starts_with("--") && arg.starts_with(name) && arg.len() > name.len() {
        Some(&arg[name.len()..])
    } else {
        None
    }
}

fn resolve_relative(cwd: &Path, value: &str) -> PathBuf {
    let path = Path::new(value);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}
