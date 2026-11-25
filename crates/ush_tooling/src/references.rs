//! Scoped navigation and conservative rename over the tolerant lexical index.
use crate::scope::Index;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reference {
    pub line: u32,
    pub start: u32,
    pub length: u32,
}
fn location(index: &Index<'_>, i: usize) -> Reference {
    let t = index.tokens[i];
    Reference {
        line: t.line,
        start: t.start,
        length: t.length,
    }
}
pub fn references(source: &str, line: u32, character: u32) -> Vec<Reference> {
    let index = Index::new(source);
    let Some(decl) = index.at(line, character).and_then(|i| index.resolved[i]) else {
        return Vec::new();
    };
    index
        .resolved
        .iter()
        .enumerate()
        .filter(|(_, d)| **d == Some(decl))
        .map(|(i, _)| location(&index, i))
        .collect()
}
pub fn definition(source: &str, line: u32, character: u32) -> Option<Reference> {
    let index = Index::new(source);
    index
        .at(line, character)
        .and_then(|i| index.resolved[i])
        .map(|i| location(&index, i))
}
pub fn prepare_rename(source: &str, line: u32, character: u32) -> Option<Reference> {
    let index = Index::new(source);
    let i = index.at(line, character)?;
    index.resolved[i]?;
    Some(location(&index, i))
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenameError {
    InvalidName,
    Conflict,
    PatternBindings,
}
impl std::fmt::Display for RenameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidName => "use a non-keyword identifier",
            Self::Conflict => "that name is already bound in this document",
            Self::PatternBindings => "safe rename of pattern bindings is not supported yet",
        })
    }
}

pub fn rename_locations(
    source: &str,
    line: u32,
    character: u32,
    new_name: &str,
) -> Result<Vec<Reference>, RenameError> {
    let mut chars = new_name.chars();
    if !chars
        .next()
        .is_some_and(|c| c == '_' || c.is_ascii_alphabetic())
        || !chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
        || crate::semantic::semantic_tokens(new_name)
            .first()
            .is_some_and(|t| t.kind == crate::SemanticTokenKind::Keyword)
    {
        return Err(RenameError::InvalidName);
    }
    let index = Index::new(source);
    // The tolerant index cannot yet prove capture safety for destructuring patterns.
    if index.names.contains(&"match") || index.names.windows(2).any(|p| p == ["if", "let"]) {
        return Err(RenameError::PatternBindings);
    }
    let Some(i) = index.at(line, character) else {
        return Ok(Vec::new());
    };
    if new_name != index.names[i]
        && index
            .names
            .iter()
            .enumerate()
            .any(|(i, n)| *n == new_name && index.resolved[i].is_some())
    {
        return Err(RenameError::Conflict);
    }
    Ok(references(source, line, character))
}

#[cfg(test)]
mod tests {
    use super::{RenameError, definition, prepare_rename, references, rename_locations};

    #[test]
    fn prepare_rename_returns_identifier_range_under_cursor() {
        let source = "let value = 1\n";
        let range = prepare_rename(source, 0, 5).expect("rename `value`");
        assert_eq!(range.line, 0);
        assert_eq!(range.start, 4);
        assert_eq!(range.length, "value".len() as u32);
    }

    #[test]
    fn prepare_rename_returns_none_on_keyword() {
        // Cursor on `let`.
        assert!(prepare_rename("let x = 1\n", 0, 0).is_none());
    }

    #[test]
    fn references_return_every_occurrence_of_a_variable() {
        let source = "let v = 1\nprint v\nprint v\n";
        let refs = references(source, 0, 4);
        assert_eq!(refs.len(), 3);
    }

    #[test]
    fn definition_points_to_the_first_occurrence() {
        let source = "let v = 1\nprint v\n";
        let def = definition(source, 1, 6).expect("v has a definition");
        assert_eq!(def.line, 0);
    }

    #[test]
    fn references_on_keyword_return_nothing() {
        let source = "let a = 1\n";
        assert!(references(source, 0, 0).is_empty());
    }

    #[test]
    fn rename_returns_every_occurrence_when_new_name_is_valid() {
        let source = "let v = 1\nprint v\n";
        let locations = rename_locations(source, 0, 4, "value").expect("rename `v` to `value`");
        assert_eq!(locations.len(), 2);
    }

    #[test]
    fn rename_rejects_invalid_identifier() {
        let source = "let v = 1\n";
        let err = rename_locations(source, 0, 4, "1bad").unwrap_err();
        assert_eq!(err, RenameError::InvalidName);

        let err = rename_locations(source, 0, 4, "with space").unwrap_err();
        assert_eq!(err, RenameError::InvalidName);
    }
}
