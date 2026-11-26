# Changelog

All notable changes to `ush` will be documented in this file.

The format is based on [Keep a Changelog][keepachangelog], and this
project adheres to [Semantic Versioning][semver].

[keepachangelog]: https://keepachangelog.com/en/1.1.0/
[semver]: https://semver.org/spec/v2.0.0.html

## [Unreleased]

### Added

- Bind the result of `match` directly with `let name = match value { ... }`.
  Arms produce a common type, or terminate with `fail`/`raise`; matches must be
  exhaustive. Values retain lexical scope and immutable bindings, and literal
  selection lowers directly to shell branches without a task or subprocess.

### Changed

- Use CLI-style bare arguments throughout `scripts/*.ush` and `install.ush`,
  retaining quotes where strings require them, and regenerate their standalone
  shell artifacts.
- Select installer and installer-test platforms with immutable match bindings.

## [0.17.0] — 2026-11-26

### Added

- `} else {` and `} else if … {` on the closing brace's line. `ush format`
  joins an `else` to the `}` before it unless a comment or blank line separates them.
- Interpolating strings: `$"…"` reads `$name` and `${name}` exactly as
  CLI-style words do, `$env::NAME` reads the environment, and `${expr}`
  takes any String, Int or Bool expression, including fields, calls and
  method calls. `$"""…"""` is the dedented multi-line form, and `\$` writes a
  dollar sign. Strings lower to quoted parameter expansion and
  concatenation, never `eval`, so `$`, backticks, quotes, newlines and `*`
  in a value stay data. `"…"`, `'…'` and `"""…"""` stay literal.
- `$env::NAME` reads a required environment variable as a String, in
  expressions, words (`--dir=$env::RUNNER_TEMP`) and interpolating strings.
  An unset or empty variable stops the program with its name and source
  line; the read infers `env` and `unknown`. `std::env::get_or` remains the
  optional form.
- List API: `len`, `is_empty`, `contains`, `first`, `last`, `join` and
  `enumerate` methods, bounds-checked `xs[i]`, `xs + ys` concatenation,
  `for (i, x) in xs.enumerate()` and tuple element access `pair.0`. An
  out-of-range index or `first`/`last` on an empty list fails with the
  source line, the contracts checker rejects indexes it can prove out of
  range, and `xs = xs + ys` on a `let mut` list appends in place.
- `glob "*.ts"` match patterns and `a | b` alternatives. A String `match`
  over literals, globs and `_` lowers to one POSIX `case`; quoted pattern
  text, spaces and `$` match literally, and ranges such as `[a-z]` expand to
  their ASCII characters so they match the same in every locale. Editors
  highlight interpolations and `glob`, and the formatter ignores braces
  inside `${…}`.

### Changed

- `shell` and `alias` reject an interpolating string, because the value would
  run as shell code.
- A String `match` lowers to `case` instead of an `if` chain.

## [0.16.0] — 2026-11-25

### Breaking changes

- **Breaking:** whitespace call arguments now use CLI-style bare words for strings and `$name` / `${name}` for variable references. Bare words support interpolation such as `--output=$path`; values are never split, globbed, or evaluated as shell source. Use `(expression)` for computed arguments and nested calls. Existing expression contexts and parenthesized calls retain their semantics.
- Update native scripts, examples, fixtures, diagnostics, formatter, syntax highlighting, and LSP navigation/completion for the new argument rules.

- Make `let` bindings immutable by default; introduce `let mut` and bare `name = value` reassignment, with compile-time mutability and type diagnostics. Shadowing keeps outer bindings intact, including function locals and loop variables.
- Support mutable aggregates, asynchronous task handles, and awaited results; evaluate structured assignment before replacing its destination.
- Teach editor highlighting, navigation, rename, hover, and formatting about mutable bindings. Separate imports from the body with one blank line.
- Migrate the native ush installer and repository scripts to explicit mutation.

