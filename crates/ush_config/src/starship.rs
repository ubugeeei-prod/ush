#[cfg(test)]
mod tests;

use std::{env, fs, path::PathBuf};

use anyhow::{Context, Result};
use directories::BaseDirs;
use serde::Deserialize;

#[derive(Debug, Clone, Default)]
pub struct StarshipPromptConfig {
    pub format: Option<String>,
    pub add_newline: bool,
    pub directory: DirectoryConfig,
    pub character: CharacterConfig,
    pub git_branch: GitBranchConfig,
}

#[derive(Debug, Clone)]
pub struct DirectoryConfig {
    pub truncation_length: usize,
    pub truncation_symbol: String,
    pub home_symbol: String,
}

impl Default for DirectoryConfig {
    fn default() -> Self {
        Self {
            truncation_length: 2,
            truncation_symbol: ".../".to_string(),
            home_symbol: "~".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CharacterConfig {
    pub success_symbol: String,
    pub error_symbol: String,
}

impl Default for CharacterConfig {
    fn default() -> Self {
        Self {
            success_symbol: "$ ".to_string(),
            error_symbol: "! ".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GitBranchConfig {
    pub format: Option<String>,
    pub symbol: String,
    pub style: String,
}

impl Default for GitBranchConfig {
    fn default() -> Self {
        Self {
            format: None,
            symbol: String::new(),
            style: "cyan".to_string(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct StarshipToml {
    format: Option<String>,
    #[serde(default)]
    add_newline: bool,
    #[serde(default)]
    directory: DirectoryToml,
    #[serde(default)]
    character: CharacterToml,
    #[serde(default)]
    git_branch: GitBranchToml,
}

#[derive(Debug, Default, Deserialize)]
struct DirectoryToml {
    truncation_length: Option<usize>,
    truncation_symbol: Option<String>,
    home_symbol: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct CharacterToml {
    success_symbol: Option<String>,
    error_symbol: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct GitBranchToml {
    format: Option<String>,
    symbol: Option<String>,
    style: Option<String>,
}

pub fn load_starship_prompt() -> Result<Option<StarshipPromptConfig>> {
    let Some(path) = starship_path() else {
        return Ok(None);
    };
    if !path.exists() {
        return Ok(None);
    }

    let source =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let config = parse_starship_prompt(&source)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    Ok(Some(config))
}

fn starship_path() -> Option<PathBuf> {
    if let Some(path) = env::var_os("STARSHIP_CONFIG") {
        return Some(PathBuf::from(path));
    }
    let base_dirs = BaseDirs::new()?;
    Some(base_dirs.config_dir().join("starship.toml"))
}

fn parse_starship_prompt(source: &str) -> Result<StarshipPromptConfig> {
    let parsed: StarshipToml = toml::from_str(source)?;
    let mut config = StarshipPromptConfig {
        format: parsed.format,
        add_newline: parsed.add_newline,
        ..StarshipPromptConfig::default()
    };

    if let Some(length) = parsed.directory.truncation_length {
        config.directory.truncation_length = length.max(1);
    }
    if let Some(symbol) = parsed.directory.truncation_symbol {
        config.directory.truncation_symbol = symbol;
    }
    if let Some(symbol) = parsed.directory.home_symbol {
        config.directory.home_symbol = symbol;
    }
    if let Some(symbol) = parsed.character.success_symbol {
        config.character.success_symbol = extract_symbol(&symbol);
    }
    if let Some(symbol) = parsed.character.error_symbol {
        config.character.error_symbol = extract_symbol(&symbol);
    }
    if let Some(format) = parsed.git_branch.format {
        config.git_branch.format = Some(format);
    }
    if let Some(symbol) = parsed.git_branch.symbol {
        config.git_branch.symbol = extract_symbol(&symbol);
    }
    if let Some(style) = parsed.git_branch.style {
        config.git_branch.style = style;
    }

    Ok(config)
}

fn extract_symbol(raw: &str) -> String {
    let trimmed = raw.trim();
    if let Some(inner) = trimmed
        .strip_prefix('[')
        .and_then(|rest| rest.split_once(']'))
    {
        return unescape_starship_text(inner.0);
    }
    unescape_starship_text(raw)
}

fn unescape_starship_text(raw: &str) -> String {
    let mut out = String::new();
    let mut escaped = false;
    for ch in raw.chars() {
        if escaped {
            out.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else {
            out.push(ch);
        }
    }
    if escaped {
        out.push('\\');
    }
    out
}
