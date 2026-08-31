#[cfg(test)]
mod tests;

use std::fs;

use anyhow::{Result, bail};

use crate::Shell;

pub(super) fn evaluate(shell: &Shell, args: &[String]) -> Result<bool> {
    if args.is_empty() {
        return Ok(false);
    }
    if args[0] == "!" {
        if args.len() == 1 {
            bail!("test `!` requires an expression");
        }
        return Ok(!evaluate(shell, &args[1..])?);
    }

    match args {
        [value] => Ok(!value.is_empty()),
        [op, value] => eval_unary(shell, op, value),
        [lhs, op, rhs] => eval_binary(lhs, op, rhs),
        _ => bail!("unsupported test expression"),
    }
}

fn eval_unary(shell: &Shell, op: &str, value: &str) -> Result<bool> {
    let path = shell.normalize_path(value);
    Ok(match op {
        "-n" => !value.is_empty(),
        "-z" => value.is_empty(),
        "-e" => path.exists(),
        "-f" => path.is_file(),
        "-d" => path.is_dir(),
        "-h" | "-L" => fs::symlink_metadata(path)
            .map(|meta| meta.file_type().is_symlink())
            .unwrap_or(false),
        "-r" => fs::File::open(path).is_ok(),
        "-s" => fs::metadata(path)
            .map(|meta| meta.len() > 0)
            .unwrap_or(false),
        "-x" => is_executable(&path),
        _ => bail!("unsupported test operator: {op}"),
    })
}

fn eval_binary(lhs: &str, op: &str, rhs: &str) -> Result<bool> {
    Ok(match op {
        "=" => lhs == rhs,
        "!=" => lhs != rhs,
        "-eq" => parse_int(lhs)? == parse_int(rhs)?,
        "-ne" => parse_int(lhs)? != parse_int(rhs)?,
        "-gt" => parse_int(lhs)? > parse_int(rhs)?,
        "-ge" => parse_int(lhs)? >= parse_int(rhs)?,
        "-lt" => parse_int(lhs)? < parse_int(rhs)?,
        "-le" => parse_int(lhs)? <= parse_int(rhs)?,
        _ => bail!("unsupported test operator: {op}"),
    })
}

fn parse_int(value: &str) -> Result<i64> {
    value
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("invalid integer for test: {value}"))
}

#[cfg(unix)]
fn is_executable(path: &std::path::Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    fs::metadata(path)
        .map(|meta| meta.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &std::path::Path) -> bool {
    path.is_file()
}
