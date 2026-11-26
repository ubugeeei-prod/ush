use ush_tooling::{SemanticTokenKind, definition, format_source, semantic_tokens};

#[test]
fn match_result_is_visible_only_after_its_initializer() {
    let source =
        "let os = \"Linux\"\nlet os = match os {\n  \"Linux\" => os\n  _ => os\n}\necho $os\n";
    for (line, column) in [(1, 15), (2, 13), (3, 7)] {
        assert_eq!(definition(source, line, column).unwrap().line, 0);
    }
    assert_eq!(definition(source, 5, 6).unwrap().line, 1);
    let fresh = "let result = match true {\n  true => result\n  false => \"no\"\n}\necho $result\n";
    assert!(definition(fresh, 1, 10).is_none());
    assert_eq!(definition(fresh, 4, 6).unwrap().line, 0);
}

#[test]
fn formatter_and_highlighter_recognize_match_initializers() {
    let source = "let os=match uname -s {\n\"Linux\"=>\"linux\"\n_=>\"other\"\n}\necho $os\n";
    let formatted = format_source(source);
    assert_eq!(
        formatted,
        "let os = match uname -s {\n  \"Linux\" => \"linux\"\n  _ => \"other\"\n}\necho $os\n"
    );
    let tokens = semantic_tokens(&formatted);
    assert!(tokens.iter().any(|t| t.line == 0
        && t.kind == SemanticTokenKind::Keyword
        && &formatted[t.start as usize..(t.start + t.length) as usize] == "match"));
    assert!(tokens.iter().any(|t| t.line == 0
        && t.kind == SemanticTokenKind::String
        && &formatted[t.start as usize..(t.start + t.length) as usize] == "-s"));
}
