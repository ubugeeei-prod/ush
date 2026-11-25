use super::{arity, integer};
use anyhow::{Context, Result, ensure};
use std::{ffi::CString, mem::MaybeUninit};

pub(super) fn format(args: &[String]) -> Result<String> {
    arity(args, 2)?;
    let seconds: libc::time_t = integer(&args[0])?;
    if args[1].is_empty() {
        return Ok(String::new());
    }
    // Restrict formats to POSIX directives. In particular, the nonstandard %s
    // can reinterpret a UTC tm as local time on some libc implementations.
    let mut chars = args[1].chars();
    let mut format = String::new();
    while let Some(ch) = chars.next() {
        if ch != '%' {
            format.push(ch);
            continue;
        }
        let directive = chars.next().context("incomplete time format directive")?;
        ensure!(
            "aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%".contains(directive),
            "unsupported time format directive: %{directive}"
        );
        match directive {
            // Darwin strftime consults TZ for these even with a UTC tm.
            'z' => format.push_str("+0000"),
            'Z' => format.push_str("UTC"),
            _ => {
                format.push('%');
                format.push(directive);
            }
        }
    }
    let format = CString::new(format)?;
    let mut tm = MaybeUninit::<libc::tm>::uninit();
    // SAFETY: seconds and tm are valid pointers; gmtime_r initializes tm on success.
    let result = unsafe { libc::gmtime_r(&seconds, tm.as_mut_ptr()) };
    ensure!(
        !result.is_null(),
        "timestamp cannot be represented as a calendar date"
    );
    // SAFETY: gmtime_r succeeded, so all fields needed by strftime are initialized.
    let tm = unsafe { tm.assume_init() };
    let mut output = vec![0u8; 65536];
    // SAFETY: output is a writable buffer of the declared size and both input
    // pointers remain valid for the duration of the call.
    let length = unsafe {
        libc::strftime(
            output.as_mut_ptr().cast(),
            output.len(),
            format.as_ptr(),
            &tm,
        )
    };
    ensure!(
        length != 0,
        "formatted date is empty or exceeds 65535 bytes"
    );
    output.truncate(length);
    String::from_utf8(output).context("formatted date is not UTF-8")
}
