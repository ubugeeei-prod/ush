use ush_compiler::UshCompiler;

#[test]
fn heterogeneous_tuple_iteration_is_rejected() {
    let error = UshCompiler::default()
        .compile_source(
            r#"
            let pair = (1, "two")
            for item in pair {
              print item
            }
        "#,
        )
        .expect_err("tuple iteration should fail");

    assert!(
        error
            .to_string()
            .contains("for-in over tuples requires all items to share one type")
    );
}

#[test]
fn logic_operators_in_a_value_are_rejected_rather_than_misread() {
    // `(a) && (b)` once lost its outer parentheses as if one pair wrapped
    // it all, and compared `x` with the text `y") && (x == "y`.
    for source in [
        "let x = \"x\"\nlet b = (x != \"y\") && (x == \"y\")\nprint b\n",
        "let x = \"x\"\nlet b = (x == \"x\") || (x == \"y\")\nprint b\n",
        "let x = \"x\"\nlet b = x != \"\" && x == \"b\"\nprint b\n",
    ] {
        let error = UshCompiler::default()
            .compile_source(source)
            .expect_err("a logic operator in a value should fail");
        assert!(
            format!("{error:#}").contains("join the conditions of `if` and `while`, not values"),
            "{source}\n=> {error:#}"
        );
    }
}

#[test]
fn parentheses_that_wrap_the_whole_expression_still_group_it() {
    let shell = UshCompiler::default()
        .compile_source("let x = \"x\"\nlet b = (x != \"y\")\nlet n = (1 + 2)\nlet t = (\"a\", \"b\")\nprint b\nprint n\nprint t.0\n")
        .expect("grouped expressions compile");
    let output = std::process::Command::new("sh")
        .args(["-c", &shell])
        .output()
        .expect("run sh");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "true\n3\na\n");
}
