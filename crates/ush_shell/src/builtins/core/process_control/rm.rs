use std::io::{self, IsTerminal, Write};

use anyhow::Result;

use crate::{Shell, ValueStream, process::ResolvedCommand};

impl Shell {
    pub(in crate::builtins) fn execute_rm(
        &mut self,
        args: &[String],
        input: ValueStream,
    ) -> Result<(ValueStream, i32)> {
        let mut filtered = Vec::new();
        let mut force_yes = false;
        for arg in args {
            if arg == "--yes" {
                force_yes = true;
            } else {
                filtered.push(arg.clone());
            }
        }

        // The guard reads from stdin, so it can only ever ask a
        // human. Editor terminals and agent runners hand `ush` a pipe
        // they never write to, and blocking on that pipe is what
        // "rm is extremely slow" turns out to be: the read waits for
        // an answer that is never coming. With no terminal to ask,
        // decline instead of blocking, and say how to proceed.
        let dangerous = rm_requests_recursive_delete(&filtered);
        if dangerous && self.options.interaction && !force_yes {
            if !io::stdin().is_terminal() {
                eprintln!(
                    "ush: refusing `rm {}`: recursive delete needs a terminal to confirm on.\n\
                     ush: pass `--yes`, or set USH_INTERACTION=false, to run it unattended.",
                    filtered.join(" ")
                );
                return Ok((ValueStream::Empty, 130));
            }

            eprint!("ush: confirm `rm {}` [y/N] ", filtered.join(" "));
            io::stderr().flush()?;
            let mut answer = String::new();
            io::stdin().read_line(&mut answer)?;
            if !matches!(answer.trim(), "y" | "Y" | "yes" | "YES") {
                return Ok((ValueStream::Empty, 130));
            }
        }

        let resolved = ResolvedCommand::new("rm", filtered);
        self.spawn_external(&resolved, input, false)
    }
}

fn rm_requests_recursive_delete(args: &[String]) -> bool {
    let mut parsing_options = true;

    for arg in args {
        if !parsing_options {
            continue;
        }
        if arg == "--" {
            parsing_options = false;
            continue;
        }
        if arg == "--recursive" || arg.starts_with("--recursive=") {
            return true;
        }
        if arg.starts_with("--") {
            continue;
        }
        let Some(flags) = arg.strip_prefix('-') else {
            continue;
        };
        if flags.is_empty() {
            continue;
        }
        if flags.chars().any(|flag| matches!(flag, 'r' | 'R')) {
            return true;
        }
    }

    false
}
