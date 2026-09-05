//! And-or lists: `a && b`, `a || b`, `a; b`, and newline-separated
//! commands.

use super::ParsedLine;
use super::fallback::contains_unquoted_keyword;

/// One element of an and-or list, together with the operator that
/// joined it to the element before it.
#[derive(Debug, Clone)]
pub struct ListItem {
    pub connector: Connector,
    pub line: ParsedLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Connector {
    /// `;` — run regardless of the previous status. Also the
    /// connector of the first element.
    Always,
    /// `&&` — run only when the previous status was `0`.
    And,
    /// `||` — run only when the previous status was non-zero.
    Or,
}

/// Splits `source` into an and-or list on unquoted `;`, `&&`, and
/// `||`, or reports `None` when the line has to stay whole.
///
/// Splitting is what lets `mkdir build && cd build` reach the `cd`
/// builtin at all: handing the whole line to `/bin/sh` runs every
/// part in a child process, where `ush` builtins (`cd`, `sammary`,
/// `tasks`, the structured helpers, session aliases) do not exist.
///
/// Lines that open a POSIX compound command are left alone. `if x;
/// then y; fi` uses `;` as an *inner* separator, so splitting it
/// would hand `/bin/sh` fragments that do not parse. The same goes
/// for a leading `(`, `{`, or `!`.
pub(super) fn split_and_or(source: &str) -> Option<Vec<(Connector, &str)>> {
    let trimmed = source.trim_start();
    if trimmed.starts_with('!') || trimmed.starts_with('(') || trimmed.starts_with('{') {
        return None;
    }
    if contains_unquoted_keyword(source) {
        return None;
    }

    let mut segments: Vec<(Connector, &str)> = Vec::new();
    let mut connector = Connector::Always;
    let mut start = 0usize;
    let mut single = false;
    let mut double = false;
    let mut escaped = false;
    let mut backtick = false;
    let mut depth = 0usize;

    let bytes = source.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        let ch = bytes[index] as char;
        if escaped {
            escaped = false;
            index += 1;
            continue;
        }
        match ch {
            '\\' if !single => escaped = true,
            '\'' if !double && !backtick => single = !single,
            '"' if !single && !backtick => double = !double,
            '`' if !single && !double => backtick = !backtick,
            _ if single || double || backtick => {}
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            _ if depth > 0 => {}
            '&' if bytes.get(index + 1) == Some(&b'&') => {
                segments.push((connector, source[start..index].trim()));
                connector = Connector::And;
                index += 2;
                start = index;
                continue;
            }
            '|' if bytes.get(index + 1) == Some(&b'|') => {
                segments.push((connector, source[start..index].trim()));
                connector = Connector::Or;
                index += 2;
                start = index;
                continue;
            }
            // A newline separates commands exactly like `;` does.
            // Without this, `ush -c $'cd build\nmake'` — and every
            // multi-line paste at the prompt — was parsed as one
            // enormous command.
            ';' | '\n' => {
                segments.push((connector, source[start..index].trim()));
                connector = Connector::Always;
                index += 1;
                start = index;
                continue;
            }
            _ => {}
        }
        index += 1;
    }

    if segments.is_empty() {
        return None;
    }
    if single || double || backtick || depth > 0 {
        return None;
    }

    segments.push((connector, source[start..].trim()));
    segments.retain(|(_, segment)| !segment.is_empty());
    (segments.len() > 1).then_some(segments)
}
