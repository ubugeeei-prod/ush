window.BENCHMARK_DATA = {
  "lastUpdate": 1795613406687,
  "repoUrl": "https://github.com/ubugeeei-prod/ush",
  "entries": {
    "Criterion microbenchmarks": [
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "538a0563412cb831d5503bf56edc958ea844faf2",
          "message": "ci: stop cancelling main-branch runs on every subsequent push (#106)\n\nEvery workflow used `group: <name>-<workflow>-${{ github.ref }}`\nwhich means rapid pushes to `main` all land in the same concurrency\ngroup. Even with `cancel-in-progress: false` set for non-PR events,\nthe GitHub Actions queue ends up cancelling earlier runs when\nseveral newer ones pile up behind them, because the runner pool\ntreats stale queue heads as superseded.\n\nSwitch every workflow's concurrency group expression to:\n\n    <name>-<workflow>-${{ github.ref == 'refs/heads/main'\n                          && github.run_id\n                          || github.ref }}\n\nSo each main-branch run gets its own unique group keyed on\n`run_id` (and therefore cannot be superseded by anything), while\nPR pushes keep the existing \"cancel the previous run on the same\nbranch\" semantics keyed on `github.ref`.\n\nTouched workflows: ci.yml, codeql.yml, secret-scan.yml,\nshellcheck.yml, dependencies.yml.\n\nCo-authored-by: Claude Opus 4.7 (1M context) <noreply@anthropic.com>",
          "timestamp": "2026-05-19T04:16:21+09:00",
          "tree_id": "d240a5070d5e58ed958cd184545a034894c8ef24",
          "url": "https://github.com/ubugeeei/ush/commit/538a0563412cb831d5503bf56edc958ea844faf2"
        },
        "date": 1779133043696,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3206,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 84444,
            "range": "± 2949",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 111491,
            "range": "± 673",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d057251b90b38fbc5eb3e37d043256d952a0e00c",
          "message": "chore: add scripts/preflight.sh as a one-command CI mirror (#107)\n\nRuns every gate CI runs (rustfmt, clippy, workspace tests, release\ntests, no_std check/tests, rustdoc, bench build, line limit,\nrustyline drift, installer, shellcheck if available, and\n`ush check`/`ush format --check` on every example) in the same\norder CI runs them, with a banner for each section so the failing\ngate is obvious in plain `sh -e` output.\n\nCONTRIBUTING.md gets a one-line pointer.\n\nCo-authored-by: Claude Opus 4.7 (1M context) <noreply@anthropic.com>",
          "timestamp": "2026-05-19T04:18:38+09:00",
          "tree_id": "7b72cfee9b9798b9c318c58625afecdf17bf4187",
          "url": "https://github.com/ubugeeei/ush/commit/d057251b90b38fbc5eb3e37d043256d952a0e00c"
        },
        "date": 1779133078590,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3042,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 81909,
            "range": "± 1742",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 114893,
            "range": "± 724",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b11b35fffaa7abe48d522ea350094d27f5b7bab1",
          "message": "test(cli): extend smoke suite with `ush compile` and `ush check` (#105)\n\nPins two more outer-CLI guarantees that downstream automation\nrelies on but that nothing was previously asserting:\n\n- `ush compile <file.ush>` lowers a trivial program to POSIX `sh`\n  (output begins with `#!/bin/sh`, and `print \"hi\"` is lowered to\n  the expected `printf` invocation).\n- `ush check <file.ush>` exits 0 for a well-typed program.\n\nThe tests use the existing `tempfile` dev-dependency to materialise\ninputs, so they remain hermetic.\n\nCo-authored-by: Claude Opus 4.7 (1M context) <noreply@anthropic.com>",
          "timestamp": "2026-05-19T04:09:42+09:00",
          "tree_id": "552f55225600b145ec2abc4b2fcee2fc0256c262",
          "url": "https://github.com/ubugeeei/ush/commit/b11b35fffaa7abe48d522ea350094d27f5b7bab1"
        },
        "date": 1779133109947,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3018,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 81180,
            "range": "± 7459",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 101637,
            "range": "± 1165",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8a3172649442e9afa8d2aacfd92e5e61b5446d8f",
          "message": "feat(ush): install a user-facing panic hook on the CLI entrypoint (#108)\n\nWith `panic = \"abort\"` set in the release profile, the default\npanic message ends up looking like a raw rustc diagnostic to anyone\nwhose shell session just died. The custom hook replaces it with a\nconsistent, end-user-friendly message that:\n\n1. clearly identifies which binary panicked,\n2. surfaces the panic payload + source location, and\n3. points at the bug tracker with the version string already in\n   the line, so a copy-paste-into-issue actually contains enough\n   information to be triaged.\n\n`main()` calls `panic_hook::install()` first, before any other\nwork runs.\n\nCo-authored-by: Claude Opus 4.7 (1M context) <noreply@anthropic.com>",
          "timestamp": "2026-05-19T10:17:14+09:00",
          "tree_id": "31fd927442bf3176d3bba8ff3d1307636fd5e2a8",
          "url": "https://github.com/ubugeeei/ush/commit/8a3172649442e9afa8d2aacfd92e5e61b5446d8f"
        },
        "date": 1779153714680,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3053,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 82824,
            "range": "± 504",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 112549,
            "range": "± 1098",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "42fb7512205fc422b08c1051b07a630f6423094c",
          "message": "chore(release): bump version to 0.7.0 (#109)\n\nA production-readiness release. Folds the contents of `[Unreleased]`\ninto a fresh `## [0.7.0] — 2026-05-19` section in CHANGELOG.md and\nbumps `[workspace.package].version` from 0.6.1 → 0.7.0.\n\nHighlights of 0.7.0 (see CHANGELOG for the full list):\n\n- Compiler enforces match exhaustiveness in the effects pass; no\n  more silent fall-through on uncovered ADT variants.\n- Shell signal helpers move to sigaction(2) + checked pid_t casts.\n- install.sh hardens its trust surface (umask, pipefail, TLS pin).\n- CI matrix across Ubuntu + macOS, MSRV gate (now 1.88),\n  cargo-audit, cargo-deny, CodeQL, Gitleaks, Shellcheck, benchmark\n  regression gate, rustdoc -D warnings, `ush check` / `ush format\n  --check` over examples.\n- Linux aarch64 release archive.\n- Release profile uses LTO + strip + panic=abort, halving binary\n  sizes; user-facing panic hook on the CLI entrypoint.\n\nCo-authored-by: Claude Opus 4.7 (1M context) <noreply@anthropic.com>",
          "timestamp": "2026-05-19T10:22:09+09:00",
          "tree_id": "55abad44ef8c36c596d02b87ee42a055586d3fdb",
          "url": "https://github.com/ubugeeei/ush/commit/42fb7512205fc422b08c1051b07a630f6423094c"
        },
        "date": 1779154124927,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3229,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 83477,
            "range": "± 349",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 104414,
            "range": "± 1655",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0a6c9b57884081c64bb8e5d0b76a219e94467421",
          "message": "ci(deps): bump the actions group with 2 updates (#92)\n\nBumps the actions group with 2 updates: [github/codeql-action](https://github.com/github/codeql-action) and [actions/labeler](https://github.com/actions/labeler).\n\n\nUpdates `github/codeql-action` from 3 to 4\n- [Release notes](https://github.com/github/codeql-action/releases)\n- [Changelog](https://github.com/github/codeql-action/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/github/codeql-action/compare/v3...v4)\n\nUpdates `actions/labeler` from 5 to 6\n- [Release notes](https://github.com/actions/labeler/releases)\n- [Commits](https://github.com/actions/labeler/compare/v5...v6)\n\n---\nupdated-dependencies:\n- dependency-name: github/codeql-action\n  dependency-version: '4'\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n  dependency-group: actions\n- dependency-name: actions/labeler\n  dependency-version: '6'\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n  dependency-group: actions\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-05-19T12:09:12+09:00",
          "tree_id": "41c13a3feadcdcbe6c873a63fe6ad708c78d8fe3",
          "url": "https://github.com/ubugeeei/ush/commit/0a6c9b57884081c64bb8e5d0b76a219e94467421"
        },
        "date": 1779160261308,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3028,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 80292,
            "range": "± 1560",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 98754,
            "range": "± 1379",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "82c17aa63e7756826ddb55e0d5f08e79480d871a",
          "message": "docs(readme): add CodeQL and Secret-scan badges (#110)\n\nThe two security workflows added in 0.7.0 had no surface presence\nin the README. Adds their badges next to the existing CI /\nShellcheck / Dependencies / License row so the security posture is\nvisible at first glance.",
          "timestamp": "2026-05-19T12:10:29+09:00",
          "tree_id": "998729d132ab3c684f5a0802f9e9cc5a2a0ac6bc",
          "url": "https://github.com/ubugeeei/ush/commit/82c17aa63e7756826ddb55e0d5f08e79480d871a"
        },
        "date": 1779160349301,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3038,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 81267,
            "range": "± 1885",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 99833,
            "range": "± 5017",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "db8af1fb46cf63a9d8a09168df3de467e4d1aca5",
          "message": "docs: link release-process.md from the docs/ index (#111)\n\nThe doc landed in 0.7.0 but only README.md pointed at it; the docs/\nindex had no entry. Adds the entry so it discoverable from inside\nthe `docs/` tree.",
          "timestamp": "2026-05-19T12:11:46+09:00",
          "tree_id": "bdab7995e2966e42db9702ca4a06b5812e627a61",
          "url": "https://github.com/ubugeeei/ush/commit/db8af1fb46cf63a9d8a09168df3de467e4d1aca5"
        },
        "date": 1779160448818,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3026,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 80295,
            "range": "± 3553",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 100137,
            "range": "± 731",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "59e35cdafb13cd430b3ba2a05e7c36ee33d77799",
          "message": "docs(readme): add a Production readiness section (#112)\n\nSummarises the CI / supply-chain / runtime hardening posture that\nlanded across the 0.7.0 cycle (correctness, static analysis, supply\nchain, runtime, release pipeline, performance gate) so a downstream\noperator can read it in one place without spelunking the workflow\nfiles.",
          "timestamp": "2026-05-19T12:13:10+09:00",
          "tree_id": "51f557e4b150098b30fa435e2d1433760165fe59",
          "url": "https://github.com/ubugeeei/ush/commit/59e35cdafb13cd430b3ba2a05e7c36ee33d77799"
        },
        "date": 1779160514827,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3021,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 80387,
            "range": "± 988",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 98312,
            "range": "± 297",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5d3eda631164188dbe3e282009e55f9634b7d295",
          "message": "test(lsp): exercise a real LSP initialize/shutdown/exit handshake (#113)\n\nThe existing smoke tests only cover the CLI surface (--version,\n--help, unknown flag). They never actually spoke LSP, which means\na regression that broke initialize parsing, exit handling, or the\nJSON-RPC framing would slip through.\n\nThe new integration test:\n\n  - launches the real `ush_lsp` binary over a stdio pipe pair,\n  - sends `initialize`, asserts the response is shape-correct,\n  - sends `initialized`,\n  - sends `shutdown`, asserts the response,\n  - sends `exit` and asserts the process terminates cleanly,\n  - aborts the whole flow with a 5s deadline if any read blocks.\n\nAdds `serde_json` as a dev-dependency in `apps/ush_lsp/Cargo.toml`\n(it was already a workspace dep) so the test can parse responses.",
          "timestamp": "2026-05-19T12:15:30+09:00",
          "tree_id": "f0ff18c004e572b4f5b37acbd5cc8be7abbeba1c",
          "url": "https://github.com/ubugeeei/ush/commit/5d3eda631164188dbe3e282009e55f9634b7d295"
        },
        "date": 1779160658140,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3021,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 79930,
            "range": "± 4552",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 99104,
            "range": "± 564",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "64d13e60ae27fd46938ecfcdcf85dc5ad2635096",
          "message": "docs: add MAINTAINERS.md (#114)\n\nDocuments who owns the project, how to reach them (issue templates,\nSECURITY.md, CODEOWNERS), and who has release authority. Companion\nto CONTRIBUTING.md / SECURITY.md / CODE-of-conduct conventions that\nproduction-grade GitHub projects ship.",
          "timestamp": "2026-05-19T12:16:29+09:00",
          "tree_id": "eaef0da10dd9ca1e85073ae0d4086b1c26284cfc",
          "url": "https://github.com/ubugeeei/ush/commit/64d13e60ae27fd46938ecfcdcf85dc5ad2635096"
        },
        "date": 1779160710155,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3239,
            "range": "± 248",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 83204,
            "range": "± 409",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 103104,
            "range": "± 2551",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "29f8db61ea33a24c7c51d16fda01910d91a9e0d2",
          "message": "ci: pin every third-party action to a full commit SHA (#115)\n\nMutable major-version tags (e.g. `actions/checkout@v6`) are\ntrust-on-first-use: the same ref can be quietly retargeted to a\ndifferent commit at any time, so a compromised maintainer account\nupstream would silently rotate the action under us. Pinning to the\nfull commit SHA removes that class of supply-chain risk; Dependabot\nalready understands the format and will continue to open bump PRs\nfor each pinned SHA.\n\nPinned with the latest stable resolved SHA, with the human-readable\nversion preserved as a trailing comment so review diffs are still\nlegible:\n\n  - actions/checkout                 -> v6.0.2\n  - actions/labeler                  -> v6.1.0\n  - actions/upload-artifact          -> v7.0.1\n  - actions/download-artifact        -> v8.0.1\n  - actions/attest-build-provenance  -> v4.1.0\n  - Swatinem/rust-cache              -> v2.9.1\n  - softprops/action-gh-release      -> v3.0.0\n  - benchmark-action/github-action-benchmark -> v1.22.1\n  - github/codeql-action/{init,autobuild,analyze} -> v4.0.0\n\nThe two `dtolnay/rust-toolchain@stable` / `@master` references are\nintentionally left as moving refs: they exist precisely so the\ntoolchain rolls forward with upstream rustc releases, and pinning\nthem would require a fresh PR every few days for no real security\ngain (the action is install-only, not a privileged step).",
          "timestamp": "2026-05-19T12:20:01+09:00",
          "tree_id": "0a3ee018b4b8a46d28e50ddf4590120702decd9c",
          "url": "https://github.com/ubugeeei/ush/commit/29f8db61ea33a24c7c51d16fda01910d91a9e0d2"
        },
        "date": 1779160927359,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3010,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 80334,
            "range": "± 978",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 99027,
            "range": "± 466",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b535a02619ba94837eda8bfcae65a3a9d024d165",
          "message": "docs(template): expand the PR validation checklist + add Security block (#116)\n\nBrings the PR template in line with what current CI actually runs:\n\n- explicit `cargo +stable fmt` (matches the workflow)\n- no_std *tests* (not just `cargo check`)\n- rustdoc with `RUSTDOCFLAGS=-D warnings`\n- vendored rustyline drift script\n- shellcheck on install.sh + scripts/*.sh\n- `ush check` + `ush format --check` on examples\n- pointer to `scripts/preflight.sh` for the one-line shortcut\n\nAdds a new `## Security` block so PRs that touch install.sh, signal\nhandling, the release pipeline, or introduce a new third-party\naction don't slip through without a note.",
          "timestamp": "2026-05-19T12:21:35+09:00",
          "tree_id": "a37a6d0b00b102c4084cfa1fe6191e9289f05112",
          "url": "https://github.com/ubugeeei/ush/commit/b535a02619ba94837eda8bfcae65a3a9d024d165"
        },
        "date": 1779161048937,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3019,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 80408,
            "range": "± 421",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 100648,
            "range": "± 500",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "175d0f0d077041dff8b1320fd40008d8a1eef76d",
          "message": "ci: fix three CI gates that surfaced after the SHA-pin merge (#117)\n\n- **MSRV**: rustyline (vendored, v18.0.0) uses `std::fs::File::lock`\n  which only stabilised in 1.89. Bump `[workspace.package].rust-version`\n  from 1.88 → 1.89.\n- **Deny / bans**: cargo-deny's `wildcards = \"deny\"` treats the\n  workspace-internal `path = \"../foo\"` deps as wildcards. The end-user\n  binaries already opt out via `publish = false`, but the library\n  crates do not yet have a publish flow. Set `wildcards = \"warn\"`\n  and `allow-wildcard-paths = true`; flip back to `deny` once the\n  library crates either publish or also go `publish = false`.\n- **Format**: re-run `cargo +stable fmt --all`. There was no drift\n  in this PR's staged diff; the gate is recorded here so the next\n  contributor has a passing baseline.",
          "timestamp": "2026-05-19T12:24:53+09:00",
          "tree_id": "b1b9c717ea91b0a66b7536948b051b1ce4cd9d87",
          "url": "https://github.com/ubugeeei/ush/commit/175d0f0d077041dff8b1320fd40008d8a1eef76d"
        },
        "date": 1779161201834,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3254,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 84450,
            "range": "± 9135",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 104753,
            "range": "± 5874",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f1513f8c1cab69fb4d7a591fa59910475ead4bf7",
          "message": "ci: enable Dependabot security alerts for Cargo (advisories only) (#118)\n\n#48 disabled routine cargo version updates because of the vendored\nrustyline pin; that decision stands. But `open-pull-requests-limit:\n0` only suppresses version-update PRs — it leaves\nsecurity-advisory-driven PRs alive. Adding the cargo ecosystem with\nthat limit means a freshly-published CVE will still produce an\nupdate PR even though normal patch bumps will not.\n\nMirror it under the existing weekly schedule and label such PRs\n`dependencies` + `security` so they can be filtered.",
          "timestamp": "2026-05-19T12:26:26+09:00",
          "tree_id": "985587c54d10e2907ea97bfdf3b4f0b4ab4147d4",
          "url": "https://github.com/ubugeeei/ush/commit/f1513f8c1cab69fb4d7a591fa59910475ead4bf7"
        },
        "date": 1779161319186,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3226,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 82996,
            "range": "± 725",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 106237,
            "range": "± 1991",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b5354ace6454138fd0e5fc442c758bb65c8828ea",
          "message": "ci(deny): restrict the cargo-deny graph to released targets (#119)\n\nWithout an explicit `[graph].targets` list, cargo-deny inspects the\ntransitive graph for *every* target on the host, including\nWindows-only deps like `winapi` that we never build. Those deps\nwere causing duplicate-version / wildcard / license warnings the\nCI gate had to skip-tree around.\n\nConstrain the graph to the four targets our release workflow\nactually publishes:\n\n  - x86_64-unknown-linux-gnu\n  - aarch64-unknown-linux-gnu\n  - x86_64-apple-darwin\n  - aarch64-apple-darwin\n\n`cargo deny --workspace check` still reports\n\"advisories ok, bans ok, licenses ok, sources ok\" locally.",
          "timestamp": "2026-05-19T12:27:47+09:00",
          "tree_id": "9c3950251831400ce97067d4e87b97849cfc9b5f",
          "url": "https://github.com/ubugeeei/ush/commit/b5354ace6454138fd0e5fc442c758bb65c8828ea"
        },
        "date": 1779161483092,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3050,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 80605,
            "range": "± 3069",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 98326,
            "range": "± 616",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6e98e009675bdfde9cf9bfeb03ce784a241b562a",
          "message": "style: re-run rustfmt with --edition 2024 across the workspace (#121)\n\nCI's Format job has been failing since the MSRV bump because the\nGitHub-hosted runner's stable rustfmt honours edition 2024 import\nsort rules (alphabetical within `use foo::{...}` groups) but the\nhost (macOS) cargo fmt does not propagate `--edition 2024` from\nCargo.toml to rustfmt the same way.\n\nRun `find apps crates -name '*.rs' | xargs rustfmt --edition 2024`\nto apply the edition-2024-equivalent sort everywhere. Only\nre-orderings inside `use` brace groups; no semantic change.",
          "timestamp": "2026-05-19T12:47:07+09:00",
          "tree_id": "3e5e3632603d2205a6cf2da62f62a829faa22f56",
          "url": "https://github.com/ubugeeei/ush/commit/6e98e009675bdfde9cf9bfeb03ce784a241b562a"
        },
        "date": 1779162536358,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3084,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 81879,
            "range": "± 1482",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 112725,
            "range": "± 1167",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5a1200978f56e2ffcb40b0be17215ca6c26c0d4d",
          "message": "docs: add docs/architecture.md (top-down map of the workspace) (#122)\n\nA single page that answers: where do new contributors start? It\nlists every crate / app + its role, the two main request flows\n(running a .ush script, running the REPL), a \"symptom → file\"\nlookup table, the stability layers (none for the Rust API, the sh\nlowering is best-effort, CLI + LSP wire are pinned by smoke tests),\nand where new things should land.\n\nIndexed from `docs/README.md`.",
          "timestamp": "2026-05-19T12:50:13+09:00",
          "tree_id": "361777f8c2aed2136b4b3b5fe3a1d6b54f943164",
          "url": "https://github.com/ubugeeei/ush/commit/5a1200978f56e2ffcb40b0be17215ca6c26c0d4d"
        },
        "date": 1779162732851,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3222,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 83387,
            "range": "± 2077",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 104159,
            "range": "± 1246",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0bd6a7a64de607c97f29f961c6c9e37cc7df9e28",
          "message": "docs(contributing): document the rustfmt + edition 2024 gotcha (#123)\n\n`cargo fmt --all --check` on macOS hosts can silently disagree with\nCI's Linux rustfmt over edition-2024 import sort rules (alphabetical\ninside `use foo::{...}` brace groups), leaving CI to fail while the\nlocal check returns clean. Document the symptom and the workaround\n(direct `rustfmt --edition 2024` on the source tree) so future\ncontributors don't waste a round-trip on it.",
          "timestamp": "2026-05-19T12:51:33+09:00",
          "tree_id": "992104049c6fc091f47de3cb86655b7699d4d599",
          "url": "https://github.com/ubugeeei/ush/commit/0bd6a7a64de607c97f29f961c6c9e37cc7df9e28"
        },
        "date": 1779162867292,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3024,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 81356,
            "range": "± 1898",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 99121,
            "range": "± 406",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ec1b56e235ae9eab0b2cc6a397a65f4950f62fa2",
          "message": "ci: clippy the no_std compiler core too (#124)\n\nThe Clippy job runs `cargo clippy --workspace --all-targets`, which\nimplicitly enables every crate's default features — i.e. it clippies\n`ush_compiler` *with* `std`. Lints that only fire under no_std\n(`panic` formatter restrictions, `alloc`-only impls, missing\nno_std-aware paths, etc.) were therefore invisible. Add a dedicated\nno_std clippy step right next to the existing `cargo check` and\n`cargo test` no_std gates.",
          "timestamp": "2026-05-19T13:55:51+09:00",
          "tree_id": "6741ebcec031e0bb139287ef4e94a431171db0bc",
          "url": "https://github.com/ubugeeei/ush/commit/ec1b56e235ae9eab0b2cc6a397a65f4950f62fa2"
        },
        "date": 1779167384160,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3248,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 83379,
            "range": "± 2144",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 106243,
            "range": "± 1591",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "adfb1062af1855d852dd998bb3e5da30aa86b151",
          "message": "docs(compiler): add a crate-level rustdoc summary (#125)\n\n`cargo doc` and docs.rs displayed `ush_compiler` with a blank\nsummary. This adds a top-level `//!` block that:\n\n- explains the `std` vs `no_std + alloc` modes,\n- walks through the four pipeline stages (parse → import resolve →\n  effects pass → codegen),\n- notes that the public surface is intentionally narrow\n  (`UshCompiler` + four `compile_*` methods + the result types).\n\n`cargo doc -p ush_compiler -- -D warnings` stays clean.",
          "timestamp": "2026-05-19T13:57:35+09:00",
          "tree_id": "3e4e1f23deb804d70230594f505c0f6e271d9587",
          "url": "https://github.com/ubugeeei/ush/commit/adfb1062af1855d852dd998bb3e5da30aa86b151"
        },
        "date": 1779170051131,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3218,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 83925,
            "range": "± 2897",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 107759,
            "range": "± 932",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cfbfb35362a5ede4b529f834dbe618fea5f5be83",
          "message": "docs: add issue-template chooser config + external links (#126)\n\n`.github/ISSUE_TEMPLATE/config.yml`:\n\n- Disables the blank-issue escape hatch so new issues always land\n  on one of the curated templates (bug report / feature request).\n- Adds three external pointers on the chooser page:\n    * Security advisories (private vulnerability reporting)\n    * docs/release-process.md\n    * docs/architecture.md\n\nHelps anyone landing on the \"New issue\" page route to the right\nplace instead of filing a malformed bug or a security report in\npublic.",
          "timestamp": "2026-05-19T13:58:34+09:00",
          "tree_id": "811f94712092254e5ebb03bd38f95fc3cc141913",
          "url": "https://github.com/ubugeeei/ush/commit/cfbfb35362a5ede4b529f834dbe618fea5f5be83"
        },
        "date": 1779170142652,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3188,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 83525,
            "range": "± 1532",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 103452,
            "range": "± 1477",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a743669107a5568c3e422340b34c02ef25897c20",
          "message": "docs: add crate-level rustdoc to ush_shell, ush_tooling, ush_config (#127)\n\nCompanion to the `ush_compiler` crate-level doc that landed in #125.\nEach library crate now opens with a `//!` block summarising what it\nowns:\n\n- `ush_shell` — interactive REPL, shell parser, dispatch, helpers,\n  stylish renderers, rustyline integration.\n- `ush_tooling` — the formatter, diagnostics, and semantic-token\n  generation that the LSP + `ush format` / `ush check` consume.\n- `ush_config` — config / profile / rc resolution and the ush-owned\n  directories on disk.\n\nSo `cargo doc` and docs.rs no longer display these crates with a\nblank summary.",
          "timestamp": "2026-05-19T14:01:36+09:00",
          "tree_id": "ab9a0b8eb569c94ca4467544c3ee361d0691ddb7",
          "url": "https://github.com/ubugeeei/ush/commit/a743669107a5568c3e422340b34c02ef25897c20"
        },
        "date": 1779171745987,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3256,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 83179,
            "range": "± 4659",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 104133,
            "range": "± 1064",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "612be62598db0f740b4629af430d2642fa6562b3",
          "message": "ci(deny): promote multiple-versions from warn to deny (#128)\n\nThe only duplicate-version pair in the workspace's Cargo.lock now\nthat the cargo-deny graph is restricted to the released targets is\n`bitflags` v1 (pulled in via `fluent-uri` → `lsp-types`) vs v2\n(everything else). Add a single `skip = [\"bitflags@1.3.2\"]` entry\nand flip `multiple-versions = \"deny\"` so any *new* duplicate that\nlands without an explicit skip will fail CI.",
          "timestamp": "2026-05-19T14:03:42+09:00",
          "tree_id": "a76e8b2be67e71752ef000ab68d2968f90d1f55e",
          "url": "https://github.com/ubugeeei/ush/commit/612be62598db0f740b4629af430d2642fa6562b3"
        },
        "date": 1779171906503,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3034,
            "range": "± 169",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 79927,
            "range": "± 8568",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 98302,
            "range": "± 1332",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a1993362899391f41a5b5c6758bd423664774a1d",
          "message": "docs: add SUPPORT.md (bugs, security, support, supported versions) (#129)\n\nStandard top-level GitHub community-health file. GitHub\nauto-surfaces it next to README on the repo page and in the\n\"Insights → Community Standards\" check.\n\nRoutes:\n\n- Bugs / feature requests → issue templates\n- Security → SECURITY.md private channel\n- Usage questions → README + docs/ index\n- Lists which release lines are supported and where the user-visible\n  changelog lives.",
          "timestamp": "2026-05-19T14:04:46+09:00",
          "tree_id": "b4c243fc0253c12f07a49e4a573a2d487ab5903a",
          "url": "https://github.com/ubugeeei/ush/commit/a1993362899391f41a5b5c6758bd423664774a1d"
        },
        "date": 1779171992229,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3239,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 83275,
            "range": "± 3960",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 103952,
            "range": "± 1367",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "618b04e4835370236ff6844272cf7411f7148c0f",
          "message": "chore(release): bump version to 0.8.0 (#130)\n\nA polish-pass release on top of 0.7.0. Focus is CI hardening,\ncontributor-facing docs, and shared infrastructure. See\nCHANGELOG.md's `## [0.8.0]` section for the full list of changes.\n\n- Cargo.toml: workspace version 0.7.0 → 0.8.0\n- CHANGELOG.md: move the [Unreleased] body into a dated\n  `## [0.8.0] — 2026-05-19` section, leave a fresh empty\n  [Unreleased] at the top, wire the new compare-link.",
          "timestamp": "2026-05-19T14:12:15+09:00",
          "tree_id": "3fbed51c11d9ee474ae02871e88e37ffc4a8ea50",
          "url": "https://github.com/ubugeeei/ush/commit/618b04e4835370236ff6844272cf7411f7148c0f"
        },
        "date": 1779171999424,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3201,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 83274,
            "range": "± 5703",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 103554,
            "range": "± 549",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "90e40df09974747b8d0999605b2564a1b73dcbf0",
          "message": "feat(lsp): implement documentHighlight and documentSymbol (#131)\n\nTwo new LSP capabilities for `.ush`:\n\n- `textDocument/documentHighlight` — given a cursor position, returns\n  every occurrence of the same identifier in the document so the\n  editor can underline / outline them. Reuses the existing semantic\n  tokenizer so \"what counts as an identifier\" matches the syntax\n  highlighter (variable / function / type / property; not keywords\n  or strings).\n- `textDocument/documentSymbol` — returns a flat outline of the\n  top-level declarations (fn / enum / trait / type / let / alias).\n  Syntactic scan; idiomatically skipped inside `\"\"\"…\"\"\"` blocks.\n\nNew module `ush_tooling::highlight` (`Highlight`, `HighlightKind`,\n`document_highlights`) and `ush_tooling::symbol` (`DocumentSymbol`,\n`SymbolKind`, `document_symbols`) provide editor-agnostic engines;\n`apps/ush_lsp` converts to `lsp_types` and wires both into\n`ServerCapabilities`.\n\nServer tests:\n- `apps/ush_lsp/tests/highlight_and_symbols.rs` drives the real\n  binary over stdio and asserts the response shape end-to-end.\n- Unit tests inside the new modules pin the cursor/keyword/whitespace\n  edge cases and the multi-line-string outline behaviour.",
          "timestamp": "2026-05-19T14:20:15+09:00",
          "tree_id": "a5cb05b6d57d632e75380430750b07efbc2245a0",
          "url": "https://github.com/ubugeeei/ush/commit/90e40df09974747b8d0999605b2564a1b73dcbf0"
        },
        "date": 1779172562682,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3035,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 81183,
            "range": "± 2769",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 100824,
            "range": "± 1561",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ef4cfa3d8d640e5146188b3543edf1b27ac11127",
          "message": "feat(lsp): implement completion and foldingRange (#133)\n\nTwo more LSP capabilities for `.ush`, on top of the documentHighlight\nand documentSymbol PR:\n\n- `textDocument/completion` — returns every `.ush` keyword plus\n  every identifier (variable / function / type) the semantic\n  tokenizer has classified in the open document. No cursor-context\n  resolution yet; the editor filters.\n- `textDocument/foldingRange` — reports folding regions for matched\n  `{ … }` blocks; correctly ignores braces inside `\"…\"`, `'…'`,\n  `\"\"\"…\"\"\"`, line comments (`# …`), and attribute brackets `#[…]`.\n\nNew modules `ush_tooling::completion` (`CompletionItem`,\n`CompletionKind`, `completions`) and `ush_tooling::folding`\n(`FoldingRange`, `folding_ranges`) carry the editor-agnostic\nengines; `apps/ush_lsp` converts to `lsp_types`, declares the new\ncapabilities, and routes the two new request handlers.\n\nUnit tests in each module pin the keyword set, deduplication,\nattribute / comment / string brace skipping, and nested block\nbehaviour.",
          "timestamp": "2026-05-19T14:26:30+09:00",
          "tree_id": "877d012b0d11bb483307f9bf47c7d6037a6e66a1",
          "url": "https://github.com/ubugeeei/ush/commit/ef4cfa3d8d640e5146188b3543edf1b27ac11127"
        },
        "date": 1779172565313,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3024,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 79814,
            "range": "± 3133",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 98867,
            "range": "± 687",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8907d1d69cb5081534e700820332003bd5f5d3cd",
          "message": "feat(lsp): implement textDocument/hover (#135)\n\nHovering over a keyword now shows its single-line documentation;\nhovering over a variable / function / type / property identifier\nshows its role plus the declaring source line as a Markdown code\nblock.\n\n- New module `ush_tooling::hover` (`Hover`, `hover`).\n- `apps/ush_lsp` declares `hover_provider` and routes\n  `textDocument/hover` to the new engine via `convert::hover`.\n- Unit tests cover keyword / identifier / whitespace / inside-string\n  cases.\n\n`ush_compiler`'s AST is private, so this is a syntax-level hover —\nthere is no inferred type yet. The wiring is in place so a richer\nhover can be layered on later without touching the LSP plumbing.",
          "timestamp": "2026-05-19T14:31:28+09:00",
          "tree_id": "5b6332adf58c127f3143c4c2fc83aa227f801a4d",
          "url": "https://github.com/ubugeeei/ush/commit/8907d1d69cb5081534e700820332003bd5f5d3cd"
        },
        "date": 1779172785860,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3187,
            "range": "± 148",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 82914,
            "range": "± 3515",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 107978,
            "range": "± 3310",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "80811d0f15a8909cc503601e462ceda980181520",
          "message": "feat(lsp): implement textDocument/{references,definition,rename} (#136)\n\nThree more LSP capabilities for `.ush`, all reusing the\nidentifier-based engine in the new `ush_tooling::references`\nmodule:\n\n- `textDocument/references` — every occurrence of the identifier\n  under the cursor.\n- `textDocument/definition` — the first occurrence (the project does\n  not track scoped declarations yet; \"definition\" = \"first sighting\"\n  inside the file).\n- `textDocument/rename` — returns a `WorkspaceEdit` with one\n  `TextEdit` per occurrence; rejects names that are not valid `.ush`\n  identifiers with a clear LSP error.\n\n`apps/ush_lsp` declares the matching `definition_provider`,\n`references_provider`, and `rename_provider` capabilities and routes\neach method.\n\nTests:\n- 5 new unit tests in `ush_tooling::references` (occurrences,\n  first-sighting, keyword-no-op, invalid-name rejection).\n- New `apps/ush_lsp/tests/refs_definition_rename.rs` drives the\n  real binary end-to-end and asserts the response shape for the\n  happy path AND the rename-with-invalid-name error.",
          "timestamp": "2026-05-19T14:54:38+09:00",
          "tree_id": "0458288ed5410a112bfe9b9dab4dab5d6973571b",
          "url": "https://github.com/ubugeeei/ush/commit/80811d0f15a8909cc503601e462ceda980181520"
        },
        "date": 1779172991505,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3035,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 80297,
            "range": "± 601",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 98598,
            "range": "± 290",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9b3b92285019e6c0bfe1a0410329c3acb4c0ea61",
          "message": "feat(lsp): implement textDocument/prepareRename (#137)\n\nThe rename provider in the previous PR validated `params.newName`\n*after* the user had already typed it. With prepareRename the LSP\nclient can validate the cursor position first and pop the rename\ninput only when the cursor actually sits on a renameable identifier,\npre-filling the existing name as the placeholder.\n\n- New `prepare_rename(source, line, char) -> Option<Reference>` in\n  `ush_tooling::references` returns the identifier range when the\n  cursor is on a renameable token, or `None` for keywords, strings,\n  whitespace, etc.\n- `apps/ush_lsp` flips `RenameOptions.prepare_provider` to `true`\n  and routes `textDocument/prepareRename` to the new engine.\n- Two new unit tests pin the identifier-range / keyword cases.",
          "timestamp": "2026-05-19T14:57:45+09:00",
          "tree_id": "47be86572d2c6e4bd52f2ccb5410afd5f4294d3d",
          "url": "https://github.com/ubugeeei/ush/commit/9b3b92285019e6c0bfe1a0410329c3acb4c0ea61"
        },
        "date": 1779173033005,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3043,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 82357,
            "range": "± 1208",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 112683,
            "range": "± 423",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cebdedcec58b0faea02595b1e549fa94c4469a81",
          "message": "docs(lsp): document every LSP capability and its backing engine (#138)\n\nThe bullet list under \"Capabilities\" in docs/lsp.md predated the\nhover / highlight / symbol / fold / completion / refs / def / rename /\nprepareRename work and only listed sync + diagnostics + format +\nsemantic tokens.\n\nReplace it with a table that pairs each LSP method with the\n`ush_tooling` module that backs it, plus a \"not yet implemented\"\nfootnote for the methods that still need typed info from the\ncompiler (signatureHelp / codeAction / inlayHint / callHierarchy /\nworkspaceSymbol).",
          "timestamp": "2026-05-19T14:59:15+09:00",
          "tree_id": "6306a5ebf76ab538656603e6dd5978e90ae1e81f",
          "url": "https://github.com/ubugeeei/ush/commit/cebdedcec58b0faea02595b1e549fa94c4469a81"
        },
        "date": 1779173444285,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3270,
            "range": "± 178",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 83086,
            "range": "± 904",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 108223,
            "range": "± 1392",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "678d9087815841403ab22406468fb37afda0cbc3",
          "message": "feat(lsp): implement textDocument/signatureHelp (#139)\n\nScans the source for `fn name(parameters)` declarations and,\nwhen the cursor is inside a call `name(arg, arg, …)`, returns\nthe matching signature with the currently-active parameter index.\n\nScope (intentionally narrow at this iteration):\n- Only `fn` declarations whose opening `(` and matching `)` are on\n  the same line are recognised. Multi-line signatures land later.\n- No scope resolution; first declaration in the file with a matching\n  name wins.\n- The active-parameter counter is plain comma counting at depth 1\n  inside the enclosing call. Commas inside string literals and inner\n  call parens are ignored.\n\n`apps/ush_lsp` declares a `signature_help_provider` whose trigger\ncharacters are `(` and `,`, and routes `textDocument/signatureHelp`.\n\nSix unit tests cover: fn parsing, active-param across a comma,\nout-of-call cursor, unknown function, no-parameter function, and\nnested calls (inner signature wins).",
          "timestamp": "2026-05-19T15:25:40+09:00",
          "tree_id": "7a2202ac5b21272574ba4c1d4ae2bae4629978b5",
          "url": "https://github.com/ubugeeei/ush/commit/678d9087815841403ab22406468fb37afda0cbc3"
        },
        "date": 1779173570885,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3034,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 80465,
            "range": "± 624",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 99699,
            "range": "± 837",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ceeb1b1482e307194e6698153bbee840d0186b9e",
          "message": "chore(release): bump version to 0.9.0 (#141)\n\nLSP build-out release. The stdio language server gains nine new\ncapabilities (documentHighlight, documentSymbol, foldingRange,\nhover, completion, definition, references, prepareRename, rename,\nsignatureHelp) so editors can drive `.ush` files the same way they\ndrive Rust / TypeScript without changing the existing format /\ncheck / publishDiagnostics / semanticTokens plumbing.\n\n- Cargo.toml: workspace version 0.8.0 → 0.9.0.\n- CHANGELOG.md: move the [Unreleased] body into a dated\n  `## [0.9.0] — 2026-05-19` section and wire the new compare-link.\n\nSee CHANGELOG for the full list. docs/lsp.md was updated in the\n0.9.0 cycle with a table of every implemented method and its\nbacking engine.",
          "timestamp": "2026-05-19T15:33:24+09:00",
          "tree_id": "629da21e58d775d5fd4f957c17ad5f669edab15b",
          "url": "https://github.com/ubugeeei/ush/commit/ceeb1b1482e307194e6698153bbee840d0186b9e"
        },
        "date": 1779173638977,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3237,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 83450,
            "range": "± 2635",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 107915,
            "range": "± 1690",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "16690ef8e20d64836024505971d4f45384c93207",
          "message": "refactor(lsp): split large modules (#144)",
          "timestamp": "2026-06-01T00:32:29+09:00",
          "tree_id": "495c18e8bda4f421f13a2fa36f7d668fe9083aa8",
          "url": "https://github.com/ubugeeei-prod/ush/commit/16690ef8e20d64836024505971d4f45384c93207"
        },
        "date": 1780241915190,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 3194,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 83682,
            "range": "± 2269",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 104280,
            "range": "± 1859",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4156642cabeec9881ad7e8ed7d89f2386015228c",
          "message": "test: expand coverage to 861 tests, fix six bugs, cut hot-path cost (#152)\n\nGrow the suite from 354 to 861 tests across every crate, then fix the\nbugs that coverage exposed and optimise the paths the tests pinned\ndown. The three parts ship together because the fixes were found by\nthe new tests and the optimisations are guarded by them.\n\nBugs found and fixed:\n\n- `continue` inside a `for` loop hung the shell. Both the range and\n  list lowerings advanced the loop counter at the end of the body, so\n  `continue` skipped it and the generated `while` spun forever.\n- `if let` on a non-matching variant aborted the script: the binding\n  was read before the tag test, so under `set -u` a payload slot\n  belonging to another variant was an unbound variable.\n- A failing `.ush` script could exit 0. The runtime source-map `EXIT`\n  trap ended on the status of its own last command rather than the\n  script's.\n- `ush check` and the LSP showed only the outermost error frame,\n  which for most compiler errors is just `line N`. They now render\n  the whole context chain and point at the innermost line.\n- `alias ls='ls --color=auto'` expanded eight times. Alias expansion\n  now refuses to re-expand a name already being expanded, which also\n  terminates mutually recursive aliases.\n- The semantic tokenizer never left multi-line-string mode, so every\n  line after a `\"\"\"` block highlighted as a string.\n\nPlus smaller fixes: `fg`/`bg` on a job that finished between the\nstatus refresh and the resume signal, `ls` errors that did not name\nthe path, `make` `define … endef` bodies leaking into completion, and\n`1 npm` reading as `1 npm task`.\n\nPerformance:\n\n- REPL prompts re-read every `PATH` directory on every command. The\n  completion name set is now cached behind a `stat` per `PATH` entry\n  plus the alias table: ~2.7 ms -> ~59 us per prompt.\n- `parse_line` is about 2x faster. The POSIX-fallback keyword scan\n  ran the line once per keyword and allocated a `String` each time;\n  it is now a single allocation-free pass. Builtin lookup uses a\n  perfect-hash set, and comment stripping and pipeline splitting\n  borrow instead of allocating.\n- Variable expansion walks bytes and borrows its input when there is\n  nothing to expand, instead of collecting into a `Vec<char>` and\n  allocating a `String` per variable name.\n- `.ush` -> `sh` compilation is ~30% faster: the output buffer counts\n  line breaks with `memchr` instead of decoding every character, and\n  block indentation no longer allocates a string per line.\n\nThe benchmark suites now cover the native and fallback parser paths,\nalias expansion, helper pipelines, and an empty compile (the fixed\nruntime prelude), so CI tracks all of the above against `main`.\n\nSnapshot suites gained an `UPDATE_SNAPSHOTS=1` path, documented in\nCONTRIBUTING.md alongside the rest of the local CI flow.\n\nCo-authored-by: Claude Opus 5 <noreply@anthropic.com>",
          "timestamp": "2026-08-31T14:55:48+09:00",
          "tree_id": "98958f41630fc61f096d4f27dc194f9ef96c8c74",
          "url": "https://github.com/ubugeeei-prod/ush/commit/4156642cabeec9881ad7e8ed7d89f2386015228c"
        },
        "date": 1788156027334,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1330,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 211,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 655,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 1152,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 1206,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 1092,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/boolean chain",
            "value": 171,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 154,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 99,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 177,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 799,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 2151,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 39928,
            "range": "± 386",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 47547,
            "range": "± 2208",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 65656,
            "range": "± 923",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "69c48c62271c7a7ef390d30c73b2870129819d2a",
          "message": "build(deps): clear four RUSTSEC advisories (#153)\n\n`cargo audit` and `cargo deny` have been failing on `main` since the\nadvisories landed. Bump the three affected crates to their fixed\nversions:\n\n- `anyhow` 1.0.102 -> 1.0.104 — RUSTSEC-2026-0190, unsoundness in\n  `Error::downcast_mut()` after `Error::context`.\n- `quick-xml` 0.40.1 -> 0.42.0 — RUSTSEC-2026-0194 (quadratic run\n  time checking a start tag for duplicate attribute names) and\n  RUSTSEC-2026-0195 (unbounded namespace-declaration allocation in\n  `NsReader`, a memory-exhaustion DoS).\n- `crossbeam-epoch` 0.9.18 -> 0.9.20 — RUSTSEC-2026-0204, invalid\n  pointer dereference in the `fmt::Pointer` impl for `Atomic` and\n  `Shared`. Transitive through `criterion`/`rayon`, so the lockfile\n  is bumped directly.\n\nNo source changes were needed: the `xml` helper is the only\n`quick-xml` consumer and its `Reader`/`Writer` usage is unchanged\nacross 0.40 -> 0.42. Every bumped crate declares an MSRV well below\nthe workspace's 1.89 (1.86, 1.68, and 1.61 respectively).\n\nCo-authored-by: Claude Opus 5 <noreply@anthropic.com>",
          "timestamp": "2026-08-31T15:14:04+09:00",
          "tree_id": "aafc9dd833d352448ebdb0a5c77cf175c31e4bd8",
          "url": "https://github.com/ubugeeei-prod/ush/commit/69c48c62271c7a7ef390d30c73b2870129819d2a"
        },
        "date": 1788157132339,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1661,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 274,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 691,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 1259,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 1314,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 1189,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/boolean chain",
            "value": 228,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 210,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 137,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 235,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 854,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 2774,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 49968,
            "range": "± 1058",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 56694,
            "range": "± 481",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 78613,
            "range": "± 368",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4be1d7254032c0d0fcc86b0cf587a5501ac66b41",
          "message": "fix: make ush usable as a login/editor shell, map generated-sh errors, add an effect system (#154)\n\n* fix(shell): make ush usable as the shell an editor terminal starts\n\nSeven things went wrong the moment `ush` was launched by something\nother than a hand-typed `ush`:\n\n- `-i` was rejected outright. Editor terminals and agent runners\n  spawn `$SHELL -i -c ...`, so every such session died at argv\n  parsing. `-i` is now accepted and loads the rc file.\n- A login shell is started with a `-` in front of `argv[0]`, not with\n  `-l`. That convention was ignored, so the profile never ran and a\n  `PATH` set up there looked discarded. Login shells now also read\n  `/etc/profile`, which is what runs `path_helper` on macOS.\n- Nothing at all was read for a bare `ush -c ...` — the invocation\n  tools actually use. Adds the `.zshenv` tier: `~/.config/ush/env.sh`\n  / `~/.ush_env`, read on every invocation, with `--env-file` and\n  `--no-env`.\n- Command lookup resolved through the *process* `PATH` and spawned a\n  bare program name, which `execvp` also resolves through the parent\n  `PATH`. A `PATH` exported at the prompt or from a startup file\n  therefore decided nothing. Lookup now uses the shell's own `PATH`\n  and spawns the resolved absolute path.\n- `a && b`, `a || b`, and `a; b` were handed to `/bin/sh` whole, so\n  `cd`, `sammary`, `tasks`, the structured helpers, and session\n  aliases were \"command not found\" in any chained line. They are now\n  parsed and run as a real and-or list, with POSIX short-circuiting.\n  Compound commands (`if`, `for`, `case`, `(`, `{`) still go to\n  `/bin/sh` in one piece. A newline separates commands too, which\n  fixes multi-line `ush -c`.\n- `rm -rf` blocked reading a confirmation from a stdin pipe nobody\n  was ever going to write to — the \"rm is extremely slow\" report. It\n  now declines with an actionable message when there is no terminal\n  to ask on.\n- The prompt could only be a literal string. `PS1` and `USH_PROMPT`\n  are now honoured, with the familiar escapes (`\\w`, `\\u`, `\\h`,\n  `\\$`, `\\e`, …) plus `\\g` for the git branch and `\\?` for the last\n  status.\n\nAlso: the completion menu now moves with the arrow keys instead of\nthrowing itself away and jumping through history; `cd` keeps the\nlogical path (`/tmp`, not `/private/tmp`) and understands `cd -`; and\na word that merely looks like a glob no longer fails the command.\n\nCo-Authored-By: Claude Opus 5 <noreply@anthropic.com>\n\n* feat(compiler): map generated-shell failures back to .ush source\n\nA `.ush` script runs as generated `sh`, so when it fails the shell\ncomplains about its own text: `/bin/sh: line 490: ...`. Nothing\nconnected that number to the program the author wrote.\n\nThe instrumentation now knows how tall its own header is, so the\nruntime report prints the line number `/bin/sh` uses alongside the\nsourcemap id, plus the exit status and the exact command to run next:\n\n    /bin/sh: line 490: definitely_not_a_real_command: command not found\n\n    ush runtime map: script.ush:3 (exit 127)\n      source : $ definitely_not_a_real_command\n      section: user-code\n      shell  : line 490 | G0451 | definitely_not_a_real_command\n      mapped : G0451\n      explain: ush explain script.ush 490\n\n`ush explain <script.ush> <LINE>...` is the other half. It accepts\neither a `/bin/sh` line number or a `G####` sourcemap id and prints\nthe generated line, the `.ush` line behind it with surrounding\ncontext, and the rest of the group the source line lowered into. A\nnumber that lands inside the scaffolding says so instead of pointing\nsomewhere misleading.\n\nCo-Authored-By: Claude Opus 5 <noreply@anthropic.com>\n\n* feat(compiler): add a side-effect system\n\n`.ush` already tracked which errors a function can raise. It said\nnothing about what a function *touches*, so a signature gave no hint\nthat a helper shells out, reads the environment, or reaches the\nnetwork — the thing you most want to know before calling it from a\nscript that runs unattended.\n\nEffects are now inferred for every function, bottom-up from the\nstdlib and propagated through calls by fixpoint (so mutual recursion\nresolves rather than reporting one side pure):\n\n    $ ush effects examples/effects.ush\n    greeting_target  env  (declared)\n    report           io, env, exec\n    slug             pure  (declared)\n    stamp            exec  (declared)\n    (top level)      io, env, exec\n\nSix effects: `io`, `fs`, `env`, `net`, `exec`, `task`. `std::string`\nand `std::regex` are pure; `std::path` is split, because half of it\nis path algebra and the other half asks the filesystem.\n\nAnnotating turns the inference into a check. `#[effects(fs, exec)]`\nis an upper bound — declaring more than the body uses is fine, so a\npublic signature can stay stable while an implementation shrinks —\nand a body that outgrows its row is a compile error, reported by\n`ush check` and by running the script. `#[pure]` allows nothing.\n\nCo-Authored-By: Claude Opus 5 <noreply@anthropic.com>\n\n* docs: document the effect system, source mapping, and startup tiers\n\nAlso cuts 0.10.0: the release carries the session fixes that make\n`ush` usable as the shell an editor terminal starts, the generated-sh\nsource mapping, and the new effect system.\n\nCo-Authored-By: Claude Opus 5 <noreply@anthropic.com>\n\n* fix(shell): keep comments and failing builtins from derailing a list\n\nTwo things surfaced once `&&`, `;`, and newlines started being parsed\nby `ush` instead of `/bin/sh`:\n\n- Comment stripping truncated the whole input at the first unquoted\n  `#`. On a single line that was invisible; on a multi-line `-c`\n  string or a paste it dropped every command after a commented one,\n  and an operator inside the comment could split the line. Comments\n  now end at their own newline, with quote state tracked across\n  lines.\n- A builtin reports a Rust error rather than an exit status, so\n  `cd missing || echo fallback` abandoned the line instead of\n  running the fallback. Inside a list the error is reported and\n  becomes status 1, which is what the next element tests.\n\nCo-Authored-By: Claude Opus 5 <noreply@anthropic.com>\n\n* refactor: split the files that outgrew the 250-line limit\n\n`scripts/check_rs_line_limit.sh` caps a Rust file at 250 lines. Four\nfiles crossed it, so each is split along the seam it already had:\n\n- `parser.rs` → `parser/list.rs` (and-or lists) and\n  `parser/comment.rs` (comment stripping)\n- `effects/side.rs` → `side/kinds.rs` (the effect bitset),\n  `side/infer.rs` (the walk), `side/table.rs` (the stdlib table)\n- `ush_compiler/lib.rs` → `tests.rs`\n- `runtime_diagnostics.rs` → `runtime_diagnostics/header.rs`\n\nCo-Authored-By: Claude Opus 5 <noreply@anthropic.com>\n\n* perf(parser): keep the and-or split off the hot path\n\nParsing every line for `&&` / `||` / `;` slowed down every line that\nhas none of them, which is most of them. Three changes put the cost\nback where it belongs:\n\n- One `memchr3` for `;`, a newline, or `&`, plus a search for `||`,\n  rejects a line before anything walks it. A pipeline's single `|`\n  is not a separator, so pipelines take the fast exit too.\n- The POSIX-keyword scan moved ahead of the byte walk and its result\n  is threaded into the fallback check, so a line is scanned for\n  keywords once rather than twice.\n- Comment stripping keeps its single-pass borrowed form for\n  single-line input, taking the multi-line path only when there is a\n  newline to worry about.\n\nAgainst `main`, every existing case is now within ~10% (several are\nfaster). Two notes on the benchmark numbers:\n\n- `parse comment only` goes 4.8 ns → 7.6 ns, the cost of learning\n  whether the input has a newline before deciding to borrow.\n- The `boolean chain` case moved out of `parse fallback line` into a\n  new `parse and-or list` group, because the thing it measures\n  changed: it used to time copying the line into a `Fallback` for\n  `/bin/sh` (~175 ns), and it now times parsing both commands\n  (~1.5 µs). That is the fix, not a regression — the old path is\n  exactly why `mkdir x && cd x` could not reach the `cd` builtin —\n  but it is a different measurement, so it gets a different name and\n  a fresh baseline.\n\nAlso fixes two things CI caught: the `ush explain` test now reads the\nline number out of dash's `sh: 490:` wording as well as bash's\n`sh: line 490:`, and the effects tests import `ToString` so the\n`no_std` build of `ush_compiler` still compiles them.\n\nCo-Authored-By: Claude Opus 5 <noreply@anthropic.com>\n\n* docs(parser): drop an intra-doc link to a test-only helper\n\n`needs_posix_fallback` is `#[cfg(test)]` now, so the link resolved\nto nothing in a normal doc build and `cargo doc --document-private-items`\nwith `-D warnings` failed.\n\nCo-Authored-By: Claude Opus 5 <noreply@anthropic.com>\n\n* perf(parser): keep the newline probe off lines without a comment\n\nComment stripping only needs to know whether the input spans several\nlines once it has actually found a comment, so the scan for a\nnewline moved behind that check. A line with no `#` in it now leaves\n`strip_comment` after exactly the walk it always did.\n\nThe one case that still costs more than before is a line that is\nnothing but a comment (`parse comment only`, 13 ns → 18 ns on CI):\nthe comment starts at the top, so the newline probe covers the whole\nline. That is the price of a comment ending at its own newline\ninstead of truncating everything after it, which is what silently\ndropped commands from a multi-line `-c` string.\n\nCo-Authored-By: Claude Opus 5 <noreply@anthropic.com>\n\n---------\n\nCo-authored-by: Claude Opus 5 <noreply@anthropic.com>",
          "timestamp": "2026-09-06T03:12:40+09:00",
          "tree_id": "a9fef280ffeaa9a1dd186db2877d6d275e0dd8ca",
          "url": "https://github.com/ubugeeei-prod/ush/commit/4be1d7254032c0d0fcc86b0cf587a5501ac66b41"
        },
        "date": 1788632324145,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1650,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 272,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 672,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 1237,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 1237,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 1155,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 214,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 135,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 244,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 121,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1507,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 1305,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 1221,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 848,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 2678,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 51164,
            "range": "± 365",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 58369,
            "range": "± 803",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 79991,
            "range": "± 2898",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "687b36d8b07c5bd5a7fe6645de419d312a6b56c8",
          "message": "feat(compiler): rebuild the effect system on Effekt's model (#155)\n\nThe first cut tracked effects with an attribute (`#[effects(fs)]`).\nThat records what a function touches but misses the point of an\neffect system: an effect is a *requirement* a computation has of its\ncaller, and the interesting operation is handling one.\n\nReworked to follow Effekt. The row lives in the signature, after the\nreturn type, and says what the body still needs:\n\n    effect log(message: String) -> ()\n\n    fn greet(name: String) -> String / { log } {\n      do log(\"greeting \" + name)\n      \"hello \" + name\n    }\n\n    try {\n      print greet(\"ubu\")\n    } with log { (message) =>\n      print \"[log] \" + message\n    }\n\n`effect` declares an operation, `do` performs it, and `try … with`\ndischarges it for the block it wraps. The handler's value is what\n`do` evaluates to, so the body carries on where it left off — a\ntail-resumptive handler, which is as far as POSIX `sh` goes without\ndelimited continuations. Handlers nest and shadow, and an effect that\nreaches the top of the program is a compile error, the way Effekt\ndemands of `main`.\n\nLowering keeps it readable: an `effect` declaration becomes a shell\nfunction that forwards to whichever handler is installed, and a\nhandler is compiled as an ordinary function the indirection points at\nfor the duration of the body. Save/restore around that binding is\nwhat makes nesting work.\n\nThe six built-ins (`io`, `fs`, `env`, `net`, `exec`, `task`) stay:\nthey describe what the generated shell does, are inferred rather than\nperformed, and no handler can take them back. `/ {}` replaces\n`#[pure]`, and an omitted row still means \"infer, don't check\".\n\nAlso fixed on the way: `parse_brace_body` panicked on a line where\n`}` precedes `{` (it sliced before checking the range ran forwards),\nand `ush format` flattened the body of a block that opens on a\nclosing line.\n\nCo-authored-by: Claude Opus 5 <noreply@anthropic.com>",
          "timestamp": "2026-09-06T04:47:45+09:00",
          "tree_id": "7b1abaf766af38ba60520e87c956765b1e102478",
          "url": "https://github.com/ubugeeei-prod/ush/commit/687b36d8b07c5bd5a7fe6645de419d312a6b56c8"
        },
        "date": 1788637979635,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1660,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 292,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 683,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 1232,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 1285,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 1149,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 206,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 134,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 244,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 121,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1486,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 1292,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 1215,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 841,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 2654,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 51447,
            "range": "± 1257",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 57741,
            "range": "± 1491",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 79284,
            "range": "± 623",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "531e011e53d53747a3b8835f86787b91b241ee2d",
          "message": "fix(deps): replace unmaintained chrono (#157)\n\n* fix(deps): replace unmaintained chrono\n\n* fix(ci): restore supported runners and advisory checks",
          "timestamp": "2026-11-20T21:31:32+09:00",
          "tree_id": "f59831ffc67294029033c852cb32236721fdfaf5",
          "url": "https://github.com/ubugeeei-prod/ush/commit/531e011e53d53747a3b8835f86787b91b241ee2d"
        },
        "date": 1795178278719,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1048,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 153,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 384,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 939,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 927,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 870,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 162,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 182,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1061,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 895,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 802,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 548,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 1845,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 10,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 30887,
            "range": "± 436",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 38685,
            "range": "± 240",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 63838,
            "range": "± 420",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f9ee68d131fa95905426332ea996d058fb336b5a",
          "message": "feat(language)!: add typed shell interoperability and portable targets (#160)\n\n* feat(language)!: add typed shell interoperability and portable targets\n\n* fix(checker): enforce return paths and propagate awaited expression failures\n\n* fix(compiler): allow unit functions to end in conditional failure\n\n* fix(release): qualify shell helpers and the MSRV-compatible line editor\n\n* perf(compiler): build the generated line index only when requested\n\n* ci(perf): compare PR and base benchmarks on the same runner",
          "timestamp": "2026-11-23T07:11:08+09:00",
          "tree_id": "206a1f6853f49075f3d8f5d6efcda0ba473a6930",
          "url": "https://github.com/ubugeeei-prod/ush/commit/f9ee68d131fa95905426332ea996d058fb336b5a"
        },
        "date": 1795385819288,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1375,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 229,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 529,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 1214,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 1150,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 1130,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 190,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 119,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 217,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 107,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1249,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 1129,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 1057,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 769,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 2315,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 8020,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 13557,
            "range": "± 205",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 29723,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 7195925,
            "range": "± 74132",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 138338,
            "range": "± 874",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9d2147bc2e4bf81741d8a6a6ad694f2b4f43481e",
          "message": "fix(runtime): preserve task status across shell scopes (#161)",
          "timestamp": "2026-11-23T07:49:13+09:00",
          "tree_id": "4d7a1208f99f06f601a0a0475610cd81d8f31208",
          "url": "https://github.com/ubugeeei-prod/ush/commit/9d2147bc2e4bf81741d8a6a6ad694f2b4f43481e"
        },
        "date": 1795388106211,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1488,
            "range": "± 159",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 243,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 590,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 1180,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 1202,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 1154,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 205,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 127,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 240,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 112,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1291,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 1146,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 1076,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 794,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 2625,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 8869,
            "range": "± 334",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 14909,
            "range": "± 302",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 33262,
            "range": "± 368",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 7395684,
            "range": "± 140617",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 144771,
            "range": "± 1017",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cde2418b6d2315c1496ae7b6c33f2359b0f2d931",
          "message": "feat(language): add shell-style calls, function values, tests and externs (#162)\n\n* feat(language): add shell-style calls, function values, tests and externs\n\n* fix(compiler): preserve function identity through aliases\n\n* fix(compiler): check assertion types before erasure",
          "timestamp": "2026-11-25T13:21:09+09:00",
          "tree_id": "5563f087a3e344222a482f0ca01c0f9bd7630d51",
          "url": "https://github.com/ubugeeei-prod/ush/commit/cde2418b6d2315c1496ae7b6c33f2359b0f2d931"
        },
        "date": 1795580795819,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1158,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 181,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 441,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 977,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 984,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 865,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 158,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 101,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 191,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 93,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1071,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 936,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 879,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 606,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 2002,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 11,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 7916,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 13305,
            "range": "± 1201",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 28893,
            "range": "± 1344",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 6777361,
            "range": "± 259289",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 137556,
            "range": "± 6944",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fc2ea77afa8b3d09ae303611704d4188c9759a8c",
          "message": "feat(tooling): bundle editor support and self-host bootstrap scripts (#163)\n\n* feat(tooling): bundle editor support and self-host bootstrap scripts\n\n* style(tooling): format extern navigation regression test\n\n* fix(tooling): preserve iterator scopes and lint generated bootstrap shells\n\n* chore(scripts): regenerate bootstrap shells with explicit empty CDPATH\n\n* fix(tooling): complete test outlines and annotate shell interop aliases\n\n* fix(ci): retain analysis coverage for embedded benchmark statistics\n\n* fix(lsp): exclude string contents from scopes and reject ambiguous renames\n\n* fix(tooling): close editor and bootstrap regression gaps before release",
          "timestamp": "2026-11-25T14:33:13+09:00",
          "tree_id": "4a065ad019ccd6f8c8ce98a57752fd0e7a739021",
          "url": "https://github.com/ubugeeei-prod/ush/commit/fc2ea77afa8b3d09ae303611704d4188c9759a8c"
        },
        "date": 1795585146833,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1500,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 241,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 600,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 1187,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 1181,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 1114,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 198,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 123,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 232,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 107,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1299,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 1150,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 1087,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 833,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 2607,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 1750,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 7908,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 27043,
            "range": "± 203",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 8928295,
            "range": "± 96182",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 160793,
            "range": "± 2118",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6c3362558fc6ce59aed855e1debb89139513e724",
          "message": "fix(scripts): implement bootstrap workflows in native ush (#164)\n\n* fix(scripts): implement bootstrap workflows in native ush\n\n* fix(compiler): preserve control flow and defer temporary storage",
          "timestamp": "2026-11-25T16:30:50+09:00",
          "tree_id": "a2a13faab6925e08c33d922986b5d9f6844461da",
          "url": "https://github.com/ubugeeei-prod/ush/commit/6c3362558fc6ce59aed855e1debb89139513e724"
        },
        "date": 1795592204384,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1512,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 244,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 578,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 1215,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 1221,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 1141,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 202,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 125,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 235,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 109,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1327,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 1172,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 1102,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 797,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 2605,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 1810,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 8434,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 27017,
            "range": "± 717",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 9242606,
            "range": "± 58009",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 173579,
            "range": "± 2678",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "91fe7a51109a454e0ba4131d4a1825e61034f461",
          "message": "fix(shell): defer user directory creation until interactive use (#166)",
          "timestamp": "2026-11-25T16:58:58+09:00",
          "tree_id": "88a243721f869364bc6cb3d39a6e546579565585",
          "url": "https://github.com/ubugeeei-prod/ush/commit/91fe7a51109a454e0ba4131d4a1825e61034f461"
        },
        "date": 1795593986988,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1043,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 154,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 384,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 919,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 924,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 888,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 162,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 101,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 180,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1044,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 899,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 806,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 547,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 1851,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 10,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 1189,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 5593,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 18966,
            "range": "± 158",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 6412272,
            "range": "± 28675",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 133501,
            "range": "± 805",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0710065c24c2c218073aa794e49091ee48cb0930",
          "message": "fix(nix): provide test utilities and select the test profile (#168)\n\n* fix(nix): provide test utilities and select the test profile\n\n* fix(nix): include the regex capture backend in checks\n\n* fix(nix): include ps for process signal tests\n\n* build(nix): pin the validated flake inputs",
          "timestamp": "2026-11-25T17:27:17+09:00",
          "tree_id": "9535bc942de2303cf96005388f79604e1bb68828",
          "url": "https://github.com/ubugeeei-prod/ush/commit/0710065c24c2c218073aa794e49091ee48cb0930"
        },
        "date": 1795595565377,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 759,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 129,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 271,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 639,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 637,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 569,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 104,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 63,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 118,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 58,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 665,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 596,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 554,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 358,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 1371,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 11,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 953,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 4564,
            "range": "± 190",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 14461,
            "range": "± 761",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 5672276,
            "range": "± 220437",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 75536,
            "range": "± 2202",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "793b11aecfbb7dc0d0249de19c7c09e60cdfeb51",
          "message": "ci(deps): bump the actions group across 1 directory with 11 updates (#159)\n\nBumps the actions group with 11 updates in the / directory:\n\n| Package | From | To |\n| --- | --- | --- |\n| [actions/checkout](https://github.com/actions/checkout) | `6.0.2` | `7.0.1` |\n| [Swatinem/rust-cache](https://github.com/swatinem/rust-cache) | `42dc69e1aa15d09112580998cf2ef0119e2e91ae` | `eedcf2fb1c22f010a0eb66b92f378c110977f266` |\n| [benchmark-action/github-action-benchmark](https://github.com/benchmark-action/github-action-benchmark) | `1.22.1` | `1.22.2` |\n| [github/codeql-action/init](https://github.com/github/codeql-action) | `7c1e4cf0b20d7c1872b26569c00ba908797a59bf` | `776237d428f23fefc3d9e0fe1962921cbcd53e17` |\n| [github/codeql-action/autobuild](https://github.com/github/codeql-action) | `7c1e4cf0b20d7c1872b26569c00ba908797a59bf` | `776237d428f23fefc3d9e0fe1962921cbcd53e17` |\n| [github/codeql-action/analyze](https://github.com/github/codeql-action) | `7c1e4cf0b20d7c1872b26569c00ba908797a59bf` | `776237d428f23fefc3d9e0fe1962921cbcd53e17` |\n| [actions/labeler](https://github.com/actions/labeler) | `6.1.0` | `7.0.0` |\n| [actions/upload-artifact](https://github.com/actions/upload-artifact) | `7.0.1` | `7.0.2` |\n| [actions/attest-build-provenance](https://github.com/actions/attest-build-provenance) | `b3e506e8c389afc651c5bacf2b8f2a1ea0557215` | `9d57eef8c06cd9d6b433effeeb7a6a77b3ff94ad` |\n| [actions/download-artifact](https://github.com/actions/download-artifact) | `8.0.1` | `8.0.2` |\n| [softprops/action-gh-release](https://github.com/softprops/action-gh-release) | `3.0.0` | `3.0.4` |\n\n\n\nUpdates `actions/checkout` from 6.0.2 to 7.0.1\n- [Release notes](https://github.com/actions/checkout/releases)\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/actions/checkout/compare/de0fac2e4500dabe0009e67214ff5f5447ce83dd...3d3c42e5aac5ba805825da76410c181273ba90b1)\n\nUpdates `Swatinem/rust-cache` from 42dc69e1aa15d09112580998cf2ef0119e2e91ae to eedcf2fb1c22f010a0eb66b92f378c110977f266\n- [Release notes](https://github.com/swatinem/rust-cache/releases)\n- [Changelog](https://github.com/Swatinem/rust-cache/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/swatinem/rust-cache/compare/42dc69e1aa15d09112580998cf2ef0119e2e91ae...eedcf2fb1c22f010a0eb66b92f378c110977f266)\n\nUpdates `benchmark-action/github-action-benchmark` from 1.22.1 to 1.22.2\n- [Release notes](https://github.com/benchmark-action/github-action-benchmark/releases)\n- [Changelog](https://github.com/benchmark-action/github-action-benchmark/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/benchmark-action/github-action-benchmark/compare/52576c92bccf6ac60c8223ec7eb2565637cae9ba...4322e5726e6334590d251fc4f92bec0efafc45dc)\n\nUpdates `github/codeql-action/init` from 7c1e4cf0b20d7c1872b26569c00ba908797a59bf to 776237d428f23fefc3d9e0fe1962921cbcd53e17\n- [Release notes](https://github.com/github/codeql-action/releases)\n- [Changelog](https://github.com/github/codeql-action/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/github/codeql-action/compare/7c1e4cf0b20d7c1872b26569c00ba908797a59bf...776237d428f23fefc3d9e0fe1962921cbcd53e17)\n\nUpdates `github/codeql-action/autobuild` from 7c1e4cf0b20d7c1872b26569c00ba908797a59bf to 776237d428f23fefc3d9e0fe1962921cbcd53e17\n- [Release notes](https://github.com/github/codeql-action/releases)\n- [Changelog](https://github.com/github/codeql-action/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/github/codeql-action/compare/7c1e4cf0b20d7c1872b26569c00ba908797a59bf...776237d428f23fefc3d9e0fe1962921cbcd53e17)\n\nUpdates `github/codeql-action/analyze` from 7c1e4cf0b20d7c1872b26569c00ba908797a59bf to 776237d428f23fefc3d9e0fe1962921cbcd53e17\n- [Release notes](https://github.com/github/codeql-action/releases)\n- [Changelog](https://github.com/github/codeql-action/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/github/codeql-action/compare/7c1e4cf0b20d7c1872b26569c00ba908797a59bf...776237d428f23fefc3d9e0fe1962921cbcd53e17)\n\nUpdates `actions/labeler` from 6.1.0 to 7.0.0\n- [Release notes](https://github.com/actions/labeler/releases)\n- [Commits](https://github.com/actions/labeler/compare/f27b608878404679385c85cfa523b85ccb86e213...bf12e9b00b37c5c0ca2b87b79b2daf7891dbda13)\n\nUpdates `actions/upload-artifact` from 7.0.1 to 7.0.2\n- [Release notes](https://github.com/actions/upload-artifact/releases)\n- [Commits](https://github.com/actions/upload-artifact/compare/043fb46d1a93c77aae656e7c1c64a875d1fc6a0a...cf430e030ddbb5b0abf93d22962f4752f3646cd9)\n\nUpdates `actions/attest-build-provenance` from b3e506e8c389afc651c5bacf2b8f2a1ea0557215 to 9d57eef8c06cd9d6b433effeeb7a6a77b3ff94ad\n- [Release notes](https://github.com/actions/attest-build-provenance/releases)\n- [Changelog](https://github.com/actions/attest-build-provenance/blob/main/RELEASE.md)\n- [Commits](https://github.com/actions/attest-build-provenance/compare/b3e506e8c389afc651c5bacf2b8f2a1ea0557215...9d57eef8c06cd9d6b433effeeb7a6a77b3ff94ad)\n\nUpdates `actions/download-artifact` from 8.0.1 to 8.0.2\n- [Release notes](https://github.com/actions/download-artifact/releases)\n- [Commits](https://github.com/actions/download-artifact/compare/3e5f45b2cfb9172054b4087a40e8e0b5a5461e7c...9000827ccba6bdab643e8b6fd33ac0654aef8333)\n\nUpdates `softprops/action-gh-release` from 3.0.0 to 3.0.4\n- [Release notes](https://github.com/softprops/action-gh-release/releases)\n- [Changelog](https://github.com/softprops/action-gh-release/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/softprops/action-gh-release/compare/b4309332981a82ec1c5618f44dd2e27cc8bfbfda...8a0cd651e5b2365bd640854a3fa5dfc70e4875f3)\n\n---\nupdated-dependencies:\n- dependency-name: actions/attest-build-provenance\n  dependency-version: 9d57eef8c06cd9d6b433effeeb7a6a77b3ff94ad\n  dependency-type: direct:production\n  dependency-group: actions\n- dependency-name: actions/checkout\n  dependency-version: 7.0.1\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n  dependency-group: actions\n- dependency-name: actions/download-artifact\n  dependency-version: 8.0.2\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n  dependency-group: actions\n- dependency-name: actions/labeler\n  dependency-version: 7.0.0\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n  dependency-group: actions\n- dependency-name: actions/upload-artifact\n  dependency-version: 7.0.2\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n  dependency-group: actions\n- dependency-name: benchmark-action/github-action-benchmark\n  dependency-version: 1.22.2\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n  dependency-group: actions\n- dependency-name: github/codeql-action/analyze\n  dependency-version: 38112626f97ac1fd7acf3df7ebca51f4677c1b38\n  dependency-type: direct:production\n  dependency-group: actions\n- dependency-name: github/codeql-action/autobuild\n  dependency-version: 38112626f97ac1fd7acf3df7ebca51f4677c1b38\n  dependency-type: direct:production\n  dependency-group: actions\n- dependency-name: github/codeql-action/init\n  dependency-version: 38112626f97ac1fd7acf3df7ebca51f4677c1b38\n  dependency-type: direct:production\n  dependency-group: actions\n- dependency-name: softprops/action-gh-release\n  dependency-version: 3.0.4\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n  dependency-group: actions\n- dependency-name: Swatinem/rust-cache\n  dependency-version: eedcf2fb1c22f010a0eb66b92f378c110977f266\n  dependency-type: direct:production\n  dependency-group: actions\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-11-25T17:31:59+09:00",
          "tree_id": "b6972c1bf7c9e6048c6004c8115d57bcd30dad3b",
          "url": "https://github.com/ubugeeei-prod/ush/commit/793b11aecfbb7dc0d0249de19c7c09e60cdfeb51"
        },
        "date": 1795595882598,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1047,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 154,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 385,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 940,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 940,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 870,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 163,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 182,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1046,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 895,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 796,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 536,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 1860,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 10,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 1193,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 5730,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 19271,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 6551580,
            "range": "± 199134",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 127229,
            "range": "± 1185",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "577176fefe965c146565e74bdadc142ea223893c",
          "message": "feat(std): add portable text filesystem and process APIs (#165)\n\n* feat(std): add portable text filesystem and process APIs\n\n* perf(compiler): skip unused std signatures during generic checking\n\n* style(compiler): format std reference export\n\n* fix(compiler): preserve full-width integer values on zsh\n\n* fix(nix): select debug workspace checks through checkType\n\n* fix(std): preserve stdout when querying terminal state",
          "timestamp": "2026-11-25T17:26:59+09:00",
          "tree_id": "b5e39ab091b2eb24b490056d09c72f120a33dbc4",
          "url": "https://github.com/ubugeeei-prod/ush/commit/577176fefe965c146565e74bdadc142ea223893c"
        },
        "date": 1795595891339,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1496,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 247,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 576,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 1150,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 1240,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 1093,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 204,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 127,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 237,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 112,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1346,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 1199,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 1140,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 840,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 2645,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 1854,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 8298,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 27415,
            "range": "± 477",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 9506100,
            "range": "± 272015",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 168249,
            "range": "± 1604",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "986b39f10ea1d4cfc2a48ebe8ed868e2d05490fa",
          "message": "fix(ci): isolate benchmark history measurements from caches (#170)",
          "timestamp": "2026-11-25T18:01:24+09:00",
          "tree_id": "bb7b37f63d79c233e2a89f29e426e2743fa55f52",
          "url": "https://github.com/ubugeeei-prod/ush/commit/986b39f10ea1d4cfc2a48ebe8ed868e2d05490fa"
        },
        "date": 1795597595931,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1050,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 156,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 383,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 950,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 913,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 873,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 162,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 179,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 996,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 873,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 789,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 545,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 1834,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 9,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 1171,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 5686,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 19204,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 6585341,
            "range": "± 21711",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 136117,
            "range": "± 1065",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6890611d011912b455bc59f8ef0e53a3ca724052",
          "message": "feat(lang): native pipelines, std::io file stages, and scoped cd/export blocks (#172)\n\n* fix(parse): run the last line of a declared `-> ()` function as a statement\n\nA trailing command in a function declared `-> ()` was parsed as the\nfunction's value and failed to compile as a returned String. A declared\nunit function has no tail value, so its last line is now a statement.\nUnannotated functions still infer their result from the last line.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* feat(lang): add pipelines, std::io file stages, and scoped cd/export blocks\n\nPipelines (`a | b`, `a |& b`) stream as statements and capture the last\nstage as values. A stage is a command, std::process::exec/output, or a\nfunction returning (); a String first stage feeds stdin byte for byte.\nstd::io::from_file, to_file and append_to lower to <, > and >> on the\nneighbouring stage. Every target uses one portable lowering: each stage\nreports its status on a private descriptor and __ush_pipe_check reports\nthe leftmost failing stage with its source line, so POSIX sh and dash\nget pipefail semantics. Stages hold their pipe ends until they report, so\na failure recorded after the reader's report was caused by that reader\nstopping early (SIGPIPE, or EPIPE where SIGPIPE is ignored) and is not a\nfailure. A line ending in | continues on the next line.\n\n`cd path { … }` and `export NAME = value, … { … }` save what they change,\nrun the body in the current shell and restore it afterwards, including\nbefore break and continue; `export NAME = value` sets a variable for the\nrest of the process scope. Effects, error rows and the contracts checker\ncover the new forms.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* feat(tooling): format continued pipelines and highlight export\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* docs: document pipelines, file stages and scoped blocks, with an example\n\nAdd the language guide sections on pipelines and redirection and on scoped\ncwd and environment, the std::io stage contracts, the lowering of both\nforms, the effects they infer, a runnable examples/pipelines.ush checked on\nevery target by scripts/test_targets.ush, and the changelog entries.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n---------\n\nCo-authored-by: Claude Opus 5.5 <noreply@anthropic.com>",
          "timestamp": "2026-11-25T19:34:24+09:00",
          "tree_id": "857be923488aa59f755b04487bc43e9b9caea236",
          "url": "https://github.com/ubugeeei-prod/ush/commit/6890611d011912b455bc59f8ef0e53a3ca724052"
        },
        "date": 1795603154986,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1160,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 183,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 428,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 886,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 883,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 824,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 161,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 97,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 188,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 84,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 993,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 857,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 821,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 582,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 1999,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 1337,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 6360,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 20434,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 7662244,
            "range": "± 193163",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 117495,
            "range": "± 3603",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "08e072d01f66ad6e50a6cc0d32001b2638643e3a",
          "message": "feat: local modules, relocatable checked-in output and compile --check (#173)\n\n* feat(compiler): link local modules and relocate checked-in output\n\n`mod \"lib/gh.ush\" as gh` imports a declarations-only `.ush` file relative\nto the importing file. Its `pub` items are reachable as `gh::item` or with\n`use gh::{item}` / `use gh::*`; private items and top-level statements in a\nmodule are diagnosed.\n\nThe linker lives in the no_std core behind a `ModuleLoader`. Each file is\nresolved against its own imports, its items move into a namespace\n(`gh::run` -> `ush_fn_gh__run`) and modules are inlined dependencies-first,\nso later passes see one program. Only reachable module functions are kept,\nshared modules link once and import cycles name every file.\n\nStatement lines now index one source space: the root keeps its numbering\nand modules follow at offsets. Errors render as\n`line R: lib/gh.ush: line N: ...`, `__ush_at` passes the module file so\nruntime failures name it, and sourcemap lines carry `source_file`. Codegen\nand the effect passes attach statement lines to errors.\n\n`--checked-in` output may now sit in any directory: the header names the\nsource relative to the output, and the script resolves source-relative\npaths from the recorded location. Adjacent outputs are unchanged.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* feat(cli): compile many scripts, check freshness, follow modules in tools\n\n`ush compile` takes several inputs with `--out-dir DIR` (relative to each\ninput's directory) and leaves unchanged outputs untouched. `--check`\ncompiles in memory and exits 1 with a short diff when an output is stale\nor missing, after processing every input.\n\n`ush check`, `ush explain`, the sourcemap JSON and the language server\nread a script's modules from disk; tooling knows the `mod` and `pub`\nkeywords and lists modules in the outline.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* docs: describe local modules and relocatable checked-in output\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* fix(compiler): report conflicting wildcard type imports like function ones\n\nTwo `use m::*` imports that offer different types under one name are now\nan error unless the file declares or imports that name explicitly, matching\nthe existing rule for functions.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* docs: index local modules and checked-in output\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* test(compiler): run module functions as pipeline stages\n\nModule functions returning `()` work as pipeline stages, qualified or\nimported, and may hold pipelines themselves. Top-level `extern` and\n`test` parsing moves into parse/items.rs to keep parse_block under the\nline cap after the pipeline dispatch landed.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n---------\n\nCo-authored-by: Claude Opus 5.5 <noreply@anthropic.com>",
          "timestamp": "2026-11-25T19:59:36+09:00",
          "tree_id": "cdbd9cbb6734b86b44322b83bdadca3e22652867",
          "url": "https://github.com/ubugeeei-prod/ush/commit/08e072d01f66ad6e50a6cc0d32001b2638643e3a"
        },
        "date": 1795604990971,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1496,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 246,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 576,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 1226,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 1217,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 1128,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 199,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 123,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 231,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 108,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1323,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 1174,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 1111,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 821,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 2542,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 1975,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 8836,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 29178,
            "range": "± 616",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 9530654,
            "range": "± 59043",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 185866,
            "range": "± 2542",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a9a05d871b6e88550b446fc329bbf197c0f1eabb",
          "message": "feat(lang): call programs outside the catalog by naming them (#176)\n\n`use $bin::{vp, \"apt-get\" as apt_get}` imports programs the command\ncatalog does not list. A named program takes any argv, may print and may\nuse every capability. Quoted names take a ush alias, and every name must be\none safe shell word. The wildcard still imports only the catalog, so typos\nstay compile errors and compiled scripts never depend on the compiling\nmachine's PATH.\n\nCo-authored-by: Claude Opus 5.5 <noreply@anthropic.com>",
          "timestamp": "2026-11-25T20:16:33+09:00",
          "tree_id": "7c9bd51bbdc679c480de0b799a0ab7cb7dddac01",
          "url": "https://github.com/ubugeeei-prod/ush/commit/a9a05d871b6e88550b446fc329bbf197c0f1eabb"
        },
        "date": 1795605769762,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 764,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 132,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 285,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 598,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 605,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 559,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 100,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 60,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 114,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 663,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 580,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 560,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 392,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 1382,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 12,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 1051,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 4861,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 15351,
            "range": "± 210",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 5608250,
            "range": "± 398736",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 80192,
            "range": "± 439",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "acfbceda92193b248c280b24e085356f792c9f0d",
          "message": "feat(lang): make bindings immutable by default and add let mut (#174)\n\n* feat(lang): make bindings immutable by default and add let mut\n\n* test(cli): track isolated binding storage in source maps",
          "timestamp": "2026-11-25T20:20:47+09:00",
          "tree_id": "43f16a11cdecc136c8010fb03dd520bce7c281aa",
          "url": "https://github.com/ubugeeei-prod/ush/commit/acfbceda92193b248c280b24e085356f792c9f0d"
        },
        "date": 1795605981655,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1503,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 241,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 578,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 1191,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 1199,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 1130,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 200,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 123,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 231,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 108,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1312,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 1176,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 1092,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 820,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 2587,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 1910,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 9722,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 30095,
            "range": "± 139",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 9924777,
            "range": "± 473985",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 189061,
            "range": "± 2205",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "83cd1fef0d3c0af231ff44b66c8e590bb87b36ba",
          "message": "feat: infer error rows for signatures that omit them (#177)\n\n* feat(compiler): infer the error row of a signature that omits it\n\nA function without `/ row` exposes the errors its body raises, as it\nalready did for effects, instead of failing with \"does not declare an error\nset\". Callers, `?` and typed failures see the inferred row; a written row\nis still a checked upper bound and `/ ()` still means pure.\n\nBoth row fixpoints run until stable and keep each previous row, so\nrecursive cycles settle on the least covering row. Rows are split into\nerrors and effects after linking, so they may name imported error types.\nThe effects report carries each body's error types.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* feat(tooling): show inferred rows in ush effects and hovers\n\n`ush effects` lists each function's error types ahead of its effects, and\na hover on a function shows its row, marked inferred or declared.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* docs: document inferred rows and drop redundant rows from scripts\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* test(compiler): infer rows through let mut assignments\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n---------\n\nCo-authored-by: Claude Opus 5.5 <noreply@anthropic.com>",
          "timestamp": "2026-11-25T20:40:53+09:00",
          "tree_id": "393c70cf4aa52321e326d25223e087b68fb9a736",
          "url": "https://github.com/ubugeeei-prod/ush/commit/83cd1fef0d3c0af231ff44b66c8e590bb87b36ba"
        },
        "date": 1795607192982,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1050,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 155,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 389,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 945,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 925,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 875,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 163,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 181,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1004,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 885,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 789,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 543,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 1848,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 9,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 1222,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 6443,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 21215,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 7001917,
            "range": "± 22115",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 149755,
            "range": "± 817",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9aa8bf53d28f23f1ff8e8cbd84b209231e51aa1c",
          "message": "feat(lang): command status, stderr stages, background commands and defer (#180)\n\n* feat(lang): command status tests, stderr stages, background commands and defer\n\nA command, a pipeline or a function returning () used as an if/while\ncondition tests whether it succeeded, with !, && and || combining it with\nBooleans; its output streams and its failure never stops the script. A\npipeline condition fails like pipefail, leftmost stage first.\n`status_of command` evaluates to its exit status as an Int and\n`allow_fail command` carries on whatever it returns.\n\nstd::io gains discard_stderr, stderr_to and stderr_append_to, which\nredirect the stderr of the stage before them while its stdout keeps\nflowing, and to_stderr, a sink that sends stdout to stderr. All lower to\nredirections on the neighbouring process.\n\n`let job = spawn command` runs a command, pipeline or unit function in\nthe background as a Task<()>, streaming its output. Task handles gain\n`job.pid` and `job.kill(signal)`, which signals the whole process tree;\na killed task does not fail its scope when joined, and `.await` keeps the\nstatus a signal produced for every later await.\n\n`defer { … }` and `defer statement` run when the enclosing function\nreturns or the script exits, last registered first, including on\nfailure and on handled signals, before unfinished tasks are joined and\ntemporary directories removed. A function that defers runs in a process\nof its own; defer is rejected in loops, cd/export blocks, async blocks,\nmethods and other deferred blocks.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* feat(tooling): highlight, complete and describe defer, spawn, status_of and allow_fail\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* build: regenerate the checked-in scripts for the new task runtime\n\nThe task runtime now runs deferred blocks from its exit trap, keeps a\nsignal's status for every await and skips killed tasks when joining.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* docs: document command status, stderr stages, background commands and defer\n\nAdd the language guide sections on command status, background commands\nand deferred cleanup, the stderr stages in the pipeline guide and the\nstd::io table, their lowering and effects, the README feature line, the\nchangelog entries, and examples/process_control.ush, run on every target\nby scripts/test_targets.ush.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n---------\n\nCo-authored-by: Claude Opus 5.5 <noreply@anthropic.com>",
          "timestamp": "2026-11-25T21:23:00+09:00",
          "tree_id": "3755086ff598a21ddc37ffb7d0e266a766b4043e",
          "url": "https://github.com/ubugeeei-prod/ush/commit/9aa8bf53d28f23f1ff8e8cbd84b209231e51aa1c"
        },
        "date": 1795609705249,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1374,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 230,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 532,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 1234,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 1151,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 1131,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 189,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 120,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 218,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 107,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1251,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 1134,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 1063,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 785,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 2322,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 1888,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 8810,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 27520,
            "range": "± 176",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 11415617,
            "range": "± 474490",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 184102,
            "range": "± 4451",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "ubuge1122@gmail.com",
            "name": "ubugeeei",
            "username": "ubugeeei"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "72e6d40aae2da7c9922aea0f845f4355a52b4619",
          "message": "feat(lang): add portable native echo output (#179)\n\nfeat(lang): add portable native echo output",
          "timestamp": "2026-11-25T22:19:51+09:00",
          "tree_id": "3241b4c4676907e507746fd511866d68d6435b71",
          "url": "https://github.com/ubugeeei-prod/ush/commit/72e6d40aae2da7c9922aea0f845f4355a52b4619"
        },
        "date": 1795613405752,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse pipeline with helper",
            "value": 1057,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/bare command",
            "value": 157,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/command with flags",
            "value": 388,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/long argument list",
            "value": 943,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/assignments",
            "value": 946,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse native line/quoted arguments",
            "value": 851,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/redirect",
            "value": 163,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/command substitution",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/shell keyword",
            "value": 179,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse fallback line/subshell",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/boolean chain",
            "value": 1000,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/sequence",
            "value": 880,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse and-or list/or fallback",
            "value": 790,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse with alias table",
            "value": 541,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse long pipeline",
            "value": 1851,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse comment only",
            "value": 9,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "compile empty ush program",
            "value": 1225,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "compile small ush program",
            "value": 6430,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "compile adt ush program",
            "value": 21166,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "compile 1000 functions",
            "value": 7165003,
            "range": "± 23202",
            "unit": "ns/iter"
          },
          {
            "name": "compile typed shell interop",
            "value": 164219,
            "range": "± 1608",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}