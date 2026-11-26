/// How deeply `${…}` interpolations may nest inside one another.
pub(crate) const MAX_HOLE_DEPTH: usize = 16;

#[derive(Clone, Copy, Default)]
pub(crate) struct ScanState {
    single: bool,
    double: bool,
    /// The double-quoted string being scanned is a `$"…"` that interpolates.
    interp: bool,
    triple: bool,
    /// Inside a `$"""…"""` template.
    template: bool,
    escaped: bool,
    pub(crate) paren: usize,
    pub(crate) brace: usize,
    pub(crate) bracket: usize,
    /// Open `${…}` holes, innermost last: the string each returns to
    /// (`true` for a template) and the brace depth that closes it.
    holes: [(bool, u16); MAX_HOLE_DEPTH],
    depth: u8,
}

impl ScanState {
    pub(crate) fn top_level(&self) -> bool {
        !self.in_string() && self.paren == 0 && self.brace == 0 && self.bracket == 0
    }

    /// Inside a string literal, including the code of its `${…}` holes:
    /// a string and its interpolations are one token to every outer scan.
    pub(crate) fn in_string(&self) -> bool {
        self.single || self.double || self.triple || self.template || self.depth > 0
    }
}

pub(crate) fn starts_triple_quote(source: &str, index: usize) -> bool {
    source.as_bytes().get(index..index + 3) == Some(br#"""""#)
}

/// `$"`, which opens an interpolating string, but not `$"""`.
pub(crate) fn starts_interpolated(source: &str, index: usize) -> bool {
    source.as_bytes().get(index..index + 2) == Some(br#"$""#) && !starts_template(source, index)
}

/// `$"""`, which opens an interpolating multi-line template.
pub(crate) fn starts_template(source: &str, index: usize) -> bool {
    source.as_bytes().get(index..index + 4) == Some(br#"$""""#)
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
    if state.template && starts_triple_quote(source, index) {
        state.template = false;
        state.escaped = false;
        return index + 3;
    }
    if state.double || state.template {
        if state.escaped {
            state.escaped = false;
        } else if bytes[index] == b'\\' {
            state.escaped = true;
        } else if ch == '"' && state.double {
            state.double = false;
            state.interp = false;
        } else if (state.template || state.interp)
            && bytes.get(index..index + 2) == Some(b"${")
            && open_hole(state)
        {
            return index + 2;
        }
        return index + ch.len_utf8();
    }
    if starts_template(source, index) {
        state.template = true;
        return index + 4;
    }
    if starts_interpolated(source, index) {
        state.double = true;
        state.interp = true;
        return index + 2;
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
        '}' if close_hole(state) => {}
        '}' if state.brace > 0 => state.brace -= 1,
        '[' => state.bracket += 1,
        ']' if state.bracket > 0 => state.bracket -= 1,
        _ => {}
    }
    index + ch.len_utf8()
}

/// Enter a `${` hole from the string being scanned. Past the nesting
/// limit the `${` stays text, and the string's own parser reports it.
fn open_hole(state: &mut ScanState) -> bool {
    let depth = usize::from(state.depth);
    if depth == MAX_HOLE_DEPTH {
        return false;
    }
    state.brace += 1;
    state.holes[depth] = (state.template, state.brace as u16);
    state.depth += 1;
    state.double = false;
    state.interp = false;
    state.template = false;
    true
}

/// Leave the innermost hole when this `}` matches its `${`.
fn close_hole(state: &mut ScanState) -> bool {
    let Some(depth) = usize::from(state.depth).checked_sub(1) else {
        return false;
    };
    let (template, brace) = state.holes[depth];
    if usize::from(brace) != state.brace {
        return false;
    }
    state.brace -= 1;
    state.depth -= 1;
    state.template = template;
    state.double = !template;
    state.interp = !template;
    true
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

/// Split at top-level `|` and `|&`, reporting whether each piece was
/// followed by `|&`. `||` is never a pipe, and a `|` inside a string,
/// parentheses, brackets or braces belongs to that inner text.
pub(crate) fn split_pipeline(source: &str) -> crate::types::HeapVec<(&str, bool)> {
    let bytes = source.as_bytes();
    let mut pieces = crate::types::HeapVec::new();
    let mut state = ScanState::default();
    let (mut start, mut index) = (0usize, 0usize);
    while index < source.len() {
        if state.top_level() && bytes[index] == b'|' {
            if bytes.get(index + 1) == Some(&b'|') {
                index += 2;
                continue;
            }
            let merge = bytes.get(index + 1) == Some(&b'&');
            pieces.push((&source[start..index], merge));
            index += 1 + usize::from(merge);
            start = index;
            continue;
        }
        index = advance(source, index, &mut state);
    }
    pieces.push((&source[start..], false));
    pieces
}
