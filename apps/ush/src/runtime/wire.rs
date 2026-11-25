use anyhow::{Result, ensure};
use std::fmt::Write;

pub(super) fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(out, "{byte:02x}").expect("String writes cannot fail");
    }
    out
}

pub(super) fn unhex(text: &str) -> Result<Vec<u8>> {
    ensure!(
        text.len().is_multiple_of(2) && text.bytes().all(|b| b.is_ascii_hexdigit()),
        "expected an even number of hexadecimal digits"
    );
    text.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| Ok(u8::from_str_radix(std::str::from_utf8(pair)?, 16)?))
        .collect()
}

pub(super) fn cells(items: impl IntoIterator<Item = impl AsRef<[u8]>>) -> String {
    let mut out = String::new();
    for item in items {
        out.push('x');
        out.push_str(&hex(item.as_ref()));
        out.push(' ');
    }
    out
}

pub(super) fn bytes(values: &[u8]) -> String {
    cells(values.iter().map(u8::to_string))
}

pub(super) fn read_bytes(wire: &str) -> Result<Vec<u8>> {
    wire.split_ascii_whitespace()
        .map(|cell| {
            let digits = cell
                .strip_prefix('x')
                .ok_or_else(|| anyhow::anyhow!("invalid byte list encoding"))?;
            let decoded = unhex(digits)?;
            Ok(std::str::from_utf8(&decoded)?.parse::<u8>()?)
        })
        .collect()
}
