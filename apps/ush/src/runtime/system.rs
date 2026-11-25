use super::{arity, index, integer, wire};
use anyhow::{Context, Result, bail, ensure};
use std::{
    collections::BTreeSet,
    fs::File,
    io::Read,
    net::ToSocketAddrs,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub(super) fn run(module: &str, name: &str, args: &[String]) -> Result<String> {
    match module {
        "int" => arithmetic(name, args),
        "random" => random(name, args),
        "os" => system(name, args),
        "time" => clock(name, args),
        "net" if name == "resolve" => {
            arity(args, 1)?;
            ensure!(!args[0].is_empty(), "host must not be empty");
            let ips: BTreeSet<_> = (args[0].as_str(), 0)
                .to_socket_addrs()?
                .map(|address| address.ip().to_string())
                .collect();
            Ok(wire::cells(ips))
        }
        _ => bail!("unknown runtime operation: {module}::{name}"),
    }
}

fn arithmetic(name: &str, args: &[String]) -> Result<String> {
    arity(args, 2)?;
    let left = integer(&args[0])?;
    let right = integer(&args[1])?;
    let result = match name {
        "add" => left.checked_add(right),
        "sub" => left.checked_sub(right),
        "mul" => left.checked_mul(right),
        "div" => left.checked_div(right),
        "rem" => left.checked_rem(right),
        "pow" => left.checked_pow(
            u32::try_from(right).context("exponent must fit an unsigned 32-bit integer")?,
        ),
        _ => bail!("unknown integer operation: {name}"),
    };
    result
        .context("integer overflow or division by zero")
        .map(|value| value.to_string())
}

fn random(name: &str, args: &[String]) -> Result<String> {
    arity(args, if name == "int" { 2 } else { 1 })?;
    let mut random = File::open("/dev/urandom")?;
    if name == "int" {
        let min = integer(&args[0])?;
        let max = integer(&args[1])?;
        ensure!(min <= max, "min must not exceed max");
        let width = (i128::from(max) - i128::from(min) + 1) as u128;
        let space = 1u128 << 64;
        let limit = space - space % width;
        loop {
            let mut bytes = [0; 8];
            random.read_exact(&mut bytes)?;
            let value = u128::from(u64::from_ne_bytes(bytes));
            if value < limit {
                return Ok((i128::from(min) + (value % width) as i128).to_string());
            }
        }
    }
    let count = index(&args[0])?;
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(count)
        .context("cannot allocate requested random bytes")?;
    bytes.resize(count, 0);
    random.read_exact(&mut bytes)?;
    match name {
        "bytes" => Ok(wire::bytes(&bytes)),
        "hex" => Ok(wire::hex(&bytes)),
        _ => bail!("unknown random operation: {name}"),
    }
}

fn system(name: &str, args: &[String]) -> Result<String> {
    arity(args, usize::from(name == "error_message"))?;
    Ok(match name {
        // SAFETY: these libc queries have no preconditions or mutable buffers.
        "euid" => unsafe { libc::geteuid() }.to_string(),
        "egid" => unsafe { libc::getegid() }.to_string(),
        "page_size" => {
            // SAFETY: _SC_PAGESIZE is a valid sysconf selector.
            let size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
            ensure!(size > 0, "page size is unavailable");
            size.to_string()
        }
        "cpu_count" => std::thread::available_parallelism()?.get().to_string(),
        "error_message" => {
            let code = i32::try_from(integer(&args[0])?)
                .context("error code must fit a signed 32-bit integer")?;
            std::io::Error::from_raw_os_error(code).to_string()
        }
        _ => bail!("unknown OS operation: {name}"),
    })
}

fn clock(name: &str, args: &[String]) -> Result<String> {
    match name {
        "unix_seconds" | "unix_millis" => {
            arity(args, 0)?;
            let elapsed = SystemTime::now().duration_since(UNIX_EPOCH)?;
            let value = if name == "unix_seconds" {
                u128::from(elapsed.as_secs())
            } else {
                elapsed.as_millis()
            };
            Ok(i64::try_from(value)
                .context("timestamp exceeds Int range")?
                .to_string())
        }
        "sleep_millis" => {
            arity(args, 1)?;
            let millis =
                u64::try_from(integer(&args[0])?).context("milliseconds must be nonnegative")?;
            std::thread::sleep(Duration::from_millis(millis));
            Ok(String::new())
        }
        "format_utc" => super::time_format::format(args),
        _ => bail!("unknown time operation: {name}"),
    }
}
