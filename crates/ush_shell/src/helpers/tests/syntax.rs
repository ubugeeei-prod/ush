use crate::helpers::lambda_syntax::{
    block_body, parse_call, parse_list_literal, parse_string_arg, parse_string_literal,
};

#[test]
fn a_call_is_split_into_a_name_and_arguments() {
    let (name, args) = parse_call("upper(it)").expect("parse");
    assert_eq!(name, "upper");
    assert_eq!(args, vec!["it".to_string()]);
}

#[test]
fn call_arguments_split_on_top_level_commas_only() {
    let (_, args) = parse_call(r#"replace(it, "a,b", "c")"#).expect("parse");
    assert_eq!(args, vec!["it", "\"a,b\"", "\"c\""]);
}

#[test]
fn nested_brackets_braces_and_parens_are_kept_together() {
    let (_, args) = parse_call("f(g(a, b), [1, 2], { x, y })").expect("parse");
    assert_eq!(args, vec!["g(a, b)", "[1, 2]", "{ x, y }"]);
}

#[test]
fn a_call_without_arguments_yields_an_empty_argument_list() {
    let (name, args) = parse_call("now()").expect("parse");
    assert_eq!(name, "now");
    assert!(args.is_empty());
}

#[test]
fn text_without_parentheses_is_not_a_call() {
    assert!(parse_call("upper it").is_err());
    assert!(parse_call("").is_err());
}

#[test]
fn list_literals_are_unwrapped_and_split() {
    assert_eq!(
        parse_list_literal(r#"["a", "b"]"#).expect("parse"),
        vec!["\"a\"", "\"b\""]
    );
    assert!(parse_list_literal("[]").expect("parse").is_empty());
    assert_eq!(
        parse_list_literal(" [ 1 , 2 ] ").expect("parse"),
        vec!["1", "2"]
    );
}

#[test]
fn text_without_brackets_is_not_a_list_literal() {
    assert!(parse_list_literal("1, 2").is_err());
}

#[test]
fn both_quote_styles_are_string_literals() {
    assert_eq!(parse_string_literal("\"ush\""), Some("ush".to_string()));
    assert_eq!(parse_string_literal("'ush'"), Some("ush".to_string()));
    assert_eq!(parse_string_literal("\"\""), Some(String::new()));
}

#[test]
fn unquoted_or_half_quoted_text_is_not_a_string_literal() {
    assert_eq!(parse_string_literal("ush"), None);
    assert_eq!(parse_string_literal("\"ush"), None);
    assert_eq!(parse_string_literal("\""), None);
    assert_eq!(parse_string_literal(""), None);
}

#[test]
fn a_missing_string_literal_is_reported_with_the_offending_text() {
    let error = parse_string_arg("it").expect_err("not a literal");
    assert!(error.to_string().contains("expected string literal"));
    assert!(error.to_string().contains("it"));
}

#[test]
fn a_block_body_is_unwrapped_and_trimmed() {
    assert_eq!(block_body("{ upper(it) }"), "upper(it)");
    assert_eq!(block_body("  {x}  "), "x");
    assert_eq!(block_body("upper(it)"), "upper(it)");
    assert_eq!(block_body("{}"), "");
}
