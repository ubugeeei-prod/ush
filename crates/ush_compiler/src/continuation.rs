//! Lines that go on to the next one: a pipeline that ends with `|` or `|&`,
//! a command that ends with ` \`, and a `[` or `(` that is still open.

use crate::scan::{ScanState, advance, split_pipeline};
use crate::util::dollar_atom;

/// Why a line goes on to the next one.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Continuation {
    /// The line ends with `|` or `|&`; the next line holds the next stage.
    Pipeline,
    /// The line ends with ` \`; the next line holds more of its words.
    Command,
    /// A `[` or `(` is still open; the next line holds more of what it encloses.
    Bracket,
}

/// How `line` goes on to the next line, if it does.
pub fn continuation(line: &str) -> Option<Continuation> {
    let line_text = line.trim();
    // A line that ends with `{` opens a block, even with a `(` left open: a
    // typo such as `fn f( {` stays on its line rather than swallowing the block.
    if !keeps_its_line(line_text) && !line_text.ends_with('{') && open_brackets(line) > 0 {
        Some(Continuation::Bracket)
    } else if continues_pipeline(line) {
        Some(Continuation::Pipeline)
    } else if continues_command(line) {
        Some(Continuation::Command)
    } else {
        None
    }
}

/// Whether a line ends with a pipe, so the next line holds the next stage.
/// Comments and inline shell (`$ …`) keep their own line structure.
pub fn continues_pipeline(line: &str) -> bool {
    let line = line.trim();
    if keeps_its_line(line) || !line.ends_with(['|', '&']) {
        return false;
    }
    let pieces = split_pipeline(line);
    pieces.len() > 1
        && pieces
            .last()
            .is_some_and(|(rest, _)| rest.trim().is_empty())
}

/// Whether a line ends with a backslash that follows a space or tab, outside
/// any string or comment, so the command's words go on on the next line.
pub fn continues_command(line: &str) -> bool {
    let line = line.trim();
    if keeps_its_line(line) {
        return false;
    }
    let Some(words) = line.strip_suffix('\\') else {
        return false;
    };
    words.ends_with([' ', '\t']) && ends_at_top_level(words)
}

/// How many `[` and `(` are still open at the end of `source`, outside strings
/// and comments. A list or a call can then go on over several lines.
pub fn open_brackets(source: &str) -> usize {
    let mut state = ScanState::default();
    let mut index = 0usize;
    let bytes = source.as_bytes();
    while index < source.len() {
        // `#[…]` is an attribute, not a comment.
        let comment = bytes[index] == b'#'
            && bytes.get(index + 1) != Some(&b'[')
            && !state.in_string()
            && (index == 0 || bytes[index - 1].is_ascii_whitespace());
        if comment {
            break;
        }
        index = advance(source, index, &mut state);
    }
    if state.in_string() {
        return 0;
    }
    state.paren + state.bracket
}

/// `line` without the ` \` that continues it.
pub(crate) fn without_backslash(line: &str) -> &str {
    line.trim_end()
        .strip_suffix('\\')
        .map_or(line, str::trim_end)
}

/// A comment, or legacy inline shell (`$ …`), whose text is not ush words.
fn keeps_its_line(line: &str) -> bool {
    let inline_shell = line.starts_with('$') && !dollar_atom(line);
    line.starts_with('#') || inline_shell
}

/// Whether `source` ends outside strings, brackets and comments.
fn ends_at_top_level(source: &str) -> bool {
    let mut state = ScanState::default();
    let mut index = 0usize;
    while index < source.len() {
        if state.top_level() && source.as_bytes()[index] == b'#' {
            return false;
        }
        index = advance(source, index, &mut state);
    }
    state.top_level()
}
