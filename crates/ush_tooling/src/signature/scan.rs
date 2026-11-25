use super::FunctionSignature;
pub(super) fn offset(source: &str, line: u32, byte: u32) -> usize {
    let start = source
        .split_inclusive('\n')
        .take(line as usize)
        .map(str::len)
        .sum::<usize>();
    (start + byte as usize).min(source.len())
}
pub(super) fn closing(tokens: &[&str], open: usize) -> Option<usize> {
    let mut depth = 0;
    for (i, t) in tokens.iter().enumerate().skip(open) {
        if *t == "(" {
            depth += 1;
        } else if *t == ")" {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}
pub(super) fn parameters(raw: &str) -> Vec<String> {
    let mut depth = 0;
    let mut start = 0;
    let mut out = Vec::new();
    for (i, c) in raw.char_indices() {
        match c {
            '<' | '(' | '[' | '{' => depth += 1,
            '>' | ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => {
                out.push(raw[start..i].trim().into());
                start = i + 1;
            }
            _ => {}
        }
    }
    if !raw[start..].trim().is_empty() {
        out.push(raw[start..].trim().into());
    }
    out
}
pub(super) fn call(prefix: &str, signatures: &[FunctionSignature]) -> Option<(String, u32)> {
    let tokens = crate::semantic::semantic_tokens(prefix);
    let lines: Vec<_> = prefix.lines().collect();
    let text = |t: &crate::SemanticToken| {
        &lines[t.line as usize][t.start as usize..(t.start + t.length) as usize]
    };
    let mut candidates = Vec::new();
    let mut depth = 0i32;
    for (i, t) in tokens.iter().enumerate() {
        let word = text(t);
        if matches!(word, "(" | "[" | "{") {
            depth += 1;
        }
        if matches!(word, ")" | "]" | "}") {
            depth -= 1;
            candidates.retain(|(_, d, _, _)| *d <= depth);
        }
        if signatures.iter().any(|s| s.name == word) && (i == 0 || text(&tokens[i - 1]) != "fn") {
            let adjacent = tokens.get(i + 1).is_some_and(|next| {
                text(next) == "(" && t.line == next.line && t.start + t.length == next.start
            });
            candidates.push((i, depth, adjacent, t.line));
        }
    }
    let &(i, call_depth, adjacent, line) = candidates.iter().rev().find(|(_, d, adj, l)| {
        *adj || (*d <= depth && *l == tokens.last().map_or(0, |t| t.line))
    })?;
    let rest = &tokens[i + 1..];
    let mut nested = 0i32;
    let mut arguments = 0u32;
    let mut active = false;
    let mut last_end = 0;
    for t in rest {
        let word = text(t);
        if adjacent {
            if word == "," && nested == 1 {
                arguments += 1;
            }
        } else if nested == 0 && (last_end == 0 || t.start > last_end) && word != "&" {
            arguments += 1;
            active = true;
        }
        match word {
            "(" | "[" | "{" => nested += 1,
            ")" | "]" | "}" => nested -= 1,
            _ => {}
        }
        last_end = t.start + t.length;
    }
    if !adjacent && active && !prefix.ends_with(char::is_whitespace) {
        arguments = arguments.saturating_sub(1);
    }
    let _ = (call_depth, line);
    Some((text(&tokens[i]).into(), arguments))
}
