//! LSP `textDocument/completion` provider.
//!
//! Returns a context-light, keyword-and-identifier completion list.
//! The provider does not look at type information — `ush_compiler`'s
//! AST is private and a full HM-style completion is a large
//! feature. What this gives editors today:
//!
//! - every `.ush` keyword is offered as a completion item.
//! - every identifier the semantic tokenizer has classified as a
//!   variable / function / type in the open document is offered
//!   too, so users get completion for things they themselves have
//!   written.

use crate::semantic::semantic_tokens;
use crate::token::{SemanticToken, SemanticTokenKind};

/// One completion suggestion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    /// Short single-line note shown in the completion popup (for
    /// keywords; empty for identifiers).
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionKind {
    Keyword,
    Variable,
    Function,
    Type,
}

const KEYWORDS: &[(&str, &str)] = &[
    ("in", "iterate over a collection"),
    ("as", "name an extern module"),
    ("try", "propagate a fallible result"),
    ("do", "invoke an effect operation"),
    ("effect", "declare an effect operation"),
    ("with", "handle an effect"),
    ("alias", "alias name = …"),
    ("async", "spawn an asynchronous task"),
    ("await", "await an async task"),
    ("break", "exit the current loop"),
    ("continue", "skip to the next iteration"),
    ("else", "else branch"),
    ("enum", "declare an enum"),
    ("extern", "import typed functions from a .sh library"),
    ("test", "declare an isolated test block"),
    ("false", "boolean false"),
    ("fn", "declare a function"),
    ("for", "for-in loop"),
    ("if", "if expression"),
    ("impl", "impl block"),
    ("let", "binding declaration"),
    ("loop", "infinite loop"),
    ("match", "pattern match"),
    ("print", "print a value"),
    (
        "fail",
        "Return a typed error or an exit status and message.",
    ),
    (
        "using",
        "Bring the typed shell commands in $bin:: into scope.",
    ),
    ("unit", "Declare a measurement unit for integer values."),
    ("where", "Require an integer predicate at the call site."),
    ("const", "Infer an integer index from the argument type."),
    ("raise", "raise a typed error"),
    ("return", "return from a function"),
    ("shell", "execute a shell command"),
    ("spawn", "spawn an async task"),
    ("trait", "declare a trait"),
    ("true", "boolean true"),
    ("type", "type alias"),
    ("use", "import std module"),
    ("while", "while loop"),
];

/// Produce a completion list for a document. The cursor position is
/// not currently consulted — keywords + every identifier ever
/// declared in the file is returned and the editor filters.
pub fn completions(source: &str) -> Vec<CompletionItem> {
    let mut out: Vec<CompletionItem> = KEYWORDS
        .iter()
        .map(|(label, detail)| CompletionItem {
            label: (*label).to_string(),
            kind: CompletionKind::Keyword,
            detail: (*detail).to_string(),
        })
        .collect();

    let imported = source
        .lines()
        .any(|line| matches!(line.trim(), "use $bin::*" | "using $bin::"));
    if imported || source.contains("$bin::") {
        for word in ush_compiler::command_names() {
            out.push(CompletionItem {
                label: if imported {
                    word.to_owned()
                } else {
                    format!("$bin::{word}")
                },
                kind: CompletionKind::Function,
                detail: "Typed shell command".into(),
            });
        }
    }
    let tokens = semantic_tokens(source);
    let lines: Vec<&str> = source.lines().collect();
    let mut seen: Vec<String> = Vec::new();

    for token in &tokens {
        let kind = match token.kind {
            SemanticTokenKind::Variable => CompletionKind::Variable,
            SemanticTokenKind::Function => CompletionKind::Function,
            SemanticTokenKind::Type => CompletionKind::Type,
            _ => continue,
        };
        let text = identifier_text(&lines, *token);
        if text.is_empty() {
            continue;
        }
        if KEYWORDS.iter().any(|(k, _)| *k == text) {
            continue;
        }
        if seen.iter().any(|prev| prev == &text) {
            continue;
        }
        seen.push(text.clone());
        out.push(CompletionItem {
            label: text,
            kind,
            detail: String::new(),
        });
    }
    out
}

fn identifier_text(lines: &[&str], token: SemanticToken) -> String {
    lines
        .get(token.line as usize)
        .and_then(|line| {
            let start = token.start as usize;
            let end = start + token.length as usize;
            line.get(start..end).map(str::to_string)
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::{CompletionKind, completions};

    #[test]
    fn keyword_list_is_offered_for_an_empty_document() {
        let items = completions("");
        let labels: Vec<&str> = items.iter().map(|c| c.label.as_str()).collect();
        for keyword in ["fn", "let", "match", "use", "enum"] {
            assert!(labels.contains(&keyword), "missing keyword: {keyword}");
        }
    }

    #[test]
    fn user_declared_identifiers_appear() {
        let source = "let greeting = \"hi\"\nfn shout(message: String) {}\n";
        let items = completions(source);
        let identifier_labels: Vec<&str> = items
            .iter()
            .filter(|c| c.kind != CompletionKind::Keyword)
            .map(|c| c.label.as_str())
            .collect();
        assert!(
            identifier_labels.contains(&"greeting"),
            "{identifier_labels:?}"
        );
        assert!(
            identifier_labels.contains(&"shout"),
            "{identifier_labels:?}"
        );
    }

    #[test]
    fn duplicates_are_dropped() {
        let source = "let x = 1\nprint x\nprint x\n";
        let items = completions(source);
        let xs: Vec<_> = items.iter().filter(|c| c.label == "x").collect();
        assert_eq!(xs.len(), 1, "got {xs:?}");
    }
}
