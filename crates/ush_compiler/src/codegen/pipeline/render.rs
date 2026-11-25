//! The shell text of a planned pipeline.

use crate::{ast::Pipeline, types::OutputString as String, util::shell_quote};

/// One process of the lowered pipeline: a command, function or text
/// stage, with the file source and sink next to it as redirections.
pub(super) struct Process {
    pub command: String,
    pub input: Option<String>,
    pub output: Option<(String, bool)>,
    pub merge_stderr: bool,
    /// The written stages it runs: its own and any source or sink.
    pub first: usize,
    pub last: usize,
}

pub(super) fn process(
    command: String,
    source: Option<(String, usize)>,
    merge: bool,
    at: usize,
) -> Process {
    let first = source.as_ref().map_or(at, |(_, index)| *index);
    Process {
        command,
        input: source.map(|(path, _)| path),
        output: None,
        merge_stderr: merge,
        first,
        last: at,
    }
}

/// `pipeline stage N failed: TEXT`, naming the process's written stages.
fn message(pipeline: &Pipeline, process: &Process) -> String {
    let mut text = String::new();
    let mut separator = "";
    for stage in &pipeline.stages[process.first..=process.last] {
        text.push_str(separator);
        text.push_str(stage.text.trim());
        separator = if stage.merge_stderr { " |& " } else { " | " };
    }
    let mut summary = String::from(text.lines().next().unwrap_or_default());
    if summary.len() > 60 || text.contains('\n') {
        summary = summary.chars().take(57).collect::<String>() + "...";
    }
    format!("pipeline stage {} failed: {summary}", process.first + 1)
}

pub(super) fn render(pipeline: &Pipeline, processes: &[Process]) -> String {
    let mut shell = String::from("{ __ush_pipe=$(\n  exec 7>&1 >&8 8>&-\n  set +e\n");
    for (index, process) in processes.iter().enumerate() {
        let mut command = format!("{} 7>&-", process.command);
        if let Some(input) = &process.input {
            command.push_str(&format!(" <{input}"));
        }
        if let Some((output, append)) = &process.output {
            command.push_str(if *append { " >>" } else { " >" });
            command.push_str(output);
        }
        if process.merge_stderr {
            command.push_str(" 2>&1");
        }
        let pipe = if index + 1 < processes.len() {
            " |"
        } else {
            ""
        };
        let status = index + 1;
        shell.push_str(&format!(
            "  {{ {command}; printf ' {status}:%s' \"$?\" >&7; }}{pipe}\n"
        ));
    }
    shell.push_str("); } 8>&1\n__ush_pipe_check \"$__ush_pipe\"");
    for process in processes {
        shell.push(' ');
        shell.push_str(&shell_quote(&message(pipeline, process)));
    }
    shell
}
