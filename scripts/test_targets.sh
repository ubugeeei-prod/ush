#!/bin/sh
# Generated from 'test_targets.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: test_targets.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'test_targets.ush' --checked-in -o 'test_targets.sh'
set -eu

__ush_lf='
'
__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'test_targets.ush'
__ush_origin='test_targets.ush'

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
  -h|--help) printf '%s' 'test_targets.ush

Usage:
  test_targets.ush [--help]
  test_targets.ush [--man [ITEM]]
  test_targets.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
test_targets.ush

SYNOPSIS
test_targets.ush [--help]
test_targets.ush [--man [ITEM]]
test_targets.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

__ush_std_env_validate() {
  case "$1" in
    ''|[0-9]*|*[!A-Za-z0-9_]*)
      printf '%s\n' "ush std::env: invalid env name: $1" >&2
      return 1
      ;;
  esac
}

ush_fn_std__env__get() {
  __ush_std_env_validate "$1" || return "$?"
  eval "printf '%s' \"\${$1-}\""
}

ush_fn_std__env__get_or() {
  __ush_std_env_validate "$1" || return "$?"
  __ush_default=$2
  eval "printf '%s' \"\${$1-\${__ush_default}}\""
}

ush_fn_std__env__set() {
  __ush_std_env_validate "$1" || return "$?"
  export "$1=$2"
}

ush_fn_std__fs__read_text() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  cat "$__ush_path"
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

