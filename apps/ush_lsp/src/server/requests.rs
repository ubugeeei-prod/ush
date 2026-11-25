use anyhow::Result;
use lsp_server::{Connection, Message, Request, Response};
use lsp_types::{
    CompletionParams, DocumentFormattingParams, DocumentHighlightParams, DocumentSymbolParams,
    DocumentSymbolResponse, FoldingRangeParams, GotoDefinitionParams, GotoDefinitionResponse,
    HoverParams, Location, ReferenceParams, RenameParams, SemanticTokensParams,
    SignatureHelpParams, TextDocumentPositionParams,
};
use ush_tooling::{
    completions, definition as ush_definition, document_highlights, document_symbols,
    folding_ranges, format_source, hover as ush_hover, prepare_rename as ush_prepare_rename,
    references as ush_refs, rename_locations, signature_help as ush_signature_help,
};

use crate::{convert, document::DocumentStore};

pub(super) fn formatting(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    let params: DocumentFormattingParams = serde_json::from_value(request.params)?;
    let source = docs.read(&params.text_document.uri)?;
    let formatted = format_source(&source);
    let edits = if source.as_ref() == formatted {
        Vec::new()
    } else {
        vec![convert::full_document_edit(&source, &formatted)]
    };
    respond_ok(connection, request.id, serde_json::to_value(edits)?)
}

pub(super) fn semantic_full(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    let params: SemanticTokensParams = serde_json::from_value(request.params)?;
    let source = docs.read(&params.text_document.uri)?;
    let tokens = convert::semantic_tokens(&source, &docs.tokens(&params.text_document.uri)?);
    respond_ok(connection, request.id, serde_json::to_value(tokens)?)
}

pub(super) fn highlight(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    let params: DocumentHighlightParams = serde_json::from_value(request.params)?;
    let source = docs.read(&params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position;
    let highlights = convert::document_highlights(
        &source,
        &document_highlights(
            &source,
            position.line,
            convert::byte_character(&source, position),
        ),
    );
    respond_ok(connection, request.id, serde_json::to_value(highlights)?)
}

pub(super) fn symbols(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    let params: DocumentSymbolParams = serde_json::from_value(request.params)?;
    let source = docs.read(&params.text_document.uri)?;
    let response = DocumentSymbolResponse::Nested(convert::document_symbols(
        &source,
        &document_symbols(&source),
    ));
    respond_ok(connection, request.id, serde_json::to_value(response)?)
}

pub(super) fn folding(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    let params: FoldingRangeParams = serde_json::from_value(request.params)?;
    let source = docs.read(&params.text_document.uri)?;
    let ranges = convert::folding_ranges(&folding_ranges(&source));
    respond_ok(connection, request.id, serde_json::to_value(ranges)?)
}

pub(super) fn completion(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    let params: CompletionParams = serde_json::from_value(request.params)?;
    let source = docs.read(&params.text_document_position.text_document.uri)?;
    let items = convert::completion_items(&completions(&source));
    respond_ok(connection, request.id, serde_json::to_value(items)?)
}

pub(super) fn hover(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    let params: HoverParams = serde_json::from_value(request.params)?;
    let source = docs.read(&params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position;
    let result = ush_hover(
        &source,
        position.line,
        convert::byte_character(&source, position),
    )
    .map(|item| convert::hover(&source, item));
    respond_ok(connection, request.id, serde_json::to_value(result)?)
}

pub(super) fn definition(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    let params: GotoDefinitionParams = serde_json::from_value(request.params)?;
    let uri = params
        .text_document_position_params
        .text_document
        .uri
        .clone();
    let source = docs.read(&uri)?;
    let position = params.text_document_position_params.position;
    let result = ush_definition(
        &source,
        position.line,
        convert::byte_character(&source, position),
    )
    .map(|reference| {
        GotoDefinitionResponse::Scalar(Location {
            uri: uri.clone(),
            range: convert::range_of_reference(&source, &reference),
        })
    });
    respond_ok(connection, request.id, serde_json::to_value(result)?)
}

pub(super) fn references(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    let params: ReferenceParams = serde_json::from_value(request.params)?;
    let uri = params.text_document_position.text_document.uri.clone();
    let source = docs.read(&uri)?;
    let position = params.text_document_position.position;
    let locations: Vec<Location> = ush_refs(
        &source,
        position.line,
        convert::byte_character(&source, position),
    )
    .into_iter()
    .filter(|reference| {
        params.context.include_declaration
            || Some(*reference)
                != ush_definition(
                    &source,
                    position.line,
                    convert::byte_character(&source, position),
                )
    })
    .map(|reference| Location {
        uri: uri.clone(),
        range: convert::range_of_reference(&source, &reference),
    })
    .collect();
    respond_ok(connection, request.id, serde_json::to_value(locations)?)
}

pub(super) fn signature_help(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    let params: SignatureHelpParams = serde_json::from_value(request.params)?;
    let source = docs.read(&params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position;
    let result = ush_signature_help(
        &source,
        position.line,
        convert::byte_character(&source, position),
    )
    .map(convert::signature_help);
    respond_ok(connection, request.id, serde_json::to_value(result)?)
}

pub(super) fn prepare_rename(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    let params: TextDocumentPositionParams = serde_json::from_value(request.params)?;
    let source = docs.read(&params.text_document.uri)?;
    let result = ush_prepare_rename(
        &source,
        params.position.line,
        convert::byte_character(&source, params.position),
    )
    .map(|reference| convert::range_of_reference(&source, &reference));
    respond_ok(connection, request.id, serde_json::to_value(result)?)
}

pub(super) fn rename(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    let params: RenameParams = serde_json::from_value(request.params)?;
    let uri = params.text_document_position.text_document.uri.clone();
    let source = docs.read(&uri)?;
    let position = params.text_document_position.position;
    match rename_locations(
        &source,
        position.line,
        convert::byte_character(&source, position),
        &params.new_name,
    ) {
        Ok(locations) => {
            let edit = convert::rename_workspace_edit(&source, &uri, &locations, &params.new_name);
            respond_ok(connection, request.id, serde_json::to_value(edit)?)
        }
        Err(error) => {
            connection.sender.send(Message::Response(Response::new_err(
                request.id,
                -32602,
                format!("cannot rename to `{}`: {error:?}", params.new_name),
            )))?;
            Ok(())
        }
    }
}

fn respond_ok(
    connection: &Connection,
    id: lsp_server::RequestId,
    value: serde_json::Value,
) -> Result<()> {
    connection
        .sender
        .send(Message::Response(Response::new_ok(id, value)))?;
    Ok(())
}
