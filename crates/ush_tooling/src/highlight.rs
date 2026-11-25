//! LSP `textDocument/documentHighlight` provider.
//!
//! Given a cursor position (zero-based line + character column),
//! returns every occurrence of the same identifier in the document
//! so the editor can underline / box them. We piggy-back on the
//! semantic-token tokenizer so the definition of "an identifier"
//! lines up with what is already highlighted as a variable /
//! function / type / property.

/// Where in the document a highlight should be drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Highlight {
    pub line: u32,
    pub start: u32,
    pub length: u32,
    pub kind: HighlightKind,
}

/// Matches the three roles LSP's `DocumentHighlightKind` distinguishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// The token that produces a binding (e.g. the `name` in `let name = …`).
    Write,
    /// A subsequent read of that binding.
    Read,
    /// Same name but a different category (function call, type ref, …).
    Text,
}

/// Compute the highlights for the identifier under the cursor.
///
/// Returns an empty `Vec` when the cursor is not on an identifier or
/// the identifier appears only once.
pub fn document_highlights(source: &str, line: u32, character: u32) -> Vec<Highlight> {
    let index = crate::scope::Index::new(source);
    let Some(decl) = index.at(line, character).and_then(|i| index.resolved[i]) else {
        return Vec::new();
    };
    index
        .tokens
        .iter()
        .enumerate()
        .filter(|(i, _)| index.resolved[*i] == Some(decl))
        .map(|(i, r)| Highlight {
            line: r.line,
            start: r.start,
            length: r.length,
            kind: if r.line == line && r.start <= character && character < r.start + r.length {
                HighlightKind::Text
            } else if i == decl {
                HighlightKind::Write
            } else {
                HighlightKind::Read
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{HighlightKind, document_highlights};

    #[test]
    fn highlights_repeated_variable() {
        let source = "let greeting = \"hi\"\nprint greeting\nprint greeting\n";
        // cursor on the `greeting` in `let greeting`
        let highlights = document_highlights(source, 0, 5);
        assert_eq!(highlights.len(), 3);
        for h in &highlights {
            assert_eq!(h.length, "greeting".len() as u32);
        }
        assert!(
            highlights
                .iter()
                .any(|h| h.kind == HighlightKind::Text && h.line == 0)
        );
    }

    #[test]
    fn keyword_does_not_highlight() {
        let source = "let a = 1\nlet b = 2\n";
        let highlights = document_highlights(source, 0, 0); // cursor on `let`
        assert!(
            highlights.is_empty(),
            "keyword `let` must not produce highlights, got: {highlights:?}",
        );
    }

    #[test]
    fn cursor_in_whitespace_returns_nothing() {
        let source = "let x = 1\n";
        let highlights = document_highlights(source, 0, 3); // between `let` and `x`
        assert!(highlights.is_empty());
    }

    #[test]
    fn single_occurrence_still_highlights_itself() {
        let source = "let only_one = 1\n";
        let highlights = document_highlights(source, 0, 4);
        assert_eq!(highlights.len(), 1);
        assert_eq!(highlights[0].kind, HighlightKind::Text);
    }
}
