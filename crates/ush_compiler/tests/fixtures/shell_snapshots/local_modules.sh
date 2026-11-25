#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/local_modules.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/local_modules.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/local_modules.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/local_modules.ush'

__ush_at() { __ush_line=$1; __ush_source=$2; __ush_file=${3:-$__ush_origin}; }
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
    printf '%s:%s: %s (exit %s)\n  %s\n' "${__ush_file:-$__ush_origin}" "$__ush_line" "$__ush_message" "$1" "$__ush_source" >&2
    __ush_line=''
  fi
}
trap '__ush_report "$?"' 0

case "${1:-}" in
  -h|--help) printf '%s' 'local_modules.ush

Usage:
  local_modules.ush [--help]
  local_modules.ush [--man [ITEM]]
  local_modules.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
local_modules.ush

SYNOPSIS
local_modules.ush [--help]
local_modules.ush [--man [ITEM]]
local_modules.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

__ush_walk_record_6_report_6_Status() {
  "$1" "$2__tag" || return "$?"
  eval "__ush_tag=\${${2}__tag}"
  case $__ush_tag in
    'report::Status::Passed')
      ;;
    'report::Status::Failed')
      ;;
    *) return 1 ;;
  esac
}

ush_fn_std__write_cell() {
  eval "__ush_cell=\${$1}"
  printf x
  printf '%s' "$__ush_cell" | od -A n -v -t x1 | tr -d ' \n'
  printf ' '
}

ush_fn_std__read_cell() {
  case $__ush_cells in x*' '*) ;; *) return 1 ;; esac
  __ush_hex=${__ush_cells%% *}
  __ush_cells=${__ush_cells#* }
  __ush_cell=$(LC_ALL=C awk -v s="${__ush_hex#x}" 'BEGIN {
    digits = "0123456789abcdef"
    for (i = 1; i <= length(s); i += 2)
      printf "%c", (index(digits, substr(s,i,1))-1)*16 + index(digits,substr(s,i+1,1))-1
    printf "."
  }') || return "$?"
  __ush_cell=${__ush_cell%.}
  eval "$1=\$__ush_cell"
}

ush_fn_text__shout() {
  __ush_fn_text__shout_arg_0="${1}"
  # shellcheck disable=SC2034
  text="${__ush_fn_text__shout_arg_0}"
  __ush_at 2 'text + "!"' '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/local_modules/text.ush'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_fn_text__shout_arg_0}"'!' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_fn_text__shout_arg_0}"'!' >&4
  fi
  return 0
}

ush_fn_text__label() {
  __ush_at 6 '"text"' '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/local_modules/text.ush'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' 'text' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' 'text' >&4
  fi
  return 0
}

ush_fn_report__line() {
  __ush_cells="${1}"
  __ush_walk_record_6_report_6_Status ush_fn_std__read_cell __ush_fn_report__line_arg_0
  __ush_fn_report__line_arg_1="${2}"
  # shellcheck disable=SC2034
  name="${__ush_fn_report__line_arg_1}"
  __ush_at 9 'match status {' '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/local_modules/report.ush'
  if [ "${__ush_fn_report__line_arg_0__tag}" = 'report::Status::Passed' ]; then
    __ush_at 10 'Status::Passed => label + ": " + text::shout name,' '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/local_modules/report.ush'
    __ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_report__label 4>&1 >&3)" || return "$?"
    __ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_text__shout "${__ush_fn_report__line_arg_1}" 4>&1 >&3)" || return "$?"
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${__ush_value_0}"': '"${__ush_value_1}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${__ush_value_0}"': '"${__ush_value_1}" >&4
    fi
    return 0
  elif [ "${__ush_fn_report__line_arg_0__tag}" = 'report::Status::Failed' ]; then
    __ush_at 11 'Status::Failed => label + ": " + name,' '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/local_modules/report.ush'
    __ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_report__label 4>&1 >&3)" || return "$?"
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${__ush_value_2}"': '"${__ush_fn_report__line_arg_1}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${__ush_value_2}"': '"${__ush_fn_report__line_arg_1}" >&4
    fi
    return 0
  fi
}

ush_fn_report__label() {
  __ush_at 16 '"report"' '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/local_modules/report.ush'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' 'report' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' 'report' >&4
  fi
  return 0
}

__ush_at 6 'print line Status::Passed "build"'
__ush_match_3__tag='report::Status::Passed'
__ush_value_5="$(__ush_return_path='' __ush_capture_return='1' ush_fn_report__line "$(__ush_walk_record_6_report_6_Status ush_fn_std__write_cell __ush_match_3)" 'build' 4>&1 >&3)" || exit "$?"
printf '%s\n' "${__ush_value_5}"
__ush_at 7 'print text::label'
__ush_value_6="$(__ush_return_path='' __ush_capture_return='1' ush_fn_text__label 4>&1 >&3)" || exit "$?"
printf '%s\n' "${__ush_value_6}"
