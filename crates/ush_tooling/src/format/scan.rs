//! Line scanning the formatter needs: braces outside strings and comments, and the
//! triple quotes that open or close a multiline string.

pub(super) fn ends_with_brace(out: &str) -> bool {
    out.strip_suffix('\n')
        .map(|text| text.rsplit('\n').next().unwrap_or(text))
        .is_some_and(|last| last.trim() == "}")
}

pub(super) fn triple_quote_toggles(line: &str) -> usize {
    let mut count = 0usize;
    let mut index = 0usize;
    while let Some(offset) = line[index..].find("\"\"\"") {
        count += 1;
        index += offset + 3;
    }
    count
}

pub(super) fn brace_delta(line: &str) -> usize {
    let mut single = false;
    let mut double = false;
    // The double-quoted string is a `$"…"`, whose `${…}` holes are code.
    let mut interpolating = false;
    let mut previous = ' ';
    // One entry per open `${…}` hole: braces opened inside it so far.
    let mut holes: Vec<usize> = Vec::new();
    let mut delta = 0isize;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        let before = core::mem::replace(&mut previous, ch);
        if single {
            single = ch != '\'';
            continue;
        }
        if double {
            match ch {
                '\\' => {
                    let _ = chars.next();
                }
                '"' => double = false,
                '$' if interpolating && chars.peek() == Some(&'{') => {
                    chars.next();
                    double = false;
                    holes.push(0);
                }
                _ => {}
            }
            continue;
        }
        match ch {
            '#' if holes.is_empty() => {
                // `#[…]` is an attribute, not a comment. Skip past
                // its matching `]` (handling nested brackets) so
                // braces inside the attribute do not throw off the
                // outer indent counter, and so the rest of the line
                // is still inspected.
                if chars.peek() == Some(&'[') {
                    chars.next();
                    let mut depth = 1isize;
                    for inner in chars.by_ref() {
                        match inner {
                            '[' => depth += 1,
                            ']' => {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                } else {
                    break;
                }
            }
            '\'' => single = true,
            '"' => {
                double = true;
                interpolating = before == '$';
            }
            '{' => match holes.last_mut() {
                Some(open) => *open += 1,
                None => delta += 1,
            },
            '}' => match holes.last_mut() {
                Some(0) => {
                    holes.pop();
                    double = true;
                    interpolating = true;
                }
                Some(open) => *open -= 1,
                None => delta -= 1,
            },
            _ => {}
        }
    }

    delta.max(0) as usize
}
