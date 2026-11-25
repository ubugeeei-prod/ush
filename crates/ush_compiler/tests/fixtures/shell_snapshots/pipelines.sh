#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/pipelines.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/pipelines.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_lf='
'
__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/pipelines.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/pipelines.ush'

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
  -h|--help) printf '%s' 'pipelines.ush

Usage:
  pipelines.ush [--help]
  pipelines.ush [--man [ITEM]]
  pipelines.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
pipelines.ush

SYNOPSIS
pipelines.ush [--help]
pipelines.ush [--man [ITEM]]
pipelines.ush [--complete [PREFIX]]
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
# reader's own status decides.
__ush_pipe_check() {
  __ush_pipe_statuses=" $1 "
  shift
  __ush_pipe_index=0
  for __ush_pipe_message do
    __ush_pipe_index=$((__ush_pipe_index + 1))
    case $__ush_pipe_statuses in
      *" $__ush_pipe_index:0 "*) continue ;;
      *" $__ush_pipe_index:"*) ;;
      *) __ush_report 1 "$__ush_pipe_message (no exit status)"; return 1 ;;
    esac
    __ush_pipe_code=${__ush_pipe_statuses#*" $__ush_pipe_index:"}
    __ush_pipe_code=${__ush_pipe_code%% *}
    __ush_pipe_earlier=${__ush_pipe_statuses%%" $__ush_pipe_index:"*}
    case $__ush_pipe_earlier in
      *" $((__ush_pipe_index + 1)):"*) continue ;;
    esac
    __ush_report "$__ush_pipe_code" "$__ush_pipe_message"
    return "$__ush_pipe_code"
  done
}

__ush_std_path_join_parts() {
  __ush_left=$1
  __ush_right=$2
  if [ -z "$__ush_left" ]; then
    printf '%s' "$__ush_right"
    return 0
  fi
  if [ -z "$__ush_right" ]; then
    printf '%s' "$__ush_left"
    return 0
  fi
  while [ "$__ush_left" != "/" ] && [ "${__ush_left%/}" != "$__ush_left" ]; do
    __ush_left=${__ush_left%/}
  done
  while [ "${__ush_right#/}" != "$__ush_right" ]; do
    __ush_right=${__ush_right#/}
  done
  case "$__ush_left" in
    /) printf '/%s' "$__ush_right" ;;
    *) printf '%s/%s' "$__ush_left" "$__ush_right" ;;
  esac
}

__ush_std_path_resolve() {
  __ush_value=$1
  case "$__ush_value" in
    __ush_pathref__:cwd:*)
      __ush_std_path_join_parts "$(pwd)" "${__ush_value#__ush_pathref__:cwd:}"
      ;;
    __ush_pathref__:source:*)
      __ush_base=${__ush_source_dir:-}
      if [ -z "$__ush_base" ]; then
        __ush_base=$(pwd)
      fi
      __ush_std_path_join_parts "$__ush_base" "${__ush_value#__ush_pathref__:source:}"
      ;;
    *) printf '%s' "$__ush_value" ;;
  esac
}

ush_fn_std__path__resolve() {
  __ush_std_path_resolve "$1"
}

ush_fn_std__string__to_upper_ascii() {
  printf '%s' "$1" | LC_ALL=C tr '[:lower:]' '[:upper:]'
}

