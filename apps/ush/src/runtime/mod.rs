//! Versioned, argv-only boundary for std APIs unavailable in portable shell.
mod bytes;
mod filesystem;
mod system;
mod text;
mod time_format;
mod wire;

use anyhow::{Context, Result, bail, ensure};
use std::io::{self, Write};

pub fn handle() -> Result<bool> {
    let mut args = std::env::args_os().skip(1);
    if args.next().as_deref() != Some(std::ffi::OsStr::new("__runtime")) {
        return Ok(false);
    }
    let args: Vec<String> = args
        .map(|a| {
            a.into_string()
                .map_err(|_| anyhow::anyhow!("runtime arguments must be UTF-8"))
        })
        .collect::<Result<_>>()?;
    ensure!(
        args.first().is_some_and(|a| a == "v1"),
        "unsupported std runtime protocol; this ush supports v1"
    );
    let operation = args.get(1).context("missing std runtime operation")?;
    let output = dispatch(operation, &args[2..]).with_context(|| operation.clone())?;
    io::stdout().lock().write_all(output.as_bytes())?;
    Ok(true)
}

fn dispatch(operation: &str, args: &[String]) -> Result<String> {
    if operation == "ping" {
        arity(args, 0)?;
        return Ok("ush-runtime-v1".into());
    }
    let Some((module, name)) = operation
        .strip_prefix("std::")
        .and_then(|s| s.split_once("::"))
    else {
        bail!("unknown std runtime operation: {operation}");
    };
    match module {
        "string" => text::run(name, args),
        "bytes" => bytes::run(name, args),
        "fs" => filesystem::run(name, args),
        "time" | "random" | "net" | "os" | "int" => system::run(module, name, args),
        _ => bail!("unknown std runtime operation: {operation}"),
    }
}

fn arity(args: &[String], expected: usize) -> Result<()> {
    ensure!(
        args.len() == expected,
        "expected {expected} arguments, received {}",
        args.len()
    );
    Ok(())
}

fn integer(value: &str) -> Result<i64> {
    value.parse().context("expected a signed 64-bit integer")
}

fn index(value: &str) -> Result<usize> {
    usize::try_from(integer(value)?).context("expected a nonnegative index")
}
