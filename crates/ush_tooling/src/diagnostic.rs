use std::{fs, path::Path};

use anyhow::{Context, Result};
use ush_compiler::UshCompiler;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UshDiagnostic {
    pub line: usize,
    pub message: String,
}

pub fn check_file(path: &Path) -> Result<Vec<UshDiagnostic>> {
    let source =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(check_source(&source))
}

pub fn check_source(source: &str) -> Vec<UshDiagnostic> {
    match UshCompiler.compile_source(source) {
        Ok(_) => Vec::new(),
        // `{:#}` renders the whole context chain. Plain `Display`
        // shows only the outermost frame, which for most compiler
        // errors is just `line N` — no use to anyone reading it.
        Err(error) => vec![parse_diagnostic(&format!("{error:#}"))],
    }
}

fn parse_diagnostic(message: &str) -> UshDiagnostic {
    let trimmed = message.trim();
    let mut rest = trimmed;
    let mut line = None;

    // Nested contexts render as `line 3: line 7: detail`; the
    // innermost number is the most specific location, and everything
    // after the last one is the message worth showing.
    while let Some((number, tail)) = leading_line_number(rest) {
        line = Some(number);
        rest = tail;
    }

    UshDiagnostic {
        line: line.unwrap_or(1).saturating_sub(1),
        message: if rest.is_empty() {
            trimmed.to_string()
        } else {
            rest.to_string()
        },
    }
}

fn leading_line_number(value: &str) -> Option<(usize, &str)> {
    let rest = value.strip_prefix("line ")?;
    let end = rest.find(':')?;
    let number = rest[..end].trim().parse::<usize>().ok()?;
    Some((number, rest[end + 1..].trim_start()))
}

#[cfg(test)]
mod tests {
    use super::{check_source, parse_diagnostic};

    #[test]
    fn a_compile_error_keeps_the_explanation_not_just_the_line() {
        let diagnostics = check_source("let x = ");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].line, 0);
        assert_eq!(diagnostics[0].message, "empty expression");
    }

    #[test]
    fn every_kind_of_syntax_error_reports_what_went_wrong() {
        for (source, needle) in [
            ("fn f( {\n}\n", "fn name(args)"),
            ("match 1 {\n", "unterminated match expression"),
            ("print 2 * 3\n", "unsupported expression"),
            ("print unknown_var\n", "unknown variable"),
            ("unknown_fn()\n", "unknown function"),
        ] {
            let diagnostics = check_source(source);
            assert_eq!(diagnostics.len(), 1, "{source:?}");
            assert!(
                diagnostics[0].message.contains(needle),
                "{source:?} produced {:?}",
                diagnostics[0].message
            );
        }
    }

    #[test]
    fn the_innermost_line_number_wins() {
        let diagnostic = parse_diagnostic("line 2: line 7: unsupported expression: -");

        assert_eq!(diagnostic.line, 6);
        assert_eq!(diagnostic.message, "unsupported expression: -");
    }

    #[test]
    fn a_message_without_a_line_prefix_is_kept_whole() {
        let diagnostic = parse_diagnostic("unknown variable: value");

        assert_eq!(diagnostic.line, 0);
        assert_eq!(diagnostic.message, "unknown variable: value");
    }

    #[test]
    fn a_bare_line_prefix_still_leaves_a_message() {
        let diagnostic = parse_diagnostic("line 4:");

        assert_eq!(diagnostic.line, 3);
        assert_eq!(diagnostic.message, "line 4:");
    }

    #[test]
    fn line_numbers_are_reported_zero_based_for_editors() {
        assert_eq!(parse_diagnostic("line 1: oops").line, 0);
        assert_eq!(parse_diagnostic("line 12: oops").line, 11);
    }

    #[test]
    fn returns_empty_diagnostics_for_valid_program() {
        assert!(check_source("print \"ok\"").is_empty());
    }

    #[test]
    fn extracts_line_information_from_compile_errors() {
        let diagnostics = check_source("let value = missing.await");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].line, 0);
        assert!(diagnostics[0].message.contains("missing"));
    }
}
