mod scan;

use scan::{brace_delta, ends_with_brace, triple_quote_toggles};

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

        // An `else` right after its `}` shares the brace's line: `} else {`.
        if (line == "else {" || line.starts_with("else if "))
            && !last_blank
            && ends_with_brace(&out)
        {
            out.pop();
            out.push(' ');
            out.push_str(&normalize_line(line));
            out.push('\n');
            indent += brace_delta(line);
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

#[cfg(test)]
mod tests;
