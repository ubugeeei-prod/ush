//! Runtime behaviour of values, functions, and typed errors.

mod support;

use ush_compiler::UshCompiler;

use support::{run, try_run};

#[test]
fn integer_addition_chains_left_to_right() {
    let output = run("print 1 + 2 + 3\nlet base = 10\nprint base + 5\n");
    assert_eq!(output, "6\n15\n");
}

#[test]
fn arithmetic_beyond_addition_is_reported_as_unsupported() {
    // `+` is the only arithmetic operator the prototype lowers today;
    // the point of this test is that the rest fail loudly at compile
    // time rather than emitting broken shell.
    for source in ["print 2 * 3\n", "print 4 - 1\n", "print 8 / 2\n"] {
        let error = UshCompiler::default()
            .compile_source(source)
            .expect_err("expected a compile error")
            .to_string();
        assert!(error.contains("line 1"), "{error}");
    }
}

#[test]
fn integer_comparisons_render_as_booleans() {
    let output = run("print 7 > 3\nprint 3 > 7\nprint 3 == 3\nprint 3 != 3\n");
    assert_eq!(output, "true\nfalse\ntrue\nfalse\n");
}

#[test]
fn strings_compare_lexicographically() {
    let output = run("print \"ant\" < \"bee\"\nprint \"bee\" < \"ant\"\n");
    assert_eq!(output, "true\nfalse\n");
}

#[test]
fn booleans_compare_by_value() {
    let output = run("print true != false\nprint true == true\n");
    assert_eq!(output, "true\ntrue\n");
}

#[test]
fn string_concatenation_mixes_types() {
    let output = run(r#"
        let name = "ush"
        let count = 3
        print "hi " + name
        print name + ":" + count
        print "" + count
    "#);

    assert_eq!(output, "hi ush\nush:3\n3\n");
}

#[test]
fn strings_keep_shell_metacharacters_literal() {
    let output = run(r#"
        print "a $HOME b"
        print "back`tick`"
        print "dollar-paren $(id)"
        print "single ' quote"
    "#);

    assert_eq!(
        output,
        "a $HOME b\nback`tick`\ndollar-paren $(id)\nsingle ' quote\n"
    );
}

#[test]
fn functions_return_their_tail_expression() {
    let output = run(r#"
        fn greet(name: String) -> String {
          "hi " + name
        }
        print greet "ush"
    "#);

    assert_eq!(output, "hi ush\n");
}

#[test]
fn an_explicit_return_ends_a_function_early() {
    let output = run(r#"
        fn classify(value: Int) -> String {
          if value < 0 {
            return "negative"
          }
          "non-negative"
        }
        print classify -1
        print classify 1
    "#);

    assert_eq!(output, "negative\nnon-negative\n");
}

#[test]
fn calls_compose_through_the_dollar_pipeline_form() {
    let output = run(r#"
        fn greet(name: String) -> String {
          "hi " + name
        }
        fn wrap(message: String) -> String {
          "<" + message + ">"
        }
        fn label() -> String {
          "ush"
        }
        print $ wrap (greet (label ()))
    "#);

    assert_eq!(output, "<hi ush>\n");
}

#[test]
fn functions_can_recurse() {
    let output = run(r#"
        fn countup(value: Int) -> String {
          if value == 3 {
            "liftoff"
          }
          else {
            countup (value + 1)
          }
        }
        print countup 0
    "#);

    assert_eq!(output, "liftoff\n");
}

#[test]
fn a_raised_error_propagates_through_the_question_mark() {
    let (status, stdout, _) = try_run(
        r#"
        enum Problem {
          MissingConfig,
        }
        fn load() -> Problem!String {
          raise Problem::MissingConfig
        }
        fn run() -> Problem!String {
          let value = load()?
          value
        }
        run()?
        print "unreachable"
    "#,
    );

    assert_ne!(status, 0);
    assert_eq!(stdout, "");
}

#[test]
fn a_successful_fallible_call_keeps_going() {
    let output = run(r#"
        enum Problem {
          MissingConfig,
        }
        fn load() -> Problem!String {
          "config"
        }
        fn run() -> Problem!String {
          let value = load()?
          "<" + value + ">"
        }
        print $ run()?
    "#);

    assert_eq!(output, "<config>\n");
}

#[test]
fn inline_shell_escapes_run_through_bin_sh() {
    let output = run("$ printf '%s\\n' hi\n$ printf '%s\\n' there\n");
    assert_eq!(output, "hi\nthere\n");
}

#[test]
fn a_shell_command_can_be_built_from_a_value() {
    let output = run(r#"
        let command = "printf '%s\n' from-shell"
        shell command
        print "after-shell"
    "#);

    assert_eq!(output, "from-shell\nafter-shell\n");
}

#[test]
fn multiline_string_literals_keep_their_line_breaks() {
    let output = run("let block = \"\"\"\n  first\n  second\n\"\"\"\nprint block\n");

    assert!(output.contains("first"));
    assert!(output.contains("second"));
}
