use std::path::Path;

use ush_config::StarshipPromptConfig;

mod path;
mod starship;
mod template;

#[cfg(test)]
mod tests;

pub(crate) use self::starship::current_git_branch;
pub(crate) use self::template::{PromptContext, render_template, wants_git_branch};

pub(crate) fn render_prompt(
    cwd: &Path,
    home: Option<&str>,
    last_status: i32,
    starship: Option<&StarshipPromptConfig>,
) -> String {
    starship
        .map(|config| starship::render_prompt(cwd, home, last_status, config))
        .unwrap_or_else(|| default_prompt(cwd, home, last_status))
}

pub(crate) fn default_prompt(cwd: &Path, home: Option<&str>, last_status: i32) -> String {
    let mark = if last_status == 0 { "$" } else { "!" };
    format!("{} {} ", path::compact_path(cwd, home), mark)
}
