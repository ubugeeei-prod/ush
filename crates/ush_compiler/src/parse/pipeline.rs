//! `stage | stage |& stage`.

use anyhow::{Result, bail};

use super::expr::parse_expr;
use crate::{
    ast::{Pipeline, Stage},
    scan::split_pipeline,
    types::HeapVec as Vec,
    util::strip_top_level_suffix,
};

/// Parse `source` as a pipeline, or `None` when it has no top-level pipe.
///
/// A `?` after the last stage covers the whole pipeline. A `?` on any
/// other stage would suggest that the stage fails on its own, which it
/// cannot: every stage runs concurrently, and the pipeline fails as one.
pub(super) fn parse_pipeline(source: &str) -> Result<Option<Pipeline>> {
    let source = source.trim();
    // Inline shell keeps its own grammar, including unbalanced `case` parens.
    if !source.contains('|') || source.starts_with("$(") {
        return Ok(None);
    }
    let pieces = split_pipeline(source);
    if pieces.len() < 2 {
        return Ok(None);
    }
    let last = pieces.len() - 1;
    let mut stages = Vec::with_capacity(pieces.len());
    for (index, (text, merge_stderr)) in pieces.into_iter().enumerate() {
        let mut text = text.trim();
        if index == last
            && let Some(inner) = strip_top_level_suffix(text, '?')
        {
            text = inner;
        }
        if text.is_empty() {
            bail!("pipeline stage {} is empty", index + 1);
        }
        if strip_top_level_suffix(text, '?').is_some() {
            bail!("`?` covers the whole pipeline; write it once, after the last stage");
        }
        stages.push(Stage {
            expr: parse_expr(text)?,
            merge_stderr,
            text: text.into(),
        });
    }
    Ok(Some(Pipeline { stages }))
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use super::parse_pipeline;
    use crate::ast::{Expr, Pipeline};

    fn stages(source: &str) -> Pipeline {
        parse_pipeline(source).unwrap().expect("a pipeline")
    }

    fn call_name(expr: &Expr) -> &str {
        match expr {
            Expr::Call(call) => &call.name,
            Expr::Var(name) => name,
            other => panic!("expected a call, found {other:?}"),
        }
    }

    #[test]
    fn splits_stages_and_marks_stderr_merges() {
        let pipeline = stages(r#"cargo "build" |& tee "build.log" | wc "-l""#);
        let names: Vec<_> = pipeline.exprs().map(call_name).collect();
        assert_eq!(names, ["cargo", "tee", "wc"]);
        let merges: Vec<_> = pipeline.stages.iter().map(|s| s.merge_stderr).collect();
        assert_eq!(merges, [true, false, false]);
        assert_eq!(pipeline.stages[1].text, r#"tee "build.log""#);
    }

    #[test]
    fn nested_pipes_are_not_stage_separators() {
        for source in [
            r#"print "a | b""#,
            r#"grep 'x|y' "file""#,
            r#"f ("a" | "b")"#,
            r#"f ["a|b", "c"]"#,
            r#"Box { label: "a|b" }"#,
            "ready || done",
            "$(printf x | wc -c)",
            "\"\"\"\n  a | b\n\"\"\"",
        ] {
            assert!(parse_pipeline(source).unwrap().is_none(), "{source}");
        }
    }

    #[test]
    fn a_trailing_question_mark_covers_the_pipeline() {
        let pipeline = stages(r#"git "log" | head "-n1"?"#);
        assert!(matches!(pipeline.stages[1].expr, Expr::Call(_)));
        let error = parse_pipeline(r#"git "log"? | head "-n1""#).unwrap_err();
        assert!(format!("{error}").contains("after the last stage"));
    }

    #[test]
    fn empty_stages_are_rejected() {
        for source in [r#"git "log" |"#, r#"| head"#, r#"a | | b"#] {
            let error = parse_pipeline(source).unwrap_err();
            assert!(format!("{error}").contains("is empty"), "{source}");
        }
    }
}
