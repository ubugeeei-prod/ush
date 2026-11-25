# Release process

This document describes how to cut a new release of `ush`. The
release pipeline is fully automated through GitHub Actions; this
note is the human-side checklist that wraps it.

## Cadence

`ush` is pre-1.0 and ships on demand rather than on a fixed
calendar. A typical release follows a focused change set — a
language feature, a runtime fix, a security hardening — and ships
once the relevant work is on `main`.

## Pre-flight

Before pressing the release button, verify each of the following
manually:

1. **`main` is green.** Every workflow listed under the project's
   _Actions_ tab must have a successful run on the tip of `main`:
   `CI`, `CodeQL`, `Shellcheck`, `Secret scan`, `Dependencies`.
2. **CHANGELOG.md** has an `## [Unreleased]` section that captures
   the changes since the last tag. Move that section into a fresh
   `## [X.Y.Z] — YYYY-MM-DD` heading and add an empty `[Unreleased]`
   placeholder back at the top.
3. **`Cargo.toml`** at the workspace level has `version = "X.Y.Z"`
   matching the tag you intend to push. The release workflow
   refuses to ship if the tag does not match the manifest version.
4. **MSRV.** `[workspace.package].rust-version` is still satisfiable
   against the dependency graph; the MSRV CI job already covers
   this on `main`, but spot-check it has not crept upward without a
   note in the changelog.

## Cutting the release

There are two equivalent paths:

### Tag push

```bash
git tag -a vX.Y.Z -m "ush vX.Y.Z"
git push origin vX.Y.Z
```

The `Release` workflow is triggered by the tag push, runs the
preflight (validates tag ↔ Cargo.toml version), builds the matrix
of targets, attests each archive, smoke-tests the installer
against the freshly-built artefact, and publishes a GitHub Release
with auto-generated notes plus a `sha256sums.txt`.

### `Cut Release` workflow

Run the `Cut Release` workflow from the Actions tab. It asks for a
version string (`vX.Y.Z`) and a target ref (typically `main`),
creates and pushes the tag for you, then chains into the same
`Release` workflow.

## Release artefacts

Each release publishes:

- `ush-x86_64-unknown-linux-gnu.tar.gz`
- `ush-aarch64-unknown-linux-gnu.tar.gz`
- `ush-x86_64-apple-darwin.tar.gz`
- `ush-aarch64-apple-darwin.tar.gz`
- `sha256sums.txt`

The installer is maintained in `install.ush`. All scripts in `scripts/` are
maintained as `.ush` sources; adjacent `.sh` files bootstrap CI and systems
without an installed compiler. Run `sh scripts/check_generated.sh` to detect
drift. POSIX platform operations and Python statistics are explicit foreign
code blocks in these sources. `flake.tynix` owns packaging and the dev shell;
`tynix compile flake.tynix -o flake.nix` generates the Nix entrypoint.

Each tarball contains the `ush` binary (including the offline editor extension), the `ush_lsp` binary,
`README.md`, and `LICENSE`. The Linux archives are produced on the
GitHub-hosted `ubuntu-latest` / `ubuntu-24.04-arm` runners; the
glibc baseline is whatever those images ship at build time.

All four archives also undergo `cargo test --release --target` on
the same runner before they are uploaded, so a release-mode-only
regression cannot ship.

## Post-flight

After the workflow has completed:

1. Sanity-check the published release page on GitHub.
2. Smoke-test the installer against the new tarball:

   ```bash
   curl -fsSL https://raw.githubusercontent.com/ubugeeei/ush/main/install.sh \
     | sh -s -- --version vX.Y.Z
   ush --version
   ```
3. If the release introduces a security fix, follow the disclosure
   timeline laid out in [`SECURITY.md`](../SECURITY.md).

## Rolling back

If a release turns out to be broken:

1. Delete the GitHub release (keep the tag — `cargo` users may
   already have it).
2. Open a tracking issue describing the regression.
3. Cut a new patch release as soon as the fix lands; do not
   re-tag an existing version.
