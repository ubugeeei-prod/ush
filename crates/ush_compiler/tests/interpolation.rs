//! `$"…"` and `$"""…"""` under every installed shell: they read `$name`,
//! `${name}`, `${expr}` and `$env::NAME` the way CLI-style words read
//! variables, values stay data whatever they contain, `"…"` stays literal,
//! and malformed or unsafe interpolations are compile errors.

mod support;

use support::each_program;
use ush_compiler::UshCompiler;

fn succeeds(source: &str, expected: &str) {
    each_program(source, |shell, _, (status, stdout, stderr)| {
        assert_eq!(status, 0, "{shell}: {stderr}");
        assert_eq!(stdout, expected, "{shell}");
    });
}

fn rejects(source: &str, message: &str) {
    let error = UshCompiler
        .compile_source(source)
        .expect_err("the program must not compile");
    let text = format!("{error:#}");
    assert!(text.contains(message), "{text}");
}

#[test]
fn holes_read_names_and_take_any_expression() {
    succeeds(
        r#"use std::string::join
type Config {
  owner: String
}
fn greet(name: String) -> String {
  "hello " + name
}
let side = "left"
let n = 41
let config = Config { owner: "ada" }
let items = ["a", "b"]
print $"ambient-$side.jsonl ambient-${side}.jsonl"
print $"${n + 1} ${n == 41} ${config.owner}"
print $"${greet "lin"} / ${greet (side + "!")}"
print $"[${join $items ", "}] [${items.join("|")}] ${items.len()}"
print $"${$"$side-$n"}"
"#,
        "ambient-left.jsonl ambient-left.jsonl\n42 true ada\nhello lin / hello left!\n[a, b] [a|b] 2\nleft-41\n",
    );
}

#[test]
fn hostile_values_stay_data() {
    succeeds(
        r#"let values = ["$HOME", "`echo pwned`", "$(echo pwned)", "a'b\"c", "*", "line1\nline2", "\\n", "${HOME}"]
for value in values {
  let wrapped = $"<$value>"
  print wrapped
  print wrapped == "<" + value + ">"
}
"#,
        concat!(
            "<$HOME>\ntrue\n<`echo pwned`>\ntrue\n<$(echo pwned)>\ntrue\n",
            "<a'b\"c>\ntrue\n<*>\ntrue\n<line1\nline2>\ntrue\n<\\n>\ntrue\n<${HOME}>\ntrue\n",
        ),
    );
}

#[test]
fn plain_and_raw_strings_stay_literal() {
    succeeds(
        r#"let name = "x"
print "${name} $name \$name"
print $"\${name} \$name"
print '${name}'
print """
  ${name}
"""
"#,
        "${name} $name \\$name\n${name} $name\n${name}\n${name}\n",
    );
}

#[test]
fn templates_dedent_interpolate_and_keep_escapes() {
    succeeds(
        r#"let rows = ["| a | 1 |", "| b | 2 |"]
let title = "Bench \"main\""
let summary = $"""
  ## $title

  | case | value |
  | --- | --- |
  ${rows.join("\n")}
  tab:\tend \${kept}
"""
print summary
"#,
        "## Bench \"main\"\n\n| case | value |\n| --- | --- |\n| a | 1 |\n| b | 2 |\ntab:\tend ${kept}\n",
    );
}

#[test]
fn only_scalars_interpolate() {
    rejects(
        "let xs = [\"a\"]\nprint $\"${xs}\"\n",
        "cannot interpolate a list",
    );
    rejects(
        "type P {\n  x: String\n}\nlet p = P { x: \"a\" }\nprint $\"$p\"\n",
        "cannot interpolate a P",
    );
}

#[test]
fn malformed_interpolations_name_the_problem() {
    rejects("print $\"a ${\"\n", "unterminated interpolation");
    rejects("print $\"a ${} b\"\n", "empty interpolation");
    rejects("print $\"cost: $5\"\n", "write `\\$` for a dollar sign");
    rejects("print $\"${HOME:-x}\"\n", "invalid interpolation `HOME:-x`");
    rejects("print $\"$missing\"\n", "unknown variable: missing");
}

#[test]
fn shell_source_and_literal_positions_refuse_interpolation() {
    rejects(
        "let cmd = \"ls\"\nshell $\"echo $cmd\"\n",
        "`shell` takes shell source, which cannot interpolate",
    );
    rejects(
        "let x = \"a\"\nalias l = $\"ls $x\"\n",
        "`alias` takes shell source",
    );
    rejects(
        "let x = \"a\"\nmatch x {\n  $\"$x\" => print \"same\"\n  _ => print \"other\"\n}\n",
        "match patterns are literal",
    );
}

#[test]
fn lowering_uses_parameter_expansion_without_eval() {
    let shell = UshCompiler
        .compile_source("let side = \"l\"\nlet n = 2\nprint $\"a-$side-${n + 1}\"\n")
        .unwrap();
    assert!(
        shell.contains(
            "printf '%s\\n' 'a-'\"${__ush_binding_1_side}\"'-'\"$((__ush_binding_2_n + 1))\""
        ),
        "{shell}"
    );
    assert!(!shell.contains("eval"), "{shell}");
}
