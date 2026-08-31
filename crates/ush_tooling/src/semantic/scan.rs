//! Byte-level scanners shared by the semantic tokenizer.

pub(super) fn string_end(line: &str, start: usize) -> usize {
    let quote = line.as_bytes()[start] as char;
    let mut escaped = false;
    for (offset, ch) in line[start + 1..].char_indices() {
        if quote == '"' && !escaped && ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == quote && !(quote == '"' && escaped) {
            return start + offset + 2;
        }
        escaped = false;
    }
    line.len()
}

/// Scans a triple-quoted string body starting at `from`, returning
/// the byte index just past the closing `"""` and whether that
/// terminator was found on this line. The flag is what lets a line
/// that closes a block string go back to ordinary highlighting
/// instead of painting the rest of the file as one string.
pub(super) fn triple_string_span(line: &str, from: usize) -> (usize, bool) {
    match line[from..].find("\"\"\"") {
        Some(offset) => (from + offset + 3, true),
        None => (line.len(), false),
    }
}

pub(super) fn variable_end(line: &str, start: usize) -> usize {
    if line[start..].starts_with("${") {
        return line[start..]
            .find('}')
            .map_or(line.len(), |offset| start + offset + 1);
    }
    take_while(line, start + 1, is_ident)
}

pub(super) fn is_shell_escape(line: &str, start: usize) -> bool {
    let prefix = line[..start].trim_end();
    line[start + 1..]
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_whitespace())
        && (prefix.is_empty() || prefix.ends_with("=>"))
}

pub(super) fn operator_end(line: &str, start: usize) -> usize {
    let pair = line.get(start..start + 2).unwrap_or("");
    if matches!(
        pair,
        "->" | "=>" | "::" | "==" | "!=" | "<=" | ">=" | "&&" | "||"
    ) {
        return start + 2;
    }
    start + 1
}

pub(super) fn next_non_space(line: &str, start: usize) -> Option<char> {
    line[start..].chars().find(|ch| !ch.is_ascii_whitespace())
}

pub(super) fn take_while(line: &str, start: usize, pred: impl Fn(char) -> bool) -> usize {
    let mut end = start;
    for (offset, ch) in line[start..].char_indices() {
        if !pred(ch) {
            break;
        }
        end = start + offset + ch.len_utf8();
    }
    end.max(start)
}

pub(super) fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

pub(super) fn is_ident(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}
