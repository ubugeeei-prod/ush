#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/error_streams.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/error_streams.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/error_streams.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/error_streams.ush'

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

__ush_jobs=''
__ush_signal=''
__ush_task_seq=0
__ush_task_dir=''
__ush_task_init() {
  [ -z "$__ush_task_dir" ] || return 0
  __ush_task_dir=$(
    umask 077
    CDPATH='' cd -P "${TMPDIR:-/tmp}" || exit 1
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
}

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
      # A task a signal ended wrote no status, or, in some shells, the
      # status of its last command; keep the one wait saw, so every later
      # await observes it too.
      if [ "$__ush_wait_fallback" -gt 128 ] || [ ! -s "$2" ]; then
        printf '%s\n' "$__ush_wait_fallback" > "$2"
      fi
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
# Report how the scope ended, run its deferred blocks, then wait for its
# unfinished tasks. A task killed through its handle does not fail the scope.
__ush_join() {
  __ush_exit=$1
  __ush_report "$__ush_exit"
  [ -z "${__ush_defers:-}" ] || __ush_run_defers
  __ush_line=''
  for __ush_pid in $__ush_jobs; do
    case " ${__ush_killed:-} " in
      *" $__ush_pid "*) wait "$__ush_pid" 2>/dev/null || : ;;
      *) wait "$__ush_pid" || { __ush_child=$?; [ "$__ush_exit" -ne 0 ] || __ush_exit=$__ush_child; } ;;
    esac
  done
}
# A task or function scope. It leaves signals alone, so a task killed
# through its handle dies at once even before it starts its command; a shell
# would otherwise hold a trapped signal until its foreground command ends.
__ush_task_scope() {
  __ush_jobs=''
  __ush_defers=''
  __ush_completion=${1:-}
  trap '__ush_end_scope "$?"' 0
  # A task's handle may now signal it: the shell has dropped the traps it
  # inherited, which would otherwise swallow the signal.
  [ -z "$__ush_completion" ] || : > "$__ush_completion.started"
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
  [ -z "$__ush_task_dir" ] || rm -rf "$__ush_task_dir"
  exit "$__ush_exit"
}
trap '__ush_finish "${__ush_signal:-$?}"' 0
trap '__ush_signal=129; exit 129' HUP
trap '__ush_signal=130; exit 130' INT
trap '__ush_signal=143; exit 143' TERM

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
  __ush_at 6 'fail (Problem::Nope)'
  __ush_report 1 'Problem::Nope'
  return 1
}

ush_fn_wrap() {
  __ush_fn_wrap_arg_0="${1}"
  # shellcheck disable=SC2034
  message="${__ush_fn_wrap_arg_0}"
  __ush_at 10 '"<" + message + ">"'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' '<'"${__ush_fn_wrap_arg_0}"'>' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' '<'"${__ush_fn_wrap_arg_0}"'>' >&4
  fi
  return 0
}

# raises: Problem
ush_fn_outer() {
  __ush_at 14 'let value = fail()?'
  __ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_fail 4>&1 >&3)" || return "$?"
  __ush_binding_14_value="${__ush_value_0}"
  # shellcheck disable=SC2034
  value="${__ush_binding_14_value}"
  __ush_at 15 'value'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_binding_14_value}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_binding_14_value}" >&4
  fi
  return 0
}

# raises: Problem | unknown
ush_fn_mixed() {
  __ush_at 19 '$ false'
  {
false
  } || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 20 'wrap(outer()?)'
  __ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_outer 4>&1 >&3)" || return "$?"
  __ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_wrap "${__ush_value_1}" 4>&1 >&3)" || return "$?"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_value_2}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_value_2}" >&4
  fi
  return 0
}

# raises: Problem
ush_fn_awaited() {
  (
    __ush_task_scope
    __ush_at 24 'let task = async fail ()'
    __ush_task_slot
    __ush_task_3__result=$__ush_slot/result
    : > "${__ush_task_3__result}"
    : > "${__ush_task_3__result}.status"
    ( __ush_task_scope "${__ush_task_3__result}.status"; __ush_return_path="${__ush_task_3__result}"; ush_fn_fail ) &
    __ush_task_3__pid=$!
    __ush_task_3__awaited=0
    __ush_jobs="${__ush_jobs} $!"
    __ush_binding_24_task__pid="${__ush_task_3__pid}"
    __ush_binding_24_task__result="${__ush_task_3__result}"
    __ush_binding_24_task__awaited="${__ush_task_3__awaited}"
    __ush_at 25 'let value = task.await'
    if [ "${__ush_binding_24_task__awaited}" = '0' ]; then
      __ush_wait_job "${__ush_binding_24_task__pid}" "${__ush_binding_24_task__result}.status" || exit "$?"
      __ush_binding_24_task__awaited='1'
    fi
    __ush_binding_25_value="$(cat "${__ush_binding_24_task__result}")"
    # shellcheck disable=SC2034
    value="${__ush_binding_25_value}"
    __ush_at 26 'value'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${__ush_binding_25_value}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${__ush_binding_25_value}" >&4
    fi
    return 0
  )
}

__ush_at 29 'print wrap (awaited ())'
__ush_task_init
__ush_at 29 'print wrap (awaited ())'
__ush_value_4="$(__ush_return_path='' __ush_capture_return='1' ush_fn_awaited 4>&1 >&3)" || exit "$?"
__ush_value_5="$(__ush_return_path='' __ush_capture_return='1' ush_fn_wrap "${__ush_value_4}" 4>&1 >&3)" || exit "$?"
printf '%s\n' "${__ush_value_5}"
