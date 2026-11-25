//! `glob "…"` and `|` match patterns under every installed shell. A glob
//! is a distinct pattern form: a quoted string still matches literally.

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
    assert!(format!("{error:#}").contains(message), "{error:#}");
}

const CLASSIFY: &str = r#"fn kind(file: String) -> String {
  match file {
    "*.ts" => "a file literally named *.ts"
    glob "*.ts" | glob "*.mts" | glob "*.cts" => "typescript"
    glob "docs/*" => "docs"
    glob "my file?.txt" => "spaced"
    "Makefile" | "GNUmakefile" => "make"
    glob "[!a-z]*" => "not lowercase"
    glob "*[[:space:]]" => "trailing space"
    glob "it's $HOME \"quoted\"" => "special"
    _ => "other"
  }
}
"#;

#[test]
fn globs_alternatives_and_literals_pick_the_first_matching_arm() {
    let source = format!(
        "{CLASSIFY}for file in [\"*.ts\", \"src/a b.ts\", \"x.mts\", \"docs/a.md\", \"my file1.txt\", \"my file12.txt\", \"Readme\", \"tab \", \"it's $HOME \\\"quoted\\\"\", \"GNUmakefile\", \"x.tsx\"] {{\n  print kind $file\n}}\n"
    );
    succeeds(
        &source,
        concat!(
            "a file literally named *.ts\ntypescript\ntypescript\ndocs\nspaced\nother\n",
            "not lowercase\ntrailing space\nspecial\nmake\nother\n",
        ),
    );
}

#[test]
fn a_string_match_lowers_to_one_case_statement() {
    let shell = UshCompiler.compile_source(CLASSIFY).unwrap();
    assert!(shell.contains("*'.ts'|*'.mts'|*'.cts')"), "{shell}");
    assert!(shell.contains("'*.ts')"), "{shell}");
    assert!(shell.contains("'my file'?'.txt')"), "{shell}");
    assert!(shell.contains("'Makefile'|'GNUmakefile')"), "{shell}");
    assert!(!shell.contains("= '*.ts' ]"), "{shell}");
}

#[test]
fn alternatives_and_globs_work_inside_variant_patterns() {
    succeeds(
        r#"enum Event {
  Push(String)
  Tag(String)
  Close
}
fn describe(event: Event) -> String {
  match event {
    Event::Push(glob "release/*") | Event::Tag(glob "v*") => "release"
    Event::Push(_) | Event::Tag(_) => "change"
    Event::Close => "closed"
  }
}
print describe Event::Push("release/1.0")
print describe Event::Tag("v2")
print describe Event::Push("main")
print describe (Event::Close)
"#,
        "release\nrelease\nchange\nclosed\n",
    );
}

#[test]
fn exhaustiveness_counts_alternatives_and_the_universal_glob() {
    succeeds(
        "let ok = true\nmatch ok {\n  true | false => print \"covered\"\n}\nmatch \"x\" {\n  glob \"*\" => print \"any\"\n}\n",
        "covered\nany\n",
    );
    rejects(
        "match \"x\" {\n  glob \"*.ts\" => print \"ts\"\n}\n",
        "non-exhaustive `match` on String",
    );
}

#[test]
fn malformed_patterns_are_compile_errors() {
    rejects(
        "match \"x\" {\n  glob \"[abc\" => print \"a\"\n  _ => print \"b\"\n}\n",
        "unterminated `[`",
    );
    rejects(
        "match \"x\" {\n  glob \"[[:nope:]]\" => print \"a\"\n  _ => print \"b\"\n}\n",
        "unknown character class",
    );
    rejects(
        "let p = \"*\"\nmatch \"x\" {\n  glob p => print \"a\"\n  _ => print \"b\"\n}\n",
        "`glob` takes a string literal",
    );
    rejects(
        "match \"x\" {\n  \"a\" | name => print name\n  _ => print \"b\"\n}\n",
        "cannot bind a name",
    );
    rejects(
        "match 1 {\n  glob \"1*\" => print \"a\"\n  _ => print \"b\"\n}\n",
        "pattern/type mismatch",
    );
}
