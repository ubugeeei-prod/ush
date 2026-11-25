use ush_tooling::{
    SemanticTokenKind, definition, references, rename_locations, semantic_tokens, signature_help,
};
#[test]
fn nested_bindings_and_parameters_do_not_rename_together() {
    let source = "let x = 1\nfn f(x: Int) -> Int {\n  let y = x\n  y\n}\nprint x\n";
    assert_eq!(definition(source, 2, 10).unwrap().line, 1);
    assert_eq!(references(source, 0, 4).len(), 2);
    assert_eq!(references(source, 1, 5).len(), 2);
    assert!(rename_locations(source, 0, 4, "fn").is_err());
    assert!(rename_locations(source, 0, 4, "y").is_err());
}
#[test]
fn comprehension_bindings_do_not_escape_the_list() {
    let source = "let x = 10\nlet ys = [x for x in 0..3 if x > 1]\nprint x\n";
    assert_eq!(definition(source, 1, 10).unwrap().start, 16);
    assert_eq!(references(source, 0, 4).len(), 2);
    assert_eq!(references(source, 1, 16).len(), 3);
}
#[test]
fn tokenizer_is_total_on_unicode_and_classifies_new_keywords() {
    let source = "print \"日本語😀\" + unknown\n😀\nlet xs = [x for x in 0..2 if true]\nextern \"x.sh\" as x {\n}\ntest {\n}\n";
    let tokens = semantic_tokens(source);
    for t in &tokens {
        assert!(
            source
                .lines()
                .nth(t.line as usize)
                .unwrap()
                .get(t.start as usize..(t.start + t.length) as usize)
                .is_some()
        );
    }
    let words: Vec<_> = tokens
        .iter()
        .filter(|t| t.kind == SemanticTokenKind::Keyword)
        .map(|t| {
            &source.lines().nth(t.line as usize).unwrap()
                [t.start as usize..(t.start + t.length) as usize]
        })
        .collect();
    for keyword in ["for", "in", "if", "true", "extern", "as", "test"] {
        assert!(words.contains(&keyword));
    }
}
#[test]
fn generic_shell_calls_track_arguments_without_splitting_nested_values() {
    let source =
        "fn apply<A, B>(items: List<A>, f: Fn<A, B>) -> List<B> {\n}\napply [1, 2] &identity\n";
    let help = signature_help(source, 2, "apply [1, 2] ".len() as u32).unwrap();
    assert_eq!(help.signature.name, "apply");
    assert_eq!(help.signature.parameters.len(), 2);
    assert_eq!(help.active_parameter, 1);
}

#[test]
fn extern_qualified_calls_and_wildcards_resolve_to_the_declared_function() {
    let source="extern \"helpers.sh\" as helpers {\n  fn hello(name: String) -> String\n}\nuse helpers::*\nprint helpers::hello \"x\"\nprint hello \"y\"\n";
    assert_eq!(definition(source,4,16).unwrap().line,1);
    assert_eq!(definition(source,5,7).unwrap().line,1);
    assert_eq!(references(source,1,6).len(),3);
}
