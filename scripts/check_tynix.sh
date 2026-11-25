#!/bin/sh
# Generated from 'check_tynix.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: check_tynix.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'check_tynix.ush' --checked-in -o 'check_tynix.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) __ush_entrypoint=$(command -v "$__ush_entrypoint") ;;
esac
__ush_source_dir=$(CDPATH= cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'check_tynix.ush'
__ush_origin='check_tynix.ush'

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
  -h|--help) printf '%s' 'check_tynix.ush

Usage:
  check_tynix.ush [--help]
  check_tynix.ush [--man [ITEM]]
  check_tynix.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
check_tynix.ush

SYNOPSIS
check_tynix.ush [--help]
check_tynix.ush [--man [ITEM]]
check_tynix.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

__ush_at 2 'shell """'
set -eu
TYNIX=${TYNIX:-tynix}
"$TYNIX" check flake.tynix
work=$(mktemp -d)
trap 'rm -rf "$work"' 0
"$TYNIX" compile flake.tynix -o "$work/flake.nix"
if ! cmp -s flake.nix "$work/flake.nix"; then
  printf '%s\n' 'flake.nix is stale; run tynix compile flake.tynix -o flake.nix' >&2
  exit 1
fi
