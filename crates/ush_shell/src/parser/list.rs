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

/// What [`split_and_or`] found.
pub(super) enum Split<'a> {
    /// The line is one command. `keyword` carries what the gate
    /// already learned about POSIX keywords, so the fallback check
    /// does not scan the line for them a second time.
    Whole {
        keyword: Option<bool>,
    },
    List(Vec<(Connector, &'a str)>),
}

/// Splits `source` into an and-or list on unquoted `;`, `&&`, and
/// `||`, or reports that the line has to stay whole.
///
/// Splitting is what lets `mkdir build && cd build` reach the `cd`
/// builtin at all: handing the whole line to `/bin/sh` runs every
/// part in a child process, where `ush` builtins (`cd`, `sammary`,
/// `tasks`, the structured helpers, session aliases) do not exist.
///
/// The checks are ordered by cost. Most lines contain no separator
/// character at all, and those are rejected by one `memchr` pass
/// before anything walks the line.
pub(super) fn split_and_or(source: &str) -> Split<'_> {
    // `|` alone does not count — a pipeline is one command — so the
    // cheap test looks for `;`, a newline, `&`, or a doubled `|`.
    if memchr::memchr3(b';', b'\n', b'&', source.as_bytes()).is_none() && !source.contains("||") {
        return Split::Whole { keyword: None };
    }

    // A line that opens a POSIX compound command keeps its
    // separators: `if x; then y; fi` uses `;` *inside* one command,
    // and splitting it would hand `/bin/sh` fragments that do not
    // parse. Same for a leading `(`, `{`, or `!`.
    let trimmed = source.trim_start();
    if trimmed.starts_with('!') || trimmed.starts_with('(') || trimmed.starts_with('{') {
        return Split::Whole { keyword: None };
    }
    if contains_unquoted_keyword(source) {
        return Split::Whole {
            keyword: Some(true),
        };
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

    let keyword = Some(false);
    if segments.is_empty() || single || double || backtick || depth > 0 {
        return Split::Whole { keyword };
    }

    segments.push((connector, source[start..].trim()));
    segments.retain(|(_, segment)| !segment.is_empty());
    if segments.len() < 2 {
        return Split::Whole { keyword };
    }
    Split::List(segments)
}
