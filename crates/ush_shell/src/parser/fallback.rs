/// Longest POSIX keyword we look for (`while`, `until`). Words longer
/// than this can never match, so the scanner stops buffering them.
const MAX_KEYWORD_LEN: usize = 5;

pub(super) fn needs_posix_fallback(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with('!') || trimmed.starts_with('(') || trimmed.starts_with('{') {
        return true;
    }

    let mut chars = line.char_indices().peekable();
    let mut single = false;
    let mut double = false;

    while let Some((index, ch)) = chars.next() {
        match ch {
            '\'' if !double => single = !single,
            '"' if !single => double = !double,
            _ if single || double => {}
            ';' | '`' | '&' | '<' => return true,
            '>' if line.get(index.saturating_sub(1)..index + 1) != Some("->") => return true,
            '$' if matches!(chars.peek(), Some((_, '('))) => return true,
            _ => {}
        }
    }

    // `&&`, `||`, and `;` no longer imply a fallback: the parser
    // splits an and-or list before it gets here, so the only ones
    // left in a segment are quoted, and `echo 'a && b'` has no
    // business spawning `/bin/sh`.
    contains_unquoted_keyword(line)
}

/// True when `line` contains any POSIX shell keyword as an unquoted
/// word. One pass over the line with a fixed-size word buffer: the
/// previous shape re-scanned the line (and allocated a `String`) once
/// per keyword, which is pure overhead on the interactive fast path.
pub(super) fn contains_unquoted_keyword(line: &str) -> bool {
    let mut single = false;
    let mut double = false;
    let mut escaped = false;
    let mut word = KeywordWord::default();

    for ch in line.chars() {
        match ch {
            '\\' if !single => {
                escaped = !escaped;
                if !word.is_empty() && !escaped && word.take_is_keyword() {
                    return true;
                }
            }
            '\'' if !double && !escaped => {
                if !word.is_empty() && word.take_is_keyword() {
                    return true;
                }
                single = !single;
            }
            '"' if !single && !escaped => {
                if !word.is_empty() && word.take_is_keyword() {
                    return true;
                }
                double = !double;
            }
            _ if single || double => escaped = false,
            _ if ch == '_' || ch.is_ascii_alphanumeric() => {
                word.push(ch);
                escaped = false;
            }
            _ => {
                if word.take_is_keyword() {
                    return true;
                }
                escaped = false;
            }
        }
    }

    word.take_is_keyword()
}

/// A word under construction, held inline. Anything longer than the
/// longest keyword is remembered only as "too long to match", so the
/// buffer never needs to grow.
#[derive(Default)]
struct KeywordWord {
    buffer: [u8; MAX_KEYWORD_LEN],
    len: usize,
    overflowed: bool,
}

impl KeywordWord {
    fn is_empty(&self) -> bool {
        self.len == 0 && !self.overflowed
    }

    fn push(&mut self, ch: char) {
        // Only `_` and ASCII alphanumerics reach here, so one byte
        // per character.
        if self.len < MAX_KEYWORD_LEN {
            self.buffer[self.len] = ch as u8;
            self.len += 1;
        } else {
            self.overflowed = true;
        }
    }

    /// Reports whether the buffered word is a keyword and starts a
    /// new word either way.
    fn take_is_keyword(&mut self) -> bool {
        let matched = !self.overflowed && is_posix_keyword(&self.buffer[..self.len]);
        self.len = 0;
        self.overflowed = false;
        matched
    }
}

fn is_posix_keyword(word: &[u8]) -> bool {
    matches!(
        word,
        b"if"
            | b"elif"
            | b"else"
            | b"for"
            | b"while"
            | b"until"
            | b"case"
            | b"do"
            | b"done"
            | b"then"
            | b"fi"
            | b"esac"
    )
}
