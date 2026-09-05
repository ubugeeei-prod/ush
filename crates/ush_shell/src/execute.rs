use std::env;
use std::io::{self, Write};

use anyhow::{Result, bail};
use ush_config::ShellKeymap;

use super::{
    ParsedLine, Shell, ValueStream,
    parser::{Connector, ListItem, Stage},
    process, repl,
};

impl Shell {
    pub fn execute(&mut self, line: &str) -> Result<i32> {
        let parsed = super::parse_line(line, &self.aliases)?;
        self.execute_parsed(&parsed)
    }

    fn execute_parsed(&mut self, parsed: &ParsedLine) -> Result<i32> {
        match parsed {
            ParsedLine::Empty => Ok(self.last_status),
            ParsedLine::Background(source) => {
                let text = self.spawn_background_job(source)?;
                print!("{text}");
                io::stdout().flush()?;
                self.finish((ValueStream::Empty, 0))
            }
            ParsedLine::Fallback(source) => {
                let result = self.run_fallback(source, ValueStream::Empty, false)?;
                self.finish(result)
            }
            ParsedLine::List(items) => self.execute_list(items),
            ParsedLine::Pipeline(pipeline) => {
                if self.options.print_ast {
                    eprintln!("{pipeline:#?}");
                }

                let mut stream = ValueStream::Empty;
                let mut status = 0;
                let last_stage = pipeline.stages.len().saturating_sub(1);

                for (index, stage) in pipeline.stages.iter().enumerate() {
                    let capture = index < last_stage;
                    let result = match stage {
                        Stage::Assignments(assignments) => {
                            self.apply_assignments(assignments, pipeline.stages.len())?
                        }
                        Stage::Builtin(spec) => self.execute_builtin(spec, stream.clone())?,
                        Stage::External(spec) if is_posix_shell_consumer(stage, index) => {
                            self.execute_posix_shell_stage(spec, stream.clone(), capture)?
                        }
                        Stage::External(spec) => {
                            self.execute_external(spec, stream.clone(), capture)?
                        }
                        Stage::Helper(helper) => helper.execute(stream.clone())?,
                    };
                    stream = result.0;
                    status = result.1;
                }

                if !stream.is_empty() {
                    print!("{}", stream.to_text()?);
                    io::stdout().flush()?;
                }

                self.last_status = status;
                Ok(status)
            }
        }
    }

    /// Runs an and-or list left to right, short-circuiting the way
    /// POSIX does: `&&` needs the previous status to be `0`, `||`
    /// needs it to be non-zero, and a skipped element leaves the
    /// status it was tested against untouched.
    fn execute_list(&mut self, items: &[ListItem]) -> Result<i32> {
        let mut status = self.last_status;
        for (index, item) in items.iter().enumerate() {
            let run = index == 0
                || match item.connector {
                    Connector::Always => true,
                    Connector::And => status == 0,
                    Connector::Or => status != 0,
                };
            if !run {
                continue;
            }
            // A builtin that fails reports a Rust error rather than
            // an exit status. Inside a list that has to become a
            // status, or `cd missing || echo fallback` would abandon
            // the line instead of running the fallback.
            status = match self.execute_parsed(&item.line) {
                Ok(status) => status,
                Err(error) => {
                    eprintln!("ush: {error:#}");
                    1
                }
            };
            self.last_status = status;
        }
        self.last_status = status;
        Ok(status)
    }

    pub fn run_repl(&mut self) -> Result<()> {
        let mut editor = repl::create_editor(
            &self.paths.history_file,
            self.config.shell.history_size,
            resolved_keymap(self.config.shell.keymap),
            self.command_names(),
            self.env.keys().cloned().collect(),
            self.cwd.clone(),
        )?;

        loop {
            if let Some(helper) = editor.helper_mut() {
                helper.refresh(
                    self.command_names(),
                    self.env.keys().cloned().collect(),
                    self.cwd.clone(),
                    self.aliases.keys().cloned().collect(),
                    self.repl_job_candidates(),
                );
            }
            match editor.readline(&self.prompt()) {
                Ok(line) => self.handle_repl_line(&mut editor, &line)?,
                Err(rustyline::error::ReadlineError::Interrupted) => eprintln!("^C"),
                Err(rustyline::error::ReadlineError::Eof) => {
                    println!();
                    break;
                }
                Err(error) => return Err(repl::classify_readline_error(error)),
            }
        }

        Ok(())
    }

    fn apply_assignments(
        &mut self,
        assignments: &[(String, String)],
        stage_count: usize,
    ) -> Result<(ValueStream, i32)> {
        if stage_count != 1 {
            bail!("standalone assignments cannot be used inside pipelines");
        }
        for (name, value) in assignments {
            self.env.insert(name.clone(), self.expand_value(value)?);
        }
        Ok((ValueStream::Empty, 0))
    }

    fn handle_repl_line(
        &mut self,
        editor: &mut rustyline::Editor<repl::UshHelper, rustyline::history::DefaultHistory>,
        line: &str,
    ) -> Result<()> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Ok(());
        }
        if let Err(error) = self.execute(trimmed) {
            eprintln!("ush: {error:#}");
            self.last_status = 1;
        }
        let _ = editor.save_history(&self.paths.history_file);
        Ok(())
    }

    fn finish(&mut self, result: (ValueStream, i32)) -> Result<i32> {
        self.last_status = result.1;
        Ok(result.1)
    }
}

fn resolved_keymap(default: ShellKeymap) -> ShellKeymap {
    env::var("USH_KEYMAP")
        .ok()
        .and_then(|value| parse_keymap(&value))
        .unwrap_or(default)
}

fn parse_keymap(value: &str) -> Option<ShellKeymap> {
    match value.trim().to_ascii_lowercase().as_str() {
        "emacs" => Some(ShellKeymap::Emacs),
        "vi" | "vim" => Some(ShellKeymap::Vi),
        _ => None,
    }
}

fn is_posix_shell_consumer(stage: &Stage, index: usize) -> bool {
    index > 0
        && matches!(
            stage,
            Stage::External(spec) if process::is_posix_shell_command(&spec.command)
        )
}

#[cfg(test)]
mod tests {
    use ush_config::ShellKeymap;

    use crate::parser::{CommandSpec, Stage};

    use super::{is_posix_shell_consumer, parse_keymap};

    #[test]
    fn detects_pipe_to_sh_from_ast_stage() {
        let stage = Stage::External(CommandSpec {
            raw: "sh".to_string(),
            command: "sh".to_string(),
            args: Vec::new(),
            assignments: Vec::new(),
        });

        assert!(!is_posix_shell_consumer(&stage, 0));
        assert!(is_posix_shell_consumer(&stage, 1));
    }

    #[test]
    fn parses_vi_keymap_aliases() {
        assert_eq!(parse_keymap("vi"), Some(ShellKeymap::Vi));
        assert_eq!(parse_keymap("vim"), Some(ShellKeymap::Vi));
        assert_eq!(parse_keymap("emacs"), Some(ShellKeymap::Emacs));
    }
}
