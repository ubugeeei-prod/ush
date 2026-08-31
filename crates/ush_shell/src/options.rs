use std::{collections::HashMap, env, path::Path, process::Command};

use anyhow::{Context, Result};

use ush_config::UshConfig;

use super::Shell;

#[derive(Debug, Clone)]
pub struct ShellOptions {
    pub stylish: bool,
    pub interaction: bool,
    pub print_ast: bool,
}

impl ShellOptions {
    pub fn resolve(
        stylish_flag: bool,
        plain_flag: bool,
        no_interaction: bool,
        print_ast: bool,
        config: &UshConfig,
    ) -> Self {
        let env_stylish = read_bool_env(["USH_STYLISH", "UBSHELL_STYLISH"]);
        let env_interaction = read_bool_env(["USH_INTERACTION", "UBSHELL_INTERACTION"]);

        let stylish = if plain_flag {
            false
        } else {
            stylish_flag || env_stylish.unwrap_or(false) || config.shell.stylish_default
        };

        let interaction = if no_interaction {
            false
        } else {
            env_interaction.unwrap_or(config.shell.interaction)
        };

        Self {
            stylish,
            interaction,
            print_ast,
        }
    }
}

pub fn run_posix_script(script: &Path, args: &[String], options: &ShellOptions) -> Result<i32> {
    let mut env_map = env::vars().collect::<HashMap<_, _>>();
    apply_runtime_flags(&mut env_map, options);

    let status = Command::new("/bin/sh")
        .arg(script)
        .args(args)
        .envs(env_map)
        .status()
        .with_context(|| format!("failed to run {}", script.display()))?;
    Ok(status.code().unwrap_or(1))
}

impl Shell {
    pub fn new(config: UshConfig, options: ShellOptions) -> Result<Self> {
        let mut env_map = env::vars().collect::<HashMap<_, _>>();
        apply_runtime_flags(&mut env_map, &options);

        let cwd = env::current_dir().context("failed to read current directory")?;
        let paths = UshConfig::runtime_paths()?;

        Ok(Self {
            aliases: config.aliases.clone(),
            config,
            options,
            env: env_map,
            cwd,
            jobs: Vec::new(),
            next_job_id: 1,
            last_status: 0,
            paths,
            command_names: Default::default(),
        })
    }
}

fn parse_bool_env(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn read_bool_env<const N: usize>(keys: [&str; N]) -> Option<bool> {
    keys.into_iter()
        .find_map(|key| env::var(key).ok().and_then(|value| parse_bool_env(&value)))
}

fn apply_runtime_flags(env_map: &mut HashMap<String, String>, options: &ShellOptions) {
    env_map.insert(
        "USH_STYLISH".to_string(),
        if options.stylish { "true" } else { "false" }.to_string(),
    );
    env_map.insert(
        "USH_INTERACTION".to_string(),
        if options.interaction { "true" } else { "false" }.to_string(),
    );
    env_map.insert(
        "UBSHELL_STYLISH".to_string(),
        if options.stylish { "true" } else { "false" }.to_string(),
    );
    env_map.insert(
        "UBSHELL_INTERACTION".to_string(),
        if options.interaction { "true" } else { "false" }.to_string(),
    );
}