### Added

- Native `echo` writes any number of Display values separated by spaces and a final newline, without imports. It works in pipelines and captures; generated `printf` keeps `-n`, backslashes, percent signs, and empty arguments literal across shells. `print` remains available for compatibility.

- A command, a pipeline or a function returning `()` used as an `if` or
  `while` condition tests whether it succeeded. Its output streams, a
  failure takes the `else` branch instead of stopping the script, and a
  pipeline fails like `pipefail`. `!` negates a condition, and parentheses
  group `&&` and `||`.
- `status_of command` evaluates to the command's exit status as an `Int`
  without silencing it, and `allow_fail command` runs it and carries on
  whatever its status. Both also take `task.await`.
- `std::io::discard_stderr`, `stderr_to` and `stderr_append_to` redirect the
  stderr of the stage before them while its stdout keeps flowing, and
  `to_stderr` sends a stage's stdout to stderr: `2>/dev/null`, `2> file`,
  `2>> file` and `>&2`.
- `let job = spawn command` runs a command, pipeline or unit function in the
  background as a `Task<()>` whose output streams. Task handles gain
  `job.pid` and `job.kill()`/`job.kill("INT")`, which signal the task and
  every process it started; a killed task does not fail its scope when the
  scope joins it.
- `defer { … }` and `defer statement` run when the enclosing function
  returns or the script exits, last registered first, including on
  failure, `std::process::exit` and handled signals, before unfinished tasks
  are joined and temporary directories removed.
- Call programs outside the command catalog by naming them:
  `use $bin::{vp, "apt-get" as apt_get}`. Quoted names take a ush alias.
- Native pipelines: `a | b | c` streams as a statement and captures the last
  stage's stdout as a value. A stage is a command, `std::process::exec` or
  `output`, or a function returning `()`. A pipeline fails like bash
  `pipefail` on every target, including dash, and the diagnostic names the
  leftmost failing stage and its line. A stage cut off by a reader that
  stopped early, as `head` does, is not a failure, whether it died of SIGPIPE
  or saw a write error because SIGPIPE was ignored. A line ending in `|`
  continues on the next line.
- `|&` sends a stage's stderr into the pipe.
- `std::io::from_file`, `to_file` and `append_to`: pipeline sources and sinks
  that lower to `<`, `>` and `>>` on the neighbouring stage, so
  `cmd |& to_file log` is `cmd > log 2>&1`.
- A String as the first stage feeds its exact bytes to stdin, covering
  here-strings and here-documents; no newline is added.
- `cd path { … }` and `export NAME = value, … { … }` scope the working
  directory and environment variables to a block, restoring the previous
  value or unset state afterwards, including on `break` and `continue`.
  `export NAME = value` sets and exports a variable for the rest of the
  script.
- `ush effects` reports pipeline, `export` and `cd` effects; the formatter
  indents continued pipelines; editors highlight `export`.
- Local modules: `mod "lib/gh.ush" as gh` imports a `.ush` file relative to
  the importing file (the alias defaults to the file stem). Reach its items as
  `gh::run` or with `use gh::{run, Status}` / `use gh::*`. Module files hold
  only declarations, and their items are private unless marked `pub`.
- Modules are inlined into one standalone script. Each module's items get a
  namespace of their own, so two modules may define the same function, and
  only reachable functions are emitted. Import cycles are diagnosed with the
  chain of files.
- Diagnostics and runtime failures inside a module name the module file and
  line; `ush check`, `ush test`, `ush effects`, `ush explain`, sourcemaps and
  the language server follow a script's modules. Sourcemap entries for module
  lines carry `source_file`.
- `ush compile --check` compiles in memory and exits 1 with a short diff when
  a committed output is stale or missing.
- `ush compile` accepts several inputs with `--out-dir DIR`, writing each as
  `DIR/<stem>.sh` beside its input; unchanged outputs are not rewritten.
