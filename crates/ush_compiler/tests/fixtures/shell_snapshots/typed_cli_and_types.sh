#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/typed_cli_and_types.ush' by ush.
# DO NOT EDIT MANUALLY.
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/typed_cli_and_types.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/typed_cli_and_types.ush'

__ush_at() { __ush_line=$1; __ush_source=$2; }
__ush_report() {
  if [ "$1" -ne 0 ] && [ -n "${__ush_line:-}" ]; then
    printf '%s:%s: %s (exit %s)\n  %s\n' "$__ush_origin" "$__ush_line" "${2:-command failed}" "$1" "$__ush_source" >&2
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

__ush_walk_record_4_User() {
  "$1" "$2__tag" || return "$?"
  eval "__ush_tag=\${${2}__tag}"
  case $__ush_tag in
    'User::User')
      "$1" "$2__name" || return "$?"
      "$1" "$2__age" || return "$?"
      ;;
    *) return 1 ;;
  esac
}

ush_fn_std__write_cell() {
  eval "__ush_cell=\${$1}"
  printf x
  printf '%s' "$__ush_cell" | od -A n -v -t x1 | tr -d ' \n'
  printf ' '
}

ush_fn_std__read_cell() {
  case $__ush_cells in x*' '*) ;; *) return 1 ;; esac
  __ush_hex=${__ush_cells%% *}
  __ush_cells=${__ush_cells#* }
  __ush_cell=$(LC_ALL=C awk -v s="${__ush_hex#x}" 'BEGIN {
    digits = "0123456789abcdef"
    for (i = 1; i <= length(s); i += 2)
      printf "%c", (index(digits, substr(s,i,1))-1)*16 + index(digits,substr(s,i+1,1))-1
    printf "."
  }') || return "$?"
  __ush_cell=${__ush_cell%.}
  eval "$1=\$__ush_cell"
}

ush_fn_greet() {
  __ush_fn_greet_arg_0="${1}"
  name="${__ush_fn_greet_arg_0}"
  __ush_fn_greet_arg_1="${2}"
  count="${__ush_fn_greet_arg_1}"
  __ush_at 2 'name + ":" + count'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "$(printf '%s' "${__ush_fn_greet_arg_0}" ':' "${__ush_fn_greet_arg_1}")" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "$(printf '%s' "${__ush_fn_greet_arg_0}" ':' "${__ush_fn_greet_arg_1}")" >&4
  fi
  return 0
}

__ush_at 10 'alias ll = "ls -la"'
alias ll='ls -la'
__ush_at 12 'let user = User { name: "ush", age: 7 }'
user__tag='User::User'
user__name='ush'
user__age=7
__ush_at 13 'print $ greet count: 3 name: "ush"'
__ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_greet 'ush' 3 4>&1 >&3)"
printf '%s\n' "${__ush_value_0}"
__ush_at 14 'print $ greet name: "mini"'
__ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_greet 'mini' 2 4>&1 >&3)"
printf '%s\n' "${__ush_value_1}"
__ush_at 15 'match user {'
__ush_cells="$(__ush_walk_record_4_User ush_fn_std__write_cell user)"
__ush_walk_record_4_User ush_fn_std__read_cell __ush_match_2
if [ "${__ush_match_2__tag}" = 'User::User' ]; then
  name="${__ush_match_2__name-}"
  age="${__ush_match_2__age-}"
  __ush_at 16 'User { name, age } => print name + ":" + age'
  printf '%s\n' "$(printf '%s' "${name}" ':' "${age}")"
elif :; then
  __ush_at 17 '_ => print "fallback"'
  printf '%s\n' 'fallback'
fi
