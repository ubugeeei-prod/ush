pub fn format_source(source: &str) -> String {
    let mut out = String::new();
    let mut indent = 0usize;
    let mut last_blank = false;
    let mut in_multiline = false;

    for raw in source.lines() {
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
        out.push_str(&"  ".repeat(indent));
        out.push_str(&normalize_line(line));
        out.push('\n');
        // The leading `}` was already paid for by `dedent`; counting
        // it again would cancel the brace that reopens on the same
        // line, and `} with log { (m) =>` would indent nothing.
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
    if let Some((left, right)) = line.split_once("=>") {
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
    if let Some((left, right)) = line.split_once("->") {
        return format!("{} -> {}", left.trim_end(), right.trim_start());
    }
    line.to_string()
}

fn assignment_parts(line: &str) -> Option<(String, String)> {
    let (left, right) = line.split_once('=')?;
    let head = left.trim_start();
    if !matches!(head.split_whitespace().next(), Some("let" | "alias")) {
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
