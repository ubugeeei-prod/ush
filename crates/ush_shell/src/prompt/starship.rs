use std::{path::Path, process::Command};

use ush_config::StarshipPromptConfig;

use self::format::{ensure_prompt_suffix, render_starship_format, strip_starship_markup};
use super::path::compact_path_with;

mod format;

pub(super) fn render_prompt(
    cwd: &Path,
    home: Option<&str>,
    last_status: i32,
    config: &StarshipPromptConfig,
) -> String {
    let directory = compact_path_with(
        cwd,
        home,
        config.directory.truncation_length,
        &config.directory.truncation_symbol,
        &config.directory.home_symbol,
    );
    let character = normalize_character(if last_status == 0 {
        &config.character.success_symbol
    } else {
        &config.character.error_symbol
    });
    let git_branch = current_git_branch(cwd).map(|branch| render_git_branch(&branch, config));

    if let Some(format) = &config.format {
        let rendered =
            render_starship_format(format, &directory, &character, git_branch.as_deref());
        return ensure_prompt_suffix(rendered);
    }

    if config.add_newline {
        return format!("{directory}\n{character}");
    }
    format!("{directory} {character}")
}

fn normalize_character(symbol: &str) -> String {
    if symbol.ends_with(char::is_whitespace) {
        symbol.to_string()
    } else {
        format!("{symbol} ")
    }
}

pub(crate) fn current_git_branch(cwd: &Path) -> Option<String> {
    let branch = git_output(cwd, &["rev-parse", "--abbrev-ref", "HEAD"])?;
    if branch != "HEAD" {
        return Some(branch);
    }
    let commit = git_output(cwd, &["rev-parse", "--short", "HEAD"])?;
    Some(commit)
}

fn git_output(cwd: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(cwd)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8(output.stdout).ok()?;
    let trimmed = text.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

fn render_git_branch(branch: &str, config: &StarshipPromptConfig) -> String {
    let git_branch = &config.git_branch;
    let format = git_branch
        .format
        .as_deref()
        .unwrap_or("[$symbol$branch]($style) ");
    let rendered = format
        .replace("$symbol", &git_branch.symbol)
        .replace("$branch", branch)
        .replace("$style", &git_branch.style);
    strip_starship_markup(&rendered)
}

#[cfg(test)]
mod tests {
    use ush_config::StarshipPromptConfig;

    #[test]
    fn renders_git_branch_markup() {
        let mut starship = StarshipPromptConfig::default();
        starship.git_branch.symbol = "|- ".into();
        starship.git_branch.format = Some(" [$symbol$branch]($style)".into());

        assert_eq!(super::render_git_branch("main", &starship), " |- main");
    }
}
