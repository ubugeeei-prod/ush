#!/bin/sh
# Generated from 'check_rustyline_upstream.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: check_rustyline_upstream.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'check_rustyline_upstream.ush' --checked-in -o 'check_rustyline_upstream.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'check_rustyline_upstream.ush'
__ush_origin='check_rustyline_upstream.ush'

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
  -h|--help) printf '%s' 'check_rustyline_upstream.ush

Usage:
  check_rustyline_upstream.ush [--help]
  check_rustyline_upstream.ush [--man [ITEM]]
  check_rustyline_upstream.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
check_rustyline_upstream.ush

SYNOPSIS
check_rustyline_upstream.ush [--help]
check_rustyline_upstream.ush [--man [ITEM]]
check_rustyline_upstream.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

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

ush_fn_std__string__starts_with() {
  VALUE="$1" PREFIX="$2" awk 'BEGIN { value = ENVIRON["VALUE"]; prefix = ENVIRON["PREFIX"]; if (substr(value, 1, length(prefix)) == prefix) { printf "true" } else { printf "false" } }'
}

ush_fn_std__string__trim_prefix() {
  VALUE="$1" PREFIX="$2" awk 'BEGIN { value = ENVIRON["VALUE"]; prefix = ENVIRON["PREFIX"]; if (substr(value, 1, length(prefix)) == prefix) { value = substr(value, length(prefix) + 1) } printf "%s", value }'
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

ush_fn_std__process__output() {
  ush_fn_std__process__exec "$@"
}

ush_fn_std__string__split() {
  [ -n "$2" ] || { printf '%s\n' 'std::string::split: separator must not be empty' >&2; return 1; }
  __ush_rest=$1
  __ush_separator=$2
  while :; do
    case $__ush_rest in
      *"$__ush_separator"*)
        __ush_std_encode_args "${__ush_rest%%"$__ush_separator"*}"
        __ush_rest=${__ush_rest#*"$__ush_separator"} ;;
      *) __ush_std_encode_args "$__ush_rest"; break ;;
    esac
  done
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

ush_fn_std__string__trim() {
  __ush_trimmed=$1
  __ush_trimmed=${__ush_trimmed#"${__ush_trimmed%%[![:space:]]*}"}
  __ush_trimmed=${__ush_trimmed%"${__ush_trimmed##*[![:space:]]}"}
  printf '%s' "$__ush_trimmed"
}

ush_fn_value() {
  __ush_fn_value_arg_0="${1}"
  # shellcheck disable=SC2034
  file="${__ush_fn_value_arg_0}"
  __ush_fn_value_arg_1="${2}"
  # shellcheck disable=SC2034
  key="${__ush_fn_value_arg_1}"
  __ush_at 8 'for line in lines (read_text file) {'
  __ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_fn_value_arg_0}")" || return "$?"
  __ush_packed_2="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__lines "${__ush_value_1}")" || return "$?"
  __ush_match_0__len=0
  ush_fn_std__unpack __ush_match_0 "${__ush_packed_2}"
  __ush_for_list_index_3=0
  while [ "${__ush_for_list_index_3}" -lt "${__ush_match_0__len}" ]; do
    __ush_for_value_4=''
    eval "__ush_for_value_4=\"\${__ush_match_0__${__ush_for_list_index_3}}\""
    line="${__ush_for_value_4}"
    __ush_for_list_index_3=$((__ush_for_list_index_3 + 1))
    __ush_at 9 'if starts_with line key {'
    if {
    __ush_value_5="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__starts_with "${line}" "${__ush_fn_value_arg_1}")" || return "$?"
    [ "${__ush_value_5}" = 'true' ]
    }; then
      __ush_at 10 'return trim (trim_prefix line key)'
      __ush_value_6="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${line}" "${__ush_fn_value_arg_1}")" || return "$?"
      __ush_value_7="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim "${__ush_value_6}")" || return "$?"
      if [ -n "${__ush_return_path:-}" ]; then
        printf '%s' "${__ush_value_7}" > "$__ush_return_path"
      elif [ "${__ush_capture_return:-0}" = '1' ]; then
        printf '%s' "${__ush_value_7}" >&4
      fi
      return 0
    fi
  done
  __ush_at 13 '""'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' '' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' '' >&4
  fi
  return 0
}

