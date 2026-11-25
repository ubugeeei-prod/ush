pub fn format_source(source: &str) -> String {
    let mut out = String::new();
    let mut indent = 0usize;
    let mut last_blank = false;
    let mut in_multiline = false;

    let mut shell_indent = None;
    let mut continued = false;
    let mut after_use = false;
    for raw in source.lines() {
        if let Some(indent) = shell_indent {
            out.push_str(raw);
            out.push('\n');
            if ush_compiler::closes_shell_capture(raw, indent) {
                shell_indent = None;
            }
            continue;
        }
        let is_use = raw.trim_start().starts_with("use ");
        if !in_multiline && !raw.trim().is_empty() {
            if after_use && !is_use && !last_blank {
                out.push('\n');
            }
            after_use = is_use;
        }
        if !in_multiline && ush_compiler::opens_shell_capture(raw) {
            shell_indent = Some(&raw[..raw.len() - raw.trim_start().len()]);
            out.push_str(raw);
            out.push('\n');
            last_blank = false;
            continue;
        }
        if in_multiline || triple_quote_toggles(raw) > 0 {
            out.push_str(raw);
            out.push('\n');
            in_multiline ^= triple_quote_toggles(raw) % 2 == 1;
            last_blank = false;
            continue;
        }
        let line = raw.trim();
        if line.is_empty() {
            if !last_blank && !out.is_empty() {
                out.push('\n');
            }
            last_blank = true;
            continue;
        }

        let dedent = usize::from(line.starts_with('}'));
        indent = indent.saturating_sub(dedent);
        // The stages after a trailing `|` sit one level deeper.
        out.push_str(&"  ".repeat(indent + usize::from(continued)));
        out.push_str(&normalize_line(line));
        if !line.starts_with('#') {
            continued = ush_compiler::continues_pipeline(line);
        }
        out.push('\n');
        // The leading `}` was already paid for by `dedent`; counting
        // it again would cancel the brace that reopens on the same
        // line, and `} with log(m) {` would indent nothing.
        indent += brace_delta(line.strip_prefix('}').unwrap_or(line));
        last_blank = false;
    }

    out
}

fn triple_quote_toggles(line: &str) -> usize {
    let mut count = 0usize;
    let mut index = 0usize;
    while let Some(offset) = line[index..].find("\"\"\"") {
        count += 1;
        index += offset + 3;
    }
    count
}

fn normalize_line(line: &str) -> String {
    if let Some((left, right)) = crate::call_words::separator(line, "=>") {
        let right = right.trim();
        if right.is_empty() {
            // A handler arm opens its body on the next line, so the
            // arrow ends the line rather than separating two halves.
            return format!("{} =>", left.trim());
        }
        return format!("{} => {}", left.trim(), right);
    }
    if let Some((left, right)) = assignment_parts(line) {
        return format!("{left} = {right}");
    }
    if let Some((left, right)) = crate::call_words::separator(line, "->") {
        return format!("{} -> {}", left.trim_end(), right.trim_start());
    }
    line.to_string()
}

fn assignment_parts(line: &str) -> Option<(String, String)> {
    let (left, right) = line.split_once('=')?;
    if right.starts_with('=') {
        return None;
    }
    let words: Vec<_> = left.split_whitespace().collect();
    let name = match words.as_slice() {
        [name] => *name,
        ["let" | "alias" | "export", name] | ["let", "mut", name] => *name,
        _ => return None,
    };
    if !name.starts_with(|ch: char| ch == '_' || ch.is_ascii_alphabetic())
        || !name
            .chars()
            .all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
    {
        return None;
    }
    Some((left.trim().to_string(), right.trim().to_string()))
}

fn brace_delta(line: &str) -> usize {
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

#[cfg(test)]
mod tests;
