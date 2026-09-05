//! Comment stripping.

use std::borrow::Cow;

/// Removes unquoted `#` comments from `source`.
///
/// The interactive path calls this for every keystroke-completed
/// line, so it borrows when there is nothing to strip. Input can
/// span several lines (a multi-line `-c` string, a paste at the
/// prompt), and a comment ends at its own newline — truncating the
/// rest of the input at the first `#` would silently drop every
/// command after a commented one.
pub(super) fn strip_comment(source: &str) -> Cow<'_, str> {
    let Some(first) = comment_start(source, 0) else {
        return Cow::Borrowed(source);
    };

    let mut out = String::with_capacity(source.len());
    let mut cursor = 0usize;
    let mut comment = Some(first);
    while let Some(start) = comment {
        out.push_str(&source[cursor..start]);
        cursor = source[start..]
            .find('\n')
            .map_or(source.len(), |offset| start + offset);
        comment = comment_start(source, cursor);
    }
    out.push_str(&source[cursor..]);
    Cow::Owned(out)
}

/// The index of the next unquoted `#` that starts a comment, scanning
/// from `from`. Quote state is tracked from the beginning of the
/// input so a `#` inside a string that opened on an earlier line is
/// still recognised as text.
fn comment_start(source: &str, from: usize) -> Option<usize> {
    let mut single = false;
    let mut double = false;
    for (index, ch) in source.char_indices() {
        match ch {
            '\'' if !double => single = !single,
            '"' if !single => double = !double,
            '#' if !single
                && !double
                && index >= from
                && (index == 0 || source[..index].ends_with(char::is_whitespace)) =>
            {
                return Some(index);
            }
            _ => {}
        }
    }
    None
}
