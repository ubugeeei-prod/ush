# Architecture

This is a top-down map of the workspace: which crate owns what, how
they fit together at runtime, and where to look first when a bug
report lands.

Detailed per-feature notes live next to this file
([`lowering.md`](./lowering.md), [`typed-errors.md`](./typed-errors.md),
[`sourcemaps.md`](./sourcemaps.md), …). This page is the index.

## Crates at a glance

| Crate | Kind | Role |
| --- | --- | --- |
| `apps/ush` | binary | The user-facing CLI (`ush`). Owns argument parsing, config loading, panic hook, subcommands. Glue between everything else. |
| `apps/ush_lsp` | binary | Stdio LSP server (`ush_lsp`). Wraps `ush_tooling`. |
| `crates/ush_compiler` | library, `no_std + alloc` capable | `.ush → sh` compiler. Parser, AST, effects passes (typed errors, side effects, match exhaustiveness), codegen, sourcemap emission, pattern matching lowering, runtime support, stdlib lowering. |
| `crates/ush_shell` | library | The interactive shell: parser, dispatch, stylish renderers, helper pipelines (`json`, `xml`, `len`, …), built-ins, REPL bindings, vendored rustyline integration. |
| `crates/ush_config` | library | Config file lookup, profile / rc resolution, ush directory paths. |
| `crates/ush_tooling` | library | Formatter, diagnostics, semantic-token generation for the LSP. |
| `vendor/rustyline` | vendored 3rd party | Patched copy of `rustyline` with custom key bindings; pinned to the tag listed in [`vendor/rustyline/UPSTREAM`](../vendor/rustyline/UPSTREAM). |

## Request flow

The two main flows operators care about:

### Running a `.ush` script

```text
ush  ──parse args──>  apps/ush::cli
              │
              ▼
     read script source
              │
              ▼
   crates/ush_compiler::compile_file_with_sourcemap
              │
              │  parse → resolve imports → effects pass
              │  (match exhaustiveness etc.) → codegen
              ▼
      generated POSIX sh
              │
              ▼
       /bin/sh executes
```

### Running an interactive REPL

```text
ush  ──parse args──>  apps/ush::cli
              │
              ▼
     ush_shell::Shell::new
              │
              │  vendored rustyline reads stdin
              ▼
       ush_shell::parser
              │
              ▼
   either a built-in or an external command, plus
   any "stylish" rendering on top.
```

## Where things live

| Symptom or task | Start in |
| --- | --- |
| `.ush` parse error / wrong AST | `crates/ush_compiler/src/parse/` |
| Wrong generated sh / lowering bug | `crates/ush_compiler/src/codegen/` |
| Match arm not lowered correctly | `crates/ush_compiler/src/matching/` |
| Match exhaustiveness diagnostic missing or wrong | `crates/ush_compiler/src/matching/exhaustiveness.rs` |
| Error effect / `Problem!T` typing | `crates/ush_compiler/src/effects/` |
| Side-effect inference and `#[effects(...)]` checking | `crates/ush_compiler/src/effects/side.rs` |
| Sourcemap entry missing / pointing at wrong line | `crates/ush_compiler/src/sourcemap.rs` |
| Built-in (`cd`, `pwd`, `port`, …) behaves wrong | `crates/ush_shell/src/builtins/` |
| Job control / signals / pipelines | `crates/ush_shell/src/process/` |
| Structured helper (`len`, `lines`, `json`, …) bug | `crates/ush_shell/src/helpers/` |
| Stylish rendering issue | `crates/ush_shell/src/style/` |
| REPL key binding / completion bug | `crates/ush_shell/src/repl/` |
| Config / rc / profile lookup wrong | `crates/ush_config/` |
| Formatter (`ush format`) bug | `crates/ush_tooling/src/format.rs` |
| Diagnostic underline at the wrong span | `crates/ush_tooling/src/diagnostic.rs` |
| LSP request / response wire issue | `apps/ush_lsp/src/server.rs` |
| Installer (`install.sh`) bug | `install.sh` (+ `scripts/test_install.sh`) |
| Release archive layout bug | `.github/workflows/release.yml` (+ `Dockerfile`) |

## Stability layers

- **Stable public Rust API** — none. Every public symbol is
  `pub(crate)` or `pub` only inside the workspace.
- **Stable POSIX-`sh` lowering** — best-effort. The generated `sh`
  for every release is run through `cargo test --release` against
  `/bin/sh` on each of the four published targets before publishing,
  but the exact emitted code can change between releases.
- **Stable CLI surface** — covered by `apps/ush/tests/cli_smoke.rs`
  and `apps/ush_lsp/tests/cli_smoke.rs`. Removing or renaming a
  documented flag is a breaking change.
- **Stable LSP wire** — covered by
  `apps/ush_lsp/tests/initialize_handshake.rs`. Removing
  initialize / shutdown / exit support is a breaking change.

## Where new things should land

- **A new `.ush` language feature** → `crates/ush_compiler/`. Add
  a fixture under `examples/`; CI's `ush check` /
  `ush format --check` gate will pick it up automatically.
- **A new built-in** → `crates/ush_shell/src/builtins/`.
- **A new stylish renderer** → `crates/ush_shell/src/style/`.
- **A new helper pipeline operator** → `crates/ush_shell/src/helpers/`.
- **A new CLI flag** → `apps/ush/src/cli.rs` (with doc-comment so
  it shows up in `--help`).
- **A new LSP capability** → `apps/ush_lsp/src/server.rs`.
- **Anything CI-shaped** → `.github/workflows/` (and remember to
  pin third-party actions to a full commit SHA — see
  [`.github/CODEOWNERS`](../.github/CODEOWNERS) and the
  `## Security` block in the PR template).
