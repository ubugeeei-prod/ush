#[cfg(test)]
mod interpolation_tests;
mod scan;
mod strings;
#[cfg(test)]
mod tests;

use self::KeywordContext::{Decl, Func, Let, NoKeyword, Shell};
use self::scan::{
    is_ident, is_ident_start, is_shell_escape, next_non_space, operator_end, take_while,
    variable_end,
};
use self::strings::{Multiline, tokenize_line};
use crate::token::{SemanticToken, SemanticTokenKind};

const KEYWORDS: &[&str] = &[
    "alias",
    "async",
    "const",
    "do",
    "export",
    "fail",
    "using",
    "unit",
    "where",
    "effect",
    "enum",
    "echo",
    "extern",
    "test",
    "fn",
    "impl",
    "let",
    "mut",
    "match",
    "print",
    "raise",
    "return",
    "shell",
    "trait",
    "try",
    "type",
    "use",
    "with",
    "as",
    "in",
    "for",
    "if",
    "else",
    "while",
    "loop",
    "break",
    "continue",
    "true",
    "false",
    "await",
    "spawn",
    "mod",
    "pub",
    "defer",
    "status_of",
    "allow_fail",
    "glob",
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
    let mut multiline = Multiline::No;
    for (line_no, line) in source.lines().enumerate() {
        multiline = tokenize_line(line_no as u32, line, &mut out, multiline);
    }
    crate::call_words::classify(source, &mut out);
    let lines: Vec<_> = source.lines().collect();
    let functions: std::collections::HashSet<_> = out
        .windows(2)
        .filter(|pair| text(&lines, pair[0]) == "fn")
        .map(|pair| text(&lines, pair[1]))
        .collect();
    for token in &mut out {
        if token.kind == SemanticTokenKind::Variable && functions.contains(text(&lines, *token)) {
            token.kind = SemanticTokenKind::Function;
        }
    }
    out
}

fn text<'a>(lines: &[&'a str], token: SemanticToken) -> &'a str {
    lines[token.line as usize]
        .get(token.start as usize..(token.start + token.length) as usize)
        .unwrap_or("")
}

/// Tokenize `line[index..end]`, returning the string left open at the end.
fn tokenize_span(
    line_no: u32,
    line: &str,
    mut index: usize,
    stop: usize,
    out: &mut Vec<SemanticToken>,
) -> Multiline {
    let mut context = NoKeyword;

    while index < stop {
        let ch = line[index..].chars().next().expect("in bounds");
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
                stop - index,
                SemanticTokenKind::Comment,
            );
            break;
        }
        if let Some((next, open)) = strings::string(line_no, line, index, out) {
            if open != Multiline::No {
                return open;
            }
            index = next;
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
                "let" | "mut" => Let,
                "enum" | "type" | "trait" | "impl" => Decl,
                _ => NoKeyword,
            };
            index = end;
            continue;
        }
        let next = operator_end(line, index);
        push(
            out,
            line_no,
            index,
            next - index,
            SemanticTokenKind::Operator,
        );
        index = next;
        context = NoKeyword;
    }
    Multiline::No
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
