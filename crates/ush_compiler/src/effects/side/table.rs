//! Which effect each stdlib entry point carries.

use super::{Effect, EffectSet};

pub(super) fn shell_effects(streams_stdout: bool) -> EffectSet {
    let mut row: EffectSet = [
        Effect::Exec,
        Effect::Fs,
        Effect::Env,
        Effect::Net,
        Effect::Stdin,
        Effect::Stderr,
    ]
    .into_iter()
    .collect();
    if streams_stdout {
        row.insert(Effect::Stdout);
    }
    row
}

/// The stdlib's effect table, keyed by fully qualified name.
///
/// `std::string` and `std::regex` are absent on purpose: they are
/// pure text transformations. `std::path` is split, because half of
/// it is path algebra and the other half asks the filesystem or the
/// environment what is actually there.
pub(super) fn builtin_call_effects(name: &str) -> EffectSet {
    if let Some(rest) = name.strip_prefix("std::") {
        return match rest {
            "process::args" | "process::arg" | "process::exit" | "process::pid" => {
                EffectSet::of(Effect::Env)
            }
            "process::chdir" => [Effect::Env, Effect::Fs].into_iter().collect(),
            "process::sleep" => EffectSet::of(Effect::Exec),
            "time::sleep_millis" => EffectSet::of(Effect::Exec),
            _ if rest.starts_with("time::")
                || rest.starts_with("random::")
                || rest.starts_with("runtime::") =>
            {
                EffectSet::of(Effect::Env)
            }
            _ if rest.starts_with("net::") => EffectSet::of(Effect::Net),
            "process::exec" => shell_effects(true),
            _ if rest.starts_with("process::") => shell_effects(false),
            "io::eprint" => EffectSet::of(Effect::Stderr),
            "io::print" | "io::println" => EffectSet::of(Effect::Stdout),
            "io::read_all" | "io::read_lines" => EffectSet::of(Effect::Stdin),
            "io::from_file"
            | "io::to_file"
            | "io::append_to"
            | "io::stderr_to"
            | "io::stderr_append_to" => EffectSet::of(Effect::Fs),
            "io::to_stderr" => EffectSet::of(Effect::Stderr),
            _ if rest.starts_with("os::") || rest.starts_with("terminal::") => {
                EffectSet::of(Effect::Env)
            }
            _ if rest.starts_with("test::") => EffectSet::of(Effect::Stderr),
            _ if rest.starts_with("fs::") => EffectSet::of(Effect::Fs),
            _ if rest.starts_with("http::") => EffectSet::of(Effect::Net),
            "command::run" => shell_effects(true),
            "command::capture" => shell_effects(false),
            "command::status" | "command::capture_stderr" => [
                Effect::Exec,
                Effect::Stdin,
                Effect::Fs,
                Effect::Env,
                Effect::Net,
            ]
            .into_iter()
            .collect(),
            "command::exists" => EffectSet::of(Effect::Exec),
            _ if rest.starts_with("env::") => EffectSet::of(Effect::Env),
            "path::cwd" | "path::from_cwd" | "path::source_root" | "path::from_source" => {
                EffectSet::of(Effect::Env)
            }
            "path::home" | "path::prepend_env" => EffectSet::of(Effect::Env),
            "path::exists" | "path::is_dir" | "path::is_file" | "path::mkdir_p"
            | "path::tmpfile" | "path::resolve" => EffectSet::of(Effect::Fs),
            _ => EffectSet::empty(),
        };
    }
    EffectSet::empty()
}

/// The same table for method syntax, which lowers to the same
/// builtins with the receiver as the first argument.
pub(super) fn builtin_method_effects(method: &str) -> EffectSet {
    match method {
        "append_text" | "copy" | "mime_type" | "mkdir_p" | "move" | "read_text" | "remove"
        | "resolve" | "sha256" | "tmpfile" | "write_text" | "exists" | "is_dir" | "is_file" => {
            EffectSet::of(Effect::Fs)
        }
        "capture" | "capture_stderr" | "run" | "status" => {
            builtin_call_effects(&format!("std::command::{method}"))
        }
        "download" | "get_bytes" => EffectSet::of(Effect::Net),
        "kill" => EffectSet::of(Effect::Task),
        _ => EffectSet::empty(),
    }
}