__ush_std_fs_path() {
  __ush_path=$(ush_fn_std__path__resolve "$1" && printf .) || return "$?"
  __ush_path=${__ush_path%.}
  [ -n "$__ush_path" ] || { printf '%s\n' 'std::fs: path must not be empty' >&2; return 1; }
  case $__ush_path in .|..|/*|./*|../*) ;; *) __ush_path=./$__ush_path ;; esac
}

ush_fn_std__io__read_all() {
  cat
}

# raises: unknown
ush_fn_shout() {
  __ush_at 5 'print (std::string::to_upper_ascii (std::io::read_all))'
  __ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__io__read_all)" || return "$?"
  __ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__to_upper_ascii "${__ush_value_0}")" || return "$?"
  printf '%s\n' "${__ush_value_1}"
}

__ush_at 8 'git "log" "--oneline" | head "-n1"'
{ __ush_pipe=$(
  exec 7>&1 >&8 8>&-
  set +e
  { (command git 'log' '--oneline') 7>&-; printf ' 1:%s' "$?" >&7; } |
  { (command head '-n1') 7>&-; printf ' 2:%s' "$?" >&7; }
); } 8>&1
__ush_pipe_check "$__ush_pipe" 'pipeline stage 1 failed: git "log" "--oneline"' 'pipeline stage 2 failed: head "-n1"' || exit "$?"
__ush_at 9 'let branch = git "branch" "--show-current" | tr "a-z" "A-Z"'
__ush_value_2="$(
  { __ush_pipe=$(
    exec 7>&1 >&8 8>&-
    set +e
    { (command git 'branch' '--show-current') 7>&-; printf ' 1:%s' "$?" >&7; } |
    { (command tr 'a-z' 'A-Z') 7>&-; printf ' 2:%s' "$?" >&7; }
  ); } 8>&1
  __ush_pipe_check "$__ush_pipe" 'pipeline stage 1 failed: git "branch" "--show-current"' 'pipeline stage 2 failed: tr "a-z" "A-Z"'
)" || { __ush_status=$?; __ush_line=''; exit "$__ush_status"; }
__ush_binding_9_branch="${__ush_value_2}"
# shellcheck disable=SC2034
branch="${__ush_binding_9_branch}"
__ush_at 10 'cargo "build" |& tee "build.log"'
{ __ush_pipe=$(
  exec 7>&1 >&8 8>&-
  set +e
  { (command cargo 'build') 7>&- 2>&1; printf ' 1:%s' "$?" >&7; } |
  { (command tee 'build.log') 7>&-; printf ' 2:%s' "$?" >&7; }
); } 8>&1
__ush_pipe_check "$__ush_pipe" 'pipeline stage 1 failed: cargo "build"' 'pipeline stage 2 failed: tee "build.log"' || exit "$?"
__ush_at 11 '"hello\n" | shout'
{ __ush_pipe=$(
  exec 7>&1 >&8 8>&-
  set +e
  { (printf '%s' 'hello'"${__ush_lf}"'') 7>&-; printf ' 1:%s' "$?" >&7; } |
  { ( __ush_capture_return='0' __ush_return_path=''; ush_fn_shout ) 7>&-; printf ' 2:%s' "$?" >&7; }
); } 8>&1
__ush_pipe_check "$__ush_pipe" 'pipeline stage 1 failed: "hello\n"' 'pipeline stage 2 failed: shout' || exit "$?"
__ush_at 12 'from_file "notes.txt" | sort | to_file "sorted.txt"'
{ __ush_pipe=$(
  exec 7>&1 >&8 8>&-
  set +e
  { (command sort) 7>&- <'notes.txt' >'sorted.txt'; printf ' 1:%s' "$?" >&7; }
); } 8>&1
__ush_pipe_check "$__ush_pipe" 'pipeline stage 1 failed: from_file "notes.txt" | sort | to_file "sorted.txt"' || exit "$?"
__ush_at 13 'printf "done\n" |'
__ush_std_fs_path "${__ush_binding_9_branch}"'.log' || { __ush_status=$?; __ush_report "$__ush_status"; exit "$__ush_status"; }
__ush_path_3=$__ush_path
{ __ush_pipe=$(
  exec 7>&1 >&8 8>&-
  set +e
  { (command printf 'done'"${__ush_lf}"'') 7>&- >>"$__ush_path_3"; printf ' 1:%s' "$?" >&7; }
); } 8>&1
__ush_pipe_check "$__ush_pipe" 'pipeline stage 1 failed: printf "done\n" | append_to (branch + ".log")' || exit "$?"
