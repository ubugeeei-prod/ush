use super::{SemanticTokenKind, semantic_token_legend};

#[test]
fn every_kind_indexes_into_the_legend() {
    let legend = semantic_token_legend();
    for (index, kind) in [
        SemanticTokenKind::Keyword,
        SemanticTokenKind::String,
        SemanticTokenKind::Number,
        SemanticTokenKind::Comment,
        SemanticTokenKind::Variable,
        SemanticTokenKind::Function,
        SemanticTokenKind::Type,
        SemanticTokenKind::Property,
        SemanticTokenKind::Operator,
        SemanticTokenKind::Decorator,
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(kind.index() as usize, index);
        assert!(legend.get(index).is_some(), "legend is missing {index}");
    }
}

#[test]
fn the_legend_has_one_entry_per_kind() {
    assert_eq!(semantic_token_legend().len(), 10);
}

#[test]
fn legend_entries_are_the_lsp_standard_names() {
    assert_eq!(
        semantic_token_legend(),
        &[
            "keyword",
            "string",
            "number",
            "comment",
            "variable",
            "function",
            "type",
            "property",
            "operator",
            "decorator",
        ]
    );
}
