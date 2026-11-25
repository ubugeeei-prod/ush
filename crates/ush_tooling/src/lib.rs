//! Editor-side helpers for `.ush` files.
//!
//! Three independent passes that the LSP, the `ush` CLI subcommands
//! (`ush format`, `ush check`), and the formatter dogfood gate
//! consume:
//!
//! - [`format_source`] — the formatter (idempotent rewriter).
//! - [`check_file`] / [`check_source_at`] / [`check_source`] — diagnostic extraction
//!   for `ush check` and LSP `textDocument/publishDiagnostics`.
//! - [`semantic_tokens`] + [`semantic_token_legend`] — the
//!   `textDocument/semanticTokens` payload.

mod completion;
mod diagnostic;
mod folding;
mod format;
mod highlight;
mod hover;
mod references;
mod semantic;
mod signature;
mod symbol;
mod token;

pub use self::{
    completion::{CompletionItem, CompletionKind, completions},
    diagnostic::{UshDiagnostic, check_file, check_source, check_source_at},
    folding::{FoldingRange, folding_ranges},
    format::format_source,
    highlight::{Highlight, HighlightKind, document_highlights},
    hover::{Hover, hover},
    references::{
        Reference, RenameError, definition, prepare_rename, references, rename_locations,
    },
    semantic::semantic_tokens,
    signature::{FunctionSignature, SignatureHelp, function_signatures, signature_help},
    symbol::{DocumentSymbol, SymbolKind, document_symbols},
    token::{SemanticToken, SemanticTokenKind, semantic_token_legend},
};

mod scope;
