# Changelog

All notable changes to `ush` will be documented in this file.

The format is based on [Keep a Changelog][keepachangelog], and this
project adheres to [Semantic Versioning][semver].

[keepachangelog]: https://keepachangelog.com/en/1.1.0/
[semver]: https://semver.org/spec/v2.0.0.html

## [Unreleased]

## [0.10.0] — 2026-09-06

### Added

- An effect system for `.ush`, modelled on [Effekt]. A function's
  effect row is written after its return type and says what the body
  still needs from its caller:

  ```ush
  effect log(message: String) -> ()

  fn greet(name: String) -> String / { log } {
    do log("greeting " + name)
    "hello " + name
  }

  try {
    print greet("ubu")
  } with log { (message) =>
    print "[log] " + message
  }
  ```

  `effect` declares an operation, `do` performs it, and `try … with`
  discharges it for the block it wraps — the handler's value is what
  `do` evaluates to, so the body carries on where it left off.
  Handlers nest and shadow, and an effect that reaches the top of the
  program without a handler is a compile error.

  Six built-in effects — `io`, `fs`, `env`, `net`, `exec`, `task` —
  are inferred from the stdlib rather than performed, and describe
  what the generated shell does; no handler can take them back. Rows
  are inferred for every function whether or not one is written, by
  fixpoint, so mutual recursion converges instead of one side being
  reported as needing nothing. A written row turns the inference into
  a check and is an upper bound, so over-declaring is allowed.
  `ush effects` lists the rows, `ush effects --undeclared` lists the
  functions still missing one. See [`docs/effects.md`].
- `ush explain <script.ush> <LINE>` maps a line number out of a
  `/bin/sh` diagnostic — or a `G####` sourcemap id — back to the
  `.ush` line behind it, with surrounding source and the rest of the
  group that line lowered into.
- A `.zshenv`-style startup tier: `~/.config/ush/env.sh` and
  `~/.ush_env` are read on *every* invocation, including
  `ush -c ...`, with `--env-file` / `--no-env` to steer it. `PATH`
  set up for `ush` now applies to the invocation tools actually use.
- `PS1` and `USH_PROMPT` are honoured, and `shell.prompt` is a
  template rather than a literal: the familiar `PS1` escapes plus
  `\g` for the git branch and `\?` for the last exit status.
- `cd -` returns to the previous directory, and `$OLDPWD` is
  maintained.

### Fixed

- `ush -i` is accepted. Editor terminals and agent runners spawn
  `$SHELL -i -c ...`, and rejecting the flag killed every such
  session at argv parsing.
- A login shell started the POSIX way — with a `-` in front of
  `argv[0]`, which is what terminal emulators do — now loads the
  profile. It was previously ignored, so a `PATH` configured there
  looked like it had been thrown away. Login shells also read
  `/etc/profile`, which is what runs `path_helper` on macOS.
- External commands resolve through the shell's own `PATH` and are
  spawned by absolute path. Lookup went through the *process*
  environment, and `execvp` resolves a bare program name the same
  way, so a `PATH` exported at the prompt or from a startup file
  decided nothing.
- `a && b`, `a || b`, and `a; b` run inside `ush` instead of being
  handed to `/bin/sh` whole. Builtins (`cd`, `sammary`, `tasks`),
  the structured helpers, and session aliases were "command not
  found" in any chained line. Compound commands (`if`, `for`,
  `case`, `(`, `{`) still go to `/bin/sh` in one piece.
- A newline separates commands, so a multi-line `ush -c` runs every
  line instead of parsing the whole string as one command.
- `rm -rf` no longer blocks reading a confirmation from a stdin pipe
  nobody is going to write to. Without a terminal to ask on it
  declines with an actionable message instead of hanging.
- The completion menu moves with the arrow keys. `Up` / `Down` used
  to fall through to history navigation, discarding the menu and the
  line; `Right` / `End` now accept the highlighted candidate.
- `cd` keeps the logical path, so `cd /tmp` reports `/tmp` rather
  than `/private/tmp` in `pwd`, `$PWD`, and the prompt.
- A word that merely looks like a glob (`echo [$?]`) no longer fails
  the whole command with a pattern-syntax error.
- The compiler no longer panics on a line where `}` comes before its
  `{`, such as a handler arm: `parse_brace_body` sliced the range
  before checking that it ran forwards.
- `ush format` keeps the body of a block that opens on a closing
  line indented, and does not leave a trailing space after an arrow
  that ends its line.
