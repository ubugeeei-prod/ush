#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/background_and_defer.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/background_and_defer.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/background_and_defer.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/background_and_defer.ush'

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
  -h|--help) printf '%s' 'background_and_defer.ush

Usage:
  background_and_defer.ush [--help]
  background_and_defer.ush [--man [ITEM]]
  background_and_defer.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
background_and_defer.ush

SYNOPSIS
background_and_defer.ush [--help]
background_and_defer.ush [--man [ITEM]]
background_and_defer.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

# Report the leftmost failing pipeline stage. $1 holds one ` INDEX:STATUS`
# per stage in completion order; each further argument is the diagnostic
# for that stage. Every stage holds its end of the pipe until it has
# reported, so a stage whose reader reported first was cut off by a
# reader that stopped early, as `head` does. Its status is SIGPIPE (141),
# or a write error where the caller ignores SIGPIPE; either way the
# reader's own status decides. An empty diagnostic, as in a condition or
# `status_of`, sets the status without reporting it.
__ush_pipe_check() {
  __ush_pipe_statuses=" $1 "
  shift
  __ush_pipe_index=0
  for __ush_pipe_message do
    __ush_pipe_index=$((__ush_pipe_index + 1))
    case $__ush_pipe_statuses in
      *" $__ush_pipe_index:0 "*) continue ;;
      *" $__ush_pipe_index:"*) ;;
      *)
        [ -z "$__ush_pipe_message" ] || __ush_report 1 "$__ush_pipe_message (no exit status)"
        return 1 ;;
    esac
    __ush_pipe_code=${__ush_pipe_statuses#*" $__ush_pipe_index:"}
    __ush_pipe_code=${__ush_pipe_code%% *}
    __ush_pipe_earlier=${__ush_pipe_statuses%%" $__ush_pipe_index:"*}
    case $__ush_pipe_earlier in
      *" $((__ush_pipe_index + 1)):"*) continue ;;
    esac
    [ -z "$__ush_pipe_message" ] || __ush_report "$__ush_pipe_code" "$__ush_pipe_message"
    return "$__ush_pipe_code"
  done
}

# Run the deferred blocks of the current scope, last registered first. A
# block that fails has reported its own line; the rest still run, and the
# first failure becomes the exit status if the scope was succeeding.
__ush_run_defers() {
  while [ -n "${__ush_defers:-}" ]; do
    __ush_defer=${__ush_defers%% *}
    __ush_defers=${__ush_defers#* }
    "$__ush_defer" || {
      __ush_status=$?
      [ "$__ush_exit" -ne 0 ] || __ush_exit=$__ush_status
    }
  done
}
# A function that defers runs in a process of its own; its exit, including
# one caused by a handled signal, runs its blocks.
__ush_defer_scope() {
  __ush_signal=''
  __ush_defers=''
  trap '__ush_defer_exit "${__ush_signal:-$?}"' 0
  trap '__ush_signal=129; exit 129' HUP
  trap '__ush_signal=130; exit 130' INT
  trap '__ush_signal=143; exit 143' TERM
}
__ush_defer_exit() {
  __ush_exit=$1
  __ush_run_defers
  exit "$__ush_exit"
}

# Send signal $2 to task $1, whose status file is $3, and every process it
# started. A task this scope has already reaped is left alone: its ID may
# belong to another process by now. A killed task's status does not fail
# the scope when it is joined; awaiting it still reports the signal.
__ush_task_kill() {
  case $2 in
    HUP|INT|QUIT|KILL|USR1|USR2|ALRM|TERM|STOP|CONT) ;;
    *) __ush_report 2 "unknown signal: $2"; return 2 ;;
  esac
  case " $__ush_jobs " in *" $1 "*) ;; *) return 0 ;; esac
  # A task that has not started yet still holds the traps of the shell it
  # was forked from; wait until it has dropped them.
  while [ ! -e "$3.started" ] && kill -0 "$1" 2>/dev/null; do
    sleep 0.01 2>/dev/null || sleep 1
  done
  __ush_killed="${__ush_killed:-} $1"
  # The task itself is this shell's child and is not stopped: some shells
  # would report the stop to a later wait as the task's status.
  __ush_kill_children=$(__ush_child_pids "$1")
  kill -s "$2" "$1" 2>/dev/null || :
  for __ush_kill_child in $__ush_kill_children; do
    __ush_kill_tree "$__ush_kill_child" "$2"
  done
}
# Each descendant is stopped before its children are listed, so none can
# start another one unseen, signalled while stopped, and continued once its
# children are signalled, so a stopped process receives the signal.
__ush_kill_tree() {
  case $2 in STOP|CONT) ;; *) kill -s STOP "$1" 2>/dev/null || return 0 ;; esac
  __ush_kill_children=$(__ush_child_pids "$1")
  kill -s "$2" "$1" 2>/dev/null || :
  for __ush_kill_child in $__ush_kill_children; do
    __ush_kill_tree "$__ush_kill_child" "$2"
  done
  [ "$2" = STOP ] || kill -s CONT "$1" 2>/dev/null || :
}
__ush_child_pids() {
  { ps -A -o pid= -o ppid= 2>/dev/null || ps -o pid= -o ppid=; } |
    awk -v parent="$1" '$2 == parent { print $1 }'
}

