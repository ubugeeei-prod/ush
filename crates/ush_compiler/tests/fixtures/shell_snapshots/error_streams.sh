#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/error_streams.ush' by ush.
# DO NOT EDIT MANUALLY.
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/error_streams.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/error_streams.ush'

__ush_at() { __ush_line=$1; __ush_source=$2; }
__ush_report() {
  if [ "$1" -ne 0 ] && [ -n "${__ush_line:-}" ]; then
    printf '%s:%s: %s (exit %s)\n  %s\n' "$__ush_origin" "$__ush_line" "${2:-command failed}" "$1" "$__ush_source" >&2
    __ush_line=''
  fi
}

__ush_jobs=''
__ush_task_seq=0
__ush_task_dir=$(
  umask 077
  CDPATH= cd -P "${TMPDIR:-/tmp}" || exit 1
  __ush_base=$(pwd -P)
  __ush_n=0
  while [ "$__ush_n" -lt 1000 ]; do
    __ush_dir=$__ush_base/ush-tasks.$$.$__ush_n
    if mkdir "$__ush_dir" 2>/dev/null; then printf '%s' "$__ush_dir"; exit 0; fi
    __ush_n=$((__ush_n + 1))
  done
  printf '%s\n' 'ush: cannot create private task directory' >&2
  exit 1
)
__ush_task_slot() {
  while :; do
    __ush_task_seq=$((__ush_task_seq + 1))
    __ush_slot=$__ush_task_dir/$__ush_task_seq
    if mkdir "$__ush_slot" 2>/dev/null; then break; fi
    [ -d "$__ush_slot" ] || return 1
  done
}
__ush_forget_job() {
  __ush_remaining=''
  for __ush_pid in $__ush_jobs; do
    if [ "$__ush_pid" != "$1" ]; then __ush_remaining="$__ush_remaining $__ush_pid"; fi
  done
  __ush_jobs=$__ush_remaining
}
__ush_join() {
  __ush_exit=$1
  __ush_report "$__ush_exit"
  for __ush_pid in $__ush_jobs; do
    wait "$__ush_pid" || { __ush_child=$?; [ "$__ush_exit" -ne 0 ] || __ush_exit=$__ush_child; }
  done
}
__ush_task_scope() {
  __ush_jobs=''
  trap '__ush_join "$?"; exit "$__ush_exit"' 0
}
__ush_finish() {
  __ush_join "$1"
  rm -rf "$__ush_task_dir"
  exit "$__ush_exit"
}
trap '__ush_finish "$?"' 0
trap 'exit 129' HUP
trap 'exit 130' INT
trap 'exit 143' TERM

case "${1:-}" in
  -h|--help) printf '%s' 'error_streams.ush

Usage:
  error_streams.ush [--help]
  error_streams.ush [--man [ITEM]]
  error_streams.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
error_streams.ush

SYNOPSIS
error_streams.ush [--help]
error_streams.ush [--man [ITEM]]
error_streams.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

# raises: Problem
ush_fn_fail() {
  __ush_at 6 'fail Problem::Nope'
  __ush_report 1 'Problem::Nope'
  return 1
}

ush_fn_wrap() {
  __ush_fn_wrap_arg_0="${1}"
  message="${__ush_fn_wrap_arg_0}"
  __ush_at 10 '"<" + message + ">"'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "$(printf '%s' '<' "${__ush_fn_wrap_arg_0}" '>')" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "$(printf '%s' '<' "${__ush_fn_wrap_arg_0}" '>')" >&4
  fi
  return 0
}

# raises: Problem
ush_fn_outer() {
  __ush_at 14 'let value = fail()?'
  __ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_fail 4>&1 >&3)" || return "$?"
  value="${__ush_value_0}"
  __ush_at 15 'value'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${value}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${value}" >&4
  fi
  return 0
}

# raises: Problem | unknown
ush_fn_mixed() {
  __ush_at 19 '$ false'
  {
false
  } || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 20 '$ wrap $ outer ()'
  {
wrap $ outer ()
  } || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
}

# raises: Problem
ush_fn_awaited() {
  (
    __ush_task_scope
    __ush_at 24 'let task = async fail ()'
    __ush_task_slot
    __ush_task_1__result=$__ush_slot/result
    : > "${__ush_task_1__result}"
    ( __ush_task_scope; __ush_return_path="${__ush_task_1__result}"; ush_fn_fail ) &
    __ush_task_1__pid=$!
    __ush_task_1__awaited=0
    __ush_jobs="${__ush_jobs} $!"
    __ush_at 25 'let value = task.await'
    if [ "${__ush_task_1__awaited}" = '0' ]; then
      wait "${__ush_task_1__pid}" || exit "$?"
      __ush_forget_job "${__ush_task_1__pid}"
      __ush_task_1__awaited='1'
    fi
    value="$(cat "${__ush_task_1__result}")"
    __ush_at 26 'value'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${value}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${value}" >&4
    fi
    return 0
  )
}

__ush_at 29 'print $ wrap $ awaited ()'
__ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_awaited 4>&1 >&3)"
__ush_value_3="$(__ush_return_path='' __ush_capture_return='1' ush_fn_wrap "${__ush_value_2}" 4>&1 >&3)"
printf '%s\n' "${__ush_value_3}"
