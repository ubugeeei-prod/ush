mod io;
mod jobs;
mod state;
use self::io::{finish_child, populate_command, write_input};
pub(crate) use self::jobs::Job;
use self::state::{StatefulShellRun, render_alias_prelude};
use super::{Shell, ValueStream, commands, signal, style};
use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
const STATE_CHUNK_FILE: &str = "USH_INTERNAL_CHUNK_FILE";
const STATE_CWD_FILE: &str = "USH_INTERNAL_STATE_CWD";
const STATE_ENV_FILE: &str = "USH_INTERNAL_STATE_ENV";
const STATE_ALIAS_FILE: &str = "USH_INTERNAL_STATE_ALIAS";
const STATE_DONE_FILE: &str = "USH_INTERNAL_STATE_DONE";

#[derive(Debug)]
pub(crate) struct ResolvedCommand {
    command: String,
    args: Vec<String>,
    env_overrides: HashMap<String, String>,
}

impl ResolvedCommand {
    pub(crate) fn new(command: &str, args: Vec<String>) -> Self {
        Self {
            command: command.to_string(),
            args,
            env_overrides: HashMap::new(),
        }
    }
}

impl Shell {
    pub(crate) fn execute_external(
        &mut self,
        spec: &super::parser::CommandSpec,
        input: ValueStream,
        capture: bool,
    ) -> Result<(ValueStream, i32)> {
        let resolved = ResolvedCommand {
            command: spec.command.clone(),
            args: self.expand_args(&spec.args)?,
            env_overrides: spec
                .assignments
                .iter()
                .map(|(name, value)| Ok((name.clone(), self.expand_value(value)?)))
                .collect::<Result<HashMap<_, _>>>()?,
        };
        if self.options.stylish
            && let Some((rendered, status)) = self.try_stylish(&resolved, &input)?
        {
            return Ok((rendered, status));
        }
        self.spawn_external(&resolved, input, capture)
    }

    pub(crate) fn spawn_external(
        &mut self,
        resolved: &ResolvedCommand,
        input: ValueStream,
        capture: bool,
    ) -> Result<(ValueStream, i32)> {
        // Resolve through the shell's own `PATH` and hand `Command`
        // an absolute program: `execvp` would otherwise search the
        // *parent* process' `PATH`, which is not the one the user
        // just exported.
        let program =
            commands::resolve_external_command(&resolved.command, &self.command_search())?;
        let mut command = Command::new(&program);
        populate_command(
            &mut command,
            &self.cwd,
            &self.env,
            &resolved.env_overrides,
            &resolved.args,
            input.is_empty(),
            capture,
        );
        self.spawn_command(
            &mut command,
            input,
            capture,
            &format!("failed to run {}", resolved.command),
        )
    }

    pub(crate) fn execute_posix_shell_stage(
        &mut self,
        spec: &super::parser::CommandSpec,
        input: ValueStream,
        capture: bool,
    ) -> Result<(ValueStream, i32)> {
        let overrides = spec
            .assignments
            .iter()
            .map(|(name, value)| Ok((name.clone(), self.expand_value(value)?)))
            .collect::<Result<HashMap<_, _>>>()?;
        let args = self.expand_args(&spec.args)?;
        let mut command = Command::new("/bin/sh");
        populate_command(
            &mut command,
            &self.cwd,
            &self.env,
            &overrides,
            &args,
            input.is_empty(),
            capture,
        );
        self.spawn_command(&mut command, input, capture, "failed to spawn /bin/sh")
    }

    pub(crate) fn run_fallback(
        &mut self,
        source: &str,
        input: ValueStream,
        capture: bool,
    ) -> Result<(ValueStream, i32)> {
        let state = StatefulShellRun::new(source, &self.aliases)?;
        let mut command = Command::new("/bin/sh");
        command.arg(state.runner_path());
        state.populate_command_env(&mut command);
        populate_command(
            &mut command,
            &self.cwd,
            &self.env,
            &HashMap::new(),
            &[],
            input.is_empty(),
            capture,
        );
        let result = self.spawn_command(&mut command, input, capture, "failed to spawn /bin/sh")?;
        state.apply(self)?;
        Ok(result)
    }

    pub fn run_compiled_script(
        &mut self,
        origin: &Path,
        script: &str,
        args: &[String],
    ) -> Result<i32> {
        let mut command = Command::new("/bin/sh");
        command
            .arg("-s")
            .arg("--")
            .args(args)
            .current_dir(&self.cwd)
            .envs(&self.env)
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        signal::prepare_foreground_command(&mut command);
        let _sigint = signal::SigintGuard::ignore()?;
        let mut child = command.spawn().with_context(|| {
            format!("failed to launch compiled script for {}", origin.display())
        })?;
        child
            .stdin
            .as_mut()
            .context("compiled script stdin unavailable")?
            .write_all(script.as_bytes())?;
        self.last_status = signal::exit_status(child.wait()?);
        Ok(self.last_status)
    }

    fn spawn_command(
        &mut self,
        command: &mut Command,
        input: ValueStream,
        capture: bool,
        context: &str,
    ) -> Result<(ValueStream, i32)> {
        signal::prepare_foreground_command(command);
        let _sigint = signal::SigintGuard::ignore()?;
        let mut child = command.spawn().with_context(|| context.to_string())?;
        write_input(&mut child, &input)?;
        finish_child(child, capture)
    }

    fn try_stylish(
        &self,
        resolved: &ResolvedCommand,
        input: &ValueStream,
    ) -> Result<Option<(ValueStream, i32)>> {
        match resolved.command.as_str() {
            "ls" => Ok(style::render_ls(&self.cwd, &resolved.args)?.map(|output| (output, 0))),
            "cat" => {
                Ok(style::render_cat(&self.cwd, &resolved.args, input)?.map(|output| (output, 0)))
            }
            "ps" => Ok(style::render_ps(&resolved.args)?.map(|output| (output, 0))),
            "kill" => Ok(style::render_kill(&resolved.args)?.map(|output| (output, 0))),
            "grep" => style::render_grep(&self.cwd, &resolved.args, input),
            "git" => Ok(style::render_git(&self.cwd, &resolved.args)?.map(|output| (output, 0))),
            "diff" => style::render_diff(&self.cwd, &resolved.args),
            _ => Ok(None),
        }
    }
}

pub(crate) fn is_posix_shell_command(command: &str) -> bool {
    matches!(command, "sh" | "/bin/sh")
}
