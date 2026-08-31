mod cat;
mod common;
mod diff;
mod git;
mod grep;
mod introspection;
mod ls;
mod ls_support;
mod process;
mod tasks;

pub(crate) use self::common::{badge, dim, human_bytes, paint, pluralize};
pub use self::{
    cat::render_cat,
    diff::render_diff,
    git::render_git,
    grep::render_grep,
    introspection::{render_aliases, render_env_map, render_history, render_lookup, render_which},
    ls::render_ls,
    process::{render_kill, render_ps},
    tasks::render_tasks,
};

/// Drops SGR escape sequences so renderer tests can assert on the
/// text a terminal actually shows.
#[cfg(test)]
pub(crate) fn strip_ansi(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch != '\u{1b}' {
            out.push(ch);
            continue;
        }
        for escape in chars.by_ref() {
            if escape == 'm' {
                break;
            }
        }
    }
    out
}
