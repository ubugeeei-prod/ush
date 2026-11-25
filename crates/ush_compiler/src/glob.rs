//! `glob "…"` match patterns, lowered to one POSIX `case` pattern word.
//!
//! `*` matches any string, `?` one character and `[…]` one character
//! from a set (`[!…]` or `[^…]` negates it; `a-z` is a range and
//! `[:alpha:]` a class). A backslash makes the next character literal.
//! Every other character is literal and is quoted in the output, so
//! spaces, quotes and `$` never reach the shell as syntax.

use anyhow::{Result, bail};

use crate::types::OutputString as String;
use crate::util::shell_value_quote;

const CLASSES: &[&str] = &[
    "alnum", "alpha", "blank", "cntrl", "digit", "graph", "lower", "print", "punct", "space",
    "upper", "xdigit",
];

/// The `case` pattern word for `pattern`, or an error naming the problem.
pub(crate) fn lower(pattern: &str) -> Result<String> {
    let chars = pattern.chars().collect::<alloc::vec::Vec<_>>();
    let mut out = String::new();
    let mut literal = String::new();
    let mut index = 0usize;
    while let Some(&ch) = chars.get(index) {
        index += 1;
        match ch {
            '*' | '?' => {
                flush(&mut out, &mut literal);
                out.push(ch);
            }
            '[' => {
                flush(&mut out, &mut literal);
                index = bracket(&chars, index, &mut out)?;
            }
            '\\' => match chars.get(index) {
                Some(&next) => {
                    literal.push(next);
                    index += 1;
                }
                None => bail!("glob pattern `{pattern}` ends with a lone `\\`"),
            },
            _ => literal.push(ch),
        }
    }
    flush(&mut out, &mut literal);
    if out.is_empty() {
        out.push_str("''");
    }
    Ok(out)
}

fn flush(out: &mut String, literal: &mut String) {
    if !literal.is_empty() {
        out.push_str(&shell_value_quote(literal));
        literal.clear();
    }
}

/// Lower the bracket expression whose members start at `start`, returning
/// the index just past its closing `]`.
fn bracket(chars: &[char], start: usize, out: &mut String) -> Result<usize> {
    let text = |from: usize| chars[from..].iter().collect::<String>();
    let mut index = start;
    out.push('[');
    if matches!(chars.get(index), Some('!' | '^')) {
        out.push('!');
        index += 1;
    }
    let first = index;
    // The last single character, which a following `-` makes a range start.
    let mut last = None;
    loop {
        let Some(&ch) = chars.get(index) else {
            bail!(
                "unterminated `[` in glob pattern at `[{}`: close it with `]`, or write `\\\\[` for a literal `[`",
                text(start)
            );
        };
        index += 1;
        match ch {
            ']' if index - 1 > first => break,
            '[' if chars.get(index) == Some(&':') => {
                let rest = text(index + 1);
                let Some((name, _)) = rest.split_once(":]") else {
                    bail!(
                        "unterminated character class in glob pattern: `[{}`",
                        text(start)
                    );
                };
                if !CLASSES.contains(&name) {
                    bail!("unknown character class `[:{name}:]` in glob pattern");
                }
                out.push_str(&format!("[:{name}:]"));
                index += name.chars().count() + 3;
                last = None;
            }
            '-' if last.is_some() && chars.get(index).is_some_and(|next| *next != ']') => {
                let (end, next) = bracket_char(chars, index)?;
                index = next;
                range(last.take().unwrap_or(end), end, out)?;
            }
            _ => {
                let (member_char, next) = bracket_char(chars, index - 1)?;
                index = next;
                member(member_char, out);
                last = Some(member_char);
            }
        }
    }
    out.push(']');
    Ok(index)
}

/// The character at `index` inside `[…]`, with a `\\` escape, and the
/// index after it.
fn bracket_char(chars: &[char], index: usize) -> Result<(char, usize)> {
    match chars.get(index) {
        Some('\\') => match chars.get(index + 1) {
            Some(&escaped) => Ok((escaped, index + 2)),
            None => bail!("glob pattern ends with a lone `\\` inside `[…]`"),
        },
        Some(&ch) => Ok((ch, index + 1)),
        None => bail!("unterminated `[` in glob pattern"),
    }
}

/// A range lists its characters, so `a-z` means the same in every locale
/// and shell; `from` itself was already written.
fn range(from: char, to: char, out: &mut String) -> Result<()> {
    if !from.is_ascii() || !to.is_ascii() || from > to {
        bail!("glob range `{from}-{to}` must run upwards between ASCII characters");
    }
    for ch in (from as u8 + 1..=to as u8).map(char::from) {
        member(ch, out);
    }
    Ok(())
}

fn member(ch: char, out: &mut String) {
    if ch.is_ascii_alphanumeric() {
        out.push(ch);
    } else if ch == '\n' {
        out.push_str("\"${__ush_lf}\"");
    } else if ch == '\'' {
        out.push_str("\"'\"");
    } else {
        out.push('\'');
        out.push(ch);
        out.push('\'');
    }
}

#[cfg(test)]
mod tests {
    use super::lower;

    #[test]
    fn quotes_literals_and_keeps_wildcards() {
        assert_eq!(lower("*.ts").unwrap(), "*'.ts'");
        assert_eq!(lower("my file?.txt").unwrap(), "'my file'?'.txt'");
        assert_eq!(lower("a'b$c").unwrap(), r#"'a'"'"'b$c'"#);
        assert_eq!(lower(r"\*literal").unwrap(), "'*literal'");
        assert_eq!(lower("").unwrap(), "''");
    }

    #[test]
    fn lowers_bracket_expressions() {
        assert_eq!(lower("[!a-e]*").unwrap(), "[!abcde]*");
        assert_eq!(lower("[0-3_-]").unwrap(), "[0123'_''-']");
        assert_eq!(lower("[^ab]").unwrap(), "[!ab]");
        assert_eq!(lower("[]x]").unwrap(), "[']'x]");
        assert_eq!(lower("[ '-]").unwrap(), "[' '\"'\"'-']");
        assert_eq!(lower("[[:space:]]").unwrap(), "[[:space:]]");
    }

    #[test]
    fn rejects_malformed_brackets() {
        for (pattern, message) in [
            ("[abc", "unterminated `[`"),
            ("[[:nope:]]", "unknown character class"),
            ("x\\", "lone"),
            ("[z-a]", "must run upwards"),
        ] {
            let error = format!("{}", lower(pattern).unwrap_err());
            assert!(error.contains(message), "{error}");
        }
    }
}
