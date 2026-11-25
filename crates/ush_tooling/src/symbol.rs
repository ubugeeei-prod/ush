//! LSP `textDocument/documentSymbol` provider.
//!
//! Returns a flat outline of the top-level declarations in a `.ush`
//! source file: every `fn`, `enum`, `trait`, `type`, top-level
//! `let`, and `alias`. This is a syntactic scan, not a full parse,
//! because the AST inside `ush_compiler` is private to that crate
//! and an outline does not need typed information — it only needs
//! "where does this name appear, and what kind of declaration is
//! it".

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSymbol {
    pub name: String,
    pub kind: SymbolKind,
    /// Zero-based line the declaration starts on.
    pub line: u32,
    /// Zero-based column of the first byte of the declaration.
    pub start: u32,
    /// Length in bytes of just the declaration *name* (not the body).
    pub length: u32,
}

/// Mirrors the subset of `lsp_types::SymbolKind` the editor cares
/// about for `.ush` outlines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Enum,
    Trait,
    Type,
    Variable,
    Alias,
    Module,
}

/// Walk the source line-by-line, looking for top-level declarations.
pub fn document_symbols(source: &str) -> Vec<DocumentSymbol> {
    let mut out = Vec::new();
    let mut in_multiline_string = false;
    for (line_no, raw) in source.lines().enumerate() {
        let triple_count = raw.matches("\"\"\"").count();
        let was_in_multiline = in_multiline_string;
        if triple_count % 2 == 1 {
            in_multiline_string = !in_multiline_string;
        }
        if was_in_multiline {
            // Entirely inside a `"""…"""` block — skip.
            continue;
        }
        let line_no = line_no as u32;
        let trimmed = raw.trim_start();
        let leading = (raw.len() - trimmed.len()) as u32;
        if leading > 0 {
            // Only top-level declarations contribute to the outline;
            // indented `let` inside a function body is local state.
            continue;
        }
        if let Some(symbol) = declaration_on_line(line_no, raw) {
            out.push(symbol);
        }
    }
    out
}

fn declaration_on_line(line_no: u32, line: &str) -> Option<DocumentSymbol> {
    if let Some(rest) = line.strip_prefix("test")
        && (rest.starts_with(char::is_whitespace) || rest.starts_with('{'))
    {
        let label = rest.trim().trim_end_matches('{').trim();
        return Some(DocumentSymbol {
            name: if label.is_empty() {
                "test".into()
            } else {
                format!("test {}", label.trim_matches('"'))
            },
            kind: SymbolKind::Function,
            line: line_no,
            start: 0,
            length: (line.trim_end_matches('{').trim_end().len()) as u32,
        });
    }
    if let Some(rest) = line.strip_prefix("extern ") {
        let (name, start) = if let Some((prefix, alias)) = line.rsplit_once(" as ") {
            (alias.trim_end_matches('{').trim(), prefix.len() + 4)
        } else {
            (rest.trim_end_matches('{').trim(), 7)
        };
        return Some(DocumentSymbol {
            name: name.trim_matches('"').into(),
            kind: SymbolKind::Module,
            line: line_no,
            start: start as u32,
            length: name.len() as u32,
        });
    }
    let (keyword, kind) = if line.starts_with("fn ") {
        ("fn", SymbolKind::Function)
    } else if line.starts_with("enum ") {
        ("enum", SymbolKind::Enum)
    } else if line.starts_with("trait ") {
        ("trait", SymbolKind::Trait)
    } else if line.starts_with("type ") {
        ("type", SymbolKind::Type)
    } else if line.starts_with("let ") {
        ("let", SymbolKind::Variable)
    } else if line.starts_with("alias ") {
        ("alias", SymbolKind::Alias)
    } else {
        return None;
    };

    let after_keyword = &line[keyword.len()..];
    let name_start_in_after = after_keyword
        .char_indices()
        .find(|(_, c)| !c.is_whitespace())
        .map(|(i, _)| i)?;
    let name_offset = keyword.len() + name_start_in_after;
    let rest = &line[name_offset..];
    let name_end_in_rest = rest
        .char_indices()
        .find(|(_, c)| !(c.is_alphanumeric() || *c == '_'))
        .map(|(i, _)| i)
        .unwrap_or(rest.len());
    if name_end_in_rest == 0 {
        return None;
    }
    let name = &rest[..name_end_in_rest];
    Some(DocumentSymbol {
        name: name.to_string(),
        kind,
        line: line_no,
        start: name_offset as u32,
        length: name_end_in_rest as u32,
    })
}

#[cfg(test)]
mod tests {
    use super::{SymbolKind, document_symbols};

    #[test]
    fn lists_top_level_fn_enum_and_let() {
        let source = "\
fn greet(name: String) -> String {
  return \"hi \" + name
}

enum Colour {
  Red,
  Blue,
}

let value = 42

alias ll = \"ls -la\"
";
        let symbols = document_symbols(source);
        let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["greet", "Colour", "value", "ll"]);

        let kinds: Vec<SymbolKind> = symbols.iter().map(|s| s.kind).collect();
        assert_eq!(
            kinds,
            vec![
                SymbolKind::Function,
                SymbolKind::Enum,
                SymbolKind::Variable,
                SymbolKind::Alias,
            ]
        );

        // The first symbol — `greet` — starts at column 3 (after `fn `).
        assert_eq!(symbols[0].line, 0);
        assert_eq!(symbols[0].start, 3);
        assert_eq!(symbols[0].length, "greet".len() as u32);
    }

    #[test]
    fn ignores_indented_let() {
        let source = "fn f() {\n  let inner = 1\n}\n";
        let symbols = document_symbols(source);
        let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["f"]);
    }

    #[test]
    fn skips_triple_quoted_blocks() {
        let source = "let page = \"\"\"\nfn fake() {}\nlet imposter = 1\n\"\"\"\nlet real = 2\n";
        let symbols = document_symbols(source);
        let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["page", "real"]);
    }
}
