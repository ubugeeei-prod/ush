#!/bin/sh
# Generated from 'check_generated.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: check_generated.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'check_generated.ush' --checked-in -o 'check_generated.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'check_generated.ush'
__ush_origin='check_generated.ush'

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
  -h|--help) printf '%s' 'check_generated.ush

Usage:
  check_generated.ush [--help]
  check_generated.ush [--man [ITEM]]
  check_generated.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
check_generated.ush

SYNOPSIS
check_generated.ush [--help]
check_generated.ush [--man [ITEM]]
check_generated.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

__ush_std_env_validate() {
  case "$1" in
    ''|[0-9]*|*[!A-Za-z0-9_]*)
      printf '%s\n' "ush std::env: invalid env name: $1" >&2
      return 1
      ;;
  esac
}

ush_fn_std__env__get_or() {
  __ush_std_env_validate "$1" || return "$?"
  __ush_default=$2
  eval "printf '%s' \"\${$1-\${__ush_default}}\""
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

ush_fn_std__path__basename() {
  basename "$(__ush_std_path_resolve "$1")"
}

ush_fn_std__string__trim_suffix() {
  VALUE="$1" SUFFIX="$2" awk 'BEGIN { value = ENVIRON["VALUE"]; suffix = ENVIRON["SUFFIX"]; if (length(suffix) > 0 && length(value) >= length(suffix) && substr(value, length(value) - length(suffix) + 1) == suffix) { value = substr(value, 1, length(value) - length(suffix)) } printf "%s", value }'
}

ush_fn_std__pack() {
  __ush_count=0
  eval "__ush_count=\${${1}__len}"
  __ush_i=0
  while [ "$__ush_i" -lt "$__ush_count" ]; do
    eval "__ush_element=\${${1}__${__ush_i}}"
    printf x
    printf '%s' "$__ush_element" | od -A n -v -t x1 | tr -d ' \n'
    printf ' '
    __ush_i=$((__ush_i + 1))
  done
}

ush_fn_std__unpack() {
  __ush_i=0
  for __ush_hex in $2; do
    __ush_element=$(LC_ALL=C awk -v s="${__ush_hex#x}" 'BEGIN {
      digits = "0123456789abcdef"
      for (i = 1; i <= length(s); i += 2)
        printf "%c", (index(digits, substr(s,i,1))-1)*16 + index(digits,substr(s,i+1,1))-1
      printf "."
    }')
    __ush_element=${__ush_element%.}
    eval "${1}__${__ush_i}=\$__ush_element"
    __ush_i=$((__ush_i + 1))
  done
  eval "${1}__len=\$__ush_i"
}

__ush_std_encode_args() {
  for __ush_arg do
    printf x
    printf '%s' "$__ush_arg" | od -A n -v -t x1 | tr -d ' \n'
    printf ' '
  done
}

ush_fn_std__fs__temp_dir() {
  mktemp -d "$__ush_task_dir/tmp.XXXXXXXXXX"
}

ush_fn_std__process__exec() {
  __ush_program=$1
  __ush_args__len=0
  __ush_arg=''
  ush_fn_std__unpack __ush_args "$2" || return "$?"
  set --
  __ush_index=0
  while [ "$__ush_index" -lt "$__ush_args__len" ]; do
    eval "__ush_arg=\${__ush_args__${__ush_index}}"
    set -- "$@" "$__ush_arg"
    __ush_index=$((__ush_index + 1))
  done
  "$__ush_program" "$@"
}

ush_fn_std__process__status() {
  if (ush_fn_std__process__exec "$@") >/dev/null 2>&1; then printf 0; else printf '%s' "$?"; fi
}

ush_fn_std__string__lines() {
  [ -n "$1" ] || return 0
  __ush_newline='
'
  __ush_carriage=$(printf '\r')
  __ush_rest=${1%"$__ush_newline"}
  while :; do
    case $__ush_rest in
      *"$__ush_newline"*)
        __ush_line=${__ush_rest%%"$__ush_newline"*}
        __ush_std_encode_args "${__ush_line%"$__ush_carriage"}"
        __ush_rest=${__ush_rest#*"$__ush_newline"} ;;
      *) __ush_std_encode_args "${__ush_rest%"$__ush_carriage"}"; break ;;
    esac
  done
}

