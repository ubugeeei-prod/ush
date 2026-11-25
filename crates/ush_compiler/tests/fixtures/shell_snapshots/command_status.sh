#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/command_status.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/command_status.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_lf='
'
__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/command_status.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/command_status.ush'

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
  -h|--help) printf '%s' 'command_status.ush

Usage:
  command_status.ush [--help]
  command_status.ush [--man [ITEM]]
  command_status.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
command_status.ush

SYNOPSIS
command_status.ush [--help]
command_status.ush [--man [ITEM]]
command_status.ush [--complete [PREFIX]]
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

__ush_at 4 'if git "diff" "--quiet" {'
if {
command git 'diff' '--quiet'
}; then
  __ush_at 5 'print "clean"'
  printf '%s\n' 'clean'
fi
__ush_at 7 'if ! grep "-Fxq" "ready" "status.txt" || git "cat-file" "-e" "HEAD:README.md" | discard_stderr {'
if {
! {
command grep '-Fxq' 'ready' 'status.txt'
} || {
(command git 'cat-file' '-e' 'HEAD:README.md') 2>/dev/null
}
}; then
  __ush_at 8 'print "check"'
  printf '%s\n' 'check'
fi
__ush_at 10 'let code = status_of cargo "test" | tee "test.log"'
if {
{ __ush_pipe=$(
  exec 7>&1 >&8 8>&-
  set +e
  { (command cargo 'test') 7>&-; printf ' 1:%s' "$?" >&7; } |
  { (command tee 'test.log') 7>&-; printf ' 2:%s' "$?" >&7; }
); } 8>&1
__ush_pipe_check "$__ush_pipe" '' ''
}; then __ush_status_0=0; else __ush_status_0=$?; fi
__ush_binding_10_code="${__ush_status_0}"
# shellcheck disable=SC2034
code="${__ush_binding_10_code}"
__ush_at 11 'allow_fail rm "-f" "stale.lock"'
command rm '-f' 'stale.lock' || :
__ush_at 12 'cargo "build" | stderr_to "build.err" | to_file "build.out"'
{ __ush_pipe=$(
  exec 7>&1 >&8 8>&-
  set +e
  { (command cargo 'build') 7>&- >'build.out' 2>'build.err'; printf ' 1:%s' "$?" >&7; }
); } 8>&1
__ush_pipe_check "$__ush_pipe" 'pipeline stage 1 failed: cargo "build" | stderr_to "build.err" | to_file "build.out"' || exit "$?"
__ush_at 13 'printf "warning\n" | to_stderr'
{ __ush_pipe=$(
  exec 7>&1 >&8 8>&-
  set +e
  { (command printf 'warning'"${__ush_lf}"'') 7>&- >&2; printf ' 1:%s' "$?" >&7; }
); } 8>&1
__ush_pipe_check "$__ush_pipe" 'pipeline stage 1 failed: printf "warning\n" | to_stderr' || exit "$?"