- `ush effects` prints each function's error types ahead of its effects, and
  editor hovers on a function show its row, marked inferred or declared.

### Changed

- A task that a signal ends reports the signal's status to every `.await`,
  and a scope's exit trap runs deferred blocks before joining its tasks.
- The last line of a function declared `-> ()` is always a statement. A
  trailing command or pipeline there streams instead of failing to compile as
  a returned String.
- `--checked-in` output may be written to any directory. The header names the
  source relative to the output, and source-relative lookups
  (`std::path::from_source`, `__ush_source_dir`, `extern` paths) resolve from
  the source's location. Adjacent outputs are unchanged.
- Compile errors in function bodies and effect checks now name the line of
  the offending statement.
- `extern` libraries declared in a module are sourced from beside the module.
- A signature without a `/ row` now has its errors inferred as well as its
  effects, so `fn head_sha() -> String { git "rev-parse" "HEAD" }` compiles.
  Callers, `?` and typed failures see the inferred row as if it were written;
  recursive functions settle on the least row covering their cycle. A written
  row is still a checked upper bound, and `/ ()` still means pure.
- Rows may name error types and effects imported from modules.
- The repository scripts drop rows that only restated what the body does.

## [0.15.0] — 2026-11-25

### Added

- 47 optional-runtime std APIs: Unicode text, byte codecs and binary files,
  filesystem metadata and atomic writes, clocks, OS randomness, DNS, system
  queries and checked integer arithmetic.
- A versioned argv-only runtime protocol and actionable missing-runtime errors.
  Portable programs still run with POSIX sh alone.
- Cross-shell runtime tests covering binary NUL, Unicode, integer overflow,
  invalid inputs, metadata, atomic replacement and UTC formatting across TZs.


- 48 portable std APIs for text, lexical paths, filesystem operations, streams,
  process state, host information and integer helpers. Generated programs use
  POSIX sh and standard utilities without an installed ush runtime.
- Literal shell quoting, byte-preserving line lists, hidden directory entries,
  safe handling of option-like paths, and explicit argument-error diagnostics.
- A standard-library reference with API contracts, portability boundaries and
  a POSIX/libc coverage roadmap.

### Native scripting and packaging

- Rewrite the installer and all repository scripts as native ush, including argument parsing, checksum lookup, PATH configuration, tests and paired benchmark orchestration.
- Add typed process argv, execution, capture, status and redirection APIs, managed temporary directories, stderr output and string splitting/lines/integer conversion.
- Support shell-style `fail 23 "message"`, and let programs reading argv handle their own help flags.
- Decode common escapes in double-quoted strings. Single-quoted and triple-quoted strings remain raw; use them for literal backslashes when migrating.
- Preserve trailing newlines when concatenating strings, and capture structured standard-library results correctly.
- Run Nix package tests with a writable isolated home and explicit test tools.

## [0.12.0] — 2026-11-25

- Bundle an offline VS Code/Cursor/VSCodium extension with `ush editor install`; improve scoped navigation, shell-style signature help, Unicode positions and incremental LSP edits.
- Link generated shell artifacts to their source, explain common runtime exit failures, and avoid Rust backtraces for ordinary CLI errors.
- Author the installer and repository scripts in ush; replace Docker packaging with a typed `flake.tynix` source and generated Nix entrypoint.
- Skip unused builtin signature construction during compilation.

### Added

- Import module exports with `use module::*`, including `use $bin::*` and
  source-relative `.sh` extern modules. Explicit imports and local functions
  override wildcards; ambiguous wildcard exports are diagnosed.
- Call functions with consecutive, space-separated arguments in statements,
  bindings and nested expressions. Zero-argument functions can be called as
  bare names. No `$` application is needed; existing call forms remain accepted.
- Pass named function values with `&name` and typed `Fn<A, B>` parameters.
  Generic callbacks specialize from their expected types; constructor kinds
  such as `F<_>` compose with higher-order functions. Actual callback errors
  and effects are checked before shell generation.