- A `#` comment ends at its own line instead of truncating the rest
  of the input, so a multi-line `-c` string or a paste keeps running
  after a commented line.
- A builtin that fails inside a list becomes a status the next
  element can test, so `cd missing || echo fallback` runs the
  fallback rather than abandoning the line.
- `continue` inside a `for` loop no longer hangs. The range and list
  lowerings advanced the loop counter at the end of the body, so a
  `continue` skipped it and the generated `while` loop spun forever.
- `if let` on a variant that does not match no longer aborts the
  script. The binding was read before the tag test, so under
  `set -u` a payload slot belonging to another variant was an
  unbound variable.
- A `.ush` script that fails now exits non-zero. The runtime
  source-map `EXIT` trap returned the status of its own last
  command, which could report a failed script as a success.
- `ush check` and the LSP report what actually went wrong. Both
  rendered only the outermost error frame, which for most compiler
  errors is just `line N`; they now render the whole context chain
  (`empty expression`, `unterminated match expression`, …) and point
  at the innermost line.
- `fg` on a job that finished between the status refresh and the
  resume signal now reports the job's exit status instead of
  failing with `failed to continue job %N`; `bg` in the same
  situation says the job is no longer running.
- `alias ls='ls --color=auto'` expands once instead of eight times.
  Alias expansion now refuses to re-expand a name already being
  expanded, matching POSIX, which also terminates mutually recursive
  aliases.
- Editor highlighting recovers after a `"""` block string. The
  semantic tokenizer never left multi-line-string mode, so the rest
  of the file was painted as one string; a block string that opens
  and closes on one line no longer leaks either.
- `ls` in stylish mode names the path it could not read instead of
  surfacing a bare OS error.
- `make` `define … endef` bodies no longer leak into target
  completion.

### Changed

- The runtime failure report from a `.ush` script names the line
  number `/bin/sh` itself uses, alongside the sourcemap id, the exit
  status, and the `ush explain` invocation for that line. The
  instrumentation header used to shift every line, so the ids in the
  report matched nothing the shell had printed.
- Stylish `tasks` counts read `1 npm task` rather than `1 npm`.

### Security

- Bump `anyhow` to 1.0.104, `quick-xml` to 0.42.0, and pull
  `crossbeam-epoch` up to 0.9.20, clearing RUSTSEC-2026-0190
  (`anyhow::Error::downcast_mut` unsoundness), RUSTSEC-2026-0194 and
  RUSTSEC-2026-0195 (`quick-xml` quadratic attribute scan and
  unbounded namespace allocation), and RUSTSEC-2026-0204
  (`crossbeam-epoch` invalid pointer dereference). `cargo audit` and
  `cargo deny` had been red on `main` since the advisories landed.

### Performance

- REPL prompts no longer re-read every `PATH` directory. The
  completion name set is cached and invalidated by a `stat` per
  `PATH` entry plus the alias table, taking the per-prompt cost from
  ~2.7 ms to ~59 µs on a typical `PATH`.
- `parse_line` is roughly 2× faster: the POSIX-fallback keyword scan
  runs once instead of once per keyword (and no longer allocates),
  builtin lookup uses a perfect-hash set, and comment stripping and
  pipeline splitting borrow instead of allocating.
- Variable expansion walks bytes and borrows its input when there is
  nothing to expand, instead of collecting the value into a
  `Vec<char>` and allocating a `String` per variable name.
- `.ush → sh` compilation is ~30% faster: the output buffer counts
  line breaks with `memchr` rather than decoding every character,
  and block indentation no longer allocates a string per line.
- Chained commands no longer spawn `/bin/sh` and six temp files per
  line. `a && b` is executed directly, so only a segment that
  genuinely needs POSIX syntax pays for a subshell.

[Effekt]: https://effekt-lang.org
[`docs/effects.md`]: ./docs/effects.md

## [0.9.0] — 2026-05-19

LSP build-out. The stdio language server gains nine new
capabilities so editors can drive `.ush` files the same way they
drive Rust, TypeScript, etc. — without any change to the existing
`ush check` / `ush format` / `publishDiagnostics` / `semanticTokens`
plumbing.

### Added

- `textDocument/documentHighlight` — every occurrence of the
  identifier under the cursor.
- `textDocument/documentSymbol` — outline of top-level `fn` / `enum`
  / `trait` / `type` / `let` / `alias`.
- `textDocument/foldingRange` — `{ … }` block folding, correctly
  ignoring braces inside strings, line comments, and `#[…]`
  attributes.
