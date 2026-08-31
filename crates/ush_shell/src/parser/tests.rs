mod aliases;
mod background;
mod fallback;
mod tokens;

use std::collections::BTreeMap;

use super::{ParsedLine, Stage, parse_line};

#[test]
fn parses_trailing_background_jobs_before_fallback() {
    let parsed = parse_line("sleep 1 &", &BTreeMap::new()).expect("parse");

    match parsed {
        ParsedLine::Background(source) => assert_eq!(source, "sleep 1"),
        other => panic!("expected background line, got {other:?}"),
    }
}

#[test]
fn keeps_boolean_and_as_posix_fallback() {
    let parsed = parse_line("true && false", &BTreeMap::new()).expect("parse");

    match parsed {
        ParsedLine::Fallback(source) => assert_eq!(source, "true && false"),
        other => panic!("expected fallback line, got {other:?}"),
    }
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
