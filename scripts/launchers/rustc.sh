#!/bin/sh
# Cargo's RUSTC_WRAPPER for the Nix dev shell (CONTRIBUTING.md, "Rust build accelerators").
# Hand-written POSIX sh rather than generated from .ush: Cargo runs it for
# every rustc call, so it stays a few builtins followed by exec.
#
# Cargo runs: rustc.sh <rustc or clippy-driver> [rustc] <arguments>
#
# - USH_RUSTC_FRONTEND_JOBS=N (N > 1) compiles crates of this checkout with
#   rustc's parallel frontend (-Zunstable-options --jobs-frontend=N) on the
#   same stable compiler, by setting RUSTC_BOOTSTRAP to that one crate's name.
#   Registry dependencies, build-script probes and Cargo's -vV/--print queries
#   reach rustc unchanged.
# - USH_KACHE=<path to kache> hands the call to kache, which shares compiled
#   crates across worktrees and target directories.
case "${USH_RUSTC_FRONTEND_JOBS:-1}" in
  '' | 0 | 1) ;;
  *)
    checkout="${0%/scripts/launchers/rustc.sh}"
    case "${CARGO_MANIFEST_DIR:-}" in
      "$checkout"/*)
        previous=
        for argument in "$@"; do
          if [ "$previous" = --crate-name ]; then
            # A build-script probe inherits its package's environment but
            # names its own crate: unstable features stay unavailable to it.
            if [ "$argument" = "${CARGO_CRATE_NAME:-}" ]; then
              RUSTC_BOOTSTRAP="$argument"
              export RUSTC_BOOTSTRAP
              set -- "$@" -Zunstable-options "--jobs-frontend=$USH_RUSTC_FRONTEND_JOBS"
            fi
            break
          fi
          previous="$argument"
        done
        ;;
    esac
    ;;
esac
if [ -n "${USH_KACHE:-}" ]; then
  exec "$USH_KACHE" "$@"
fi
exec "$@"