- Build lists with `[value for item in items if condition]`, including multiple
  generators, filters, nested structured values and empty results. Bindings are
  scoped to the comprehension.
- Write named `test "name" { ... }` and anonymous `test { ... }` blocks.
  `ush test` isolates cases, reports each result, and continues after failures.
  Assertions require an explicit `std::test` import or qualified call.
- Load shell libraries with `extern "helpers.sh" as helpers { fn ... }`.
  Declared scalar arguments are passed as quoted argv values, scalar results
  are checked, and nonzero exit status propagates. Checked-in artifacts retain
  source-relative library paths.

### Fixed

- Parse grouped nested arguments and qualified arguments without confusing
  their parentheses or `::` separators with call punctuation and labels.
- Clamp descending range lengths to zero when materializing an empty list.


## [0.11.1] — 2026-11-23

### Fixed

- Preserve the original exit status and source locations when awaiting a failed
  task. Cleanup reaps each child once. Sibling tasks can await the same handle
  and read its completed result.

## [0.11.0] — 2026-11-23

### Language and runtime

- **Breaking syntax change.** Write `-> Value / Error, effect` and
  `with log(message) { … }`. Use `/ ()` for an explicitly pure function.
  Return typed failures with `fail Error::Variant` or supply a shell exit
  status with `fail(code, message)`.
- Track stdin, stdout and stderr in function effects. Declare `io` to permit
  all three. Inspect inferred rows with `ush effects`.
- Type-check generic and inferred functions, higher-kinded parameters, units,
  length-indexed arrays and integer refinements with `ush check`.
- Call 290 cataloged shell utilities and common commands through `$bin::`.
  Import unqualified calls with `using $bin::`. Capture inline shell pipelines
  with `$( … )?`, including scripts that read standard input.
- Start ordinary functions with `async f(args)` and join them with `job.await`.
  Tasks use private result storage and propagate exit status.
- Select standalone sh, bash, zsh and ash output. Runtime failures identify the
  original .ush line and source expression. Shell output links only the library
  helpers the script uses.

### Added

- `ush compile script.ush --checked-in -o script.sh` emits a deterministic,
  relocatable POSIX shell artifact for repositories that commit generated
  scripts. The output identifies its `.ush` source, carries a
  `DO NOT EDIT MANUALLY` warning and regeneration command, and keeps
  source-relative paths working without embedding a build-machine path.

### Changed

- Updated vendored rustyline to 18.0.1, including the completion prompt fix.
  Upstream monitoring checks both new stable releases and updates to the
  maintained major through the Cargo registry index. Rustyline 19 uses Rust
  1.95 syntax while the workspace supports Rust 1.89.
- Replaced the unmaintained `chrono` dependency with `time` for local file
  timestamps, resolving RUSTSEC-2026-0543 without suppressing the advisory.

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

[Unreleased]: https://github.com/ubugeeei-prod/ush/compare/v0.17.0...HEAD
[0.17.0]: https://github.com/ubugeeei-prod/ush/compare/v0.16.0...v0.17.0
[0.16.0]: https://github.com/ubugeeei-prod/ush/compare/v0.15.0...v0.16.0
[0.11.1]: https://github.com/ubugeeei-prod/ush/releases/tag/v0.11.1
[0.11.0]: https://github.com/ubugeeei-prod/ush/releases/tag/v0.11.0
[0.10.0]: https://github.com/ubugeeei/ush/releases/tag/v0.10.0
[0.9.0]: https://github.com/ubugeeei/ush/releases/tag/v0.9.0
[0.8.0]: https://github.com/ubugeeei/ush/releases/tag/v0.8.0
[0.7.0]: https://github.com/ubugeeei/ush/releases/tag/v0.7.0
[0.6.1]: https://github.com/ubugeeei/ush/releases/tag/v0.6.1