- `textDocument/hover` — Markdown tooltip: keyword help, or "role +
  declaring line" for an identifier.
- `textDocument/completion` — every `.ush` keyword plus every
  identifier the semantic tokenizer has classified in the open
  document.
- `textDocument/definition` — first occurrence of the identifier
  under the cursor.
- `textDocument/references` — every occurrence of the identifier
  under the cursor.
- `textDocument/prepareRename` — range to highlight before the
  rename popup.
- `textDocument/rename` — `WorkspaceEdit` with one `TextEdit` per
  occurrence; rejects new names that are not valid `.ush`
  identifiers with a clear LSP error.
- `textDocument/signatureHelp` — `(` / `,` triggers a popup with the
  function signature, parameter list, and the currently-active
  parameter index.

### Changed

- Each LSP engine lives in `crates/ush_tooling` as a pure-Rust,
  editor-agnostic module (`highlight`, `symbol`, `folding`, `hover`,
  `completion`, `references`, `signature`); the LSP wire layer in
  `apps/ush_lsp` is responsible only for `lsp_types` conversion and
  routing.
- `docs/lsp.md` documents every implemented capability and its
  backing module, plus the methods that still need typed-info work.

## [0.8.0] — 2026-05-19

A polish-pass release on top of 0.7.0's production-readiness work.
Focus is CI hardening, contributor-facing docs, and shared
infrastructure.

### Added

- `MAINTAINERS.md` and `SUPPORT.md` for community-health coverage.
- `scripts/preflight.sh` runs every gate CI runs, in the same order,
  behind a single `sh scripts/preflight.sh`.
- `docs/architecture.md` — top-down map of the workspace + symptom
  → file table; `docs/release-process.md` indexed from the docs
  README; `docs/release-process.md` referenced from the README's
  Release section.
- New CI jobs / steps: dedicated **`no_std` clippy** gate, every
  `.ush` example dogfooded through `ush format --check`, and an
  end-to-end LSP `initialize/shutdown/exit` smoke test.
- CLI smoke test now covers `ush compile` and `ush check`.
- Crate-level rustdoc on `ush_compiler`, `ush_shell`, `ush_tooling`,
  and `ush_config` so `cargo doc` / docs.rs no longer show blank
  summaries.
- `apps/ush` now installs a user-facing panic hook that prints a
  triage-ready message + tracker link.
- `.github/labeler.yml` auto-labels PRs by changed paths;
  `.github/CODEOWNERS` auto-requests review.
- `.github/ISSUE_TEMPLATE/config.yml` adds an issue chooser with
  links to the security advisory form, release process, and
  architecture overview.
- Dependabot now also watches the `cargo` ecosystem in
  security-advisory-only mode.
- README adds a "Production readiness" section + CodeQL / Secret
  scan badges.

### Changed

- **CI concurrency** — every workflow's `concurrency.group` is now
  keyed on `run_id` for main pushes, so successive merges no longer
  cancel each other.
- **MSRV** bumped from 1.88 to **1.89** (vendored rustyline uses
  `std::fs::File::lock`, stabilised in 1.89).
- **Workspace stable rustfmt** — re-formatted with
  `rustfmt --edition 2024` so CI's import-sort matches the local
  edition-2024 layout.
- **PR template** — wider validation checklist + a `## Security`
  block; matches what current CI runs.
- **CONTRIBUTING.md** documents the rustfmt + edition-2024 gotcha
  and the `scripts/preflight.sh` shortcut.
- **cargo-deny** graph is now restricted to the four released
  targets; `bans.multiple-versions = "deny"` with a single
  `bitflags@1.3.2` skip; BSL-1.0 added to the allowed-license set.
- **GitHub Actions** — every third-party action is pinned to a
  full commit SHA (with a trailing comment giving the human-readable
  version) to remove the trust-on-first-use risk of mutable major
  tags.

## [0.7.0] — 2026-05-19

A production-readiness release. Everything below was previously
tracked under `[Unreleased]`.

### Added

- Started this changelog. Older releases are summarised below from the
  GitHub release notes; future releases should land their entries here
  first under `[Unreleased]` and then be cut into a version section.
- `SECURITY.md` with a private-disclosure policy.
- `CONTRIBUTING.md` describing the local-CI flow and project layout,
  plus a `scripts/preflight.sh` one-command CI mirror.
- Issue and pull-request templates under `.github/`, plus a
  `CODEOWNERS` file.