__ush_at 9 'let mut compiler = get_or "USH_COMPILER" ""'
__ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get_or 'USH_COMPILER' '')" || exit "$?"
__ush_binding_9_compiler="${__ush_value_0}"
# shellcheck disable=SC2034
compiler="${__ush_binding_9_compiler}"
__ush_at 10 'if compiler == "" {'
if {
[ "$(if [ "${__ush_binding_9_compiler}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 11 'cargo "build" "--locked" "-p" "ush"'
  command cargo 'build' '--locked' '-p' 'ush'
  __ush_at 12 'compiler = "target/debug/ush"'
  __ush_binding_9_compiler='target/debug/ush'
  # shellcheck disable=SC2034
  compiler="${__ush_binding_9_compiler}"
fi
__ush_at 14 'let work = temp_dir'
__ush_task_init
__ush_at 14 'let work = temp_dir'
__ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
__ush_binding_14_work="${__ush_value_1}"
# shellcheck disable=SC2034
work="${__ush_binding_14_work}"
__ush_at 15 'for source in lines (find "install.ush" "scripts" "-name" "*.ush" "-type" "f") {'
__ush_value_3="$(__ush_return_path='' __ush_capture_return='1' command find 'install.ush' 'scripts' '-name' '*.ush' '-type' 'f')" || exit "$?"
__ush_packed_4="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__lines "${__ush_value_3}")" || exit "$?"
__ush_match_2__len=0
ush_fn_std__unpack __ush_match_2 "${__ush_packed_4}"
__ush_for_list_index_5=0
while [ "${__ush_for_list_index_5}" -lt "${__ush_match_2__len}" ]; do
  __ush_for_value_6=''
  eval "__ush_for_value_6=\"\${__ush_match_2__${__ush_for_list_index_5}}\""
  source="${__ush_for_value_6}"
  __ush_for_list_index_5=$((__ush_for_list_index_5 + 1))
  __ush_at 16 'let name = basename source'
  __ush_value_7="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__basename "${source}")" || exit "$?"
  __ush_binding_16_name="${__ush_value_7}"
  # shellcheck disable=SC2034
  name="${__ush_binding_16_name}"
  __ush_at 17 'let artifact = trim_suffix source ".ush" + ".sh"'
  __ush_value_8="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_suffix "${source}" '.ush')" || exit "$?"
  __ush_binding_17_artifact="${__ush_value_8}"'.sh'
  # shellcheck disable=SC2034
  artifact="${__ush_binding_17_artifact}"
  __ush_at 18 'let generated = work + "/" + trim_suffix name ".ush" + ".sh"'
  __ush_value_9="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_suffix "${__ush_binding_16_name}" '.ush')" || exit "$?"
  __ush_binding_18_generated="${__ush_binding_14_work}"'/'"${__ush_value_9}"'.sh'
  # shellcheck disable=SC2034
  generated="${__ush_binding_18_generated}"
  __ush_at 19 'cp source (work + "/" + name)'
  command cp "${source}" "${__ush_binding_14_work}"'/'"${__ush_binding_16_name}"
  __ush_at 20 'exec compiler ["compile", work + "/" + name, "--checked-in", "-o", generated]'
  __ush_match_10__len='5'
  # shellcheck disable=SC2034
  __ush_match_10__0='compile'
  # shellcheck disable=SC2034
  __ush_match_10__1="${__ush_binding_14_work}"'/'"${__ush_binding_16_name}"
  # shellcheck disable=SC2034
  __ush_match_10__2='--checked-in'
  # shellcheck disable=SC2034
  __ush_match_10__3='-o'
  # shellcheck disable=SC2034
  __ush_match_10__4="${__ush_binding_18_generated}"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec "${__ush_binding_9_compiler}" "$(ush_fn_std__pack __ush_match_10)" )
  __ush_at 21 'if status "cmp" ["-s", artifact, generated] != 0 {'
  if {
  __ush_match_12__len='3'
  # shellcheck disable=SC2034
  __ush_match_12__0='-s'
  # shellcheck disable=SC2034
  __ush_match_12__1="${__ush_binding_17_artifact}"
  # shellcheck disable=SC2034
  __ush_match_12__2="${__ush_binding_18_generated}"
  __ush_value_14="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__status 'cmp' "$(ush_fn_std__pack __ush_match_12)")" || exit "$?"
  [ "$(if [ "${__ush_value_14}" -ne 0 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 22 'fail 1 ("stale artifact: " + artifact + "; regenerate from " + source)'
    __ush_status=1
    if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
    __ush_report "$__ush_status" 'stale artifact: '"${__ush_binding_17_artifact}"'; regenerate from '"${source}"
    exit "$__ush_status"
  fi
done
