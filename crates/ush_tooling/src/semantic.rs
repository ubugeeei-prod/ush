mod scan;
#[cfg(test)]
mod tests;

use self::KeywordContext::{Decl, Func, Let, NoKeyword, Shell};
use self::scan::{
    is_ident, is_ident_start, is_shell_escape, next_non_space, operator_end, string_end,
    take_while, triple_string_span, variable_end,
};
use crate::token::{SemanticToken, SemanticTokenKind};

const KEYWORDS: &[&str] = &[
    "alias", "async", "do", "effect", "enum", "fn", "impl", "let", "match", "print", "raise",
    "return", "shell", "trait", "try", "type", "use", "with",
];

#[derive(Clone, Copy)]
enum KeywordContext {
    NoKeyword,
    Decl,
    Func,
    Let,
    Shell,
}

pub fn semantic_tokens(source: &str) -> Vec<SemanticToken> {
    let mut out = Vec::new();
    let mut in_multiline = false;
    for (line_no, line) in source.lines().enumerate() {
        in_multiline = tokenize_line(line_no as u32, line, &mut out, in_multiline);
    }
    out
}

fn tokenize_line(
    line_no: u32,
    line: &str,
    out: &mut Vec<SemanticToken>,
    mut in_multiline: bool,
) -> bool {
    let bytes = line.as_bytes();
    let mut index = 0usize;
    let mut context = NoKeyword;

    while index < bytes.len() {
        if in_multiline {
            let (end, closed) = triple_string_span(line, index);
            push(out, line_no, index, end - index, SemanticTokenKind::String);
            index = end;
            in_multiline = !closed;
            continue;
        }
        let ch = bytes[index] as char;
        if ch.is_ascii_whitespace() {
            index += 1;
            continue;
        }
        if line[index..].starts_with("#[") {
            let end = line[index..]
                .find(']')
                .map_or(line.len(), |offset| index + offset + 1);
            push(
                out,
                line_no,
                index,
                end - index,
                SemanticTokenKind::Decorator,
            );
            break;
        }
        if ch == '#' {
            push(
                out,
                line_no,
                index,
                line.len() - index,
                SemanticTokenKind::Comment,
            );
            break;
        }
        if line[index..].starts_with("\"\"\"") {
            let (end, closed) = triple_string_span(line, index + 3);
            push(out, line_no, index, end - index, SemanticTokenKind::String);
            index = end;
            in_multiline = !closed;
            context = NoKeyword;
            continue;
        }
        if matches!(ch, '"' | '\'') {
            let end = string_end(line, index);
            push(out, line_no, index, end - index, SemanticTokenKind::String);
            index = end;
            context = NoKeyword;
            continue;
        }
        if ch.is_ascii_digit() {
            let end = take_while(line, index, |value| value.is_ascii_digit());
            push(out, line_no, index, end - index, SemanticTokenKind::Number);
            index = end;
            context = NoKeyword;
            continue;
        }
        if ch == '$' {
            if is_shell_escape(line, index) {
                push(out, line_no, index, 1, SemanticTokenKind::Operator);
                index += 1;
                context = Shell;
                continue;
            }
            let end = variable_end(line, index);
            push(
                out,
                line_no,
                index,
                end - index,
                SemanticTokenKind::Variable,
            );
            index = end;
            context = NoKeyword;
            continue;
        }
        if is_ident_start(ch) {
            let end = take_while(line, index, is_ident);
            let ident = &line[index..end];
            let kind = classify_ident(ident, context, next_non_space(line, end));
            push(out, line_no, index, end - index, kind);
            context = match ident {
                "fn" => Func,
                "let" => Let,
                "enum" | "type" | "trait" | "impl" => Decl,
                _ => NoKeyword,
            };
            index = end;
            continue;
        }
        let end = operator_end(line, index);
        push(
            out,
            line_no,
            index,
            end - index,
            SemanticTokenKind::Operator,
        );
        index = end;
        context = NoKeyword;
    }
    in_multiline
}

fn classify_ident(ident: &str, context: KeywordContext, next: Option<char>) -> SemanticTokenKind {
    if KEYWORDS.contains(&ident) {
        return SemanticTokenKind::Keyword;
    }
    if matches!(context, Func | Shell) || next == Some('(') {
        return SemanticTokenKind::Function;
    }
    if matches!(context, Decl) || ident.chars().next().is_some_and(char::is_uppercase) {
        return SemanticTokenKind::Type;
    }
    if next == Some(':') {
        return SemanticTokenKind::Property;
    }
    if matches!(context, Let) {
        return SemanticTokenKind::Variable;
    }
    SemanticTokenKind::Variable
}

fn push(
    out: &mut Vec<SemanticToken>,
    line: u32,
    start: usize,
    len: usize,
    kind: SemanticTokenKind,
) {
    out.push(SemanticToken {
        line,
        start: start as u32,
        length: len as u32,
        kind,
    });
}
