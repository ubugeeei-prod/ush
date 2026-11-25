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
    let source = "extern \"helpers.sh\" as helpers {\n  fn hello(name: String) -> String\n}\nuse helpers::*\nprint helpers::hello \"x\"\nprint hello \"y\"\n";
    assert_eq!(definition(source, 4, 16).unwrap().line, 1);
    assert_eq!(definition(source, 5, 7).unwrap().line, 1);
    assert_eq!(references(source, 1, 6).len(), 3);
}

#[test]
fn comprehension_iterable_reads_the_outer_binding_before_shadowing() {
    let source = "let xs = [1, 2]\nlet ys = [xs for xs in xs if xs > 1]\nprint xs\n";
    assert_eq!(definition(source, 1, 23).unwrap().line, 0);
    assert_eq!(definition(source, 1, 10).unwrap().start, 17);
    assert_eq!(references(source, 0, 4).len(), 3);
}

#[test]
fn outline_includes_named_tests_and_extern_modules() {
    let symbols = ush_tooling::document_symbols(
        "extern \"helpers.sh\" as helpers {\n}\ntest \"日本語😀\" {\n}\ntest {\n}\n",
    );
    assert_eq!(
        symbols.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(),
        ["helpers", "test 日本語😀", "test"]
    );
}

#[test]
fn braces_and_keywords_in_multiline_strings_do_not_change_binding_scopes() {
    let source =
        "let x = 1\nfn f(x: Int) -> Int {\n  let text = \"\"\"\n}\nfn\n\"\"\"\n  x\n}\nprint x\n";
    assert_eq!(definition(source, 6, 2).unwrap().line, 1);
    assert_eq!(references(source, 0, 4).len(), 2);
}

#[test]
fn rename_does_not_speculate_about_pattern_binding_capture() {
    let source = "let x = 1\nmatch value {\n  Some(x) => print x\n}\nprint x\n";
    assert!(rename_locations(source, 0, 4, "y").is_err());
}

#[test]
fn extern_renames_preserve_foreign_bindings() {
    let source = "extern \"helpers.sh\" as helpers {\n  fn hello(name: String) -> String\n}\nuse helpers::*\nprint hello \"x\"\n";
    assert!(ush_tooling::prepare_rename(source, 4, 7).is_none());
    assert!(rename_locations(source, 4, 7, "greet").is_err());
}

#[test]
fn parameter_hover_uses_its_declaration_role() {
    let source = "fn f(x: Int) -> Int {\n  x\n}\n";
    assert!(
        ush_tooling::hover(source, 1, 2)
            .unwrap()
            .contents
            .starts_with("**variable**")
    );
    assert!(
        ush_tooling::hover(source, 0, 3)
            .unwrap()
            .contents
            .starts_with("**function**")
    );
}

#[test]
fn signature_recovery_and_nested_calls_do_not_invent_arguments() {
    let declarations = "fn broken\nfn outer(x: Int, y: Int, z: Int) {}\nfn inner(x: Int) {}\n";
    let signatures = ush_tooling::function_signatures(declarations);
    assert_eq!(
        signatures
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        ["outer", "inner"]
    );
    for (call, expected) in [
        ("outer(inner(42), ", Some(("outer", 1))),
        ("outer(inner(", Some(("inner", 0))),
        ("outer(unknown(", None),
        ("outer(unknown(42), ", Some(("outer", 1))),
    ] {
        let source = format!("{declarations}{call}");
        let help = signature_help(&source, 3, call.len() as u32);
        assert_eq!(
            help.as_ref()
                .map(|h| (h.signature.name.as_str(), h.active_parameter)),
            expected,
            "{call}"
        );
    }
    let source = format!("{declarations}outer(\"\"\"\n,\n\"\"\", ");
    assert_eq!(signature_help(&source, 5, 5).unwrap().active_parameter, 1);
}

#[test]
fn callback_reference_is_one_shell_argument() {
    let declarations = "fn apply<A, B>(items: List<A>, f: Fn<A, B>) -> List<B> {\n}\n";
    for call in ["apply [1, 2] &iden", "apply [1, 2] &identity"] {
        let source = format!("{declarations}{call}");
        assert_eq!(
            signature_help(&source, 2, call.len() as u32)
                .unwrap()
                .active_parameter,
            1
        );
    }
}
