# Contributing to `ush`

Thanks for your interest in `ush`. The project is still pre-1.0 and
moves quickly; that means small, focused contributions are easier to
land than sweeping refactors.

## Quick start

```bash
git clone https://github.com/ubugeeei/ush
cd ush
cargo build                     # builds the workspace
cargo test --workspace          # runs the full suite
```

The workspace targets stable Rust. The current MSRV (minimum
supported Rust version) is declared in `Cargo.toml`'s
`[workspace.package]` as `rust-version`; it is bumped only in minor
releases.

## Run the same checks CI runs

Before pushing, please run the same gates that CI runs. They are
intentionally cheap:

```bash
cargo +stable fmt --all --check                              # CI uses stable rustfmt
cargo clippy --locked --workspace --all-targets --no-deps -- -D warnings
cargo test --locked --workspace
cargo test --locked --workspace --release
cargo check --locked -p ush_compiler --no-default-features   # no_std core
cargo test --locked -p ush_compiler --no-default-features --lib  # no_std tests
cargo bench --locked -p ush_shell --bench parser --no-run    # parser bench builds
cargo bench --locked -p ush_compiler --bench compile --no-run # compile bench builds
RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace \
    --no-deps --document-private-items                       # rustdoc must be warning-clean
sh scripts/check_rs_line_limit.sh                            # 250-line file cap
sh scripts/check_rustyline_upstream.sh                       # vendored rustyline drift
sh scripts/test_install.sh                                   # installer flow
sh scripts/test_rustc_wrapper.sh                             # Cargo's rustc wrapper
shellcheck --severity=warning install.sh scripts/*.sh scripts/launchers/*.sh # shell-script lint
```

For `.ush` files specifically (when changing the formatter or the
example corpus):

```bash
for f in examples/*.ush; do
    cargo run --locked -p ush --quiet -- check "$f"
    cargo run --locked -p ush --quiet -- format --check "$f"
done
```

### Updating snapshots

Two suites compare generated output against committed fixtures: the
compiler shell snapshots in
`crates/ush_compiler/tests/fixtures/shell_snapshots/` (plus the
catalog next to them) and the `apps/ush` stdout fixtures. When a
codegen or renderer change is deliberate, regenerate them and review
the diff before committing:

```bash
UPDATE_SNAPSHOTS=1 cargo test --workspace
git diff -- '*/tests/fixtures/*' '*/src/fixtures/*'
```

### rustfmt + edition 2024 gotcha

CI runs `cargo fmt --all --check` on the GitHub-hosted Linux
runner. Depending on your host rustfmt version, `cargo fmt` may not
fully apply the edition-2024 import-sort rules (alphabetical inside
`use foo::{...}` brace groups), even though it reports
`[bin (2024)]` for every crate. If CI's Format job complains and
local `cargo +stable fmt --all --check` is silent, run rustfmt
directly with the edition explicitly:

```bash
find apps crates -name '*.rs' -type f | xargs rustfmt --edition 2024
```

That has the same effect CI does.

CI also runs the parser and compile benchmarks on PRs and on `main`
and fails the PR if a microbench regresses by more than 25% versus
the latest `main` baseline (stored on the `gh-pages` branch). To
inspect locally, run e.g. `cargo bench --locked -p ush_shell --bench
parser`.

Any of these failing in CI is a hard block. The line-limit script
caps individual Rust files at 250 lines (sources only, tests inside a
file are counted); split larger files into modules.

There is also a single-command wrapper that runs every one of the
checks above in the same order CI runs them:

```bash
sh scripts/preflight.sh
```

### Rust build accelerators

`nix develop` speeds up local Rust builds on the same stable compiler. Cargo
runs every `rustc` call through `scripts/launchers/rustc.sh`
(`RUSTC_WRAPPER`), which:

- compiles this checkout's crates with rustc's parallel frontend
  (`USH_RUSTC_FRONTEND_JOBS`, default 8). It passes
  `-Zunstable-options --jobs-frontend=N` and sets `RUSTC_BOOTSTRAP` to that
  one crate's name. Registry dependencies, build-script probes and Cargo's
  `-vV`/`--print` queries reach `rustc` unchanged: a crate-wide or
  workspace-wide `RUSTC_BOOTSTRAP=1` would let dependency build scripts
  detect "nightly" and change what they compile. `RUSTFLAGS` is not used
  because Cargo hashes it into every crate's metadata, so changing the
  thread count would rebuild everything.
- hands the call to [kache](https://github.com/kunobi-ninja/kache) 2.3.1
  (`USH_KACHE`). kache shares compiled crates across every worktree and
  target directory on the machine and clones them on copy-on-write
  filesystems instead of storing them again. `kache stats` reports hits and
  the store size, and `kache clean` frees space.

To turn either off, enter the shell with `USH_RUSTC_FRONTEND_JOBS=1` or
`USH_KACHE=` (empty), for example while measuring compile times or
benchmarks. kache rewrites the paths in debug information, so release
artifacts are never built through it.

CI does not use the wrapper. On CI's runners, rustc 1.100.0 with a warm
dependency cache gave these p50 wall times over three rotated rounds. The
workspace crates were rebuilt with `clippy --workspace --all-targets --no-deps`
and with `test --workspace --no-run`:

| Runner | `clippy` j1 / j4 / j8 | test build j1 / j4 / j8 |
| --- | --- | --- |
| `ubuntu-latest` (4 vCPU) | 2.7 / 3.2 / 3.3 s | 10.7 / 11.8 / 11.9 s |
| `macos-15` (3 vCPU) | 8.1 / 7.3 / 7.4 s | 25.8 / 24.4 / 29.3 s |

On so few cores, Cargo's crate-level parallelism already keeps every core
busy. Extra frontend threads cost time on Linux and only add noise on
macOS. Revisit this if CI moves to larger runners. The MSRV job also could
not use the wrapper: rustc 1.89 has no `--jobs-frontend`.

## Workspace layout

- `apps/ush` — CLI binary
- `apps/ush_lsp` — stdio LSP server
- `crates/ush_config` — config loading
- `crates/ush_compiler` — `.ush → sh` compiler (`no_std + alloc` capable)
- `crates/ush_shell` — interactive shell, parser, stylish I/O
- `crates/ush_tooling` — formatter, diagnostics, semantic tokens
- `vendor/rustyline` — vendored line-editor; do not edit in random
  PRs (see `vendor/rustyline/`'s upstream-sync notes)

## Filing an issue

Use the templates under `.github/ISSUE_TEMPLATE/` so the maintainer
can triage without a back-and-forth. For anything security-related,
follow [SECURITY.md](./SECURITY.md) instead of opening a public
issue.

## Opening a pull request

- Match the existing style: short summary, notes (any subtlety the
  reviewer needs to know), and a validation list (the commands you
  ran). `.github/PULL_REQUEST_TEMPLATE.md` pre-fills this.
- One topic per PR. If you noticed unrelated cleanups while you were
  there, please open a follow-up.
- Keep `Cargo.lock` up to date — CI uses `--locked` everywhere.
- New behaviour needs a test or an explicit "why a test would be
  too expensive" note.

## Commits

- Use the imperative present tense ("add", "fix", "refactor").
- Reference issues with `Closes #NN` or `Refs #NN`.
- Don't squash unrelated changes into one commit.
