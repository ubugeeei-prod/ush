pub(super) fn split(text: &str, separator: Option<char>) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut depth = 0usize;
    let mut quote = None;
    let mut escaped = false;
    for (i, c) in text.char_indices() {
        if let Some(q) = quote {
            if escaped {
                escaped = false;
            } else if c == '\\' && q == '"' {
                escaped = true;
            } else if c == q {
                quote = None;
            }
            continue;
        }
        if matches!(c, '"' | '\'') {
            quote = Some(c);
            continue;
        }
        if matches!(c, '(' | '[' | '{') {
            depth += 1;
        }
        if matches!(c, ')' | ']' | '}') {
            depth = depth.saturating_sub(1);
        }
        if depth == 0 && separator.map_or(c.is_whitespace(), |s| c == s) {
            let part = text[start..i].trim();
            if !part.is_empty() {
                parts.push(part);
            }
            start = i + c.len_utf8();
        }
    }
    let part = text[start..].trim();
    if !part.is_empty() {
        parts.push(part);
    }
    parts
}
