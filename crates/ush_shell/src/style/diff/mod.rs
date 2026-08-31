mod model;
mod parse;
mod render;
#[cfg(test)]
mod tests;

use std::{path::Path, process::Command};

use anyhow::Result;

use crate::helpers::ValueStream;

use self::{
    parse::{build_diff_command_args, parse_diff_args},
    render::{render_diff_clean, render_diff_report},
};

pub fn render_diff(cwd: &Path, args: &[String]) -> Result<Option<(ValueStream, i32)>> {
    let Some(options) = parse_diff_args(args) else {
        return Ok(None);
    };

    let output = match Command::new("diff")
        .args(build_diff_command_args(&options))
        .current_dir(cwd)
        .output()
    {
        Ok(output) => output,
        Err(_) => return Ok(None),
    };

    let status = output.status.code().unwrap_or(1);
    if status > 1 {
        return Ok(None);
    }

    let stdout = String::from_utf8(output.stdout)
        .unwrap_or_else(|error| String::from_utf8_lossy(&error.into_bytes()).to_string());
    let rendered = if status == 0 {
        render_diff_clean(&options)
    } else {
        render_diff_report(&options, &stdout)
    };
    Ok(Some((ValueStream::Text(rendered), status)))
}
