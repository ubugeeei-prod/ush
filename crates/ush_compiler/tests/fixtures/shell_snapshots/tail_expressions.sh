#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/tail_expressions.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/tail_expressions.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/tail_expressions.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/tail_expressions.ush'

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
  -h|--help) printf '%s' 'tail_expressions.ush

Usage:
  tail_expressions.ush [--help]
  tail_expressions.ush [--man [ITEM]]
  tail_expressions.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
tail_expressions.ush

SYNOPSIS
tail_expressions.ush [--help]
tail_expressions.ush [--man [ITEM]]
tail_expressions.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

ush_fn_trace() {
  __ush_at 2 'print "trace"'
  printf '%s\n' 'trace'
  __ush_at 3 '"value"'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' 'value' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' 'value' >&4
  fi
  return 0
}

ush_fn_choose() {
  __ush_fn_choose_arg_0="${1}"
  # shellcheck disable=SC2034
  flag="${__ush_fn_choose_arg_0}"
  __ush_at 7 'match flag {'
  if [ "${__ush_fn_choose_arg_0}" = "true" ]; then
    __ush_at 8 'true => "yes",'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'yes' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'yes' >&4
    fi
    return 0
  elif :; then
    __ush_at 9 '_ => "no",'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'no' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'no' >&4
    fi
    return 0
  fi
}

ush_fn_run() {
  __ush_at 14 'trace();'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_trace ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 15 'choose true'
  __ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_choose 'true' 4>&1 >&3)"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_value_0}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_value_0}" >&4
  fi
  return 0
}

__ush_at 18 'print $ run ()'
__ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_run 4>&1 >&3)"
printf '%s\n' "${__ush_value_1}"
