#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/core_runtime.ush' by ush.
# DO NOT EDIT MANUALLY.
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/core_runtime.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/core_runtime.ush'

__ush_at() { __ush_line=$1; __ush_source=$2; }
__ush_report() {
  if [ "$1" -ne 0 ] && [ -n "${__ush_line:-}" ]; then
    printf '%s:%s: %s (exit %s)\n  %s\n' "$__ush_origin" "$__ush_line" "${2:-command failed}" "$1" "$__ush_source" >&2
    __ush_line=''
  fi
}
trap '__ush_report "$?"' 0

case "${1:-}" in
  -h|--help) printf '%s' 'core_runtime.ush

Usage:
  core_runtime.ush [--help]
  core_runtime.ush [--man [ITEM]]
  core_runtime.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
core_runtime.ush

SYNOPSIS
core_runtime.ush [--help]
core_runtime.ush [--man [ITEM]]
core_runtime.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

__ush_walk_record_8_Envelope() {
  "$1" "$2__tag" || return "$?"
  eval "__ush_tag=\${${2}__tag}"
  case $__ush_tag in
    'Envelope::Wrap')
      __ush_walk_record_8_Response "$1" "$2__0" || return "$?"
      ;;
    'Envelope::Empty')
      ;;
    *) return 1 ;;
  esac
}

__ush_walk_record_8_Response() {
  "$1" "$2__tag" || return "$?"
  eval "__ush_tag=\${${2}__tag}"
  case $__ush_tag in
    'Response::Ok')
      "$1" "$2__0" || return "$?"
      ;;
    'Response::Err')
      "$1" "$2__0" || return "$?"
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
  message="${__ush_fn_greet_arg_0}"
  __ush_fn_greet_arg_1="${2}"
  count="${__ush_fn_greet_arg_1}"
  __ush_at 12 'message + ":" + count'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "$(printf '%s' "${__ush_fn_greet_arg_0}" ':' "${__ush_fn_greet_arg_1}")" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "$(printf '%s' "${__ush_fn_greet_arg_0}" ':' "${__ush_fn_greet_arg_1}")" >&4
  fi
  return 0
}

ush_fn_wrap() {
  __ush_fn_wrap_arg_0="${1}"
  message="${__ush_fn_wrap_arg_0}"
  __ush_at 16 '"<" + message + ">"'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "$(printf '%s' '<' "${__ush_fn_wrap_arg_0}" '>')" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "$(printf '%s' '<' "${__ush_fn_wrap_arg_0}" '>')" >&4
  fi
  return 0
}

__ush_at 19 'let wrapped = Envelope::Wrap(Response::Ok("done"))'
wrapped__tag='Envelope::Wrap'
wrapped__0__tag='Response::Ok'
wrapped__0__0='done'
__ush_at 20 'match wrapped {'
__ush_cells="$(__ush_walk_record_8_Envelope ush_fn_std__write_cell wrapped)"
__ush_walk_record_8_Envelope ush_fn_std__read_cell __ush_match_0
if [ "${__ush_match_0__tag}" = 'Envelope::Wrap' ] && [ "${__ush_match_0__0__tag}" = 'Response::Ok' ]; then
  message="${__ush_match_0__0__0-}"
  __ush_at 21 'Envelope::Wrap(Response::Ok(message)) => print $ wrap message'
  __ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_wrap "${message}" 4>&1 >&3)"
  printf '%s\n' "${__ush_value_1}"
elif :; then
  __ush_at 22 '_ => print "fallback"'
  printf '%s\n' 'fallback'
fi
__ush_at 25 'let value = greet "hello" 2'
__ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_greet 'hello' 2 4>&1 >&3)"
value="${__ush_value_2}"
__ush_at 26 'print value'
printf '%s\n' "${value}"
__ush_at 27 'print $ wrap (greet "team" 3)'
__ush_value_3="$(__ush_return_path='' __ush_capture_return='1' ush_fn_greet 'team' 3 4>&1 >&3)"
__ush_value_4="$(__ush_return_path='' __ush_capture_return='1' ush_fn_wrap "${__ush_value_3}" 4>&1 >&3)"
printf '%s\n' "${__ush_value_4}"
