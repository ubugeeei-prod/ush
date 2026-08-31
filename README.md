# ush

[![CI](https://github.com/ubugeeei/ush/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/ubugeeei/ush/actions/workflows/ci.yml)
[![CodeQL](https://github.com/ubugeeei/ush/actions/workflows/codeql.yml/badge.svg?branch=main)](https://github.com/ubugeeei/ush/actions/workflows/codeql.yml)
[![Secret scan](https://github.com/ubugeeei/ush/actions/workflows/secret-scan.yml/badge.svg?branch=main)](https://github.com/ubugeeei/ush/actions/workflows/secret-scan.yml)
[![Shellcheck](https://github.com/ubugeeei/ush/actions/workflows/shellcheck.yml/badge.svg?branch=main)](https://github.com/ubugeeei/ush/actions/workflows/shellcheck.yml)
[![Dependencies](https://github.com/ubugeeei/ush/actions/workflows/dependencies.yml/badge.svg?branch=main)](https://github.com/ubugeeei/ush/actions/workflows/dependencies.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

`ush` is an experimental Rust shell with a simple idea:

- Be POSIX-first. Existing shell scripts and one-liners should keep working.
- Keep stdio Unix-simple by default, but allow richer human-facing output with `--stylish`.
- Add a modern `.ush` language, but compile it to portable `sh` instead of inventing a separate runtime.
- Make the interactive shell feel fast, lightweight, and pleasant to edit in.

> [!WARNING]
> `ush` is still an early WIP / prototype.
> Expect rough edges, missing POSIX coverage, incomplete language features, and breaking changes.
> It is not yet a production shell replacement.

## Concept

`ush` is split into two layers:

- `ush_shell`: an interactive shell runtime with POSIX-friendly execution, REPL UX, stylish mode, and structured helper pipelines
- `ush_compiler`: a `.ush` to `sh` compiler, where scripts stay portable because execution still happens through POSIX `sh`

That means the project can explore a more modern shell language without giving up the portability and compatibility of `sh`.
The `no_std` target is only the compiler core. The app binary and interactive shell runtime are still intentionally `std`-based because they need processes, terminals, files, and OS integration.

Language direction:

- aim for a real small language closer to MoonBit in seriousness than to shell macros
- keep the everyday feel somewhere between Rust, Zig, and Go
- prefer strong data modeling and a practical stdlib over clever shell-only tricks
- keep POSIX `sh` as the lowered runtime contract

## Status

This repository is an MVP focused on architecture and interaction experiments.

Implemented today:

- Interactive shell REPL in Rust
- `-c` command execution
- `.sh` / POSIX scripts executed through `/bin/sh`
- `.ush` scripts compiled to `sh` and then executed by `/bin/sh`
- Generated `.ush` output stays within POSIX `sh` syntax and POSIX command usage
- Prototype typed language features: `type { ... }`, enums, traits, marker `impl`, `match`, typed `fn`, Zig-style error signatures like `Problem!String`, and Rust-like `?` propagation
- Rust-like tail expressions in value-returning functions, where the final expression returns and `;` keeps it as a statement
- Rust-like `std::module::function` paths plus `use` imports for selected std helpers, including `std::env`, `std::path`, `std::fs`, `std::command`, `std::string`, `std::http`, and `std::regex` with capture support
- Labeled function arguments plus parameter attributes such as `#[default(...)]` and `#[alias("n")]`
- `alias name = "..."` declarations in `.ush`
- `bin.ush` as a generated CLI entrypoint, with flags/defaults/completion derived from the `bin(...)` signature
- `crates/ush_compiler` builds as `no_std + alloc` with CompactString, SmallVec, bumpalo, memchr, phf, and Fx-hashed maps in the core path
- `apps/ush` and `crates/ush_shell` remain `std`-based by design
- Installer patterns such as `curl -fsSL https://... | sh` are detected from the parsed pipeline and executed through POSIX `/bin/sh`
- `.ush` inline shell escapes via `$ command ...`, alongside `shell expr` for dynamic command strings
- Builtins: `:`, `.`, `cd`, `pwd`, `echo`, `true`, `false`, `alias`, `unalias`, `jobs`, `wait`, `disown`, `fg`, `bg`, `port`, `stop`, `history`, `export`, `unset`, `confirm`, `input`, `select`, `env`, `command`, `which`, `type`, `test`, `[`, `help`, `source`, `rm`
- Login/profile startup loading via `--login`, `--profile-file`, `--rc-file`, legacy `~/.ush_profile` / `~/.ushrc`, and rc defaults from `~/.config/ush/.config.ush` or `~/.config.ush`
- Builtin utility: `sammary` for recursive file and type summaries across paths and globs, with lockfiles excluded by default
- Safety prompt for dangerous recursive `rm` unless `--yes` or `USH_INTERACTION=false`
- Stylish renderers for `pwd`, `ls`, `cat`, `ps`, and `kill`
- Stylish command introspection for `which`, `type`, `command -v`, and `command -V`, with `which` listing every candidate and marking the one `ush` will run
- Structured helpers: `len`, `lines`, `json`, `xml`, `html`, `car`, `cdr`, `head`, `tail`, `take`, `drop`, `nth`, `enumerate`, `swap`, `fst`, `snd`, `frev`, `fsort`, `funiq`, `fjoin`, `map`, `fmap`, `flat`, `ffmap`, `fzip`, `each`, `filter`, `ffilter`, `any`, `fany`, `some`, `fsome`
- Environment-variable expansion, `~` expansion, and simple glob expansion
- Criterion benchmark skeleton for parser/profiling work
- GitHub Releases, `curl` installer, `nix`, and Docker packaging entry points
- Emacs-style and opt-in Vi-style REPL editing modes
- `ush format` and `ush check` commands for formatter and typechecking passes
- `ush_lsp` with document formatting, diagnostics, and semantic tokens for editor integration

Not there yet:

- Full native POSIX grammar coverage inside the Rust runtime
- Richer typed structured values beyond text / JSON helpers
- Broader language features such as HM inference, generics, HKT, modules, `yield`, and real green-thread scheduling
- Inherent `impl Type { ... }` methods and a Rust-complete type system; the current prototype is still a small subset
- A truly finished shell UX; editing, completion, and IME behavior are still being tuned

Workspace layout:

- `apps/ush`: CLI binary
- `crates/ush_config`: config loading and runtime paths
- `crates/ush_compiler`: `.ush` to `sh` compiler core, `no_std + alloc` capable
- `crates/ush_shell`: interactive shell, parser, stylish I/O, helpers
- `crates/ush_tooling`: formatter, diagnostics, and semantic token generation
- `apps/ush_lsp`: stdio LSP server for editors

## Usage

Quick install:

```bash
curl -fsSL https://raw.githubusercontent.com/ubugeeei/ush/main/install.sh | sh
exec "$SHELL" -l
ush --version
```

The installer tries to stay zero-config:

- it installs into the first writable personal bin directory already on your `PATH`
- otherwise it falls back to `~/.local/bin`
- if that directory is not on your `PATH`, it appends the export line to `~/.zshrc`, `~/.bashrc`, or `~/.profile`

If you want an explicit location instead:

```bash
curl -fsSL https://raw.githubusercontent.com/ubugeeei/ush/main/install.sh | sh -s -- --bin-dir "$HOME/.local/bin"
```

`nix profile install ...` and the Docker image already expose `ush` on `PATH`.

```bash
cargo run -p ush
```

Run a one-liner:

```bash
cargo run -p ush -- -c 'printf "a\nb\n" | len'
```

Kill the process that is listening on a port:

```bash
cargo run -p ush -- -c 'port 3341 | stop'
```

Inspect command resolution order and see which candidate is current:

```bash
cargo run -p ush -- -s -c 'which echo'
```

Enable stylish mode:

```bash
cargo run -p ush -- -s -c 'ls crates'
```

Force stylish mode globally:

```bash
export USH_STYLISH=true
```

Opt into the Vi-style REPL keymap, which is useful in environments such as Codex Desktop where `Cmd` shortcuts may be intercepted before they reach the shell:

```bash
export USH_KEYMAP=vi
```

Disable interactive confirmations:

```bash
export USH_INTERACTION=false
```

Load login/profile startup files explicitly:

```bash
cargo run -p ush -- --login
cargo run -p ush -- --profile-file ~/.config/ush/profile.sh -c 'echo $PWD'
cargo run -p ush -- --rc-file ~/.config.ush
```

## Interactive Editing

The REPL is tuned around `rustyline`'s Emacs mode with extra bindings for shell-heavy navigation:

- `Ctrl-A` / `Ctrl-E`: jump to line start/end
- `Ctrl-C`: interrupt the current prompt or child command and return control to `ush`
- `Ctrl-L`: clear the screen
- `Ctrl-P` / `Ctrl-N`: previous and next history entry
- `Ctrl-U` / `Ctrl-K`: kill to the line start/end
- `Ctrl-W`: kill the previous shell word
- `Up` / `Down`: previous and next history entry
- `Shift-Left` / `Shift-Right`: extend a visible character selection
- `Shift-Up` / `Shift-Down`: behaves like normal history movement when the terminal forwards those keys
- `Option-Up` / `Option-Down`: prefix history search
- `Option-Left` / `Option-Right`: word-wise cursor movement
- `Option-Shift-Left` / `Option-Shift-Right`: extend selection word-by-word
- `Ctrl-Left` / `Ctrl-Right`: word-wise movement on terminals that send control-arrow escapes
- `Ctrl-Shift-Up` / `Ctrl-Shift-Down`: select to the line start/end on terminals that map document-edge shortcuts there
- `Ctrl-Alt-Shift-Left` / `Ctrl-Alt-Shift-Right`: extend selection across big shell tokens
- `Ctrl-Alt-Shift-Up` / `Ctrl-Alt-Shift-Down`: extra line-edge selection aliases for macOS terminal mappings
- `Home` / `End`: jump to line start/end, and `Shift-Home` / `Shift-End` selects to the edge
- `Cmd-Left` / `Cmd-Right`: jump to line start/end when the terminal forwards them as `Super` cursor keys
- `Cmd-Shift-Left` / `Cmd-Shift-Right`: extend selection to the line edges when the terminal forwards `Super+Shift`, and `Cmd-Shift-Up` / `Cmd-Shift-Down` map to the same line-edge selection inside the single-line REPL

When a selection is active, typing replaces it and `Backspace` / `Delete` / `Ctrl-W` / `Ctrl-U` / `Ctrl-K` remove it, so keyboard-only editing feels closer to a native text field even inside the terminal.

Tab completion is context-aware instead of just dumping filesystem entries. In particular, `git` commands now complete subcommands, common flags, branch/tag/remote/stash names, recent commits, and pathspecs relative to the current shell directory, while the inline hint shows a short usage reminder for the argument you are typing.

If you opt into `USH_KEYMAP=vi` or `shell.keymap = "vi"`, `ush` switches the REPL to `rustyline`'s Vi editing mode instead. That is the recommended workaround for Codex Desktop, where `Cmd`-modified keys are often intercepted by the host app before the shell can read them.

## Structured Helpers

`ush` keeps normal Unix pipes, but helper stages can operate on structured values:

```bash
printf "alpha\nbeta\ngamma\n" | filter(\it -> contains(it, "a")) | len
printf "hello\nworld\n" | map(\it -> upper(it))
printf "hello\nworld\n" | fmap(\it -> upper(it))
printf "hello\n" | map(\line -> { upper(line) })
printf "alpha\nbeta\ngamma\n" | car
printf "alpha\nbeta\ngamma\n" | cdr
printf "alpha\nbeta\ngamma\n" | take(2)
printf "alpha\nbeta\ngamma\n" | drop(1)
printf "alpha\nbeta\ngamma\n" | nth(1)
printf "alpha\nbeta\ngamma\n" | enumerate(1)
printf "beta\nalpha\nbeta\n" | fsort | funiq | fjoin(",")
printf "alpha\nbeta\ngamma\n" | flat(\head, rest -> [head, "tail", rest])
printf "alpha\nbeta\n" | fzip(["1", "2"])
printf "alpha\nbeta\n" | fzip(["1", "2"]) | swap
printf "alpha\nbeta\n" | fzip(["1", "2"]) | fst
printf "alpha\nbeta\n" | fzip(["1", "2"]) | snd
cat package.json | json | len
cat feed.xml | xml
curl -fsSL https://example.com | html
```

Currently supported helper forms:

- `len`
- `length` (compatibility alias)
- `lines`
- `json`
- `xml`
- `html`
- `car`
- `cdr`
- `head`
- `tail`
- `take(2)`
- `drop(1)`
- `nth(1)`
- `enumerate(1)`
- `swap`
- `fst`
- `snd`
- `frev`
- `fsort`
- `funiq`
- `fjoin(",")`
- `map(\it -> upper(it))`
- `fmap(\it -> upper(it))`
- `map(\it -> lower(it))`
- `map(\it -> trim(it))`
- `map(\it -> replace(it, "from", "to"))`
- `flat(\head, rest -> [head, rest])`
- `ffmap(\head, rest -> [head, rest])`
- `fzip(["left", "right"])`
- `each(\it -> print(it))`
- `filter(\it -> contains(it, "foo"))`
- `ffilter(\it -> contains(it, "foo"))`
- `filter(\it -> starts_with(it, "foo"))`
- `filter(\it -> ends_with(it, "foo"))`
- `any(\it -> contains(it, "foo"))`
- `fany(\it -> contains(it, "foo"))`
- `some(\it -> contains(it, "foo"))`
- `fsome(\it -> contains(it, "foo"))`

`html` writes the current stream into a temporary HTML file and opens it in your default browser.
If `json` cannot parse the stream, `ush` falls back to this browser flow instead of failing immediately.
`xml` pretty-prints valid XML and falls back to the same browser flow if the input is not valid XML.
`car` and `cdr` are Lisp-style head and tail helpers over the current line stream.
`head` and `tail` are plain aliases for `car` and `cdr`.
`take`, `drop`, `nth`, and `enumerate` are Rust-style line-stream helpers, with `nth` using zero-based indexing.
`fst` and `snd` project the first and second fields from tab-separated pair streams such as `fzip(...)`.
`swap` flips those tab-separated pair streams.
`frev`, `fsort`, and `funiq` reverse, lexicographically sort, and de-duplicate line streams.
`fjoin("...")` collapses the current line stream into one line using a literal delimiter.
`flat` is a small stream-level flat-map that binds `head` and `rest`, where `rest` splices the remaining lines into the output list.
`fmap`, `ffmap`, `ffilter`, `fany`, and `fsome` are functional aliases for the corresponding helpers.
`fzip` zips the current line stream against a literal right-hand list or multiline string and emits tab-separated pairs.
Helper lambdas also accept `\name -> expr`, `\name -> { expr }`, zero-arg forms like `\-> { "ok" }`, and two-arg `flat(\head, rest -> [...])`.

## Stylish Mode

The shell-level `-s` / `--stylish` flag swaps some Unix commands into richer output without changing their names:

- `pwd`
- `ls`
- `cat`
- `ps`
- `kill`

When stylish mode is off, `ush` stays close to classic Unix text output.
Compiled `.ush` scripts keep POSIX stdio and run under `/bin/sh`; stylish rendering is only an interactive shell feature.

## Config

Config is resolved from:

- `~/.config/ush/config.pkl`
- `~/.config/ush/config.json`

Default shell rc loading is separate from structured config. For interactive sessions, `ush` now prefers `~/.config/ush/.config.ush`, falls back to `~/.config.ush`, and still accepts legacy `rc.sh` / `.ushrc`.

Example `config.pkl`:

```pkl
{
  shell {
    stylishDefault = true
    interaction = true
    historySize = 10000
    keymap = "vi"
    prompt = "ush> "
  }

  aliases {
    ll = "ls -la"
    gs = "git status -sb"
  }
}
```

Because Pkl tooling differs by version, `ush` tries a few `pkl eval` JSON output flag variants before falling back to JSON config.
Legacy `~/.config/ubsh` config files and `UBSHELL_*` env vars are still accepted for compatibility.

## Ush Scripts

`.ush` files are compiled to `sh`; execution still happens in `/bin/sh`.

Small example:

```text
let greeting = "hello"
print greeting + " world"
$ printf '%s\n' from-ush
match greeting {
  "hello" => print "matched"
  _ => print "fallback"
}
```

Current highlights:

- `let`, `print`, `match`, typed `fn`, `enum`, `type`, marker `trait`, and Rust-like tail expressions
- `""" ... """` multiline strings with common-indent dedent
- `std::env`, `std::path`, `std::fs`, `std::command`, and `std::string` helpers via fully-qualified calls or top-level `use`, plus method-style path/string flows like `path.resolve()`, `path.exists()`, `path.read_text()`, and `name.trim_suffix(".ush")`
- `raise` plus typed error signatures like `Problem!String`, with Rust-like `?` propagation
- `$ command ...` for inline shell execution and `shell expr` for dynamic command strings
- `async` / `.await`
- `bin(...)` entrypoints for generated CLI tools
- `#|` doc comments for generated `--help`, `--man`, and completion text, including std-like sections such as notes, warnings, errors, and see-also links
- sectioned sourcemaps with generated-line summaries, reverse source indexes, and runtime failure mapping back to `.ush` lines

Useful commands:

```bash
cargo run -p ush -- examples/hello.ush
cargo run -p ush -- scripts/bootstrap.sh --flag value
cargo run -p ush -- examples/control_flow.ush
cargo run -p ush -- compile examples/hello.ush
cargo run -p ush -- test examples/smoke_test.ush
cargo run -p ush -- compile examples/hello.ush --sourcemap /tmp/hello.sh.map.json
cargo run -p ush -- format examples/hello.ush --stdout
cargo run -p ush -- check examples/hello.ush
cargo run -p ush_lsp
cargo run -p ush -- examples/std_modules.ush
cargo run -p ush -- examples/http_regex.ush
cargo run -p ush -- -c "sammary 'crates/ush_shell/src'"
cargo run -p ush -- -c "sammary --include-lock ."
```

`.ush` files are compiled and then executed by `/bin/sh`; `.sh` files are forwarded straight to `/bin/sh` with their arguments.

Start here for more detail:

- `docs/language-vision.md` for the language design target and ergonomics direction
- `examples/README.md` for runnable samples
- `docs/README.md` for guide index
- `docs/sourcemaps.md` for the sourcemap JSON format, sections, reverse lookup, and runtime diagnostics
- `docs/typed-errors.md` for a step-by-step walkthrough of `Problem!T`, `raise`, inferred `# raises:`, `?`, and external-command `unknown`
- `docs/lsp.md` for editor integration with `ush_lsp`

## Install

### curl

`install.sh` downloads the matching GitHub Releases archive and installs `ush` plus `ush_lsp`.
By default it picks the first writable personal bin directory already on `PATH`.
If none is available, it falls back to `~/.local/bin` and updates your shell rc automatically on POSIX shells.
It refuses to install unless it can verify the archive against the release `sha256sums.txt` with `sha256sum`, `shasum`, `openssl`, or `python3`.
Release archives are currently published for:

- macOS `x86_64`
- macOS `aarch64`
- Linux `x86_64-unknown-linux-gnu`
- Linux `aarch64-unknown-linux-gnu`

The Linux archives are built on `ubuntu-latest` (x86_64) and
`ubuntu-24.04-arm` (aarch64), so the glibc baseline is whatever the
GitHub Actions image ships at the time of the build. Distributions
older than the runner image's glibc require building from source.

```bash
curl -fsSL https://raw.githubusercontent.com/ubugeeei/ush/main/install.sh | sh
```

Pin a release version:

```bash
curl -fsSL https://raw.githubusercontent.com/ubugeeei/ush/main/install.sh | sh -s -- --version v0.6.0
```

Install into a custom bin directory:

```bash
curl -fsSL https://raw.githubusercontent.com/ubugeeei/ush/main/install.sh | sh -s -- --bin-dir "$HOME/.ush/bin"
```

Skip automatic `PATH` updates:

```bash
curl -fsSL https://raw.githubusercontent.com/ubugeeei/ush/main/install.sh | sh -s -- --no-modify-path
```

Override the checksum manifest URL:

```bash
curl -fsSL https://raw.githubusercontent.com/ubugeeei/ush/main/install.sh | sh -s -- --checksum-url https://example.com/sha256sums.txt
```

### nix

```bash
nix profile install github:ubugeeei/ush
```

### source

If your platform does not have a prebuilt release archive yet, build from source:

```bash
cargo build --release -p ush -p ush_lsp
mkdir -p "$HOME/.local/bin"
install -m 755 target/release/ush "$HOME/.local/bin/ush"
install -m 755 target/release/ush_lsp "$HOME/.local/bin/ush_lsp"
```

### Docker

The Docker image is meant to be a distribution target and base image: a small environment where `ush` is already installed and ready to use.

```bash
docker build -t ush .
docker run --rm -it ush
```

Use it as a base image:

```dockerfile
FROM ush

RUN printf "a\nb\n" | len
CMD ["ush"]
```

## Benchmarks

```bash
cargo bench -p ush_shell
```

The parser suite covers the native fast path, the POSIX fallback
path, alias expansion, and helper pipelines; the compiler suite
covers an empty program (the fixed runtime prelude), a small
program, and an ADT-heavy one. CI compares every benchmark against
the `main` baseline and fails a PR on a regression above 25%.

## Release

GitHub Actions provides two release paths:

- Push a `v*` tag to run the release pipeline directly
- Run `Cut Release` from the Actions tab to create and push a tag, then call the same release pipeline

`Cut Release` asks for a version like `v0.2.0` and a target ref such as `main`.

See [`docs/release-process.md`](./docs/release-process.md) for the full pre-flight checklist, the matrix of published artefacts, and the rollback procedure.

## CI

GitHub Actions runs formatting, the Rust 250-line file limit check, workspace tests, release tests, bench build checks, and the `.ush` async / ADT examples on every pull request and push to `main`.

A separate `Dependencies` workflow watches `vendor/rustyline` against `crates.io` on a weekly cron via [`scripts/check_rustyline_upstream.sh`](./scripts/check_rustyline_upstream.sh) and fails when the pinned tag in [`vendor/rustyline/UPSTREAM`](./vendor/rustyline/UPSTREAM) is behind upstream, so security advisories surface even between human reviews.

## Production readiness

`ush` is still a pre-1.0 prototype (see the warning at the top of this file), but the 0.7.0 line ships with a deliberately strict CI / supply-chain posture so a production-style deployment is feasible inside the supported scope:

- **Correctness gates** — `cargo test --workspace` on Ubuntu and macOS, `cargo test --release` on every published target (Linux `x86_64`/`aarch64`, macOS `x86_64`/`aarch64`), `no_std` test suite, MSRV gate (currently 1.88), rustdoc with `-D warnings`, and `ush check` + `ush format --check` over the committed example corpus.
- **Static analysis** — clippy `-D warnings`, workspace-wide lints denying `todo!()` / `dbg!()` / `unimplemented!()` / `unused_must_use`, plus CodeQL (Rust, `security-and-quality` query pack) on every PR and a weekly cron.
- **Supply chain** — `cargo audit`, `cargo deny` (licenses / sources / advisories / bans), weekly Gitleaks secret scan, a vendored-`rustyline` drift watcher, and Dependabot subscribed to GitHub Actions updates so action versions never silently age.
- **Runtime hardening** — signal helpers use `sigaction(2)` with checked `pid_t` casts; the compiler enforces `match` exhaustiveness at the effects pass; `install.sh` pins HTTPS + TLS 1.2, sets `umask 077`, and best-effort `set -o pipefail`; the docker image runs as a non-root user.
- **Release pipeline** — tag push (or the `Cut Release` workflow) triggers preflight (tag matches `Cargo.toml`), per-target build + test + attestation, installer smoke-test against the freshly-built archive, and a GitHub Release with `sha256sums.txt`. Procedure is documented in [`docs/release-process.md`](./docs/release-process.md).
- **Performance gate** — parser and `.ush → sh` compile benchmarks are tracked against the `main` baseline on `gh-pages`; PRs fail when a microbench regresses by more than 25%.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for the local-CI flow,
workspace layout, and PR conventions. The full list of user-visible
changes lives in [CHANGELOG.md](./CHANGELOG.md). Security reports go
through the private channel described in [SECURITY.md](./SECURITY.md)
— please do not open a public issue for them.

## License

MIT
