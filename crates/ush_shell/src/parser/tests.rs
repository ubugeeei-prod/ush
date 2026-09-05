mod aliases;
mod background;
mod fallback;
mod tokens;

use std::collections::BTreeMap;

use super::{Connector, ParsedLine, Stage, parse_line};

#[test]
fn parses_trailing_background_jobs_before_fallback() {
    let parsed = parse_line("sleep 1 &", &BTreeMap::new()).expect("parse");

    match parsed {
        ParsedLine::Background(source) => assert_eq!(source, "sleep 1"),
        other => panic!("expected background line, got {other:?}"),
    }
}

#[test]
fn splits_boolean_and_into_an_and_or_list() {
    let parsed = parse_line("true && false", &BTreeMap::new()).expect("parse");

    let ParsedLine::List(items) = parsed else {
        panic!("expected an and-or list");
    };
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].connector, Connector::Always);
    assert_eq!(items[1].connector, Connector::And);
    assert!(matches!(items[1].line, ParsedLine::Pipeline(_)));
}

#[test]
fn keeps_compound_commands_whole_for_posix_fallback() {
    // `;` separates the *inside* of an `if`, so splitting on it
    // would hand `/bin/sh` fragments that do not parse on their own.
    let parsed = parse_line("if true; then echo hi; fi", &BTreeMap::new()).expect("parse");

    match parsed {
        ParsedLine::Fallback(source) => assert_eq!(source, "if true; then echo hi; fi"),
        other => panic!("expected fallback line, got {other:?}"),
    }
}

#[test]
fn leaves_quoted_operators_alone() {
    let parsed = parse_line("echo 'a && b'", &BTreeMap::new()).expect("parse");

    assert!(matches!(parsed, ParsedLine::Pipeline(_)));
}

#[test]
fn treats_a_newline_as_a_command_separator() {
    let parsed = parse_line("echo one\necho two", &BTreeMap::new()).expect("parse");

    let ParsedLine::List(items) = parsed else {
        panic!("expected an and-or list");
    };
    assert_eq!(items.len(), 2);
    assert!(items.iter().all(|item| item.connector == Connector::Always));
}

#[test]
fn alias_expansion_preserves_quoted_suffix_arguments() {
    let aliases = BTreeMap::from([("gm".to_string(), "git commit -m".to_string())]);
    let parsed = parse_line("gm 'simplify readme'", &aliases).expect("parse");

    match parsed {
        ParsedLine::Pipeline(pipeline) => match &pipeline.stages[0] {
            Stage::External(spec) => {
                assert_eq!(spec.raw, "git commit -m 'simplify readme'");
                assert_eq!(spec.command, "git");
                assert_eq!(
                    spec.args,
                    vec![
                        "commit".to_string(),
                        "-m".to_string(),
                        "simplify readme".to_string()
                    ]
                );
            }
            other => panic!("expected external stage, got {other:?}"),
        },
        other => panic!("expected pipeline, got {other:?}"),
    }
}

#[test]
fn alias_expansion_runs_after_leading_assignments() {
    let aliases = BTreeMap::from([("gm".to_string(), "git commit -m".to_string())]);
    let parsed = parse_line("EDITOR=vim gm 'simplify readme'", &aliases).expect("parse");

    match parsed {
        ParsedLine::Pipeline(pipeline) => match &pipeline.stages[0] {
            Stage::External(spec) => {
                assert_eq!(spec.raw, "EDITOR=vim git commit -m 'simplify readme'");
                assert_eq!(
                    spec.assignments,
                    vec![("EDITOR".to_string(), "vim".to_string())]
                );
                assert_eq!(spec.command, "git");
                assert_eq!(
                    spec.args,
                    vec![
                        "commit".to_string(),
                        "-m".to_string(),
                        "simplify readme".to_string()
                    ]
                );
            }
            other => panic!("expected external stage, got {other:?}"),
        },
        other => panic!("expected pipeline, got {other:?}"),
    }
}

#[test]
fn quoted_command_word_does_not_trigger_alias_expansion() {
    let aliases = BTreeMap::from([("gm".to_string(), "git commit -m".to_string())]);
    let parsed = parse_line("'gm' 'simplify readme'", &aliases).expect("parse");

    match parsed {
        ParsedLine::Pipeline(pipeline) => match &pipeline.stages[0] {
            Stage::External(spec) => {
                assert_eq!(spec.raw, "'gm' 'simplify readme'");
                assert_eq!(spec.command, "gm");
                assert_eq!(spec.args, vec!["simplify readme".to_string()]);
            }
            other => panic!("expected external stage, got {other:?}"),
        },
        other => panic!("expected pipeline, got {other:?}"),
    }
}
