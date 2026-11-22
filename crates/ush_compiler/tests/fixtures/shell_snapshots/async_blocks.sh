#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/async_blocks.ush' by ush.
# DO NOT EDIT MANUALLY.
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/async_blocks.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/async_blocks.ush'

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
__ush_wait_job() {
  __ush_wait_fallback=127
  case " $__ush_jobs " in
    *" $1 "*)
      __ush_wait_fallback=0
      wait "$1" 2>/dev/null || __ush_wait_fallback=$?
      __ush_remaining=''
      for __ush_pid in $__ush_jobs; do
        if [ "$__ush_pid" != "$1" ]; then __ush_remaining="$__ush_remaining $__ush_pid"; fi
      done
      __ush_jobs=$__ush_remaining ;;
  esac
  # Only the launching shell can reap the child. Other task scopes may
  # also await its result, including after the launching shell has reaped it.
  while ! IFS= read -r __ush_wait_status < "$2"; do
    if ! kill -0 "$1" 2>/dev/null; then
      IFS= read -r __ush_wait_status < "$2" || return "$__ush_wait_fallback"
      break
    fi
    sleep 0.01 2>/dev/null || sleep 1
  done
  return "$__ush_wait_status"
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
  __ush_completion=${1:-}
  trap '__ush_end_scope "$?"' 0
}
__ush_end_scope() {
  __ush_join "$1"
  if [ -n "$__ush_completion" ]; then
    printf '%s\n' "$__ush_exit" > "$__ush_completion"
  fi
  exit "$__ush_exit"
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
  -h|--help) printf '%s' 'async_blocks.ush

Usage:
  async_blocks.ush [--help]
  async_blocks.ush [--man [ITEM]]
  async_blocks.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
async_blocks.ush

SYNOPSIS
async_blocks.ush [--help]
async_blocks.ush [--man [ITEM]]
async_blocks.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

ush_fn_worker() {
  __ush_fn_worker_arg_0="${1}"
  message="${__ush_fn_worker_arg_0}"
  __ush_at 2 'message'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_fn_worker_arg_0}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_fn_worker_arg_0}" >&4
  fi
  return 0
}

__ush_at 5 'print "main"'
printf '%s\n' 'main'
__ush_at 6 'let task = async worker "worker"'
__ush_task_slot
__ush_task_0__result=$__ush_slot/result
: > "${__ush_task_0__result}"
: > "${__ush_task_0__result}.status"
( __ush_task_scope "${__ush_task_0__result}.status"; __ush_return_path="${__ush_task_0__result}"; ush_fn_worker 'worker' ) &
__ush_task_0__pid=$!
__ush_task_0__awaited=0
__ush_jobs="${__ush_jobs} $!"
__ush_at 7 'let block = async {'
ush_fn___ush_task_block_1() {
  __ush_at 8 'let prefix = "work"'
  prefix='work'
  __ush_at 9 'prefix + "bench"'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "$(printf '%s' "${prefix}" 'bench')" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "$(printf '%s' "${prefix}" 'bench')" >&4
  fi
  return 0
}
__ush_task_slot
__ush_task_2__result=$__ush_slot/result
: > "${__ush_task_2__result}"
: > "${__ush_task_2__result}.status"
( __ush_task_scope "${__ush_task_2__result}.status"; __ush_return_path="${__ush_task_2__result}"; ush_fn___ush_task_block_1 ) &
__ush_task_2__pid=$!
__ush_task_2__awaited=0
__ush_jobs="${__ush_jobs} $!"
__ush_at 11 'let number = async {'
ush_fn___ush_task_block_3() {
  __ush_at 12 'if true {'
  if {
  [ 'true' = 'true' ]
  }; then
    __ush_at 13 'return 40'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 40 > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 40 >&4
    fi
    return 0
  fi
  __ush_at 15 '0'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' 0 > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' 0 >&4
  fi
  return 0
}
__ush_task_slot
__ush_task_4__result=$__ush_slot/result
: > "${__ush_task_4__result}"
: > "${__ush_task_4__result}.status"
( __ush_task_scope "${__ush_task_4__result}.status"; __ush_return_path="${__ush_task_4__result}"; ush_fn___ush_task_block_3 ) &
__ush_task_4__pid=$!
__ush_task_4__awaited=0
__ush_jobs="${__ush_jobs} $!"
__ush_at 17 'print "after"'
printf '%s\n' 'after'
__ush_at 18 'let result = task.await'
if [ "${__ush_task_0__awaited}" = '0' ]; then
  __ush_wait_job "${__ush_task_0__pid}" "${__ush_task_0__result}.status" || exit "$?"
  __ush_task_0__awaited='1'
fi
result="$(cat "${__ush_task_0__result}")"
__ush_at 19 'let next = block.await'
if [ "${__ush_task_2__awaited}" = '0' ]; then
  __ush_wait_job "${__ush_task_2__pid}" "${__ush_task_2__result}.status" || exit "$?"
  __ush_task_2__awaited='1'
fi
next="$(cat "${__ush_task_2__result}")"
__ush_at 20 'let value = number.await'
if [ "${__ush_task_4__awaited}" = '0' ]; then
  __ush_wait_job "${__ush_task_4__pid}" "${__ush_task_4__result}.status" || exit "$?"
  __ush_task_4__awaited='1'
fi
value="$(cat "${__ush_task_4__result}")"
__ush_at 21 'print result'
printf '%s\n' "${result}"
__ush_at 22 'print next'
printf '%s\n' "${next}"
__ush_at 23 'print value + 2'
printf '%s\n' $((value + 2))
