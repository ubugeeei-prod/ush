use ush_tooling::{
    SemanticTokenKind, definition, format_source, rename_locations, semantic_tokens,
};

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

#[test]
fn rename_distinguishes_match_results_outer_bindings_and_each_arm() {
    let source = "let os = \"Linux\"\nlet other = \"outer\"\nlet os = match os {\n  \"Linux\" => os\n  other => other\n}\necho $os $other\n";
    let positions = |line, column, name| {
        rename_locations(source, line, column, name)
            .unwrap()
            .iter()
            .map(|r| (r.line, r.start))
            .collect::<Vec<_>>()
    };
    assert_eq!(positions(6, 6, "platform"), [(2, 4), (6, 6)]);
    assert_eq!(positions(0, 4, "input"), [(0, 4), (2, 15), (3, 13)]);
    assert_eq!(positions(4, 2, "fallback"), [(4, 2), (4, 11)]);
    assert_eq!(positions(1, 4, "outer"), [(1, 4), (6, 10)]);
    assert!(rename_locations(source, 0, 4, "other").is_err());
    assert!(rename_locations(source, 4, 2, "_").is_err());
    let arms = "let x = match \"x\" {\n  other => other\n  other => other\n}\n";
    assert_eq!(definition(arms, 1, 12).unwrap().line, 1);
    assert_eq!(definition(arms, 2, 12).unwrap().line, 2);
}
