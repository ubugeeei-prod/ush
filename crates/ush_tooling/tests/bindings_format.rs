use ush_tooling::{
    SemanticTokenKind, definition, document_symbols, format_source, hover, references,
    rename_locations, semantic_tokens,
};

#[test]
fn mutable_declaration_and_assignments_share_one_symbol() {
    let source = "let mut count = 0\ncount = count + 1\nif true {\n  let count = 9\n  print count\n}\nprint count\n";
    assert_eq!(definition(source, 1, 0).unwrap().start, 8);
    assert_eq!(definition(source, 4, 8).unwrap().line, 3);
    assert_eq!(references(source, 0, 8).len(), 4);
    assert_eq!(rename_locations(source, 1, 0, "total").unwrap().len(), 4);
    assert!(rename_locations(source, 1, 0, "mut").is_err());
    let symbols = document_symbols(source);
    assert_eq!(symbols.len(), 1);
    assert_eq!((&*symbols[0].name, symbols[0].start), ("count", 8));
    assert!(hover(source, 0, 4).unwrap().contents.contains("mutable"));
    assert!(
        semantic_tokens(source)
            .iter()
            .any(|t| t.line == 0 && t.start == 4 && t.kind == SemanticTokenKind::Keyword)
    );
}

#[test]
fn formatter_separates_imports_and_formats_both_binding_forms() {
    let source =
        "use std::test::assert_eq\nuse std::string::*\nlet mut x=1\nx=x + 1\nassert_eq x 2\n";
    let formatted = format_source(source);
    assert_eq!(
        formatted,
        "use std::test::assert_eq\nuse std::string::*\n\nlet mut x = 1\nx = x + 1\nassert_eq x 2\n"
    );
    assert_eq!(format_source(&formatted), formatted);
    ush_compiler::UshCompiler
        .compile_source(&formatted)
        .unwrap();
}

#[test]
fn import_spacing_preserves_comments_literals_and_existing_blank_lines() {
    for (source, expected) in [
        (
            "use std::test::*\n\n\n# tests\ntest \"x\" {\n}\n",
            "use std::test::*\n\n# tests\ntest \"x\" {\n}\n",
        ),
        (
            "let text = \"\"\"\nuse a::*\nbody\n\"\"\"\n",
            "let text = \"\"\"\nuse a::*\nbody\n\"\"\"\n",
        ),
        (
            "use std::string::*\nlet text = \"\"\"\nraw\n\"\"\"\n",
            "use std::string::*\n\nlet text = \"\"\"\nraw\n\"\"\"\n",
        ),
        ("x == 1\nprint \"a=b\"\n", "x == 1\nprint \"a=b\"\n"),
    ] {
        assert_eq!(format_source(source), expected);
    }
}
