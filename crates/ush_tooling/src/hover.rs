//! LSP hover descriptions for keywords and scoped declarations.
//! Function declarations include their written or inferred effect row.

use std::path::Path;

use crate::semantic::semantic_tokens;
use crate::token::{SemanticToken, SemanticTokenKind};

/// Markdown body for the hover popover, plus the source range it
/// describes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hover {
    pub contents: String,
    pub line: u32,
    pub start: u32,
    pub length: u32,
}

const KEYWORDS: &[(&str, &str)] = &[
    ("alias", "Declares a shell-style alias."),
    (
        "async",
        "Spawns an asynchronous task; produces a task handle.",
    ),
    (
        "await",
        "Awaits an async task handle and yields its result.",
    ),
    ("break", "Exits the innermost loop."),
    (
        "continue",
        "Skips to the next iteration of the innermost loop.",
    ),
    ("else", "Else branch of an `if` expression."),
    ("enum", "Declares an enum."),
    (
        "export",
        "Sets environment variables for the rest of the script, or for one block with `{ … }`.",
    ),
    ("false", "Boolean false literal."),
    ("fn", "Declares a function."),
    ("for", "For-in loop over a list, tuple, or range."),
    ("if", "Conditional expression."),
    ("impl", "Implementation block for a trait."),
    (
        "let",
        "Declares an immutable binding. Use `let mut name = value` to allow reassignment with `name = value`.",
    ),
    (
        "mut",
        "Makes a `let` binding mutable. Reassignment uses `name = value` and preserves its type.",
    ),
    ("loop", "Infinite loop; exit with `break`."),
    (
        "mod",
        "Imports a local `.ush` module: `mod \"lib/gh.ush\" as gh`.",
    ),
    ("pub", "Lets other files import this module item."),
    (
        "match",
        "Pattern-match an expression. The compiler enforces exhaustiveness.",
    ),
    (
        "echo",
        "Write arguments separated by spaces, then a newline. No options or escape processing; use printf for formatting.",
    ),
    (
        "print",
        "Print a value to stdout. Legacy syntax; prefer echo.",
    ),
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
    ("raise", "Raise a typed error (`T / Problem`)."),
    ("return", "Return from the enclosing function."),
    ("shell", "Run a `.ush`-built shell expression."),
    (
        "spawn",
        "Run a command in the background: `let job = spawn cmd`. Its output streams; \
         `job.await`, `job.pid` and `job.kill()` work on the handle.",
    ),
    (
        "status_of",
        "Run a command, streaming its output, and evaluate to its exit status as an `Int`.",
    ),
    (
        "allow_fail",
        "Run a command, streaming its output, and continue whatever its exit status.",
    ),
    (
        "defer",
        "Run a statement or block when the enclosing function returns or the script exits, \
         last registered first.",
    ),
    ("trait", "Declares a trait."),
    ("true", "Boolean true literal."),
    ("type", "Declares a type alias or `type { ... }`."),
    (
        "use",
        "Import a function or all exports with `use module::*`.",
    ),
    (
        "extern",
        "Declare typed functions from a source-relative `.sh` library.",
    ),
    ("test", "An isolated test block executed by `ush test`."),
    ("while", "While loop."),
];

pub fn hover(source: &str, line: u32, character: u32) -> Option<Hover> {
    hover_at(source, None, line, character)
}

/// A hover for `source` as the contents of `path`, whose modules are
/// read from disk. A function's hover adds its row, written or inferred.
pub fn hover_at(source: &str, path: Option<&Path>, line: u32, character: u32) -> Option<Hover> {
    let tokens = semantic_tokens(source);
    let cursor = tokens
        .iter()
        .copied()
        .find(|t| t.line == line && t.start <= character && character < t.start + t.length)?;
    let lines: Vec<&str> = source.lines().collect();
    let text = token_text(&lines, cursor);
    if text.is_empty() {
        return None;
    }

    let body = if let Some(decl) = crate::references::definition(source, line, character) {
        let snippet = lines.get(decl.line as usize).copied().unwrap_or("").trim();
        let role = match tokens
            .iter()
            .find(|t| t.line == decl.line && t.start == decl.start)
            .map(|t| t.kind)
        {
            Some(SemanticTokenKind::Function) => "function",
            Some(SemanticTokenKind::Type) => "type",
            _ => "variable",
        };
        let row = (role == "function")
            .then(|| crate::rows::function_row(source, path, &text))
            .flatten()
            .unwrap_or_default();
        format!("**{role}** `{text}`\n\n```ush\n{snippet}\n```{row}")
    } else {
        match cursor.kind {
            SemanticTokenKind::Keyword => keyword_hover(&text)?,
            SemanticTokenKind::Variable => identifier_hover(&text, "variable", &lines, &tokens),
            SemanticTokenKind::Function => identifier_hover(&text, "function", &lines, &tokens),
            SemanticTokenKind::Type => identifier_hover(&text, "type", &lines, &tokens),
            SemanticTokenKind::Property => identifier_hover(&text, "property", &lines, &tokens),
            SemanticTokenKind::Decorator => format!("**attribute** `{text}`"),
            _ => return None,
        }
    };
    Some(Hover {
        contents: body,
        line: cursor.line,
        start: cursor.start,
        length: cursor.length,
    })
}

fn keyword_hover(keyword: &str) -> Option<String> {
    KEYWORDS
        .iter()
        .find(|(k, _)| *k == keyword)
        .map(|(_, doc)| format!("**keyword** `{keyword}` — {doc}"))
}

fn identifier_hover(name: &str, role: &str, lines: &[&str], tokens: &[SemanticToken]) -> String {
    // Look for the first occurrence of this name and return that
    // line as a short "declared at" snippet.
    for token in tokens {
        if !matches!(
            token.kind,
            SemanticTokenKind::Variable
                | SemanticTokenKind::Function
                | SemanticTokenKind::Type
                | SemanticTokenKind::Property
        ) {
            continue;
        }
        if token_text(lines, *token) != name {
            continue;
        }
        if let Some(line) = lines.get(token.line as usize) {
            return format!("**{role}** `{name}`\n\n```ush\n{}\n```", line.trim_start());
        }
    }
    format!("**{role}** `{name}`")
}

fn token_text(lines: &[&str], token: SemanticToken) -> String {
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
    use super::hover;

    #[test]
    fn hovers_describe_keywords() {
        let h = hover("let value = 1\n", 0, 0).expect("hover on `let`");
        assert!(h.contents.contains("keyword"));
        assert!(h.contents.contains("let"));
    }

    #[test]
    fn hovers_on_identifier_show_role_and_declaring_line() {
        let h =
            hover("let greeting = \"hi\"\nprint greeting\n", 1, 6).expect("hover on `greeting`");
        assert!(h.contents.contains("variable"));
        assert!(h.contents.contains("greeting"));
        // The declaring line should be quoted.
        assert!(h.contents.contains("let greeting"));
    }

    #[test]
    fn hover_returns_none_for_whitespace() {
        assert!(hover("let x = 1\n", 0, 3).is_none());
    }

    #[test]
    fn hover_returns_none_for_strings() {
        // Cursor inside the string literal.
        assert!(hover("print \"hi\"\n", 0, 8).is_none());
    }
}