- `docs/release-process.md` documenting the pre-flight checklist,
  the matrix of published artefacts, and the rollback procedure.
- Root-level `.editorconfig` pins charset / EOL / indent.
- `Shellcheck` CI workflow lints `install.sh` and every
  `scripts/*.sh`.
- `Benchmark` CI job tracks parser and full `.ush → sh` compile-time
  benchmarks against the `main` baseline (on `gh-pages`) and fails
  PRs on >25% regression.
- `Deny` CI job runs `cargo-deny` for license / source / advisory /
  bans enforcement.
- `CodeQL` and `Gitleaks` workflows add SAST and secret-scanning to
  every PR and to a weekly cron.
- Compiler now enforces `match` exhaustiveness during the effects
  pass: missing variants on enums, missing arms on `Bool` and `Unit`,
  and literal-only matches on `String` / `Int` / tuples / lists are
  now rejected with a clear diagnostic instead of compiling to a
  silently-fall-through shell branch.
- New CI gate: every `examples/*.ush` is type-checked through
  `ush check` *and* `ush format --check`.
- README links the CI / Shellcheck / Dependencies workflow badges
  and the MIT shield.
- `.dockerignore` keeps the docker build context lean; the docker
  image now runs as the non-root `ush` user (uid 1000).
- Linux `aarch64` release archive (`ush-aarch64-unknown-linux-gnu.tar.gz`).
- `ush_lsp` gains `--version` / `-V` and `--help` / `-h` flags.
- `ush` gains a user-facing panic hook that prints a triage-ready
  message instead of a raw rustc panic.

### Changed

- Workspace declares `rust-version = "1.88"` as the MSRV (bumped
  during the 0.7.0 cycle to match what `criterion` and `home`
  require).
- CI matrixes `clippy` and `test` jobs across Ubuntu and macOS, adds
  an MSRV gate, runs `cargo audit` and `cargo deny`, gates rustdoc
  on warnings, and runs the `no_std` test suite (not just `cargo
  check`).
- Workflow `concurrency.group` is now keyed on `run_id` for `main`
  pushes so successive merges don't cancel each other.
- Shell signal helpers use `sigaction(2)` instead of `signal(3)`, and
  the SIGCONT pgid path rejects PIDs that do not fit `pid_t`.
- The compiler refuses to silently fall back when `canonicalize()`
  fails; the codegen invariant for control-flow statements is now
  an error rather than `unreachable!()`.
- The formatter no longer mis-parses `#[attr]` as a line comment,
  fixing un-indented function bodies after parameter attributes.
- `install.sh` hardens its trust surface: `umask 077`, best-effort
  `set -o pipefail`, no more clobber of the caller's `$TMPDIR`, and
  `curl` / `wget` flags pin HTTPS + TLS 1.2 with retries on remote
  URLs (local `file://` URLs in CI smoke-tests bypass these flags).
- Release-binary profile now strips debuginfo, runs thin-LTO with a
  single codegen unit, and uses `panic = "abort"`. The Linux/macOS
  release archives drop from ~5.6 MB to ~2.9 MB for `ush` and from
  ~2.7 MB to ~1.3 MB for `ush_lsp`.
- Workspace-wide lints deny `todo!()`, `dbg!()`, `unimplemented!()`,
  and `unused_must_use`.
- Cargo manifests carry full metadata (`description`, `repository`,
  `homepage`, `readme`, `keywords`, `categories`, `authors`). Apps
  are `publish = false`.
- `ush --help` and `ush_lsp --help` now document every flag and
  subcommand.

## [0.6.1] — 2026-05-17

Maintenance release. See the
[GitHub release notes](https://github.com/ubugeeei/ush/releases/tag/v0.6.1)
for the full diff.

## Older releases

For 0.6.0 and earlier, refer to the
[GitHub releases page](https://github.com/ubugeeei/ush/releases).

[Unreleased]: https://github.com/ubugeeei/ush/compare/v0.10.0...HEAD
[0.10.0]: https://github.com/ubugeeei/ush/releases/tag/v0.10.0
[0.9.0]: https://github.com/ubugeeei/ush/releases/tag/v0.9.0
[0.8.0]: https://github.com/ubugeeei/ush/releases/tag/v0.8.0
[0.7.0]: https://github.com/ubugeeei/ush/releases/tag/v0.7.0
[0.6.1]: https://github.com/ubugeeei/ush/releases/tag/v0.6.1
