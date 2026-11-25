//! Mark unquoted call words as strings before building the navigation index.
mod scan;
use crate::{SemanticToken, SemanticTokenKind as Kind};
use scan::split;

pub(super) fn classify(source: &str, tokens: &mut Vec<SemanticToken>) {
    for (line_no, line) in source.lines().enumerate() {
        let end = tokens
            .iter()
            .find(|t| t.line == line_no as u32 && t.kind == Kind::Comment)
            .map_or(line.len(), |t| t.start as usize);
        let mut ranges = Vec::new();
        visit(&line[..end], line.as_ptr() as usize, &mut ranges);
        for (start, end) in ranges {
            // Multiline strings and shell source are already one opaque token.
            if tokens.iter().any(|t| {
                t.line == line_no as u32
                    && t.kind == Kind::String
                    && t.start as usize <= start
                    && (t.start + t.length) as usize >= end
            }) {
                continue;
            }
            tokens.retain(|t| {
                t.line != line_no as u32 || (t.start as usize) < start || t.start as usize >= end
            });
            tokens.push(SemanticToken {
                line: line_no as u32,
                start: start as u32,
                length: (end - start) as u32,
                kind: Kind::String,
            });
        }
    }
    tokens.sort_by_key(|t| (t.line, t.start));
}

fn visit(text: &str, base: usize, ranges: &mut Vec<(usize, usize)>) {
    let text = text.trim().trim_end_matches([';', ',']).trim();
    if text.is_empty() || text.starts_with('#') || text.starts_with("$ ") {
        return;
    }
    let words = split(text, None);
    let Some(&head) = words.first() else {
        return;
    };
    if matches!(
        head,
        "fn" | "pub"
            | "use"
            | "using"
            | "extern"
            | "mod"
            | "enum"
            | "type"
            | "trait"
            | "impl"
            | "effect"
            | "unit"
            | "shell"
    ) {
        return;
    }
    if let Some((left, tail)) = text.split_once("=>")
        && (split(left, None).len() <= 1
            || left.trim_start().starts_with(['"', '\''])
            || left.contains(" {"))
    {
        visit(tail, base, ranges);
        return;
    }
    if let Some(at) = words.iter().position(|w| *w == "=") {
        if let Some(value) = words.get(at + 1) {
            visit(&text[offset(text, value)..], base, ranges);
        }
        return;
    }
    if matches!(head, "let" | "const" | "alias" | "export") {
        if let Some((_, value)) = text.split_once('=') {
            visit(value, base, ranges);
        }
        return;
    }
    if head == "for" {
        if let Some(at) = words.iter().position(|w| *w == "in")
            && let Some(value) = words.get(at + 1)
        {
            visit(
                text[offset(text, value)..].trim_end_matches('{'),
                base,
                ranges,
            );
        }
        return;
    }
    if matches!(
        head,
        "return"
            | "print"
            | "raise"
            | "if"
            | "while"
            | "match"
            | "async"
            | "do"
            | "spawn"
            | "defer"
            | "allow_fail"
            | "status_of"
    ) {
        visit(text[head.len()..].trim_end_matches('{'), base, ranges);
        return;
    }
    let pipe = split(text, Some('|'));
    if pipe.len() > 1 {
        for part in pipe {
            visit(part.trim_start_matches('&'), base, ranges);
        }
        return;
    }
    if words.iter().any(|w| {
        matches!(
            *w,
            "+" | "==" | "!=" | "<" | ">" | "<=" | ">=" | "->" | "::" | "&&" | "||"
        )
    }) {
        let mut start = 0;
        for word in &words {
            if matches!(
                *word,
                "+" | "==" | "!=" | "<" | ">" | "<=" | ">=" | "->" | "::" | "&&" | "||"
            ) {
                let end = offset(text, word);
                visit(&text[start..end], base, ranges);
                start = end + word.len();
            }
        }
        visit(&text[start..], base, ranges);
        return;
    }
    if words.len() > 1 && target(head) && !words[1].starts_with('{') {
        for word in &words[1..] {
            if word.ends_with(':') {
                continue;
            }
            if explicit(word) {
                visit(word, base, ranges);
            } else {
                literal_parts(word, base, ranges);
            }
        }
        return;
    }
    let text = text.trim_end_matches('?');
    if (text.starts_with('(') && text.ends_with(')'))
        || (text.starts_with('[') && text.ends_with(']'))
    {
        let inner = &text[1..text.len() - 1];
        if let Some((value, clauses)) = inner.split_once(" for ") {
            visit(value, base, ranges);
            for clause in clauses.split(" for ") {
                if let Some((_, expr)) = clause.split_once(" in ") {
                    visit(expr.split(" if ").next().unwrap_or(expr), base, ranges);
                }
            }
        } else {
            for item in split(inner, Some(',')) {
                visit(item, base, ranges);
            }
        }
    } else if let Some(open) = text.find('(')
        && text.ends_with(')')
    {
        for item in split(&text[open + 1..text.len() - 1], Some(',')) {
            let value = item
                .split_once(':')
                .filter(|(_, v)| !v.starts_with(':'))
                .map_or(item, |(_, v)| v);
            visit(value, base, ranges);
        }
    }
}

fn explicit(word: &str) -> bool {
    word.starts_with(['(', '[', '"', '\'', '&'])
        || word.contains('(')
        || word.parse::<i64>().is_ok()
        || matches!(word, "true" | "false")
        || word.chars().next().is_some_and(|c| c.is_ascii_digit())
}
fn target(word: &str) -> bool {
    let word = word.strip_prefix("$bin::").unwrap_or(word);
    word.chars()
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && word
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | ':'))
}
fn literal_parts(word: &str, base: usize, ranges: &mut Vec<(usize, usize)>) {
    let mut rest = word;
    while let Some(at) = rest.find('$') {
        mark(&rest[..at], base, ranges);
        let tail = &rest[at + 1..];
        let end = if tail.starts_with('{') {
            tail.find('}').map_or(tail.len(), |i| i + 1)
        } else {
            tail.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .unwrap_or(tail.len())
        };
        rest = &tail[end..];
    }
    mark(rest, base, ranges);
}
fn mark(text: &str, base: usize, ranges: &mut Vec<(usize, usize)>) {
    if !text.is_empty() {
        let start = text.as_ptr() as usize - base;
        ranges.push((start, start + text.len()));
    }
}
fn offset(whole: &str, part: &str) -> usize {
    part.as_ptr() as usize - whole.as_ptr() as usize
}

pub(super) fn separator<'a>(line: &'a str, separator: &str) -> Option<(&'a str, &'a str)> {
    if !line.contains(separator) {
        return None;
    }
    crate::semantic_tokens(line).into_iter().find_map(|token| {
        let start = token.start as usize;
        let end = start + token.length as usize;
        (token.kind == crate::SemanticTokenKind::Operator && &line[start..end] == separator)
            .then(|| (&line[..start], &line[end..]))
    })
}
