#!/bin/sh
# Generated from 'check_generated.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: check_generated.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'check_generated.ush' --checked-in -o 'check_generated.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'check_generated.ush'
__ush_origin='check_generated.ush'

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
  -h|--help) printf '%s' 'check_generated.ush

Usage:
  check_generated.ush [--help]
  check_generated.ush [--man [ITEM]]
  check_generated.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
check_generated.ush

SYNOPSIS
check_generated.ush [--help]
check_generated.ush [--man [ITEM]]
check_generated.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

__ush_at 2 'shell """'
set -eu
if [ -z "${USH_COMPILER:-}" ]; then
  cargo build --locked -p ush
  USH_COMPILER=target/debug/ush
fi
work=$(mktemp -d)
trap 'rm -rf "$work"' 0
for source in install.ush scripts/*.ush; do
  name=${source##*/}
  artifact=${source%.ush}.sh
  cp "$source" "$work/$name"
  "$USH_COMPILER" compile "$work/$name" --checked-in -o "$work/${name%.ush}.sh"
  if ! cmp -s "$artifact" "$work/${name%.ush}.sh"; then
    printf 'stale artifact: %s; regenerate from %s\n' "$artifact" "$source" >&2
    exit 1
  fi
done
# Expose the real foreign Python body to CodeQL, keeping .ush as the only editable source.
{
  printf '%s\n' '# Generated from ../scripts/compare_benchmarks.ush. DO NOT EDIT MANUALLY.'
  awk '/^python3 - / { body=1; next } /^USH_BENCH_PY$/ { body=0 } body { print }' scripts/compare_benchmarks.ush
} > "$work/benchmark_statistics.py"
if ! cmp -s generated/benchmark_statistics.py "$work/benchmark_statistics.py"; then
  printf '%s\n' 'generated/benchmark_statistics.py is stale; regenerate from scripts/compare_benchmarks.ush' >&2
  exit 1
fi
