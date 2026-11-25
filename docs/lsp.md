# ush_lsp

`ush_lsp` is a stdio Language Server Protocol server for `.ush` files.
Release archives and the install script place it next to `ush`, so an editor can
usually invoke `ush_lsp` directly after installation.

## Editor installation

```sh
ush editor install vscode
ush editor install cursor
ush editor install vscodium
# Export an offline VSIX or use an editor command outside PATH:
ush editor install --output ush.vsix
ush editor install cursor --cli /path/to/cursor
```

The complete extension is embedded in `ush`; installation needs no npm,
network download or marketplace account. The package uses the adjacent
`ush_lsp` executable when present. `ush.server.path` overrides it in the editor.
TextMate highlighting works before the server starts; semantic tokens refine
function names. The extension activates only in trusted workspaces.

## Capabilities

Diagnostics, formatting, semantic highlighting, completion, hover, definition,
references, rename, signature help, document symbols and folding are available.
The server accepts incremental or full changes, tracks document versions,
invalidates cached tokens on edits and clears diagnostics when documents close.
All protocol positions use UTF-16, including Japanese text and emoji. Invalid
requests return JSON-RPC errors without terminating the server.

Navigation distinguishes lexical bindings, function parameters, nested blocks
and comprehensions. Renaming rejects keywords and existing bound names to avoid
capture. Rename in documents containing `match` or `if let` patterns is conservatively
rejected until pattern binding resolution is available. `includeDeclaration` is honored for references. Signature help handles
generics, nested parameter types, parentheses and shell-style consecutive arguments.

The index is deliberately tolerant of incomplete programs. It currently works
within each document; cross-file workspace symbols, type-directed member completion,
inlay hints and call hierarchies are not implemented. Unknown or ambiguous symbols
are left unresolved rather than renamed speculatively.

## Invocation

For an installed release:

```bash
ush_lsp
```

From a source checkout:

```bash
cargo run -p ush_lsp
```

The server speaks LSP over stdin/stdout. Do not wrap it in `ush -c` or another
shell command that writes prompts or extra output to stdout.

## Neovim Example

```lua
vim.lsp.start({
  name = "ush",
  cmd = { "ush_lsp" },
  root_dir = vim.fs.root(0, { "Cargo.toml", ".git" }),
})
```

## VS Code-Compatible Clients

Configure a stdio language server command:

```json
{
  "command": "ush_lsp",
  "args": [],
  "filetypes": ["ush"]
}
```

If you installed from source and have not copied the binary onto `PATH`, use the
absolute path to `target/release/ush_lsp` or run it through your editor's normal
Cargo task integration.
