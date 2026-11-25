use ush_tooling::format_source;

#[test]
fn else_joins_the_brace_that_closes_its_branch() {
    let source =
        "if a {\n  print \"a\"\n}\nelse if b {\n  print \"b\"\n}\nelse {\n  print \"c\"\n}\n";
    assert_eq!(
        format_source(source),
        "if a {\n  print \"a\"\n} else if b {\n  print \"b\"\n} else {\n  print \"c\"\n}\n"
    );
}

#[test]
fn a_comment_or_blank_line_keeps_else_on_its_own_line() {
    let source = "if a {\n  print \"a\"\n}\n# otherwise\nelse {\n  print \"c\"\n}\n";
    assert_eq!(format_source(source), source);
    let formatted = format_source("if a {\n  print \"a\"\n} else {\n  print \"c\"\n}\n");
    assert_eq!(format_source(&formatted), formatted);
}
