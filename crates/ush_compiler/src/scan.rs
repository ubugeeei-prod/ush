#[derive(Clone, Copy, Default)]
pub(crate) struct ScanState {
    single: bool,
    double: bool,
    triple: bool,
    escaped: bool,
    pub(crate) paren: usize,
    pub(crate) brace: usize,
    pub(crate) bracket: usize,
}

impl ScanState {
    pub(crate) fn top_level(&self) -> bool {
        !self.single
            && !self.double
            && !self.triple
            && self.paren == 0
            && self.brace == 0
            && self.bracket == 0
    }

    pub(crate) fn in_string(&self) -> bool {
        self.single || self.double || self.triple
    }
}

pub(crate) fn starts_triple_quote(source: &str, index: usize) -> bool {
    source.as_bytes().get(index..index + 3) == Some(br#"""""#)
}

pub(crate) fn advance(source: &str, index: usize, state: &mut ScanState) -> usize {
    if state.triple {
        return advance_triple(source, index, state);
    }

    let bytes = source.as_bytes();
    let ch = source[index..]
        .chars()
        .next()
        .expect("scanner index must be in bounds");

    if state.single {
        if ch == '\'' {
            state.single = false;
        }
        return index + ch.len_utf8();
    }
    if state.double {
        if state.escaped {
            state.escaped = false;
        } else if bytes[index] == b'\\' {
            state.escaped = true;
        } else if ch == '"' {
            state.double = false;
        }
        return index + ch.len_utf8();
    }
    if starts_triple_quote(source, index) {
        state.triple = true;
        return index + 3;
    }

    match ch {
        '\'' => state.single = true,
        '"' => state.double = true,
        '(' => state.paren += 1,
        ')' if state.paren > 0 => state.paren -= 1,
        '{' => state.brace += 1,
        '}' if state.brace > 0 => state.brace -= 1,
        '[' => state.bracket += 1,
        ']' if state.bracket > 0 => state.bracket -= 1,
        _ => {}
    }
    index + ch.len_utf8()
}

pub(crate) fn brace_delta(source: &str) -> isize {
    let mut delta = 0isize;
    let mut state = ScanState::default();
    let mut index = 0usize;

    while index < source.len() {
        let byte = source.as_bytes()[index];
        if state.top_level() && byte == b'#' {
            break;
        }
        if state.top_level() && byte == b'{' {
            delta += 1;
        } else if state.top_level() && byte == b'}' {
            delta -= 1;
        }
        index = advance(source, index, &mut state);
    }

    delta
}

pub(crate) fn triple_quote_toggles(source: &str) -> usize {
    let mut count = 0usize;
    let mut index = 0usize;
    while let Some(offset) = source[index..].find("\"\"\"") {
        count += 1;
        index += offset + 3;
    }
    count
}

fn advance_triple(source: &str, index: usize, state: &mut ScanState) -> usize {
    if starts_triple_quote(source, index) {
        state.triple = false;
        return index + 3;
    }
    index
        + source[index..]
            .chars()
            .next()
            .expect("scanner index must be in bounds")
            .len_utf8()
}

pub fn opens_shell_capture(line: &str) -> bool {
    let head = line.trim_end().strip_suffix("$(");
    head.is_some_and(|head| {
        !head.trim_start().starts_with(['#', '$'])
            && (head.trim().is_empty()
                || head.ends_with(char::is_whitespace)
                || head.ends_with('='))
    })
}

pub fn closes_shell_capture(line: &str, indent: &str) -> bool {
    line.strip_prefix(indent)
        .is_some_and(|rest| matches!(rest.trim_end(), ")" | ")?" | ");" | ")?;"))
}
