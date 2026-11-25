#!/bin/sh
# Generated from 'test_targets.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: test_targets.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'test_targets.ush' --checked-in -o 'test_targets.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'test_targets.ush'
__ush_origin='test_targets.ush'

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
  -h|--help) printf '%s' 'test_targets.ush

Usage:
  test_targets.ush [--help]
  test_targets.ush [--man [ITEM]]
  test_targets.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
test_targets.ush

SYNOPSIS
test_targets.ush [--help]
test_targets.ush [--man [ITEM]]
test_targets.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

__ush_at 2 'shell """'
set -eu

ush=${USH_BIN:-target/debug/ush}
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' 0
ln -s "$(command -v busybox)" "$tmp/ash"
PATH=$tmp:$PATH
export PATH

for target in sh bash zsh ash; do
  sh scripts/test_task_await.sh "$ush" "$target"
  for source in examples/*.ush; do
    "$ush" compile "$source" --target "$target" -o "$tmp/script"
    "$target" -n "$tmp/script"
  done
  "$ush" compile examples/contracts.ush --target "$target" -o "$tmp/script"
  chmod +x "$tmp/script"
  actual=$("$tmp/script")
  [ "$actual" = "$(printf '%s\n' 2 20 ready 'done')" ]

  "$ush" compile examples/collections.ush --target "$target" -o "$tmp/script"
  actual=$("$target" "$tmp/script")
  [ "$actual" = "$(printf '%s\n' 2 3 4 3)" ]
  "$ush" compile examples/extern.ush --target "$target" -o "$tmp/script"
  actual=$("$target" "$tmp/script")
  [ "$actual" = "$(printf '%s\n' 'hello ush' 42)" ]
  "$ush" --shell-target "$target" test examples/smoke_test.ush

  "$ush" compile examples/error_streams.ush --target "$target" -o "$tmp/script"
  code=0
  "$tmp/script" >"$tmp/out" 2>"$tmp/err" || code=$?
  [ "$code" -eq 1 ]
  grep -F 'error_streams.ush:6: Problem::MissingConfig' "$tmp/err"
done

# Exercise the portable target explicitly with dash and check its shell dialect.
"$ush" compile examples/contracts.ush -o "$tmp/script"
dash "$tmp/script"
shellcheck -s sh --severity=error "$tmp/script"
