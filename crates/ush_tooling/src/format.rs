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
    let mut delta = 0isize;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '#' if !single && !double => {
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
            '\'' if !double => single = !single,
            '"' if !single => double = !double,
            '\\' if double => {
                let _ = chars.next();
            }
            '{' if !single && !double => delta += 1,
            '}' if !single && !double => delta -= 1,
            _ => {}
        }
    }

    delta.max(0) as usize
}

#[cfg(test)]
mod tests {
    use super::format_source;

    #[test]
    fn formats_indentation_and_spacing() {
        let formatted = format_source(
            "fn greet(name: String)->String {\nprint name\nmatch name {\n\"x\"=>print \"ok\"\n}\n}\n",
        );

        assert_eq!(
            formatted,
            concat!(
                "fn greet(name: String) -> String {\n",
                "  print name\n",
                "  match name {\n",
                "    \"x\" => print \"ok\"\n",
                "  }\n",
                "}\n"
            )
        );
    }

    #[test]
    fn collapses_repeated_blank_lines() {
        let formatted = format_source("print \"a\"\n\n\nprint \"b\"\n");
        assert_eq!(formatted, "print \"a\"\n\nprint \"b\"\n");
    }

    #[test]
    fn preserves_multiline_string_blocks() {
        let formatted = format_source("let page = \"\"\"\n  <div>\n    ok\n  </div>\n\"\"\"\n");
        assert_eq!(
            formatted,
            "let page = \"\"\"\n  <div>\n    ok\n  </div>\n\"\"\"\n"
        );
    }

    #[test]
    fn indents_pipeline_continuations_and_export_bindings() {
        let formatted = format_source(
            "cd \"x\" {\nexport A=\"1\" {\ngit \"log\" |\n# first only\nhead \"-n1\"\n}\n}\n",
        );
        assert_eq!(
            formatted,
            concat!(
                "cd \"x\" {\n",
                "  export A = \"1\" {\n",
                "    git \"log\" |\n",
                "      # first only\n",
                "      head \"-n1\"\n",
                "  }\n",
                "}\n",
            ),
        );
    }

    #[test]
    fn leaves_the_export_command_and_its_strings_alone() {
        assert_eq!(format_source("export \"A=b\"\n"), "export \"A=b\"\n");
    }

    #[test]
    fn attributes_do_not_swallow_following_braces() {
        // `#[default(2)]` used to be parsed as a comment, which made
        // `brace_delta` return 0 for the `fn` line — the body never
        // got indented.
        let formatted = format_source("fn bin(#[default(2)] count: Int) {\nprint count\n}\n");
        assert_eq!(
            formatted,
            concat!(
                "fn bin(#[default(2)] count: Int) {\n",
                "  print count\n",
                "}\n",
            ),
        );
    }
}
