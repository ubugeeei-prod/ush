#!/bin/sh
# Generated from 'check_rs_line_limit.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: check_rs_line_limit.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'check_rs_line_limit.ush' --checked-in -o 'check_rs_line_limit.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) __ush_entrypoint=$(command -v "$__ush_entrypoint") ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'check_rs_line_limit.ush'
__ush_origin='check_rs_line_limit.ush'

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
  -h|--help) printf '%s' 'check_rs_line_limit.ush

Usage:
  check_rs_line_limit.ush [--help]
  check_rs_line_limit.ush [--man [ITEM]]
  check_rs_line_limit.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
check_rs_line_limit.ush

SYNOPSIS
check_rs_line_limit.ush [--help]
check_rs_line_limit.ush [--man [ITEM]]
check_rs_line_limit.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

__ush_at 2 'shell """'
set -eu

limit=250
failed=0

tmp_file="$(mktemp "${TMPDIR:-/tmp}/ush_rs_line_limit.XXXXXX")"
trap 'rm -f "$tmp_file"' EXIT INT TERM

find apps crates -type f -name '*.rs' | sort > "$tmp_file"

while IFS= read -r file; do
  lines=$(wc -l < "$file")
  if [ "$lines" -gt "$limit" ]; then
    printf '%s has %s lines (limit: %s)\n' "$file" "$lines" "$limit" >&2
    failed=1
  fi
done < "$tmp_file"

if [ "$failed" -ne 0 ]; then
  exit 1
fi
