#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/scoped_blocks.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/scoped_blocks.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/scoped_blocks.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/scoped_blocks.ush'

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
  -h|--help) printf '%s' 'scoped_blocks.ush

Usage:
  scoped_blocks.ush [--help]
  scoped_blocks.ush [--man [ITEM]]
  scoped_blocks.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
scoped_blocks.ush

SYNOPSIS
scoped_blocks.ush [--help]
scoped_blocks.ush [--man [ITEM]]
scoped_blocks.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

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

__ush_std_fs_path() {
  __ush_path=$(ush_fn_std__path__resolve "$1" && printf .) || return "$?"
  __ush_path=${__ush_path%.}
  [ -n "$__ush_path" ] || { printf '%s\n' 'std::fs: path must not be empty' >&2; return 1; }
  case $__ush_path in .|..|/*|./*|../*) ;; *) __ush_path=./$__ush_path ;; esac
}

ush_fn_std__process__chdir() {
  __ush_std_fs_path "$1" || return "$?"
  CDPATH='' cd "$__ush_path" || return "$?"
}

__ush_at 3 'export RUST_BACKTRACE = "1"'
export RUST_BACKTRACE='1'
__ush_at 4 'cd "crates" {'
__ush_cwd_0=$PWD
ush_fn_std__process__chdir 'crates' || { __ush_status=$?; __ush_report "$__ush_status"; exit "$__ush_status"; }
  __ush_at 5 'export CARGO_TERM_COLOR = "never", RUSTFLAGS = "-D warnings" {'
  __ush_had_CARGO_TERM_COLOR_1=${CARGO_TERM_COLOR+x} __ush_saved_CARGO_TERM_COLOR_1=${CARGO_TERM_COLOR-}
  export CARGO_TERM_COLOR='never'
  __ush_had_RUSTFLAGS_2=${RUSTFLAGS+x} __ush_saved_RUSTFLAGS_2=${RUSTFLAGS-}
  export RUSTFLAGS='-D warnings'
    __ush_at 6 'cargo "build"'
    command cargo 'build'
  if [ -n "$__ush_had_CARGO_TERM_COLOR_1" ]; then export CARGO_TERM_COLOR="$__ush_saved_CARGO_TERM_COLOR_1"; else unset CARGO_TERM_COLOR; fi
  if [ -n "$__ush_had_RUSTFLAGS_2" ]; then export RUSTFLAGS="$__ush_saved_RUSTFLAGS_2"; else unset RUSTFLAGS; fi
cd "$__ush_cwd_0" || { __ush_status=$?; __ush_report "$__ush_status" 'cannot return to the directory the cd block started in'; exit "$__ush_status"; }
__ush_at 9 'for name in ["a", "b"] {'
__ush_match_3__len='2'
# shellcheck disable=SC2034
__ush_match_3__0='a'
# shellcheck disable=SC2034
__ush_match_3__1='b'
__ush_for_list_index_4=0
while [ "${__ush_for_list_index_4}" -lt "${__ush_match_3__len}" ]; do
  __ush_for_value_5=''
  eval "__ush_for_value_5=\"\${__ush_match_3__${__ush_for_list_index_4}}\""
  name="${__ush_for_value_5}"
  __ush_for_list_index_4=$((__ush_for_list_index_4 + 1))
  __ush_at 10 'cd name {'
  __ush_cwd_6=$PWD
  ush_fn_std__process__chdir "${name}" || { __ush_status=$?; __ush_report "$__ush_status"; exit "$__ush_status"; }
    __ush_at 11 'if name == "b" {'
    if {
    [ "$(if [ "${name}" = 'b' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }; then
      __ush_at 12 'break'
      cd "$__ush_cwd_6" || { __ush_status=$?; __ush_report "$__ush_status" 'cannot return to the directory the cd block started in'; exit "$__ush_status"; }
      break
    fi
    __ush_at 14 'ls'
    command ls
  cd "$__ush_cwd_6" || { __ush_status=$?; __ush_report "$__ush_status" 'cannot return to the directory the cd block started in'; exit "$__ush_status"; }
done