ush_fn_std__fs__temp_dir() {
  mktemp -d "$__ush_task_dir/tmp.XXXXXXXXXX"
}

# raises: unknown
ush_fn_serve() {
  __ush_fn_serve_arg_0="${1}"
  # shellcheck disable=SC2034
  port="${__ush_fn_serve_arg_0}"
  (
    __ush_defer_scope
    __ush_task_scope
    __ush_at 5 'let server = spawn python3 "-m" "http.server" $port'
    ush_fn___ush_task_block_0() {
      __ush_at 5 'let server = spawn python3 "-m" "http.server" $port'
      command python3 '-m' 'http.server' "${__ush_fn_serve_arg_0}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
    }
    __ush_task_slot
    __ush_task_1__result=$__ush_slot/result
    : > "${__ush_task_1__result}"
    : > "${__ush_task_1__result}.status"
    ( __ush_task_scope "${__ush_task_1__result}.status"; __ush_return_path="${__ush_task_1__result}"; ush_fn___ush_task_block_0 ) &
    __ush_task_1__pid=$!
    __ush_task_1__awaited=0
    __ush_jobs="${__ush_jobs} $!"
    __ush_binding_5_server__pid="${__ush_task_1__pid}"
    __ush_binding_5_server__result="${__ush_task_1__result}"
    __ush_binding_5_server__awaited="${__ush_task_1__awaited}"
    __ush_at 6 'defer server.kill()'
    __ush_defer_2() {
      __ush_at 6 'defer server.kill()'
      __ush_task_kill "${__ush_binding_5_server__pid}" TERM "${__ush_binding_5_server__result}.status" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
    }
    __ush_defers="__ush_defer_2 ${__ush_defers:-}"
    __ush_at 7 'curl "-sf" ("http://127.0.0.1:" + port)'
    command curl '-sf' 'http://127.0.0.1:'"${__ush_fn_serve_arg_0}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  )
}

__ush_at 10 'let work = std::fs::temp_dir'
__ush_task_init
__ush_at 10 'let work = std::fs::temp_dir'
__ush_value_3="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
__ush_binding_10_work="${__ush_value_3}"
# shellcheck disable=SC2034
work="${__ush_binding_10_work}"
__ush_at 11 'defer {'
__ush_defer_4() {
  __ush_at 12 'rm "-rf" (work + "/cache")'
  command rm '-rf' "${__ush_binding_10_work}"'/cache' || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
}
__ush_defers="__ush_defer_4 ${__ush_defers:-}"
__ush_at 14 'let build = spawn cargo "build" |& to_file "build.log"'
ush_fn___ush_task_block_5() {
  __ush_at 14 'let build = spawn cargo "build" |& to_file "build.log"'
  { __ush_pipe=$(
    exec 7>&1 >&8 8>&-
    set +e
    { (command cargo 'build') 7>&- >'build.log' 2>&1; printf ' 1:%s' "$?" >&7; }
  ); } 8>&1
  __ush_pipe_check "$__ush_pipe" 'pipeline stage 1 failed: cargo "build" |& to_file "build.log"' || return "$?"
}
__ush_task_slot
__ush_task_6__result=$__ush_slot/result
: > "${__ush_task_6__result}"
: > "${__ush_task_6__result}.status"
( __ush_task_scope "${__ush_task_6__result}.status"; __ush_return_path="${__ush_task_6__result}"; ush_fn___ush_task_block_5 ) &
__ush_task_6__pid=$!
__ush_task_6__awaited=0
__ush_jobs="${__ush_jobs} $!"
__ush_binding_14_build__pid="${__ush_task_6__pid}"
__ush_binding_14_build__result="${__ush_task_6__result}"
__ush_binding_14_build__awaited="${__ush_task_6__awaited}"
__ush_at 15 'let status = status_of build.await'
if { [ "${__ush_binding_14_build__awaited}" = '1' ] || { __ush_wait_job "${__ush_binding_14_build__pid}" "${__ush_binding_14_build__result}.status" && __ush_binding_14_build__awaited='1'; }; }; then __ush_status_7=0; else __ush_status_7=$?; fi
__ush_binding_15_status="${__ush_status_7}"
# shellcheck disable=SC2034
status="${__ush_binding_15_status}"
__ush_at 16 'print "build exited " + status'
printf '%s\n' 'build exited '"${__ush_binding_15_status}"
__ush_at 17 'serve "8000"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_serve '8000' )
