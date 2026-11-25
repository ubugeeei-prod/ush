#!/bin/sh
# Generated from 'preflight.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: preflight.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'preflight.ush' --checked-in -o 'preflight.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'preflight.ush'
__ush_origin='preflight.ush'

__ush_at() { __ush_line=$1; __ush_source=$2; }
__ush_report() {
  if [ "$1" -ne 0 ] && [ -n "${__ush_line:-}" ]; then
    __ush_message=${2:-command failed}
    if [ "$#" -lt 2 ]; then
      case "$1" in
        127) __ush_message='command not found; check the command name and PATH' ;;
        126) __ush_message='command cannot be executed; check permissions and interpreter' ;;
        130) __ush_message='interrupted' ;;
        137) __ush_message='process killed (SIGKILL)' ;;
        143) __ush_message='terminated (SIGTERM)' ;;
      esac
    fi
    printf '%s:%s: %s (exit %s)\n  %s\n' "$__ush_origin" "$__ush_line" "$__ush_message" "$1" "$__ush_source" >&2
    __ush_line=''
  fi
}
trap '__ush_report "$?"' 0

case "${1:-}" in
  -h|--help) printf '%s' 'preflight.ush

Usage:
  preflight.ush [--help]
  preflight.ush [--man [ITEM]]
  preflight.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
preflight.ush

SYNOPSIS
preflight.ush [--help]
preflight.ush [--man [ITEM]]
preflight.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

ush_fn_banner() {
  __ush_fn_banner_arg_0="${1}"
  # shellcheck disable=SC2034
  name="${__ush_fn_banner_arg_0}"
  __ush_at 4 'print "=== " + name + " ==="'
  printf '%s\n' "$(printf '%s' '=== ' "${__ush_fn_banner_arg_0}" ' ===')"
}

__ush_at 6 'banner "rustfmt"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'rustfmt' )
__ush_at 7 'cargo "fmt" "--all" "--check"'
command cargo 'fmt' '--all' '--check'
__ush_at 8 'banner "clippy"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'clippy' )
__ush_at 9 'cargo "clippy" "--locked" "--workspace" "--all-targets" "--no-deps" "--" "-D" "warnings"'
command cargo 'clippy' '--locked' '--workspace' '--all-targets' '--no-deps' '--' '-D' 'warnings'
__ush_at 10 'banner "workspace tests"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'workspace tests' )
__ush_at 11 'cargo "test" "--locked" "--workspace"'
command cargo 'test' '--locked' '--workspace'
__ush_at 12 'banner "release tests"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'release tests' )
__ush_at 13 'cargo "test" "--locked" "--workspace" "--release"'
command cargo 'test' '--locked' '--workspace' '--release'
__ush_at 14 'banner "no_std"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'no_std' )
__ush_at 15 'cargo "check" "--locked" "-p" "ush_compiler" "--no-default-features"'
command cargo 'check' '--locked' '-p' 'ush_compiler' '--no-default-features'
__ush_at 16 'cargo "test" "--locked" "-p" "ush_compiler" "--no-default-features" "--lib"'
command cargo 'test' '--locked' '-p' 'ush_compiler' '--no-default-features' '--lib'
__ush_at 17 'cargo "clippy" "--locked" "-p" "ush_compiler" "--no-default-features" "--lib" "--no-deps" "--" "-D" "warnings"'
command cargo 'clippy' '--locked' '-p' 'ush_compiler' '--no-default-features' '--lib' '--no-deps' '--' '-D' 'warnings'
__ush_at 18 'banner "generated artifacts"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'generated artifacts' )
__ush_at 19 'sh "scripts/check_generated.sh"'
command sh 'scripts/check_generated.sh'
__ush_at 20 'banner "line limit"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'line limit' )
__ush_at 21 'sh "scripts/check_rs_line_limit.sh"'
command sh 'scripts/check_rs_line_limit.sh'
__ush_at 22 'banner "installer"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'installer' )
__ush_at 23 'sh "scripts/test_install.sh"'
command sh 'scripts/test_install.sh'
__ush_at 24 'banner "rustdoc"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'rustdoc' )
__ush_at 25 'shell "RUSTDOCFLAGS='"'"'-D warnings'"'"' cargo doc --locked --workspace --no-deps --document-private-items"'
RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps --document-private-items
__ush_at 26 'banner "shell targets"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'shell targets' )
__ush_at 27 'sh "scripts/test_targets.sh"'
command sh 'scripts/test_targets.sh'
__ush_at 28 'banner "editor"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'editor' )
__ush_at 29 'shell "(cd editors/vscode && npm ci --ignore-scripts && npm test && npm run build)"'
(cd editors/vscode && npm ci --ignore-scripts && npm test && npm run build)
__ush_at 30 'banner "bench builds"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'bench builds' )
__ush_at 31 'cargo "bench" "--locked" "-p" "ush_shell" "--no-run"'
command cargo 'bench' '--locked' '-p' 'ush_shell' '--no-run'
__ush_at 32 'cargo "bench" "--locked" "-p" "ush_compiler" "--no-run"'
command cargo 'bench' '--locked' '-p' 'ush_compiler' '--no-run'
__ush_at 33 'banner "vendored dependency"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'vendored dependency' )
__ush_at 34 'sh "scripts/check_rustyline_upstream.sh"'
command sh 'scripts/check_rustyline_upstream.sh'
__ush_at 35 'banner "examples and shellcheck"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'examples and shellcheck' )
__ush_at 36 'shell """'
for example in examples/*.ush; do
  cargo run --locked -p ush --quiet -- check "$example"
  cargo run --locked -p ush --quiet -- format --check "$example"
done
if command -v shellcheck >/dev/null 2>&1; then
  shellcheck --severity=warning install.sh scripts/*.sh
fi
__ush_at 45 'banner "preflight passed"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'preflight passed' )
