use super::format_source;

#[test]
fn formats_indentation_and_spacing() {
    let formatted = format_source(
        "fn greet(name: String)->String {\nprint name\nmatch name {\n\"x\"=>print \"ok\"\n}\n}\n",
    );

    assert_eq!(
        formatted,
        concat!(
            "fn greet(name: String) -> String {\n",
            "  print name\n",
            "  match name {\n",
            "    \"x\" => print \"ok\"\n",
            "  }\n",
            "}\n"
        )
    );
}

#[test]
fn collapses_repeated_blank_lines() {
    let formatted = format_source("print \"a\"\n\n\nprint \"b\"\n");
    assert_eq!(formatted, "print \"a\"\n\nprint \"b\"\n");
}

#[test]
fn preserves_multiline_string_blocks() {
    let formatted = format_source("let page = \"\"\"\n  <div>\n    ok\n  </div>\n\"\"\"\n");
    assert_eq!(
        formatted,
        "let page = \"\"\"\n  <div>\n    ok\n  </div>\n\"\"\"\n"
    );
}

#[test]
fn indents_pipeline_continuations_and_export_bindings() {
    let formatted = format_source(
        "cd \"x\" {\nexport A=\"1\" {\ngit \"log\" |\n# first only\nhead \"-n1\"\n}\n}\n",
    );
    assert_eq!(
        formatted,
        concat!(
            "cd \"x\" {\n",
            "  export A = \"1\" {\n",
            "    git \"log\" |\n",
            "      # first only\n",
            "      head \"-n1\"\n",
            "  }\n",
            "}\n",
        ),
    );
}

#[test]
fn leaves_the_export_command_and_its_strings_alone() {
    assert_eq!(format_source("export \"A=b\"\n"), "export \"A=b\"\n");
}

#[test]
fn attributes_do_not_swallow_following_braces() {
    // `#[default(2)]` used to be parsed as a comment, which made
    // `brace_delta` return 0 for the `fn` line — the body never
    // got indented.
    let formatted = format_source("fn bin(#[default(2)] count: Int) {\nprint count\n}\n");
    assert_eq!(
        formatted,
        concat!(
            "fn bin(#[default(2)] count: Int) {\n",
            "  print count\n",
            "}\n",
        ),
    );
}

#[test]
fn braces_inside_interpolations_do_not_indent() {
    let formatted =
        format_source("if ok {\nprint $\"${items.join(\"{\")} and ${n}\"\nlet s = \"}${x\"\n}\n");
    assert_eq!(
        formatted,
        concat!(
            "if ok {\n",
            "  print $\"${items.join(\"{\")} and ${n}\"\n",
            "  let s = \"}${x\"\n",
            "}\n",
        ),
    );
}

#[test]
fn preserves_interpolating_templates() {
    let source = "if ok {\n  let page = $\"\"\"\n    ## $title\n      - ${item}\n  \"\"\"\n}\n";
    assert_eq!(format_source(source), source);
}
