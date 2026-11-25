use super::requests::respond_ok;
use crate::{convert, document::DocumentStore};
use anyhow::Result;
use lsp_server::{Connection, Request};
use lsp_types::CompletionParams;

pub(super) fn completion(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    let params: CompletionParams = serde_json::from_value(request.params)?;
    let source = docs.read(&params.text_document_position.text_document.uri)?;
    let pos = params.text_document_position.position;
    let items = convert::completion_items(&ush_tooling::completions_at(
        &source,
        pos.line,
        convert::byte_character(&source, pos),
    ));
    respond_ok(connection, request.id, serde_json::to_value(items)?)
}