# raises: unknown
ush_fn_first() {
  __ush_fn_first_arg_0="${1}"
  # shellcheck disable=SC2034
  text="${__ush_fn_first_arg_0}"
  __ush_fn_first_arg_1="${2}"
  # shellcheck disable=SC2034
  separator="${__ush_fn_first_arg_1}"
  __ush_at 16 'for part in split text separator {'
  __ush_packed_9="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__split "${__ush_fn_first_arg_0}" "${__ush_fn_first_arg_1}")" || return "$?"
  __ush_match_8__len=0
  ush_fn_std__unpack __ush_match_8 "${__ush_packed_9}"
  __ush_for_list_index_10=0
  while [ "${__ush_for_list_index_10}" -lt "${__ush_match_8__len}" ]; do
    __ush_for_value_11=''
    eval "__ush_for_value_11=\"\${__ush_match_8__${__ush_for_list_index_10}}\""
    part="${__ush_for_value_11}"
    __ush_for_list_index_10=$((__ush_for_list_index_10 + 1))
    __ush_at 17 'return part'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${part}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${part}" >&4
    fi
    return 0
  done
  __ush_at 19 '""'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' '' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' '' >&4
  fi
  return 0
}

__ush_at 21 'let pinned = trim_prefix (value "vendor/rustyline/UPSTREAM" "tag:") "v"'
__ush_value_12="$(__ush_return_path='' __ush_capture_return='1' ush_fn_value 'vendor/rustyline/UPSTREAM' 'tag:' 4>&1 >&3)" || exit "$?"
__ush_value_13="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${__ush_value_12}" 'v')" || exit "$?"
__ush_binding_21_pinned="${__ush_value_13}"
# shellcheck disable=SC2034
pinned="${__ush_binding_21_pinned}"
__ush_at 22 'let vendored = std::string::trim_suffix (trim_prefix (value "vendor/rustyline/Cargo.toml" "version =") "\"") "\""'
__ush_value_14="$(__ush_return_path='' __ush_capture_return='1' ush_fn_value 'vendor/rustyline/Cargo.toml' 'version =' 4>&1 >&3)" || exit "$?"
__ush_value_15="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${__ush_value_14}" '"')" || exit "$?"
__ush_value_16="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_suffix "${__ush_value_15}" '"')" || exit "$?"
__ush_binding_22_vendored="${__ush_value_16}"
# shellcheck disable=SC2034
vendored="${__ush_binding_22_vendored}"
__ush_at 23 'if pinned == "" || vendored != pinned {'
if {
{
[ "$(if [ "${__ush_binding_21_pinned}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
} || {
[ "$(if [ "${__ush_binding_22_vendored}" != "${__ush_binding_21_pinned}" ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}
}; then
  __ush_at 24 'fail 1 "check_rustyline_upstream: UPSTREAM tag and vendored Cargo.toml version must agree"'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'check_rustyline_upstream: UPSTREAM tag and vendored Cargo.toml version must agree'
  exit "$__ush_status"
fi
__ush_at 26 'let work = temp_dir'
__ush_task_init
__ush_at 26 'let work = temp_dir'
__ush_value_17="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
__ush_binding_26_work="${__ush_value_17}"
# shellcheck disable=SC2034
work="${__ush_binding_26_work}"
__ush_at 27 'let index = work + "/index.jsonl"'
__ush_binding_27_index="${__ush_binding_26_work}"'/index.jsonl'
# shellcheck disable=SC2034
index="${__ush_binding_27_index}"
__ush_at 28 'curl "-fsSL" "--retry" "3" "https://index.crates.io/ru/st/rustyline" "-o" index'
command curl '-fsSL' '--retry' '3' 'https://index.crates.io/ru/st/rustyline' '-o' "${__ush_binding_27_index}"
__ush_at 30 'let versions = output "jq" ["-sr", "[.[] | select(.yanked == false) | .vers | select(test(\"^[0-9]+[.][0-9]+[.][0-9]+$\")) | split(\".\") | map(tonumber)] | sort | .[] | map(tostring) | join(\".\")", index]'
__ush_match_18__len='3'
# shellcheck disable=SC2034
__ush_match_18__0='-sr'
# shellcheck disable=SC2034
__ush_match_18__1='[.[] | select(.yanked == false) | .vers | select(test("^[0-9]+[.][0-9]+[.][0-9]+$")) | split(".") | map(tonumber)] | sort | .[] | map(tostring) | join(".")'
# shellcheck disable=SC2034
__ush_match_18__2="${__ush_binding_27_index}"
__ush_value_20="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output 'jq' "$(ush_fn_std__pack __ush_match_18)")" || exit "$?"
__ush_binding_30_versions="${__ush_value_20}"
# shellcheck disable=SC2034
versions="${__ush_binding_30_versions}"
__ush_at 31 'let mut latest = ""'
__ush_binding_31_latest=''
# shellcheck disable=SC2034
latest="${__ush_binding_31_latest}"
__ush_at 32 'let mut patch = ""'
__ush_binding_32_patch=''
# shellcheck disable=SC2034
patch="${__ush_binding_32_patch}"
__ush_at 33 'let major = first pinned "."'
__ush_value_21="$(__ush_return_path='' __ush_capture_return='1' ush_fn_first "${__ush_binding_21_pinned}" '.' 4>&1 >&3)" || exit "$?"
__ush_binding_33_major="${__ush_value_21}"
# shellcheck disable=SC2034
major="${__ush_binding_33_major}"
__ush_at 34 'for version in lines versions {'
__ush_packed_23="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__lines "${__ush_binding_30_versions}")" || exit "$?"
__ush_match_22__len=0
ush_fn_std__unpack __ush_match_22 "${__ush_packed_23}"
__ush_for_list_index_24=0
while [ "${__ush_for_list_index_24}" -lt "${__ush_match_22__len}" ]; do
  __ush_for_value_25=''
  eval "__ush_for_value_25=\"\${__ush_match_22__${__ush_for_list_index_24}}\""
  version="${__ush_for_value_25}"
  __ush_for_list_index_24=$((__ush_for_list_index_24 + 1))
  __ush_at 35 'latest = version'
  __ush_binding_31_latest="${version}"
  # shellcheck disable=SC2034
  latest="${__ush_binding_31_latest}"
  __ush_at 36 'if first version "." == major {'
  if {
  __ush_value_26="$(__ush_return_path='' __ush_capture_return='1' ush_fn_first "${version}" '.' 4>&1 >&3)" || exit "$?"
  [ "$(if [ "${__ush_value_26}" = "${__ush_binding_33_major}" ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 37 'patch = version'
    __ush_binding_32_patch="${version}"
    # shellcheck disable=SC2034
    patch="${__ush_binding_32_patch}"
  fi
done
__ush_at 40 'if latest == "" || patch == "" {'
if {
{
[ "$(if [ "${__ush_binding_31_latest}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
} || {
[ "$(if [ "${__ush_binding_32_patch}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}
}; then
  __ush_at 41 'fail 1 "check_rustyline_upstream: registry returned no stable versions"'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'check_rustyline_upstream: registry returned no stable versions'
  exit "$__ush_status"
fi
__ush_at 43 'let reviewed = value "vendor/rustyline/UPSTREAM" "reviewed:"'
__ush_value_27="$(__ush_return_path='' __ush_capture_return='1' ush_fn_value 'vendor/rustyline/UPSTREAM' 'reviewed:' 4>&1 >&3)" || exit "$?"
__ush_binding_43_reviewed="${__ush_value_27}"
# shellcheck disable=SC2034
reviewed="${__ush_binding_43_reviewed}"
__ush_at 44 'echo ("pinned: " + pinned + ", latest in major: " + patch + ", latest overall: " + latest)'
printf '%s\n' 'pinned: '"${__ush_binding_21_pinned}"', latest in major: '"${__ush_binding_32_patch}"', latest overall: '"${__ush_binding_31_latest}"
__ush_at 45 'if pinned != patch || pinned != latest && reviewed != "v" + latest {'
if {
{
[ "$(if [ "${__ush_binding_21_pinned}" != "${__ush_binding_32_patch}" ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
} || {
{
[ "$(if [ "${__ush_binding_21_pinned}" != "${__ush_binding_31_latest}" ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
} && {
[ "$(if [ "${__ush_binding_43_reviewed}" != 'v'"${__ush_binding_31_latest}" ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}
}
}; then
  __ush_at 46 'fail 1 ("vendored rustyline is behind upstream; review https://github.com/kkawakam/rustyline/releases/tag/v" + latest + " and update vendor/rustyline/UPSTREAM")'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'vendored rustyline is behind upstream; review https://github.com/kkawakam/rustyline/releases/tag/v'"${__ush_binding_31_latest}"' and update vendor/rustyline/UPSTREAM'
  exit "$__ush_status"
fi
__ush_at 48 'echo "Vendored rustyline is current in its major and upstream releases have been reviewed."'
printf '%s\n' 'Vendored rustyline is current in its major and upstream releases have been reviewed.'
