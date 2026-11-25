//! Diagnostics for a buffer that imports local modules.

use std::fs;

use ush_tooling::{check_source, check_source_at};

#[test]
fn module_errors_land_on_the_importing_line() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join("lib")).unwrap();
    fs::write(
        dir.path().join("lib/text.ush"),
        "pub fn shout(text: String) -> String {\n  text + missing\n}\n",
    )
    .unwrap();
    let path = dir.path().join("main.ush");
    fs::write(&path, "print 1\n").unwrap();

    // The buffer, not the file on disk, is what gets checked.
    let buffer = "print 1\nmod \"lib/text.ush\"\nprint text::shout \"a\"\n";
    let diagnostics = check_source_at(buffer, &path);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].line, 1);
    // Editors pass absolute paths, so the module is named absolutely.
    let message = &diagnostics[0].message;
    assert!(message.starts_with('/'), "{message}");
    assert!(
        message.ends_with("/lib/text.ush: line 2: unknown variable: missing"),
        "{message}"
    );

    let fixed = "print 1\nmod \"lib/text.ush\"\n";
    assert!(check_source_at(fixed, &path).is_empty());
}

#[test]
fn a_buffer_without_a_file_cannot_import() {
    let diagnostics = check_source("mod \"lib/text.ush\"\n");
    assert_eq!(diagnostics[0].line, 0);
    assert!(diagnostics[0].message.contains("no directory"));
}