ush_fn_std__command__exists() {
  if command -v "$1" >/dev/null 2>&1; then printf '%s' 'true'; else printf '%s' 'false'; fi
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

ush_fn_std__test__assert() {
  if [ "$1" = true ]; then return 0; fi
  printf '%s\n' 'assert failed: expected true' >&2
  return 1
}

ush_fn_std__test__assert_eq() {
  if [ "$1" = "$2" ]; then return 0; fi
  printf 'assert_eq failed\n  actual: %s\n  expected: %s\n' "$1" "$2" >&2
  return 1
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

ush_fn_std__process__output() {
  ush_fn_std__process__exec "$@"
}

ush_fn_std__process__redirect() {
  if (ush_fn_std__process__exec "$1" "$2") > "$3" 2> "$4"; then printf 0; else printf '%s' "$?"; fi
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

ush_fn_std__string__contains() {
  case $1 in *"$2"*) printf true ;; *) printf false ;; esac
}

__ush_std_fs_path() {
  __ush_path=$(ush_fn_std__path__resolve "$1" && printf .) || return "$?"
  __ush_path=${__ush_path%.}
  [ -n "$__ush_path" ] || { printf '%s\n' 'std::fs: path must not be empty' >&2; return 1; }
  case $__ush_path in .|..|/*|./*|../*) ;; *) __ush_path=./$__ush_path ;; esac
}

ush_fn_std__path__is_absolute() {
  case $1 in /*) printf true ;; *) printf false ;; esac
}

ush_fn_std__process__chdir() {
  __ush_std_fs_path "$1" || return "$?"
  CDPATH='' cd "$__ush_path" || return "$?"
}

# raises: unknown
ush_fn_run_fixture() {
  __ush_fn_run_fixture_arg_0="${1}"
  # shellcheck disable=SC2034
  compiler="${__ush_fn_run_fixture_arg_0}"
  __ush_fn_run_fixture_arg_1="${2}"
  # shellcheck disable=SC2034
  target="${__ush_fn_run_fixture_arg_1}"
  __ush_fn_run_fixture_arg_2="${3}"
  # shellcheck disable=SC2034
  source="${__ush_fn_run_fixture_arg_2}"
  __ush_fn_run_fixture_arg_3="${4}"
  # shellcheck disable=SC2034
  directory="${__ush_fn_run_fixture_arg_3}"
  __ush_at 11 'std::process::chdir directory'
  ush_fn_std__process__chdir "${__ush_fn_run_fixture_arg_3}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 12 'output compiler ["--shell-target", target, "test", source]'
  __ush_match_0__len='4'
  # shellcheck disable=SC2034
  __ush_match_0__0='--shell-target'
  # shellcheck disable=SC2034
  __ush_match_0__1="${__ush_fn_run_fixture_arg_1}"
  # shellcheck disable=SC2034
  __ush_match_0__2='test'
  # shellcheck disable=SC2034
  __ush_match_0__3="${__ush_fn_run_fixture_arg_2}"
  __ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${__ush_fn_run_fixture_arg_0}" "$(ush_fn_std__pack __ush_match_0)")" || return "$?"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_value_2}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_value_2}" >&4
  fi
  return 0
}

__ush_at 15 'let root = pwd'
__ush_value_3="$(__ush_return_path='' __ush_capture_return='1' command pwd)" || exit "$?"
root="${__ush_value_3}"
__ush_at 16 'let compiler = get_or "USH_BIN" "target/debug/ush"'
__ush_value_4="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get_or 'USH_BIN' 'target/debug/ush')" || exit "$?"
compiler="${__ush_value_4}"
__ush_at 17 'if std::path::is_absolute compiler == false {'
if {
__ush_value_5="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__is_absolute "${compiler}")" || exit "$?"
[ "$(if [ "${__ush_value_5}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 18 'let compiler = root + "/" + compiler'
  compiler="${root}"'/'"${compiler}"
fi
__ush_at 20 'let work = temp_dir'
__ush_task_init
__ush_at 20 'let work = temp_dir'
__ush_value_6="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
work="${__ush_value_6}"
__ush_at 21 'let targets = ["sh", "bash", "zsh"]'
targets__len='3'
# shellcheck disable=SC2034
targets__0='sh'
# shellcheck disable=SC2034
targets__1='bash'
# shellcheck disable=SC2034
targets__2='zsh'
__ush_at 22 'if exists "busybox" {'
if {
__ush_value_7="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__exists 'busybox')" || exit "$?"
[ "${__ush_value_7}" = 'true' ]
}; then
  __ush_at 23 'let busybox = output "command" ["-v", "busybox"]'
  __ush_match_8__len='2'
  # shellcheck disable=SC2034
  __ush_match_8__0='-v'
  # shellcheck disable=SC2034
  __ush_match_8__1='busybox'
  __ush_value_10="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output 'command' "$(ush_fn_std__pack __ush_match_8)")" || exit "$?"
  busybox="${__ush_value_10}"
  __ush_at 24 'ln "-s" busybox (work + "/ash")'
  command ln '-s' "${busybox}" "${work}"'/ash'
  __ush_at 25 'set "PATH" (work + ":" + get "PATH")'
  __ush_value_11="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'PATH')" || exit "$?"
  ush_fn_std__env__set 'PATH' "${work}"':'"${__ush_value_11}"
  __ush_at 26 'let targets = ["sh", "bash", "zsh", "ash"]'
  targets__len='4'
  # shellcheck disable=SC2034
  targets__0='sh'
  # shellcheck disable=SC2034
  targets__1='bash'
  # shellcheck disable=SC2034
  targets__2='zsh'
  # shellcheck disable=SC2034
  targets__3='ash'
else
  __ush_at 28 'else if uname "-s" == "Linux" {'
  if {
  __ush_value_12="$(__ush_return_path='' __ush_capture_return='1' command uname '-s')" || exit "$?"
  [ "$(if [ "${__ush_value_12}" = 'Linux' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 29 'fail 1 "BusyBox is required to check the ash target on Linux"'
    __ush_status=1
    if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
    __ush_report "$__ush_status" 'BusyBox is required to check the ash target on Linux'
    exit "$__ush_status"
  fi
fi
__ush_at 31 'for target in targets {'
__ush_match_13__len="${targets__len}"
__ush_copy_index_14=0
while [ "$__ush_copy_index_14" -lt "${targets__len}" ]; do
  eval "__ush_copy_value_15=\"\${targets__$__ush_copy_index_14}\""
  eval "__ush_match_13__${__ush_copy_index_14}=\$__ush_copy_value_15"
  __ush_copy_index_14=$((__ush_copy_index_14 + 1))
done
__ush_for_list_index_16=0
while [ "${__ush_for_list_index_16}" -lt "${__ush_match_13__len}" ]; do
  __ush_for_value_17=''
  eval "__ush_for_value_17=\"\${__ush_match_13__${__ush_for_list_index_16}}\""
  target="${__ush_for_value_17}"
  __ush_for_list_index_16=$((__ush_for_list_index_16 + 1))
  __ush_at 32 'sh "scripts/test_task_await.sh" compiler target'
  command sh 'scripts/test_task_await.sh' "${compiler}" "${target}"
  __ush_at 33 'for name in ["strings", "filesystem", "paths"] {'
  __ush_match_18__len='3'
  # shellcheck disable=SC2034
  __ush_match_18__0='strings'
  # shellcheck disable=SC2034
  __ush_match_18__1='filesystem'
  # shellcheck disable=SC2034
  __ush_match_18__2='paths'
  __ush_for_list_index_19=0
  while [ "${__ush_for_list_index_19}" -lt "${__ush_match_18__len}" ]; do
    __ush_for_value_20=''
    eval "__ush_for_value_20=\"\${__ush_match_18__${__ush_for_list_index_19}}\""
    name="${__ush_for_value_20}"
    __ush_for_list_index_19=$((__ush_for_list_index_19 + 1))
    __ush_at 34 'let source = root + "/crates/ush_compiler/tests/fixtures/portable/" + name + ".ush"'
    source="${root}"'/crates/ush_compiler/tests/fixtures/portable/'"${name}"'.ush'
    __ush_at 35 'assert (contains (run_fixture compiler target source work) "1 passed; 0 failed")'
    __ush_value_21="$(__ush_return_path='' __ush_capture_return='1' ush_fn_run_fixture "${compiler}" "${target}" "${source}" "${work}" 4>&1 >&3)" || exit "$?"
    __ush_value_22="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_21}" '1 passed; 0 failed')" || exit "$?"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_22}" )
  done
  __ush_at 37 'for source in lines (find "examples" "-name" "*.ush" "-type" "f") {'
  __ush_value_24="$(__ush_return_path='' __ush_capture_return='1' command find 'examples' '-name' '*.ush' '-type' 'f')" || exit "$?"
  __ush_packed_25="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__lines "${__ush_value_24}")" || exit "$?"
  __ush_match_23__len=0
  ush_fn_std__unpack __ush_match_23 "${__ush_packed_25}"
  __ush_for_list_index_26=0
  while [ "${__ush_for_list_index_26}" -lt "${__ush_match_23__len}" ]; do
    __ush_for_value_27=''
    eval "__ush_for_value_27=\"\${__ush_match_23__${__ush_for_list_index_26}}\""
    source="${__ush_for_value_27}"
    __ush_for_list_index_26=$((__ush_for_list_index_26 + 1))
    __ush_at 38 'exec compiler ["compile", source, "--target", target, "-o", work + "/script"]'
    __ush_match_28__len='6'
    # shellcheck disable=SC2034
    __ush_match_28__0='compile'
    # shellcheck disable=SC2034
    __ush_match_28__1="${source}"
    # shellcheck disable=SC2034
    __ush_match_28__2='--target'
    # shellcheck disable=SC2034
    __ush_match_28__3="${target}"
    # shellcheck disable=SC2034
    __ush_match_28__4='-o'
    # shellcheck disable=SC2034
    __ush_match_28__5="${work}"'/script'
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec "${compiler}" "$(ush_fn_std__pack __ush_match_28)" )
    __ush_at 39 'exec target ["-n", work + "/script"]'
    __ush_match_30__len='2'
    # shellcheck disable=SC2034
    __ush_match_30__0='-n'
    # shellcheck disable=SC2034
    __ush_match_30__1="${work}"'/script'
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec "${target}" "$(ush_fn_std__pack __ush_match_30)" )
  done
  __ush_at 41 'exec compiler ["compile", "examples/contracts.ush", "--target", target, "-o", work + "/script"]'
  __ush_match_32__len='6'
  # shellcheck disable=SC2034
  __ush_match_32__0='compile'
  # shellcheck disable=SC2034
  __ush_match_32__1='examples/contracts.ush'
  # shellcheck disable=SC2034
  __ush_match_32__2='--target'
  # shellcheck disable=SC2034
  __ush_match_32__3="${target}"
  # shellcheck disable=SC2034
  __ush_match_32__4='-o'
  # shellcheck disable=SC2034
  __ush_match_32__5="${work}"'/script'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec "${compiler}" "$(ush_fn_std__pack __ush_match_32)" )
  __ush_at 42 'chmod "+x" (work + "/script")'
  command chmod '+x' "${work}"'/script'
  __ush_at 43 'assert_eq (output target [work + "/script"]) "2\n20\nready\ndone"'
  __ush_match_34__len='1'
  # shellcheck disable=SC2034
  __ush_match_34__0="${work}"'/script'
  __ush_value_36="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${target}" "$(ush_fn_std__pack __ush_match_34)")" || exit "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_36}" '2'"${__ush_lf}"'20'"${__ush_lf}"'ready'"${__ush_lf}"'done' )
  __ush_at 44 'exec compiler ["compile", "examples/collections.ush", "--target", target, "-o", work + "/script"]'
  __ush_match_37__len='6'
  # shellcheck disable=SC2034
  __ush_match_37__0='compile'
  # shellcheck disable=SC2034
  __ush_match_37__1='examples/collections.ush'
  # shellcheck disable=SC2034
  __ush_match_37__2='--target'
  # shellcheck disable=SC2034
  __ush_match_37__3="${target}"
  # shellcheck disable=SC2034
  __ush_match_37__4='-o'
  # shellcheck disable=SC2034
  __ush_match_37__5="${work}"'/script'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec "${compiler}" "$(ush_fn_std__pack __ush_match_37)" )
  __ush_at 45 'assert_eq (output target [work + "/script"]) "2\n3\n4\n3"'
  __ush_match_39__len='1'
  # shellcheck disable=SC2034
  __ush_match_39__0="${work}"'/script'
  __ush_value_41="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${target}" "$(ush_fn_std__pack __ush_match_39)")" || exit "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_41}" '2'"${__ush_lf}"'3'"${__ush_lf}"'4'"${__ush_lf}"'3' )
  __ush_at 46 'exec compiler ["compile", "examples/extern.ush", "--target", target, "-o", work + "/script"]'
  __ush_match_42__len='6'
  # shellcheck disable=SC2034
  __ush_match_42__0='compile'
  # shellcheck disable=SC2034
  __ush_match_42__1='examples/extern.ush'
  # shellcheck disable=SC2034
  __ush_match_42__2='--target'
  # shellcheck disable=SC2034
  __ush_match_42__3="${target}"
  # shellcheck disable=SC2034
  __ush_match_42__4='-o'
  # shellcheck disable=SC2034
  __ush_match_42__5="${work}"'/script'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec "${compiler}" "$(ush_fn_std__pack __ush_match_42)" )
  __ush_at 47 'assert_eq (output target [work + "/script"]) "hello ush\n42"'
  __ush_match_44__len='1'
  # shellcheck disable=SC2034
  __ush_match_44__0="${work}"'/script'
  __ush_value_46="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${target}" "$(ush_fn_std__pack __ush_match_44)")" || exit "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_46}" 'hello ush'"${__ush_lf}"'42' )
  __ush_at 48 'exec compiler ["compile", "examples/pipelines.ush", "--target", target, "-o", work + "/script"]'
  __ush_match_47__len='6'
  # shellcheck disable=SC2034
  __ush_match_47__0='compile'
  # shellcheck disable=SC2034
  __ush_match_47__1='examples/pipelines.ush'
  # shellcheck disable=SC2034
  __ush_match_47__2='--target'
  # shellcheck disable=SC2034
  __ush_match_47__3="${target}"
  # shellcheck disable=SC2034
  __ush_match_47__4='-o'
  # shellcheck disable=SC2034
  __ush_match_47__5="${work}"'/script'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec "${compiler}" "$(ush_fn_std__pack __ush_match_47)" )
  __ush_at 49 'let piped = "first: apple\nSHOUTED\nTO STDERR\nhello from a child process\n1"'
  piped='first: apple'"${__ush_lf}"'SHOUTED'"${__ush_lf}"'TO STDERR'"${__ush_lf}"'hello from a child process'"${__ush_lf}"'1'
  __ush_at 50 'assert_eq (output target [work + "/script"]) piped'
  __ush_match_49__len='1'
  # shellcheck disable=SC2034
  __ush_match_49__0="${work}"'/script'
  __ush_value_51="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${target}" "$(ush_fn_std__pack __ush_match_49)")" || exit "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_51}" "${piped}" )
  __ush_at 51 'exec compiler ["--shell-target", target, "test", "examples/smoke_test.ush"]'
  __ush_match_52__len='4'
  # shellcheck disable=SC2034
  __ush_match_52__0='--shell-target'
  # shellcheck disable=SC2034
  __ush_match_52__1="${target}"
  # shellcheck disable=SC2034
  __ush_match_52__2='test'
  # shellcheck disable=SC2034
  __ush_match_52__3='examples/smoke_test.ush'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec "${compiler}" "$(ush_fn_std__pack __ush_match_52)" )
  __ush_at 52 'exec compiler ["compile", "examples/error_streams.ush", "--target", target, "-o", work + "/script"]'
  __ush_match_54__len='6'
  # shellcheck disable=SC2034
  __ush_match_54__0='compile'
  # shellcheck disable=SC2034
  __ush_match_54__1='examples/error_streams.ush'
  # shellcheck disable=SC2034
  __ush_match_54__2='--target'
  # shellcheck disable=SC2034
  __ush_match_54__3="${target}"
  # shellcheck disable=SC2034
  __ush_match_54__4='-o'
  # shellcheck disable=SC2034
  __ush_match_54__5="${work}"'/script'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec "${compiler}" "$(ush_fn_std__pack __ush_match_54)" )
  __ush_at 53 'assert_eq (redirect target [work + "/script"] (work + "/out") (work + "/err")) 1'
  __ush_match_56__len='1'
  # shellcheck disable=SC2034
  __ush_match_56__0="${work}"'/script'
  __ush_value_58="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect "${target}" "$(ush_fn_std__pack __ush_match_56)" "${work}"'/out' "${work}"'/err')" || exit "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_58}" 1 )
  __ush_at 54 'assert (contains (read_text (work + "/err")) "error_streams.ush:6: Problem::MissingConfig")'
  __ush_value_59="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${work}"'/err')" || exit "$?"
  __ush_value_60="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_59}" 'error_streams.ush:6: Problem::MissingConfig')" || exit "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_60}" )
done
__ush_at 56 'exec compiler ["compile", "examples/contracts.ush", "-o", work + "/script"]'
__ush_match_61__len='4'
# shellcheck disable=SC2034
__ush_match_61__0='compile'
# shellcheck disable=SC2034
__ush_match_61__1='examples/contracts.ush'
# shellcheck disable=SC2034
__ush_match_61__2='-o'
# shellcheck disable=SC2034
__ush_match_61__3="${work}"'/script'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec "${compiler}" "$(ush_fn_std__pack __ush_match_61)" )
__ush_at 57 'if exists "dash" {'
if {
__ush_value_63="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__exists 'dash')" || exit "$?"
[ "${__ush_value_63}" = 'true' ]
}; then
  __ush_at 58 'exec "dash" [work + "/script"]'
  __ush_match_64__len='1'
  # shellcheck disable=SC2034
  __ush_match_64__0="${work}"'/script'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec 'dash' "$(ush_fn_std__pack __ush_match_64)" )
fi
__ush_at 60 'exec "shellcheck" ["-s", "sh", "--severity=error", work + "/script"]'
__ush_match_66__len='4'
# shellcheck disable=SC2034
__ush_match_66__0='-s'
# shellcheck disable=SC2034
__ush_match_66__1='sh'
# shellcheck disable=SC2034
__ush_match_66__2='--severity=error'
# shellcheck disable=SC2034
__ush_match_66__3="${work}"'/script'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec 'shellcheck' "$(ush_fn_std__pack __ush_match_66)" )
