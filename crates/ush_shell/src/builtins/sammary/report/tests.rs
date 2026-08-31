use std::path::Path;

use super::{
    SummaryRow, count_lines, is_lock_file, render_plain, render_stylish, summarize_types, type_name,
};
use crate::style::strip_ansi;

fn row(path: &str, kind: &str, lines: usize, bytes: u64) -> SummaryRow {
    SummaryRow {
        path: path.to_string(),
        kind: kind.to_string(),
        lines,
        bytes,
    }
}

#[test]
fn the_type_of_a_file_is_its_extension() {
    assert_eq!(type_name(Path::new("src/main.rs")), "rs");
    assert_eq!(type_name(Path::new("README.md")), "md");
    assert_eq!(type_name(Path::new("archive.tar.gz")), "gz");
}

#[test]
fn files_without_an_extension_are_grouped_under_none() {
    assert_eq!(type_name(Path::new("Makefile")), "(none)");
    assert_eq!(type_name(Path::new("src/LICENSE")), "(none)");
    assert_eq!(type_name(Path::new(".gitignore")), "(none)");
}

#[test]
fn lock_files_get_their_own_type() {
    assert_eq!(type_name(Path::new("Cargo.lock")), "lock");
    assert_eq!(type_name(Path::new("a/package-lock.json")), "lock");
}

#[test]
fn every_known_lockfile_name_is_recognized() {
    for name in [
        "Cargo.lock",
        "Gemfile.lock",
        "Podfile.lock",
        "Pipfile.lock",
        "poetry.lock",
        "composer.lock",
        "package-lock.json",
        "pnpm-lock.yaml",
        "yarn.lock",
        "bun.lock",
        "bun.lockb",
        "uv.lock",
        "custom.lock",
    ] {
        assert!(is_lock_file(Path::new(name)), "{name}");
    }
}

#[test]
fn ordinary_files_are_not_lockfiles() {
    assert!(!is_lock_file(Path::new("src/main.rs")));
    assert!(!is_lock_file(Path::new("lock")));
    assert!(!is_lock_file(Path::new("locked.rs")));
    assert!(!is_lock_file(Path::new("")));
}

#[test]
fn line_counting_treats_a_trailing_newline_as_a_terminator() {
    assert_eq!(count_lines(b""), 0);
    assert_eq!(count_lines(b"a"), 1);
    assert_eq!(count_lines(b"a\n"), 1);
    assert_eq!(count_lines(b"a\nb"), 2);
    assert_eq!(count_lines(b"a\nb\n"), 2);
    assert_eq!(count_lines(b"\n"), 1);
    assert_eq!(count_lines(b"\n\n"), 2);
}

#[test]
fn line_counting_works_on_binary_content() {
    assert_eq!(count_lines(&[0x00, 0x0a, 0xff]), 2);
}

#[test]
fn types_are_grouped_and_sorted_by_name() {
    let types = summarize_types(&[
        row("b.rs", "rs", 10, 100),
        row("a.md", "md", 5, 50),
        row("a.rs", "rs", 20, 200),
    ]);

    assert_eq!(
        types,
        vec![("md".to_string(), 1, 5, 50), ("rs".to_string(), 2, 30, 300),]
    );
}

#[test]
fn grouping_an_empty_row_set_yields_no_types() {
    assert!(summarize_types(&[]).is_empty());
}

#[test]
fn the_plain_report_is_tab_separated_with_totals() {
    let rows = [row("a.rs", "rs", 3, 30), row("b.md", "md", 2, 20)];
    let types = summarize_types(&rows);
    let rendered = render_plain(&rows, &types, 5, 50);

    assert_eq!(
        rendered,
        "lines\tbytes\tpath\n\
         3\t30\ta.rs\n\
         2\t20\tb.md\n\
         5\t50\tTOTAL (2 files)\n\
         \n\
         type\tfiles\tlines\tbytes\n\
         md\t1\t2\t20\n\
         rs\t1\t3\t30\n\
         TOTAL\t2\t5\t50\n"
    );
}

#[test]
fn the_plain_report_still_renders_its_headers_when_empty() {
    let rendered = render_plain(&[], &[], 0, 0);
    assert!(rendered.starts_with("lines\tbytes\tpath\n"));
    assert!(rendered.contains("0\t0\tTOTAL (0 files)\n"));
    assert!(rendered.ends_with("TOTAL\t0\t0\t0\n"));
}

#[test]
fn the_stylish_report_summarizes_files_then_types() {
    let rows = [row("a.rs", "rs", 3, 30)];
    let types = summarize_types(&rows);
    let rendered = strip_ansi(&render_stylish(&rows, &types, 3, 30));

    assert!(rendered.starts_with("sammary 1 file, 3 lines, 30 B\n"));
    assert!(rendered.contains("\nfiles\na.rs [rs] 3 lines, 30 B\n"));
    assert!(rendered.contains("\ntypes\nrs 1 file, 3 lines, 30 B\n"));
}

#[test]
fn the_stylish_report_pluralizes_counts() {
    let rows = [row("a.rs", "rs", 3, 30), row("b.rs", "rs", 1, 10)];
    let types = summarize_types(&rows);
    let rendered = strip_ansi(&render_stylish(&rows, &types, 4, 40));

    assert!(rendered.starts_with("sammary 2 files, 4 lines, 40 B\n"));
    assert!(rendered.contains("b.rs [rs] 1 line, 10 B\n"));
}
