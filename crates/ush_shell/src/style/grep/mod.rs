mod model;
mod parse;
mod render;
#[cfg(test)]
mod tests;

use std::path::Path;

use anyhow::Result;

use crate::helpers::ValueStream;

use self::{
    parse::{build_grep_command_args, capture_command_output, parse_grep_args},
    render::{parse_grep_output, render_grep_no_matches, render_grep_report},
};

pub fn render_grep(
    cwd: &Path,
    args: &[String],
    input: &ValueStream,
) -> Result<Option<(ValueStream, i32)>> {
    let Some(options) = parse_grep_args(args) else {
        return Ok(None);
    };
    if options.targets.is_empty() && input.is_empty() {
        return Ok(None);
    }

    let mut command = std::process::Command::new("grep");
    command
        .args(build_grep_command_args(&options))
        .current_dir(cwd);

    let output = match capture_command_output(&mut command, input) {
        Ok(output) => output,
        Err(_) => return Ok(None),
    };
    let status = output.status.code().unwrap_or(1);
    if status > 1 {
        return Ok(None);
    }

    let stdout = String::from_utf8(output.stdout)
        .unwrap_or_else(|error| String::from_utf8_lossy(&error.into_bytes()).to_string());
    let rendered = if status == 1 {
        render_grep_no_matches(&options)
    } else {
        render_grep_report(&options, &parse_grep_output(&stdout))
    };
    Ok(Some((ValueStream::Text(rendered), status)))
}
