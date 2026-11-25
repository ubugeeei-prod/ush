#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/typed_cli_and_types.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/typed_cli_and_types.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/typed_cli_and_types.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/typed_cli_and_types.ush'

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
  -h|--help) printf '%s' 'typed_cli_and_types.ush

Usage:
  typed_cli_and_types.ush [--help]
  typed_cli_and_types.ush [--man [ITEM]]
  typed_cli_and_types.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
typed_cli_and_types.ush

SYNOPSIS
typed_cli_and_types.ush [--help]
typed_cli_and_types.ush [--man [ITEM]]
typed_cli_and_types.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

ush_fn_greet() {
  __ush_fn_greet_arg_0="${1}"
  # shellcheck disable=SC2034
  name="${__ush_fn_greet_arg_0}"
  __ush_fn_greet_arg_1="${2}"
  # shellcheck disable=SC2034
  count="${__ush_fn_greet_arg_1}"
  __ush_at 2 'name + ":" + count'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_fn_greet_arg_0}"':'"${__ush_fn_greet_arg_1}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_fn_greet_arg_0}"':'"${__ush_fn_greet_arg_1}" >&4
  fi
  return 0
}

__ush_at 10 'alias ll = "ls -la"'
alias ll='ls -la'
__ush_at 12 'let user = User { name: "ush", age: 7 }'
__ush_binding_12_user__tag='User::User'
# shellcheck disable=SC2034
__ush_binding_12_user__name='ush'
# shellcheck disable=SC2034
__ush_binding_12_user__age=7
__ush_at 13 'print $ greet count: 3 name: "ush"'
__ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_greet 'ush' 3 4>&1 >&3)" || exit "$?"
printf '%s\n' "${__ush_value_0}"
__ush_at 14 'print $ greet name: "mini"'
__ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_greet 'mini' 2 4>&1 >&3)" || exit "$?"
printf '%s\n' "${__ush_value_1}"
__ush_at 15 'match user {'
if [ "${__ush_binding_12_user__tag}" = 'User::User' ]; then
  name="${__ush_binding_12_user__name-}"
  age="${__ush_binding_12_user__age-}"
  __ush_at 16 'User { name, age } => print name + ":" + age'
  printf '%s\n' "${name}"':'"${age}"
elif :; then
  __ush_at 17 '_ => print "fallback"'
  printf '%s\n' 'fallback'
fi
