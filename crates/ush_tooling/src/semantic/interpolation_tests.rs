use super::semantic_tokens;
use crate::token::{SemanticToken, SemanticTokenKind as Kind};

fn tokens(source: &str) -> Vec<(u32, Kind, String)> {
    let lines = source.lines().collect::<Vec<_>>();
    semantic_tokens(source)
        .into_iter()
        .map(|token: SemanticToken| {
            let line = lines[token.line as usize];
            let text = &line[token.start as usize..(token.start + token.length) as usize];
            (token.line, token.kind, text.to_string())
        })
        .collect()
}

#[test]
fn interpolations_split_a_string_into_text_variables_and_code() {
    let found = tokens(r#"print $"a $name ${xs.len()} b""#);
    let kinds = found
        .iter()
        .map(|(_, kind, text)| (*kind, text.as_str()))
        .collect::<Vec<_>>();
    assert_eq!(
        kinds,
        vec![
            (Kind::Keyword, "print"),
            (Kind::String, "$\"a "),
            (Kind::Variable, "$name"),
            (Kind::String, " "),
            (Kind::Operator, "${"),
            (Kind::Variable, "xs"),
            (Kind::Operator, "."),
            (Kind::Function, "len"),
            (Kind::Operator, "("),
            (Kind::Operator, ")"),
            (Kind::Operator, "}"),
            (Kind::String, " b\""),
        ]
    );
}

#[test]
fn plain_strings_and_escaped_dollars_stay_string_text() {
    let found = tokens(r#"print "a ${b} $c" $"cost \${x}""#);
    assert_eq!(found[1], (0, Kind::String, r#""a ${b} $c""#.into()));
    assert_eq!(found[2], (0, Kind::String, r#"$"cost \${x}""#.into()));
}

#[test]
fn strings_nested_in_an_interpolation_are_their_own_tokens() {
    let found = tokens(r#"print $"${items.join(", ")}! $env::HOME""#);
    assert!(found.contains(&(0, Kind::String, "\", \"".into())));
    assert!(found.contains(&(0, Kind::Function, "join".into())));
    assert!(found.contains(&(0, Kind::Variable, "$env::HOME".into())));
}

#[test]
fn templates_interpolate_across_lines_and_raw_blocks_do_not() {
    let found = tokens("let t = $\"\"\"\n  # $title\n\"\"\"\nlet r = \"\"\"\n  ${raw}\n\"\"\"\n");
    assert!(found.contains(&(1, Kind::Variable, "$title".into())));
    assert!(found.contains(&(1, Kind::String, "  # ".into())));
    assert!(found.contains(&(2, Kind::String, "\"\"\"".into())));
    assert!(found.contains(&(4, Kind::String, "  ${raw}".into())));
}

#[test]
fn glob_patterns_are_highlighted() {
    let found = tokens(r#"glob "*.ts" => print $RUNNER"#);
    assert_eq!(found[0], (0, Kind::Keyword, "glob".into()));
}
