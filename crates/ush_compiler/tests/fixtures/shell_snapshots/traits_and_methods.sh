#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/traits_and_methods.ush' by ush.
# DO NOT EDIT MANUALLY.
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/traits_and_methods.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/traits_and_methods.ush'

__ush_at() { __ush_line=$1; __ush_source=$2; }
__ush_report() {
  if [ "$1" -ne 0 ] && [ -n "${__ush_line:-}" ]; then
    printf '%s:%s: %s (exit %s)\n  %s\n' "$__ush_origin" "$__ush_line" "${2:-command failed}" "$1" "$__ush_source" >&2
    __ush_line=''
  fi
}
trap '__ush_report "$?"' 0

case "${1:-}" in
  -h|--help) printf '%s' 'traits_and_methods.ush

Usage:
  traits_and_methods.ush [--help]
  traits_and_methods.ush [--man [ITEM]]
  traits_and_methods.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
traits_and_methods.ush

SYNOPSIS
traits_and_methods.ush [--help]
traits_and_methods.ush [--man [ITEM]]
traits_and_methods.ush [--complete [PREFIX]]
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

__ush_at 22 'let user = User { name: "ush", age: 7 }'
user__tag='User::User'
user__name='ush'
user__age=7
__ush_at 23 'print user'
__ush_cells="$(__ush_walk_record_4_User ush_fn_std__write_cell user)"
__ush_walk_record_4_User ush_fn_std__read_cell __ush_match_0
__ush_method_1() {
__ush_at 12 'self.name + ":" + self.age'
__ush_cells="$(__ush_walk_record_4_User ush_fn_std__write_cell __ush_match_0)"
__ush_walk_record_4_User ush_fn_std__read_cell __ush_match_2
__ush_cells="$(__ush_walk_record_4_User ush_fn_std__write_cell __ush_match_0)"
__ush_walk_record_4_User ush_fn_std__read_cell __ush_match_4
if [ -n "${__ush_return_path:-}" ]; then
  printf '%s' "$(printf '%s' "${__ush_match_2__name}" ':' "${__ush_match_4__age}")" > "$__ush_return_path"
elif [ "${__ush_capture_return:-0}" = '1' ]; then
  printf '%s' "$(printf '%s' "${__ush_match_2__name}" ':' "${__ush_match_4__age}")" >&4
fi
return 0
}
__ush_value_6="$(__ush_capture_return='1' __ush_return_path='' __ush_method_1 4>&1 >&3)"
printf '%s\n' "${__ush_value_6}"
__ush_at 24 'print format user'
__ush_cells="$(__ush_walk_record_4_User ush_fn_std__write_cell user)"
__ush_walk_record_4_User ush_fn_std__read_cell __ush_match_7
__ush_method_8() {
__ush_at 12 'self.name + ":" + self.age'
__ush_cells="$(__ush_walk_record_4_User ush_fn_std__write_cell __ush_match_7)"
__ush_walk_record_4_User ush_fn_std__read_cell __ush_match_9
__ush_cells="$(__ush_walk_record_4_User ush_fn_std__write_cell __ush_match_7)"
__ush_walk_record_4_User ush_fn_std__read_cell __ush_match_11
if [ -n "${__ush_return_path:-}" ]; then
  printf '%s' "$(printf '%s' "${__ush_match_9__name}" ':' "${__ush_match_11__age}")" > "$__ush_return_path"
elif [ "${__ush_capture_return:-0}" = '1' ]; then
  printf '%s' "$(printf '%s' "${__ush_match_9__name}" ':' "${__ush_match_11__age}")" >&4
fi
return 0
}
__ush_value_13="$(__ush_capture_return='1' __ush_return_path='' __ush_method_8 4>&1 >&3)"
__ush_value_14="${__ush_value_13}"
printf '%s\n' "${__ush_value_14}"
__ush_at 25 'print user.prefixed("id")'
__ush_cells="$(__ush_walk_record_4_User ush_fn_std__write_cell user)"
__ush_walk_record_4_User ush_fn_std__read_cell __ush_match_15
__ush_prefix_16='id'
__ush_method_17() {
__ush_at 18 'prefix + ":" + self.name'
__ush_cells="$(__ush_walk_record_4_User ush_fn_std__write_cell __ush_match_15)"
__ush_walk_record_4_User ush_fn_std__read_cell __ush_match_18
if [ -n "${__ush_return_path:-}" ]; then
  printf '%s' "$(printf '%s' "${__ush_prefix_16}" ':' "${__ush_match_18__name}")" > "$__ush_return_path"
elif [ "${__ush_capture_return:-0}" = '1' ]; then
  printf '%s' "$(printf '%s' "${__ush_prefix_16}" ':' "${__ush_match_18__name}")" >&4
fi
return 0
}
__ush_value_20="$(__ush_capture_return='1' __ush_return_path='' __ush_method_17 4>&1 >&3)"
__ush_value_21="${__ush_value_20}"
printf '%s\n' "${__ush_value_21}"
__ush_at 26 'print () == ()'
printf '%s\n' "$(if :; then printf '%s' true; else printf '%s' false; fi)"
__ush_at 27 'print "ant" < "bee"'
printf '%s\n' "$(if [ 'ant' != 'bee' ] && [ "$(printf '%s\n%s\n' 'ant' 'bee' | LC_ALL=C sort | head -n 1)" = 'ant' ]; then printf '%s' true; else printf '%s' false; fi)"
