mod requests;

use anyhow::Result;
use lsp_server::{Connection, Message, Notification, Request, Response};
use lsp_types::{
    CompletionOptions, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DidSaveTextDocumentParams, FoldingRangeProviderCapability,
    HoverProviderCapability, OneOf, PublishDiagnosticsParams, RenameOptions, SemanticTokenType,
    SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions,
    SemanticTokensServerCapabilities, ServerCapabilities, SignatureHelpOptions,
    TextDocumentSyncCapability, TextDocumentSyncKind, WorkDoneProgressOptions,
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, DidSaveTextDocument,
        Notification as _,
    },
    request::{
        Completion, DocumentHighlightRequest, DocumentSymbolRequest, FoldingRangeRequest,
        Formatting, GotoDefinition, HoverRequest, PrepareRenameRequest, References, Rename,
        Request as _, SemanticTokensFullRequest, SignatureHelpRequest,
    },
};
use ush_tooling::{check_source, check_source_at, semantic_token_legend};

use crate::{convert, document::DocumentStore};

pub fn run(connection: Connection) -> Result<()> {
    connection.initialize(serde_json::to_value(capabilities())?)?;
    let mut docs = DocumentStore::default();

    for message in &connection.receiver {
        match message {
            Message::Request(request) => {
                if connection.handle_shutdown(&request)? {
                    return Ok(());
                }
                if let Err(error) = handle_request(&connection, &mut docs, request.clone()) {
                    connection.sender.send(Message::Response(Response::new_err(
                        request.id.clone(),
                        -32602,
                        error.to_string(),
                    )))?;
                }
            }
            Message::Notification(notification) => {
                if let Err(error) =
                    handle_notification(&connection, &mut docs, notification.clone())
                {
                    eprintln!("ush_lsp: {}: {error}", notification.method);
                }
            }
            Message::Response(_) => {}
        }
    }

    Ok(())
}

fn capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        document_formatting_provider: Some(OneOf::Left(true)),
        document_highlight_provider: Some(OneOf::Left(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        rename_provider: Some(OneOf::Right(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: WorkDoneProgressOptions::default(),
        })),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![":".into(), ".".into()]),
            ..CompletionOptions::default()
        }),
        signature_help_provider: Some(SignatureHelpOptions {
            trigger_characters: Some(vec!["(".to_string(), ",".to_string(), " ".into()]),
            retrigger_characters: None,
            work_done_progress_options: WorkDoneProgressOptions::default(),
        }),
        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
            SemanticTokensOptions {
                legend: SemanticTokensLegend {
                    token_types: semantic_token_legend()
                        .iter()
                        .map(|name| SemanticTokenType::new(name))
                        .collect(),
                    token_modifiers: Vec::new(),
                },
                full: Some(SemanticTokensFullOptions::Bool(true)),
                range: None,
                ..SemanticTokensOptions::default()
            },
        )),
        ..ServerCapabilities::default()
    }
}

fn handle_request(
    connection: &Connection,
    docs: &mut DocumentStore,
    request: Request,
) -> Result<()> {
    match request.method.as_str() {
        Formatting::METHOD => requests::formatting(connection, docs, request),
        SemanticTokensFullRequest::METHOD => requests::semantic_full(connection, docs, request),
        DocumentHighlightRequest::METHOD => requests::highlight(connection, docs, request),
        DocumentSymbolRequest::METHOD => requests::symbols(connection, docs, request),
        FoldingRangeRequest::METHOD => requests::folding(connection, docs, request),
        Completion::METHOD => requests::completion(connection, docs, request),
        HoverRequest::METHOD => requests::hover(connection, docs, request),
        GotoDefinition::METHOD => requests::definition(connection, docs, request),
        References::METHOD => requests::references(connection, docs, request),
        Rename::METHOD => requests::rename(connection, docs, request),
        PrepareRenameRequest::METHOD => requests::prepare_rename(connection, docs, request),
        SignatureHelpRequest::METHOD => requests::signature_help(connection, docs, request),
        _ => {
            connection.sender.send(Message::Response(Response::new_err(
                request.id,
                -32601,
                "method not supported".to_string(),
            )))?;
            Ok(())
        }
    }
}

fn handle_notification(
    connection: &Connection,
    docs: &mut DocumentStore,
    notification: Notification,
) -> Result<()> {
    match notification.method.as_str() {
        DidOpenTextDocument::METHOD => {
            let params: DidOpenTextDocumentParams = serde_json::from_value(notification.params)?;
            let uri = params.text_document.uri;
            let text = params.text_document.text;
            docs.open(uri.clone(), text.clone(), params.text_document.version);
            publish_diagnostics(connection, &uri, &text, docs.version(&uri))
        }
        DidChangeTextDocument::METHOD => {
            let params: DidChangeTextDocumentParams = serde_json::from_value(notification.params)?;
            let uri = params.text_document.uri;
            if docs.update(&uri, params.text_document.version, params.content_changes)? {
                return publish_diagnostics(
                    connection,
                    &uri,
                    &docs.read(&uri)?,
                    docs.version(&uri),
                );
            }
            Ok(())
        }
        DidSaveTextDocument::METHOD => {
            let params: DidSaveTextDocumentParams = serde_json::from_value(notification.params)?;
            let uri = params.text_document.uri;
            let text = docs.read(&uri)?;
            publish_diagnostics(connection, &uri, &text, docs.version(&uri))
        }
        DidCloseTextDocument::METHOD => {
            let params: DidCloseTextDocumentParams = serde_json::from_value(notification.params)?;
            docs.close(&params.text_document.uri);
            publish_diagnostics(connection, &params.text_document.uri, "", None)
        }
        _ => Ok(()),
    }
}

fn publish_diagnostics(
    connection: &Connection,
    uri: &lsp_types::Uri,
    source: &str,
    version: Option<i32>,
) -> Result<()> {
    let params = PublishDiagnosticsParams {
        uri: uri.clone(),
        // A file on disk can import modules from beside it.
        diagnostics: convert::diagnostics(
            source,
            &match crate::paths::file_path(uri) {
                Some(path) => check_source_at(source, &path),
                None => check_source(source),
            },
        ),
        version,
    };
    connection
        .sender
        .send(Message::Notification(Notification::new(
            "textDocument/publishDiagnostics".to_string(),
            params,
        )))?;
    Ok(())
}
