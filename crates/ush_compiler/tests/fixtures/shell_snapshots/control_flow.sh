#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/control_flow.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/control_flow.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/control_flow.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/control_flow.ush'

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
  -h|--help) printf '%s' 'control_flow.ush

Usage:
  control_flow.ush [--help]
  control_flow.ush [--man [ITEM]]
  control_flow.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
control_flow.ush

SYNOPSIS
control_flow.ush [--help]
control_flow.ush [--man [ITEM]]
control_flow.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

ush_fn_pick() {
  __ush_fn_pick_arg_0="${1}"
  # shellcheck disable=SC2034
  flag="${__ush_fn_pick_arg_0}"
  __ush_at 2 'if flag {'
  if {
  [ "${__ush_fn_pick_arg_0}" = 'true' ]
  }; then
    __ush_at 3 '"yes"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'yes' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'yes' >&4
    fi
    return 0
  else
    __ush_at 6 '"no"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'no' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'no' >&4
    fi
    return 0
  fi
}

__ush_at 10 'print $ pick true'
__ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_pick 'true' 4>&1 >&3)" || exit "$?"
printf '%s\n' "${__ush_value_0}"
__ush_at 12 'for item in 0..3 {'
__ush_for_index_1=0
while [ "${__ush_for_index_1}" -lt 3 ]; do
  item="${__ush_for_index_1}"
  __ush_for_index_1=$((__ush_for_index_1 + 1))
  __ush_at 13 'print item'
  printf '%s\n' $((item))
done
__ush_at 16 'let items = [3, 4]'
items__len='2'
# shellcheck disable=SC2034
items__0=3
# shellcheck disable=SC2034
items__1=4
__ush_at 17 'for item in items {'
__ush_match_2__len="${items__len}"
__ush_copy_index_3=0
while [ "$__ush_copy_index_3" -lt "${items__len}" ]; do
  eval "__ush_copy_value_4=\"\${items__$__ush_copy_index_3}\""
  eval "__ush_match_2__${__ush_copy_index_3}=\$__ush_copy_value_4"
  __ush_copy_index_3=$((__ush_copy_index_3 + 1))
done
__ush_for_list_index_5=0
while [ "${__ush_for_list_index_5}" -lt "${__ush_match_2__len}" ]; do
  __ush_for_value_6=''
  eval "__ush_for_value_6=\"\${__ush_match_2__${__ush_for_list_index_5}}\""
  item="${__ush_for_value_6}"
  __ush_for_list_index_5=$((__ush_for_list_index_5 + 1))
  __ush_at 18 'print item'
  printf '%s\n' $((item))
done
__ush_at 21 'let pair = (5, 6)'
pair__len='2'
# shellcheck disable=SC2034
pair__0=5
# shellcheck disable=SC2034
pair__1=6
__ush_at 22 'for item in pair {'
__ush_match_7__len='2'
__ush_match_7__0="${pair__0}"
__ush_match_7__1="${pair__1}"
item=$((__ush_match_7__0))
__ush_at 23 'print item'
printf '%s\n' $((item))
item=$((__ush_match_7__1))
__ush_at 23 'print item'
printf '%s\n' $((item))
__ush_at 26 'let count = 0'
count=0
__ush_at 27 'while count < 2 {'
while {
[ "$(if [ $((count)) -lt 2 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; do
  __ush_at 28 'print count'
  printf '%s\n' $((count))
  __ush_at 29 'let count = count + 1'
  count=$((count + 1))
done
__ush_at 32 'loop {'
while :; do
  __ush_at 33 'print 9'
  printf '%s\n' 9
  __ush_at 34 'break'
  break
done
__ush_at 42 'let maybe = Option::Some(7)'
maybe__tag='Option::Some'
# shellcheck disable=SC2034
maybe__0=7
__ush_at 43 'if let Option::Some(it) = maybe && it == 7 {'
if {
{
it="${maybe__0-}"
[ "${maybe__tag}" = 'Option::Some' ]
} && {
[ "$(if [ $((it)) -eq 7 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}
}; then
  __ush_at 44 'print it'
  printf '%s\n' $((it))
fi
