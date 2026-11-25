//! Editor support for command status, background commands and `defer`.

use ush_tooling::{SemanticTokenKind, completions, format_source, hover, semantic_tokens};

const SOURCE: &str = r#"use $bin::*
use std::io::discard_stderr

let server = spawn python3 "-m" "http.server"
defer server.kill()
defer {
  print "cleanup"
}
if ! git "cat-file" "-e" "HEAD:x" | discard_stderr {
  allow_fail git "fetch"
}
let code = status_of cargo "test"
"#;

fn keywords(source: &str) -> Vec<String> {
    let lines: Vec<&str> = source.lines().collect();
    semantic_tokens(source)
        .into_iter()
        .filter(|token| token.kind == SemanticTokenKind::Keyword)
        .map(|token| {
            let line = lines[token.line as usize];
            line[token.start as usize..(token.start + token.length) as usize].to_string()
        })
        .collect()
}

#[test]
fn the_new_forms_are_highlighted_as_keywords() {
    let keywords = keywords(SOURCE);
    for keyword in ["spawn", "defer", "allow_fail", "status_of"] {
        assert!(
            keywords.iter().any(|k| k == keyword),
            "{keyword}: {keywords:?}"
        );
    }
}

#[test]
fn the_formatter_indents_defer_blocks_and_keeps_negated_conditions() {
    let messy = "defer {\nprint \"a\"\n}\nif ! git \"diff\" {\nallow_fail rm \"x\"\n}\n";
    let formatted = format_source(messy);
    assert!(
        formatted.contains("defer {\n  print \"a\"\n}\n"),
        "{formatted}"
    );
    assert!(
        formatted.contains("if ! git \"diff\" {\n  allow_fail rm \"x\"\n}"),
        "{formatted}"
    );
    assert_eq!(format_source(SOURCE), SOURCE);
    assert_eq!(format_source(&formatted), formatted);
}

#[test]
fn completion_and_hover_explain_the_keywords() {
    let labels: Vec<String> = completions("").into_iter().map(|c| c.label).collect();
    for keyword in ["defer", "status_of", "allow_fail", "spawn"] {
        assert!(labels.iter().any(|label| label == keyword), "{keyword}");
    }
    let text = hover(SOURCE, 4, 1).expect("hover on defer").contents;
    assert!(text.contains("returns or the script exits"), "{text}");
}
