#!/bin/sh
# Generated from 'preflight.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: preflight.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'preflight.ush' --checked-in -o 'preflight.sh'
set -eu

__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'preflight.ush'
__ush_origin='preflight.ush'

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
  -h|--help) printf '%s' 'preflight.ush

Usage:
  preflight.ush [--help]
  preflight.ush [--man [ITEM]]
  preflight.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
preflight.ush

SYNOPSIS
preflight.ush [--help]
preflight.ush [--man [ITEM]]
preflight.ush [--complete [PREFIX]]
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

ush_fn_std__env__set() {
  __ush_std_env_validate "$1" || return "$?"
  export "$1=$2"
}

ush_fn_std__path__cwd() {
  pwd
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

__ush_std_encode_args() {
  for __ush_arg do
    printf x
    printf '%s' "$__ush_arg" | od -A n -v -t x1 | tr -d ' \n'
    printf ' '
  done
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

ush_fn_banner() {
  __ush_fn_banner_arg_0="${1}"
  # shellcheck disable=SC2034
  name="${__ush_fn_banner_arg_0}"
  __ush_at 10 'echo ("=== " + name + " ===")'
  printf '%s\n' '=== '"${__ush_fn_banner_arg_0}"' ===' || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
}

__ush_at 12 'banner "rustfmt"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'rustfmt' )
__ush_at 13 'cargo "fmt" "--all" "--check"'
command cargo 'fmt' '--all' '--check'
__ush_at 14 'banner "clippy"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'clippy' )
__ush_at 15 'cargo "clippy" "--locked" "--workspace" "--all-targets" "--no-deps" "--" "-D" "warnings"'
command cargo 'clippy' '--locked' '--workspace' '--all-targets' '--no-deps' '--' '-D' 'warnings'
__ush_at 16 'banner "workspace tests"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'workspace tests' )
__ush_at 17 'cargo "test" "--locked" "--workspace"'
command cargo 'test' '--locked' '--workspace'
__ush_at 18 'banner "release tests"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'release tests' )
__ush_at 19 'cargo "test" "--locked" "--workspace" "--release"'
command cargo 'test' '--locked' '--workspace' '--release'
__ush_at 20 'banner "no_std"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'no_std' )
__ush_at 21 'cargo "check" "--locked" "-p" "ush_compiler" "--no-default-features"'
command cargo 'check' '--locked' '-p' 'ush_compiler' '--no-default-features'
__ush_at 22 'cargo "test" "--locked" "-p" "ush_compiler" "--no-default-features" "--lib"'
command cargo 'test' '--locked' '-p' 'ush_compiler' '--no-default-features' '--lib'
__ush_at 23 'cargo "clippy" "--locked" "-p" "ush_compiler" "--no-default-features" "--lib" "--no-deps" "--" "-D" "warnings"'
command cargo 'clippy' '--locked' '-p' 'ush_compiler' '--no-default-features' '--lib' '--no-deps' '--' '-D' 'warnings'
__ush_at 24 'banner "generated artifacts"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'generated artifacts' )
__ush_at 25 'sh "scripts/check_generated.sh"'
command sh 'scripts/check_generated.sh'
__ush_at 26 'banner "line limit"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'line limit' )
__ush_at 27 'sh "scripts/check_rs_line_limit.sh"'
command sh 'scripts/check_rs_line_limit.sh'
__ush_at 28 'banner "installer"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'installer' )
__ush_at 29 'sh "scripts/test_install.sh"'
command sh 'scripts/test_install.sh'
__ush_at 30 'banner "rustdoc"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'rustdoc' )
__ush_at 31 'set "RUSTDOCFLAGS" "-D warnings"'
ush_fn_std__env__set 'RUSTDOCFLAGS' '-D warnings'
__ush_at 32 'cargo "doc" "--locked" "--workspace" "--no-deps" "--document-private-items"'
command cargo 'doc' '--locked' '--workspace' '--no-deps' '--document-private-items'
__ush_at 33 'banner "shell targets"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'shell targets' )
__ush_at 34 'sh "scripts/test_targets.sh"'
command sh 'scripts/test_targets.sh'
__ush_at 35 'banner "editor"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'editor' )
__ush_at 36 'let root = cwd'
__ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__cwd)" || exit "$?"
__ush_binding_36_root="${__ush_value_0}"
# shellcheck disable=SC2034
root="${__ush_binding_36_root}"
__ush_at 37 'cd "editors/vscode"'
command cd 'editors/vscode'
__ush_at 38 'npm "ci" "--ignore-scripts"'
command npm 'ci' '--ignore-scripts'
__ush_at 39 'npm "test"'
command npm 'test'
__ush_at 40 'npm "run" "build"'
command npm 'run' 'build'
__ush_at 41 'cd root'
command cd "${__ush_binding_36_root}"
__ush_at 42 'banner "bench builds"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'bench builds' )
__ush_at 43 'cargo "bench" "--locked" "-p" "ush_shell" "--no-run"'
command cargo 'bench' '--locked' '-p' 'ush_shell' '--no-run'
__ush_at 44 'cargo "bench" "--locked" "-p" "ush_compiler" "--no-run"'
command cargo 'bench' '--locked' '-p' 'ush_compiler' '--no-run'
__ush_at 45 'banner "vendored dependency"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'vendored dependency' )
__ush_at 46 'sh "scripts/check_rustyline_upstream.sh"'
command sh 'scripts/check_rustyline_upstream.sh'
__ush_at 47 'banner "examples and shellcheck"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'examples and shellcheck' )
__ush_at 48 'for example in lines (find "examples" "-name" "*.ush" "-type" "f") {'
__ush_value_2="$(__ush_return_path='' __ush_capture_return='1' command find 'examples' '-name' '*.ush' '-type' 'f')" || exit "$?"
__ush_packed_3="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__lines "${__ush_value_2}")" || exit "$?"
__ush_match_1__len=0
ush_fn_std__unpack __ush_match_1 "${__ush_packed_3}"
__ush_for_list_index_4=0
while [ "${__ush_for_list_index_4}" -lt "${__ush_match_1__len}" ]; do
  __ush_for_value_5=''
  eval "__ush_for_value_5=\"\${__ush_match_1__${__ush_for_list_index_4}}\""
  example="${__ush_for_value_5}"
  __ush_for_list_index_4=$((__ush_for_list_index_4 + 1))
  __ush_at 49 'cargo "run" "--locked" "-p" "ush" "--quiet" "--" "check" example'
  command cargo 'run' '--locked' '-p' 'ush' '--quiet' '--' 'check' "${example}"
  __ush_at 50 'cargo "run" "--locked" "-p" "ush" "--quiet" "--" "format" "--check" example'
  command cargo 'run' '--locked' '-p' 'ush' '--quiet' '--' 'format' '--check' "${example}"
done
__ush_at 52 'if exists "shellcheck" {'
if {
__ush_value_6="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__exists 'shellcheck')" || exit "$?"
[ "${__ush_value_6}" = 'true' ]
}; then
  __ush_at 53 'for artifact in lines (find "install.sh" "scripts" "-name" "*.sh" "-type" "f") {'
  __ush_value_8="$(__ush_return_path='' __ush_capture_return='1' command find 'install.sh' 'scripts' '-name' '*.sh' '-type' 'f')" || exit "$?"
  __ush_packed_9="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__lines "${__ush_value_8}")" || exit "$?"
  __ush_match_7__len=0
  ush_fn_std__unpack __ush_match_7 "${__ush_packed_9}"
  __ush_for_list_index_10=0
  while [ "${__ush_for_list_index_10}" -lt "${__ush_match_7__len}" ]; do
    __ush_for_value_11=''
    eval "__ush_for_value_11=\"\${__ush_match_7__${__ush_for_list_index_10}}\""
    artifact="${__ush_for_value_11}"
    __ush_for_list_index_10=$((__ush_for_list_index_10 + 1))
    __ush_at 54 'exec "shellcheck" ["--severity=warning", artifact]'
    __ush_match_12__len='2'
    # shellcheck disable=SC2034
    __ush_match_12__0='--severity=warning'
    # shellcheck disable=SC2034
    __ush_match_12__1="${artifact}"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec 'shellcheck' "$(ush_fn_std__pack __ush_match_12)" )
  done
fi
__ush_at 57 'banner "preflight passed"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_banner 'preflight passed' )
