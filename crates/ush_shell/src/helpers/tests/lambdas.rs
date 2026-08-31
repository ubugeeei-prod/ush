use crate::helpers::{HelperInvocation, ValueStream};

fn run(helper: &str, input: &[&str]) -> (String, i32) {
    let invocation = HelperInvocation::parse(helper)
        .unwrap_or_else(|| panic!("{helper} is not a helper"))
        .unwrap_or_else(|error| panic!("{helper} failed to parse: {error}"));
    let lines = input.iter().map(|line| (*line).to_string()).collect();
    let (output, status) = invocation
        .execute(ValueStream::Lines(lines))
        .expect("execute");
    (output.to_text().expect("text"), status)
}

fn parse_error(helper: &str) -> String {
    HelperInvocation::parse(helper)
        .unwrap_or_else(|| panic!("{helper} is not a helper"))
        .expect_err("expected a parse error")
        .to_string()
}

#[test]
fn both_lambda_head_styles_are_accepted() {
    assert_eq!(run("map(it -> upper(it))", &["a"]).0, "A\n");
    assert_eq!(run(r"map(\it -> upper(it))", &["a"]).0, "A\n");
    assert_eq!(run(r"map(\line -> upper(line))", &["a"]).0, "A\n");
}

#[test]
fn every_string_transform_is_supported() {
    assert_eq!(run("map(it -> upper(it))", &["ush"]).0, "USH\n");
    assert_eq!(run("map(it -> lower(it))", &["USH"]).0, "ush\n");
    assert_eq!(run("map(it -> trim(it))", &["  ush  "]).0, "ush\n");
    assert_eq!(
        run(r#"map(it -> replace(it, "a", "b"))"#, &["banana"]).0,
        "bbnbnb\n"
    );
}

#[test]
fn an_identity_lambda_passes_lines_through() {
    assert_eq!(run("map(it -> it)", &["a", "b"]).0, "a\nb\n");
    assert_eq!(run("map(it -> print(it))", &["a"]).0, "a\n");
}

#[test]
fn a_constant_body_replaces_every_line() {
    assert_eq!(run(r#"map(it -> "x")"#, &["a", "b"]).0, "x\nx\n");
    assert_eq!(run(r"map(\-> {})", &["a"]).0, "\n");
}

#[test]
fn transforms_over_literals_are_folded_at_parse_time() {
    assert_eq!(
        run(r#"map(it -> upper("ush"))"#, &["a", "b"]).0,
        "USH\nUSH\n"
    );
    assert_eq!(
        run(r#"map(it -> replace("banana", "a", "o"))"#, &["x"]).0,
        "bonono\n"
    );
}

#[test]
fn each_behaves_like_map() {
    assert_eq!(run("each(it -> upper(it))", &["a"]).0, "A\n");
}

#[test]
fn every_predicate_is_supported() {
    assert_eq!(
        run(r#"filter(it -> contains(it, "s"))"#, &["ush", "abc"]).0,
        "ush\n"
    );
    assert_eq!(
        run(r#"filter(it -> starts_with(it, "u"))"#, &["ush", "abc"]).0,
        "ush\n"
    );
    assert_eq!(
        run(r#"filter(it -> ends_with(it, "c"))"#, &["ush", "abc"]).0,
        "abc\n"
    );
    assert_eq!(
        run(r#"filter(it -> eq(it, "ush"))"#, &["ush", "abc"]).0,
        "ush\n"
    );
}

#[test]
fn constant_predicates_keep_or_drop_everything() {
    assert_eq!(run("filter(it -> true)", &["a", "b"]).0, "a\nb\n");
    assert_eq!(run("filter(it -> false)", &["a", "b"]).0, "");
    assert_eq!(run(r"filter(\-> {})", &["a"]).0, "");
}

#[test]
fn any_reports_a_boolean_and_an_exit_status() {
    assert_eq!(
        run(r#"any(it -> contains(it, "s"))"#, &["ush"]),
        ("true\n".to_string(), 0)
    );
    assert_eq!(
        run(r#"any(it -> contains(it, "z"))"#, &["ush"]),
        ("false\n".to_string(), 1)
    );
    assert_eq!(
        run(r#"some(it -> eq(it, "ush"))"#, &["ush"]),
        ("true\n".to_string(), 0)
    );
}

#[test]
fn the_f_prefixed_aliases_resolve_to_the_same_helpers() {
    assert_eq!(run("fmap(it -> upper(it))", &["a"]).0, "A\n");
    assert_eq!(run(r#"ffilter(it -> eq(it, "a"))"#, &["a", "b"]).0, "a\n");
    assert_eq!(run("fany(it -> true)", &["a"]).0, "true\n");
    assert_eq!(run("fsome(it -> true)", &["a"]).0, "true\n");
}

#[test]
fn predicates_over_literals_are_folded_at_parse_time() {
    assert_eq!(
        run(r#"filter(it -> contains("ush", "s"))"#, &["a", "b"]).0,
        "a\nb\n"
    );
    assert_eq!(
        run(r#"filter(it -> contains("ush", "z"))"#, &["a", "b"]).0,
        ""
    );
}

#[test]
fn a_head_without_an_arrow_is_rejected() {
    assert!(parse_error("map(it)").contains("lambda"));
}

#[test]
fn an_unsupported_head_is_rejected() {
    assert!(parse_error("map(x -> upper(x))").contains("lambda"));
}

#[test]
fn several_lambda_arguments_are_rejected() {
    let message = parse_error(r"map(\a, b -> upper(a))");
    assert!(message.contains("at most one argument"), "{message}");
}

#[test]
fn unknown_transforms_and_predicates_are_rejected() {
    assert!(parse_error("map(it -> reverse(it))").contains("unsupported transform"));
    assert!(parse_error("filter(it -> matches(it))").contains("unsupported predicate"));
}

#[test]
fn a_transform_over_an_unknown_binding_is_rejected() {
    assert!(parse_error("map(it -> upper(other))").contains("unsupported transform"));
    assert!(parse_error(r#"filter(it -> eq(other, "a"))"#).contains("unsupported predicate"));
}

#[test]
fn an_unbalanced_helper_invocation_is_rejected() {
    assert!(parse_error("map)it -> it(").contains("invalid helper invocation"));
}

#[test]
fn text_that_is_not_a_helper_is_left_for_the_command_dispatcher() {
    assert!(HelperInvocation::parse("ls -la").is_none());
    assert!(HelperInvocation::parse("grep(pattern)").is_none());
    assert!(HelperInvocation::parse("").is_none());
}

#[test]
fn helpers_tolerate_surrounding_whitespace() {
    assert_eq!(run("  map(it -> upper(it))  ", &["a"]).0, "A\n");
}
