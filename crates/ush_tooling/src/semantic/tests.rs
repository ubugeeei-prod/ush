use super::semantic_tokens;
use crate::token::{SemanticToken, SemanticTokenKind};

fn kinds(source: &str) -> Vec<(SemanticTokenKind, String)> {
    let lines = source.lines().collect::<Vec<_>>();
    semantic_tokens(source)
        .into_iter()
        .map(|token| (token.kind, text_of(&lines, token)))
        .collect()
}

fn text_of(lines: &[&str], token: SemanticToken) -> String {
    let line = lines[token.line as usize];
    let start = token.start as usize;
    let end = (token.start + token.length) as usize;
    line[start..end.min(line.len())].to_string()
}

fn kind_of(source: &str, needle: &str) -> SemanticTokenKind {
    kinds(source)
        .into_iter()
        .find(|(_, text)| text == needle)
        .unwrap_or_else(|| panic!("no token for {needle}"))
        .0
}

#[test]
fn every_keyword_is_tagged_as_a_keyword() {
    for keyword in [
        "alias", "async", "enum", "fn", "impl", "let", "match", "print", "raise", "return",
        "shell", "trait", "type", "use",
    ] {
        assert_eq!(
            kind_of(&format!("{keyword} value"), keyword),
            SemanticTokenKind::Keyword,
            "{keyword}"
        );
    }
}

#[test]
fn a_let_binding_names_a_variable() {
    assert_eq!(
        kind_of("let value = 1", "value"),
        SemanticTokenKind::Variable
    );
}

#[test]
fn a_function_declaration_names_a_function() {
    assert_eq!(
        kind_of("fn greet(name: String) -> String {", "greet"),
        SemanticTokenKind::Function
    );
}

#[test]
fn a_call_target_is_a_function_even_without_a_declaration() {
    assert_eq!(
        kind_of("print upper(it)", "upper"),
        SemanticTokenKind::Function
    );
}

#[test]
fn declarations_and_capitalized_names_are_types() {
    assert_eq!(kind_of("enum Option {", "Option"), SemanticTokenKind::Type);
    assert_eq!(kind_of("trait Show {", "Show"), SemanticTokenKind::Type);
    assert_eq!(
        kind_of("let value: Int = 1", "Int"),
        SemanticTokenKind::Type
    );
}

#[test]
fn a_name_followed_by_a_colon_is_a_property() {
    assert_eq!(
        kind_of("fn greet(name: String) {", "name"),
        SemanticTokenKind::Property
    );
}

#[test]
fn numbers_and_strings_get_their_own_kinds() {
    assert_eq!(kind_of("let value = 42", "42"), SemanticTokenKind::Number);
    assert_eq!(
        kind_of("let value = \"text\"", "\"text\""),
        SemanticTokenKind::String
    );
    assert_eq!(
        kind_of("let value = 'text'", "'text'"),
        SemanticTokenKind::String
    );
}

#[test]
fn escaped_quotes_do_not_end_a_double_quoted_string() {
    assert_eq!(
        kind_of(r#"let value = "a \" b""#, r#""a \" b""#),
        SemanticTokenKind::String
    );
}

#[test]
fn an_unterminated_string_runs_to_the_end_of_the_line() {
    let tokens = kinds("let value = \"open");
    assert_eq!(tokens.last().expect("token").1, "\"open");
}

#[test]
fn comments_swallow_the_rest_of_the_line() {
    let tokens = kinds("let value = 1 # trailing note");
    assert_eq!(
        tokens.last().expect("token"),
        &(SemanticTokenKind::Comment, "# trailing note".to_string())
    );
}

#[test]
fn attributes_are_tagged_as_decorators() {
    assert_eq!(
        kind_of("#[default(\"x\")] name: String", "#[default(\"x\")]"),
        SemanticTokenKind::Decorator
    );
}

#[test]
fn a_leading_dollar_is_the_shell_escape_operator() {
    let tokens = kinds("$ printf '%s' hi");
    assert_eq!(tokens[0], (SemanticTokenKind::Operator, "$".to_string()));
    assert_eq!(tokens[1].0, SemanticTokenKind::Function);
}

#[test]
fn a_shell_escape_is_recognized_after_a_match_arrow() {
    let tokens = kinds("  _ => $ printf '%s' hi");
    assert!(
        tokens
            .iter()
            .any(|(kind, text)| *kind == SemanticTokenKind::Operator && text == "$")
    );
}

#[test]
fn a_dollar_inside_a_command_is_a_variable_reference() {
    assert_eq!(kind_of("print $HOME", "$HOME"), SemanticTokenKind::Variable);
    assert_eq!(
        kind_of("print ${HOME}", "${HOME}"),
        SemanticTokenKind::Variable
    );
}

#[test]
fn two_character_operators_are_kept_together() {
    for operator in ["->", "=>", "::", "==", "!=", "<=", ">=", "&&", "||"] {
        let source = format!("a {operator} b");
        assert_eq!(
            kind_of(&source, operator),
            SemanticTokenKind::Operator,
            "{operator}"
        );
    }
}

#[test]
fn triple_quoted_strings_span_several_lines() {
    let source = "let block = \"\"\"\n  body\n\"\"\"\nprint block\n";
    let string_lines = semantic_tokens(source)
        .into_iter()
        .filter(|token| token.kind == SemanticTokenKind::String)
        .map(|token| token.line)
        .collect::<Vec<_>>();

    assert_eq!(string_lines, vec![0, 1, 2]);
}

#[test]
fn code_after_a_block_string_goes_back_to_normal_highlighting() {
    let source = "let block = \"\"\"\n  body\n\"\"\"\nprint block\n";
    let tail = semantic_tokens(source)
        .into_iter()
        .filter(|token| token.line == 3)
        .map(|token| token.kind)
        .collect::<Vec<_>>();

    assert_eq!(
        tail,
        vec![SemanticTokenKind::Keyword, SemanticTokenKind::Variable]
    );
}

#[test]
fn a_block_string_that_opens_and_closes_on_one_line_does_not_leak() {
    let source = "let block = \"\"\"body\"\"\"\nprint block\n";
    let tail = semantic_tokens(source)
        .into_iter()
        .filter(|token| token.line == 1)
        .map(|token| token.kind)
        .collect::<Vec<_>>();

    assert_eq!(
        tail,
        vec![SemanticTokenKind::Keyword, SemanticTokenKind::Variable]
    );
}

#[test]
fn a_block_string_can_be_followed_by_more_code_on_its_closing_line() {
    let source = "let block = \"\"\"\n  body\n\"\"\" + suffix\n";
    let closing = semantic_tokens(source)
        .into_iter()
        .filter(|token| token.line == 2)
        .map(|token| token.kind)
        .collect::<Vec<_>>();

    assert_eq!(
        closing,
        vec![
            SemanticTokenKind::String,
            SemanticTokenKind::Operator,
            SemanticTokenKind::Variable
        ]
    );
}

#[test]
fn tokens_carry_their_line_and_column() {
    let tokens = semantic_tokens("let a = 1\nlet b = 2\n");
    let second_line = tokens
        .iter()
        .filter(|token| token.line == 1)
        .collect::<Vec<_>>();

    assert_eq!(second_line[0].start, 0);
    assert_eq!(second_line[0].length, 3);
    assert_eq!(second_line[1].start, 4);
}

#[test]
fn an_empty_source_produces_no_tokens() {
    assert!(semantic_tokens("").is_empty());
    assert!(semantic_tokens("\n\n").is_empty());
    assert!(semantic_tokens("   ").is_empty());
}
