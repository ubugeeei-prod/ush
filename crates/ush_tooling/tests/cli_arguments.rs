use ush_tooling::{
    SemanticTokenKind as Kind, definition, format_source, references, rename_locations,
    semantic_tokens,
};

#[test]
fn bare_words_never_resolve_to_local_variables() {
    let source = "let name = \"two words\"\necho name $name --output=${name}/file\nlet output = echo name $name\n";
    let refs = references(source, 0, 4);
    assert_eq!(refs.len(), 4, "{refs:?}");
    assert!(definition(source, 1, 5).is_none());
    assert_eq!(definition(source, 1, 11).unwrap().line, 0);
    assert_eq!(definition(source, 1, 27).unwrap().line, 0);
    let edits = rename_locations(source, 0, 4, "value").unwrap();
    for edit in edits {
        let line = source.lines().nth(edit.line as usize).unwrap();
        assert_eq!(
            &line[edit.start as usize..(edit.start + edit.length) as usize],
            "name"
        );
    }
    let tokens = semantic_tokens(source);
    assert!(
        tokens
            .iter()
            .any(|t| t.line == 1 && t.start == 5 && t.length == 4 && t.kind == Kind::String)
    );
}

#[test]
fn grouped_expressions_and_pipeline_stages_keep_navigation() {
    let source = "let name = \"ush\"\nfn greet(value: String) -> String {\n value\n}\necho (greet $name) name\n\"hello\" | echo name $name\n";
    assert_eq!(references(source, 0, 4).len(), 3);
    assert_eq!(definition(source, 4, 7).unwrap().line, 1);
    assert!(definition(source, 4, 19).is_none());
    assert!(definition(source, 5, 16).is_none());
}

#[test]
fn formatting_preserves_argument_bytes_and_variable_braces() {
    let source = "use $bin::*\nlet name=\"ush\"\necho --output=${name}/a a=>b a->b\necho done\n";
    let expected =
        "use $bin::*\n\nlet name = \"ush\"\necho --output=${name}/a a=>b a->b\necho done\n";
    assert_eq!(format_source(source), expected);
    assert_eq!(format_source(expected), expected);
    ush_compiler::UshCompiler.compile_source(expected).unwrap();
}

#[test]
fn completion_after_a_sigil_offers_variable_names() {
    let source = "let name = \"ush\"\necho literal $na\n";
    let items = ush_tooling::completions_at(source, 1, 16);
    assert_eq!(
        items
            .iter()
            .map(|item| item.label.as_str())
            .collect::<Vec<_>>(),
        ["name"]
    );
    assert!(
        items
            .iter()
            .all(|item| item.kind == ush_tooling::CompletionKind::Variable)
    );
    assert!(items.iter().all(|item| !item.label.starts_with('$')));
    assert!(
        !ush_tooling::completions(source)
            .iter()
            .any(|item| item.label == "literal")
    );
}
