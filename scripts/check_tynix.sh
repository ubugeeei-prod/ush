#!/bin/sh
# Generated from 'check_tynix.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: check_tynix.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'check_tynix.ush' --checked-in -o 'check_tynix.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'check_tynix.ush'
__ush_origin='check_tynix.ush'

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
  [ -z "$__ush_task_dir" ] || rm -rf "$__ush_task_dir"
  exit "$__ush_exit"
}
trap '__ush_finish "$?"' 0
trap 'exit 129' HUP
trap 'exit 130' INT
trap 'exit 143' TERM

case "${1:-}" in
  -h|--help) printf '%s' 'check_tynix.ush

Usage:
  check_tynix.ush [--help]
  check_tynix.ush [--man [ITEM]]
  check_tynix.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
check_tynix.ush

SYNOPSIS
check_tynix.ush [--help]
check_tynix.ush [--man [ITEM]]
check_tynix.ush [--complete [PREFIX]]
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

__ush_at 6 'let compiler = get_or "TYNIX" "tynix"'
__ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get_or 'TYNIX' 'tynix')" || exit "$?"
__ush_binding_6_compiler="${__ush_value_0}"
# shellcheck disable=SC2034
compiler="${__ush_binding_6_compiler}"
__ush_at 7 'let work = temp_dir'
__ush_task_init
__ush_at 7 'let work = temp_dir'
__ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
__ush_binding_7_work="${__ush_value_1}"
# shellcheck disable=SC2034
work="${__ush_binding_7_work}"
__ush_at 8 'exec compiler ["check", "flake.tynix"]'
__ush_match_2__len='2'
# shellcheck disable=SC2034
__ush_match_2__0='check'
# shellcheck disable=SC2034
__ush_match_2__1='flake.tynix'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec "${__ush_binding_6_compiler}" "$(ush_fn_std__pack __ush_match_2)" )
__ush_at 9 'exec compiler ["compile", "flake.tynix", "-o", work + "/flake.nix"]'
__ush_match_4__len='4'
# shellcheck disable=SC2034
__ush_match_4__0='compile'
# shellcheck disable=SC2034
__ush_match_4__1='flake.tynix'
# shellcheck disable=SC2034
__ush_match_4__2='-o'
# shellcheck disable=SC2034
__ush_match_4__3="${__ush_binding_7_work}"'/flake.nix'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec "${__ush_binding_6_compiler}" "$(ush_fn_std__pack __ush_match_4)" )
__ush_at 10 'if status "cmp" ["-s", "flake.nix", work + "/flake.nix"] != 0 {'
if {
__ush_match_6__len='3'
# shellcheck disable=SC2034
__ush_match_6__0='-s'
# shellcheck disable=SC2034
__ush_match_6__1='flake.nix'
# shellcheck disable=SC2034
__ush_match_6__2="${__ush_binding_7_work}"'/flake.nix'
__ush_value_8="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__status 'cmp' "$(ush_fn_std__pack __ush_match_6)")" || exit "$?"
[ "$(if [ "${__ush_value_8}" -ne 0 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 11 'fail 1 "flake.nix is stale; run tynix compile flake.tynix -o flake.nix"'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'flake.nix is stale; run tynix compile flake.tynix -o flake.nix'
  exit "$__ush_status"
fi
