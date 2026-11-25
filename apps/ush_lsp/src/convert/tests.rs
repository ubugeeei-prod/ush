use super::{diagnostics, full_document_edit};

#[test]
fn creates_a_full_document_edit() {
    let edit = full_document_edit("print \"a\"\n", "print \"b\"\n");
    assert_eq!(edit.range.end.line, 1);
    assert_eq!(edit.new_text, "print \"b\"\n");
}

#[test]
fn maps_diagnostics_to_ranges() {
    let diagnostics = diagnostics(
        "print \"ok\"\nprint missing\n",
        &[ush_tooling::UshDiagnostic {
            line: 1,
            message: "unknown variable".to_string(),
        }],
    );

    assert_eq!(diagnostics[0].range.start.line, 1);
}

#[test]
fn positions_use_utf16_not_unicode_scalar_or_byte_counts() {
    use lsp_types::Position;
    let source = "print \"😀日本\" + value";
    let start = source.find("value").unwrap() as u32;
    let units = source[..start as usize].encode_utf16().count() as u32;
    assert_eq!(super::utf16_column(source, 0, start), units);
    assert_eq!(
        super::byte_character(source, Position::new(0, units)),
        start
    );
    assert_eq!(
        full_document_edit(source, "").range.end.character,
        source.encode_utf16().count() as u32
    );
    let tokens = super::semantic_tokens(source, &ush_tooling::semantic_tokens(source));
    assert_eq!(
        tokens.data.iter().map(|t| t.delta_start).sum::<u32>(),
        units
    );
}
