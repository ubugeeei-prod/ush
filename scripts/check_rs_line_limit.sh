#!/bin/sh
# Generated from 'check_rs_line_limit.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: check_rs_line_limit.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'check_rs_line_limit.ush' --checked-in -o 'check_rs_line_limit.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'check_rs_line_limit.ush'
__ush_origin='check_rs_line_limit.ush'

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
  -h|--help) printf '%s' 'check_rs_line_limit.ush

Usage:
  check_rs_line_limit.ush [--help]
  check_rs_line_limit.ush [--man [ITEM]]
  check_rs_line_limit.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
check_rs_line_limit.ush

SYNOPSIS
check_rs_line_limit.ush [--help]
check_rs_line_limit.ush [--man [ITEM]]
check_rs_line_limit.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

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

ush_fn_std____extern__int() {
  __ush_sign=''
  __ush_limit=9223372036854775807
  case $1 in
    -*) __ush_sign=-; __ush_digits=${1#-}; __ush_limit=9223372036854775808 ;;
    *) __ush_digits=$1 ;;
  esac
  case $__ush_digits in
    ''|*[!0-9]*) printf '%s\n' 'extern returned an invalid Int' >&2; return 1 ;;
  esac
  while [ "${__ush_digits#0}" != "$__ush_digits" ]; do __ush_digits=${__ush_digits#0}; done
  if [ -z "$__ush_digits" ]; then printf 0; return 0; fi
  if [ "${#__ush_digits}" -gt 19 ] || { [ "${#__ush_digits}" -eq 19 ] && [ "$(printf '%s\n%s\n' "$__ush_digits" "$__ush_limit" | LC_ALL=C sort | tail -n 1)" != "$__ush_limit" ]; }; then
    printf '%s\n' 'extern Int is outside the signed 64-bit range' >&2
    return 1
  fi
  printf '%s%s' "$__ush_sign" "$__ush_digits"
}

__ush_std_encode_args() {
  for __ush_arg do
    printf x
    printf '%s' "$__ush_arg" | od -A n -v -t x1 | tr -d ' \n'
    printf ' '
  done
}

ush_fn_std__process__exit() {
  case $1 in ''|*[!0-9]*) return 1 ;; esac
  [ "$1" -le 255 ] || return 1
  exit "$1"
}

ush_fn_std__io__eprint() {
  printf '%s\n' "$1" >&2
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

ush_fn_std__string__to_int() {
  if __ush_integer=$(ush_fn_std____extern__int "$1" 2>/dev/null); then
    printf '%s' "$__ush_integer"
  else
    printf '%s\n' 'std::string::to_int: expected a signed 64-bit decimal integer' >&2
    return 1
  fi
}

__ush_at 7 'let mut failed = 0'
__ush_binding_7_failed=0
# shellcheck disable=SC2034
failed="${__ush_binding_7_failed}"
__ush_at 8 'for file in lines (find apps crates -type f -name *.rs) {'
__ush_value_1="$(__ush_return_path='' __ush_capture_return='1' command find 'apps' 'crates' '-type' 'f' '-name' '*.rs')" || exit "$?"
__ush_packed_2="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__lines "${__ush_value_1}")" || exit "$?"
__ush_match_0__len=0
ush_fn_std__unpack __ush_match_0 "${__ush_packed_2}"
__ush_for_list_index_3=0
while [ "${__ush_for_list_index_3}" -lt "${__ush_match_0__len}" ]; do
  __ush_for_value_4=''
  eval "__ush_for_value_4=\"\${__ush_match_0__${__ush_for_list_index_3}}\""
  file="${__ush_for_value_4}"
  __ush_for_list_index_3=$((__ush_for_list_index_3 + 1))
  __ush_at 9 'let mut count = ""'
  __ush_binding_9_count=''
  # shellcheck disable=SC2034
  count="${__ush_binding_9_count}"
  __ush_at 10 'for word in split (trim (wc -l $file)) " " {'
  __ush_value_6="$(__ush_return_path='' __ush_capture_return='1' command wc '-l' "${file}")" || exit "$?"
  __ush_value_7="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim "${__ush_value_6}")" || exit "$?"
  __ush_packed_8="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__split "${__ush_value_7}" ' ')" || exit "$?"
  __ush_match_5__len=0
  ush_fn_std__unpack __ush_match_5 "${__ush_packed_8}"
  __ush_for_list_index_9=0
  while [ "${__ush_for_list_index_9}" -lt "${__ush_match_5__len}" ]; do
    __ush_for_value_10=''
    eval "__ush_for_value_10=\"\${__ush_match_5__${__ush_for_list_index_9}}\""
    word="${__ush_for_value_10}"
    __ush_for_list_index_9=$((__ush_for_list_index_9 + 1))
    __ush_at 11 'count = word'
    __ush_binding_9_count="${word}"
    # shellcheck disable=SC2034
    count="${__ush_binding_9_count}"
    __ush_at 12 'break'
    break
  done
  __ush_at 14 'if to_int $count > 250 {'
  if {
  __ush_value_11="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__to_int "${__ush_binding_9_count}")" || exit "$?"
  [ "$(if [ "${__ush_value_11}" -gt 250 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 15 'eprint (file + " has " + count + " lines (limit: 250)")'
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint "${file}"' has '"${__ush_binding_9_count}"' lines (limit: 250)' )
    __ush_at 16 'failed = 1'
    __ush_binding_7_failed=1
    # shellcheck disable=SC2034
    failed="${__ush_binding_7_failed}"
  fi
done
__ush_at 19 'exit $failed'
ush_fn_std__process__exit "${__ush_binding_7_failed}"
