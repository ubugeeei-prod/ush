use std::collections::HashMap;

use lsp_types::{
    CompletionItem as LspCompletionItem, CompletionItemKind, Diagnostic, DiagnosticSeverity,
    DocumentHighlight, DocumentHighlightKind, DocumentSymbol as LspDocumentSymbol,
    FoldingRange as LspFoldingRange, FoldingRangeKind, Hover as LspHover, HoverContents,
    MarkupContent, MarkupKind, ParameterInformation, ParameterLabel, Position, Range,
    SemanticToken, SemanticTokens, SignatureHelp as LspSignatureHelp, SignatureInformation,
    SymbolKind as LspSymbolKind, TextEdit, Uri, WorkspaceEdit,
};
use ush_tooling::{
    CompletionItem as UshCompletionItem, CompletionKind as UshCompletionKind,
    DocumentSymbol as UshDocumentSymbol, FoldingRange as UshFoldingRange, Highlight, HighlightKind,
    Hover as UshHover, Reference, SemanticToken as UshToken, SignatureHelp as UshSignatureHelp,
    SymbolKind as UshSymbolKind, UshDiagnostic,
};

pub fn diagnostics(source: &str, items: &[UshDiagnostic]) -> Vec<Diagnostic> {
    items
        .iter()
        .map(|item| Diagnostic {
            range: line_range(source, item.line),
            severity: Some(DiagnosticSeverity::ERROR),
            message: item.message.clone(),
            ..Diagnostic::default()
        })
        .collect()
}

pub fn full_document_edit(source: &str, formatted: &str) -> TextEdit {
    TextEdit {
        range: Range {
            start: Position::new(0, 0),
            end: end_position(source),
        },
        new_text: formatted.to_string(),
    }
}

pub fn semantic_tokens(source: &str, items: &[UshToken]) -> SemanticTokens {
    let mut prev_line = 0u32;
    let mut prev_start = 0u32;
    let mut data = Vec::new();

    for item in items {
        let start = utf16_column(source, item.line, item.start);
        let end = utf16_column(source, item.line, item.start + item.length);
        let delta_line = item.line - prev_line;
        let delta_start = if delta_line == 0 {
            start - prev_start
        } else {
            start
        };
        data.push(SemanticToken {
            delta_line,
            delta_start,
            length: end - start,
            token_type: item.kind.index(),
            token_modifiers_bitset: 0,
        });
        prev_line = item.line;
        prev_start = start;
    }

    SemanticTokens {
        result_id: None,
        data,
    }
}

fn line_range(source: &str, line: usize) -> Range {
    let text = source.lines().nth(line).unwrap_or("");
    Range {
        start: Position::new(line as u32, 0),
        end: Position::new(line as u32, text.encode_utf16().count() as u32),
    }
}

pub fn document_highlights(source: &str, items: &[Highlight]) -> Vec<DocumentHighlight> {
    items
        .iter()
        .map(|item| DocumentHighlight {
            range: Range {
                start: Position::new(item.line, utf16_column(source, item.line, item.start)),
                end: Position::new(
                    item.line,
                    utf16_column(source, item.line, item.start + item.length),
                ),
            },
            kind: Some(match item.kind {
                HighlightKind::Write => DocumentHighlightKind::WRITE,
                HighlightKind::Read => DocumentHighlightKind::READ,
                HighlightKind::Text => DocumentHighlightKind::TEXT,
            }),
        })
        .collect()
}

pub fn signature_help(help: UshSignatureHelp) -> LspSignatureHelp {
    let parameters = help
        .signature
        .parameters
        .iter()
        .map(|label| ParameterInformation {
            label: ParameterLabel::Simple(label.clone()),
            documentation: None,
        })
        .collect();
    LspSignatureHelp {
        signatures: vec![SignatureInformation {
            label: help.signature.label,
            documentation: None,
            parameters: Some(parameters),
            active_parameter: Some(help.active_parameter),
        }],
        active_signature: Some(0),
        active_parameter: Some(help.active_parameter),
    }
}

pub fn range_of_reference(source: &str, reference: &Reference) -> Range {
    Range {
        start: Position::new(
            reference.line,
            utf16_column(source, reference.line, reference.start),
        ),
        end: Position::new(
            reference.line,
            utf16_column(source, reference.line, reference.start + reference.length),
        ),
    }
}

pub fn rename_workspace_edit(
    source: &str,
    uri: &Uri,
    locations: &[Reference],
    new_name: &str,
) -> WorkspaceEdit {
    // Uri caches parse state internally; keys are never mutated after insertion.
    #[allow(clippy::mutable_key_type)]
    let mut changes: HashMap<Uri, Vec<TextEdit>> = HashMap::new();
    let edits: Vec<TextEdit> = locations
        .iter()
        .map(|reference| TextEdit {
            range: range_of_reference(source, reference),
            new_text: new_name.to_string(),
        })
        .collect();
    changes.insert(uri.clone(), edits);
    WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    }
}

pub fn hover(source: &str, item: UshHover) -> LspHover {
    LspHover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: item.contents,
        }),
        range: Some(Range {
            start: Position::new(item.line, utf16_column(source, item.line, item.start)),
            end: Position::new(
                item.line,
                utf16_column(source, item.line, item.start + item.length),
            ),
        }),
    }
}

pub fn folding_ranges(items: &[UshFoldingRange]) -> Vec<LspFoldingRange> {
    items
        .iter()
        .map(|item| LspFoldingRange {
            start_line: item.start_line,
            start_character: None,
            end_line: item.end_line,
            end_character: None,
            kind: Some(FoldingRangeKind::Region),
            collapsed_text: None,
        })
        .collect()
}

pub fn completion_items(items: &[UshCompletionItem]) -> Vec<LspCompletionItem> {
    items
        .iter()
        .map(|item| LspCompletionItem {
            label: item.label.clone(),
            kind: Some(match item.kind {
                UshCompletionKind::Keyword => CompletionItemKind::KEYWORD,
                UshCompletionKind::Variable => CompletionItemKind::VARIABLE,
                UshCompletionKind::Function => CompletionItemKind::FUNCTION,
                UshCompletionKind::Type => CompletionItemKind::CLASS,
            }),
            detail: if item.detail.is_empty() {
                None
            } else {
                Some(item.detail.clone())
            },
            ..LspCompletionItem::default()
        })
        .collect()
}

pub fn document_symbols(source: &str, items: &[UshDocumentSymbol]) -> Vec<LspDocumentSymbol> {
    items
        .iter()
        .map(|item| {
            let range = Range {
                start: Position::new(item.line, utf16_column(source, item.line, item.start)),
                end: Position::new(
                    item.line,
                    utf16_column(source, item.line, item.start + item.length),
                ),
            };
            #[allow(deprecated)]
            LspDocumentSymbol {
                name: item.name.clone(),
                detail: None,
                kind: match item.kind {
                    UshSymbolKind::Function => LspSymbolKind::FUNCTION,
                    UshSymbolKind::Enum => LspSymbolKind::ENUM,
                    UshSymbolKind::Trait => LspSymbolKind::INTERFACE,
                    UshSymbolKind::Type => LspSymbolKind::STRUCT,
                    UshSymbolKind::Variable => LspSymbolKind::VARIABLE,
                    UshSymbolKind::Alias => LspSymbolKind::CONSTANT,
                    UshSymbolKind::Module => LspSymbolKind::MODULE,
                },
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            }
        })
        .collect()
}

use positions::end_position;
pub use positions::{byte_character, utf16_column};
mod positions;
#[cfg(test)]
mod tests;
