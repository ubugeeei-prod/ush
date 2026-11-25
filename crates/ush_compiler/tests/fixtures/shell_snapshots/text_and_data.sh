#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/text_and_data.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/text_and_data.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_lf='
'
__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/text_and_data.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/text_and_data.ush'

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
  -h|--help) printf '%s' 'text_and_data.ush

Usage:
  text_and_data.ush [--help]
  text_and_data.ush [--man [ITEM]]
  text_and_data.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
text_and_data.ush

SYNOPSIS
text_and_data.ush [--help]
text_and_data.ush [--man [ITEM]]
text_and_data.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

__ush_std_list_join() {
  eval "__ush_list_len=\${${1}__len}"
  __ush_std_list_joined=''
  __ush_list_index=0
  while [ "$__ush_list_index" -lt "$__ush_list_len" ]; do
    eval "__ush_list_item=\${${1}__${__ush_list_index}}"
    if [ "$__ush_list_index" -gt 0 ]; then __ush_std_list_joined=$__ush_std_list_joined$2; fi
    __ush_std_list_joined=$__ush_std_list_joined$__ush_list_item
    __ush_list_index=$((__ush_list_index + 1))
  done
}

__ush_at 1 'let side = "left"'
__ush_binding_1_side='left'
# shellcheck disable=SC2034
side="${__ush_binding_1_side}"
__ush_at 2 'let files = ["a.ts", "b c.md"]'
__ush_binding_2_files__len='2'
# shellcheck disable=SC2034
__ush_binding_2_files__0='a.ts'
# shellcheck disable=SC2034
__ush_binding_2_files__1='b c.md'
__ush_at 3 'let dir = $env::RUNNER_TEMP'
__ush_env_0=${RUNNER_TEMP-}
if [ -z "${__ush_env_0}" ]; then __ush_report 1 'environment variable RUNNER_TEMP is not set or is empty'; exit 1; fi
__ush_binding_3_dir="${__ush_env_0}"
# shellcheck disable=SC2034
dir="${__ush_binding_3_dir}"
__ush_at 4 'print $"$dir/ambient-$side.jsonl"'
printf '%s\n' "${__ush_binding_3_dir}"'/ambient-'"${__ush_binding_1_side}"'.jsonl'
__ush_at 5 'print files[1]'
__ush_index_1=1
if [ "${__ush_index_1}" -lt 0 ] || [ "${__ush_index_1}" -ge "${__ush_binding_2_files__len}" ]; then __ush_report 1 "index ${__ush_index_1} is out of range for a list of length ${__ush_binding_2_files__len}"; exit 1; fi
__ush_item_2=''
eval "__ush_item_2=\"\${__ush_binding_2_files__${__ush_index_1}}\""
printf '%s\n' "${__ush_item_2}"
__ush_at 6 'print files.len()'
__ush_value_3=${__ush_binding_2_files__len}
printf '%s\n' "${__ush_value_3}"
__ush_at 7 'for (i, file) in files.enumerate() {'
__ush_for_list_index_4=0
while [ "${__ush_for_list_index_4}" -lt "${__ush_binding_2_files__len}" ]; do
  __ush_for_7__len=2
  __ush_for_7__0="${__ush_for_list_index_4}"
  __ush_for_7__1=''
  eval "__ush_for_7__1=\"\${__ush_binding_2_files__${__ush_for_list_index_4}}\""
  __ush_for_list_index_4=$((__ush_for_list_index_4 + 1))
  __ush_at 7 'for (i, file) in files.enumerate() {'
  __ush_binding_7_i="${__ush_for_7__0}"
  # shellcheck disable=SC2034
  i="${__ush_binding_7_i}"
  __ush_at 7 'for (i, file) in files.enumerate() {'
  __ush_binding_7_file="${__ush_for_7__1}"
  # shellcheck disable=SC2034
  file="${__ush_binding_7_file}"
  __ush_at 8 'match file {'
  case "${__ush_binding_7_file}" in
    *'.ts'|*'.mts')
      __ush_at 9 'glob "*.ts" | glob "*.mts" => print $"$i: typescript"'
      printf '%s\n' "${__ush_binding_7_i}"': typescript'
      ;;
    *)
      __ush_at 10 '_ => print $"$i: other"'
      printf '%s\n' "${__ush_binding_7_i}"': other'
      ;;
  esac
done
__ush_at 13 'print $"""'
__ush_std_list_join __ush_binding_2_files ', '
__ush_value_7=$__ush_std_list_joined
printf '%s\n' '## '"${__ush_binding_1_side}""${__ush_lf}""${__ush_value_7}"
