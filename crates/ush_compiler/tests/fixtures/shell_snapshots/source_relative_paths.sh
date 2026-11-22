#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/source_relative_paths.ush' by ush.
# DO NOT EDIT MANUALLY.
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/source_relative_paths.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/source_relative_paths.ush'

__ush_at() { __ush_line=$1; __ush_source=$2; }
__ush_report() {
  if [ "$1" -ne 0 ] && [ -n "${__ush_line:-}" ]; then
    printf '%s:%s: %s (exit %s)\n  %s\n' "$__ush_origin" "$__ush_line" "${2:-command failed}" "$1" "$__ush_source" >&2
    __ush_line=''
  fi
}
trap '__ush_report "$?"' 0

case "${1:-}" in
  -h|--help) printf '%s' 'source_relative_paths.ush

Usage:
  source_relative_paths.ush [--help]
  source_relative_paths.ush [--man [ITEM]]
  source_relative_paths.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
source_relative_paths.ush

SYNOPSIS
source_relative_paths.ush [--help]
source_relative_paths.ush [--man [ITEM]]
source_relative_paths.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

ush_fn_std__fs__remove() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  rm -f "$__ush_path"
}

ush_fn_std__fs__tmpfile() {
  __ush_n=0
  while [ "$__ush_n" -lt 1000 ]; do
    __ush_tmp_path=${TMPDIR:-/tmp}/ush.$$.$__ush_n.tmp
    if (umask 077; set -C; : > "$__ush_tmp_path") 2>/dev/null; then
      printf '%s' "$__ush_tmp_path"
      return 0
    fi
    __ush_n=$((__ush_n + 1))
  done
  return 1
}

ush_fn_std__http__get() {
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$1"
    return "$?"
  fi
  if command -v wget >/dev/null 2>&1; then
    wget -qO- "$1"
    return "$?"
  fi
  printf '%s\n' 'ush std::http::get: curl or wget is required' >&2
  return 1
}

ush_fn_std__http__download() {
  __ush_target=$(ush_fn_std__path__resolve "$2")
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$1" -o "$__ush_target"
    return "$?"
  fi
  if command -v wget >/dev/null 2>&1; then
    wget -qO "$__ush_target" "$1"
    return "$?"
  fi
  printf '%s\n' 'ush std::http::download: curl or wget is required' >&2
  return 1
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

__ush_std_path_make_ref() {
  __ush_kind=$1
  __ush_value=$2
  case "$__ush_value" in
    ''|/*|__ush_pathref__:*) printf '%s' "$__ush_value" ;;
    *) printf '__ush_pathref__:%s:%s' "$__ush_kind" "$__ush_value" ;;
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

__ush_std_path_join() {
  __ush_left=$1
  __ush_right=$2
  case "$__ush_left" in
    __ush_pathref__:cwd:*)
      case "$__ush_right" in
        /*) printf '%s' "$__ush_right" ;;
        __ush_pathref__:*) __ush_std_path_join_parts "$(__ush_std_path_resolve "$__ush_left")" "$(__ush_std_path_resolve "$__ush_right")" ;;
        *) printf '__ush_pathref__:cwd:%s' "$(__ush_std_path_join_parts "${__ush_left#__ush_pathref__:cwd:}" "$__ush_right")" ;;
      esac
      ;;
    __ush_pathref__:source:*)
      case "$__ush_right" in
        /*) printf '%s' "$__ush_right" ;;
        __ush_pathref__:*) __ush_std_path_join_parts "$(__ush_std_path_resolve "$__ush_left")" "$(__ush_std_path_resolve "$__ush_right")" ;;
        *) printf '__ush_pathref__:source:%s' "$(__ush_std_path_join_parts "${__ush_left#__ush_pathref__:source:}" "$__ush_right")" ;;
      esac
      ;;
    *) __ush_std_path_join_parts "$(__ush_std_path_resolve "$__ush_left")" "$(__ush_std_path_resolve "$__ush_right")" ;;
  esac
}

ush_fn_std__path__tmpfile() {
  ush_fn_std__fs__tmpfile
}

ush_fn_std__path__from_cwd() {
  __ush_std_path_make_ref cwd "$1"
}

ush_fn_std__path__from_source() {
  __ush_std_path_make_ref source "$1"
}

ush_fn_std__path__resolve() {
  __ush_std_path_resolve "$1"
}

ush_fn_std__path__join() {
  __ush_std_path_join "$1" "$2"
}

__ush_at 4 'let source_root = from_source "."'
__ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__from_source '.')"
source_root="${__ush_value_0}"
__ush_at 5 'let payload = source_root.join("payload.txt")'
__ush_value_1=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__join "${source_root}" 'payload.txt')
payload="${__ush_value_1}"
__ush_at 6 'let cwd_file = from_cwd "notes.txt"'
__ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__from_cwd 'notes.txt')"
cwd_file="${__ush_value_2}"
__ush_at 7 'let url = "file://" + payload.resolve()'
__ush_value_3=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__resolve "${payload}")
url="$(printf '%s' 'file://' "${__ush_value_3}")"
__ush_at 8 'let copy = tmpfile()'
__ush_value_4="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__tmpfile)"
copy="${__ush_value_4}"
__ush_at 10 'print payload'
printf '%s\n' "${payload}"
__ush_at 11 'print $ cwd_file.resolve()'
__ush_value_5=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__resolve "${cwd_file}")
printf '%s\n' "${__ush_value_5}"
__ush_at 12 'print $ get url'
__ush_value_6="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__http__get "${url}")"
printf '%s\n' "${__ush_value_6}"
__ush_at 13 'download url copy'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__http__download "${url}" "${copy}" )
__ush_at 14 'copy.remove()'
__ush_value_7=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__remove "${copy}")
