#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/stdlib_and_shell.ush' by ush.
# DO NOT EDIT MANUALLY.
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/stdlib_and_shell.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/stdlib_and_shell.ush'

__ush_at() { __ush_line=$1; __ush_source=$2; }
__ush_report() {
  if [ "$1" -ne 0 ] && [ -n "${__ush_line:-}" ]; then
    printf '%s:%s: %s (exit %s)\n  %s\n' "$__ush_origin" "$__ush_line" "${2:-command failed}" "$1" "$__ush_source" >&2
    __ush_line=''
  fi
}
trap '__ush_report "$?"' 0

case "${1:-}" in
  -h|--help) printf '%s' 'stdlib_and_shell.ush

Usage:
  stdlib_and_shell.ush [--help]
  stdlib_and_shell.ush [--man [ITEM]]
  stdlib_and_shell.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
stdlib_and_shell.ush

SYNOPSIS
stdlib_and_shell.ush [--help]
stdlib_and_shell.ush [--man [ITEM]]
stdlib_and_shell.ush [--complete [PREFIX]]
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
  __ush_std_env_validate "$1"
  __ush_default=$2
  eval "printf '%s' \"\${$1-\${__ush_default}}\""
}

ush_fn_std__env__set() {
  __ush_std_env_validate "$1"
  export "$1=$2"
}

ush_fn_std__fs__read_text() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  cat "$__ush_path"
}

ush_fn_std__fs__write_text() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  printf '%s' "$2" > "$__ush_path"
}

ush_fn_std__fs__append_text() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  printf '%s' "$2" >> "$__ush_path"
}

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

ush_fn_std__path__resolve() {
  __ush_std_path_resolve "$1"
}

ush_fn_std__path__join() {
  __ush_std_path_join "$1" "$2"
}

ush_fn_std__command__capture() {
  /bin/sh -c "$1"
}

ush_fn_std__command__status() {
  if /bin/sh -c "$1" >/dev/null 2>/dev/null; then
    __ush_status=0
  else
    __ush_status=$?
  fi
  printf '%s' "$__ush_status"
}

ush_fn_std__regex__capture() {
  if ! command -v perl >/dev/null 2>&1; then
    printf '%s\n' 'ush std::regex::capture: perl is required' >&2
    return 1
  fi
  LC_ALL=C LANG=C LC_CTYPE=C perl -e 'my ($value, $pattern, $index) = @ARGV; $index = int($index); exit 0 if $index < 1; if ($value =~ /$pattern/) { if (defined $-[$index] && $-[$index] >= 0) { print substr($value, $-[$index], $+[$index] - $-[$index]); } }' -- "$1" "$2" "$3"
}

ush_fn_std__string__starts_with() {
  VALUE="$1" PREFIX="$2" awk 'BEGIN { value = ENVIRON["VALUE"]; prefix = ENVIRON["PREFIX"]; if (substr(value, 1, length(prefix)) == prefix) { printf "true" } else { printf "false" } }'
}

ush_fn_std__string__ends_with() {
  VALUE="$1" SUFFIX="$2" awk 'BEGIN { value = ENVIRON["VALUE"]; suffix = ENVIRON["SUFFIX"]; if (length(suffix) == 0) { printf "true"; exit } start = length(value) - length(suffix) + 1; if (start < 1) { printf "false"; exit } if (substr(value, start) == suffix) { printf "true" } else { printf "false" } }'
}

ush_fn_std__string__replace() {
  VALUE="$1" FROM="$2" TO="$3" awk 'BEGIN { value = ENVIRON["VALUE"]; from = ENVIRON["FROM"]; to = ENVIRON["TO"]; if (from == "") { printf "%s", value; exit } out = ""; while ((idx = index(value, from)) > 0) { out = out substr(value, 1, idx - 1) to; value = substr(value, idx + length(from)) } printf "%s", out value }'
}

__ush_at 7 'let page = """'
page='<div>
  hello
</div>'
__ush_at 13 'env_set "USH_STD_IMPORT" "ready"'
ush_fn_std__env__set 'USH_STD_IMPORT' 'ready'
__ush_at 14 'let path = tmpfile()'
__ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__tmpfile)"
path="${__ush_value_0}"
__ush_at 15 'path.write_text("alpha")'
__ush_value_1=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__write_text "${path}" 'alpha')
__ush_at 16 'path.append_text(":beta")'
__ush_value_2=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__append_text "${path}" ':beta')
__ush_at 18 'let greeting = "hello"'
greeting='hello'
__ush_at 19 '$ printf '"'"'%s\n'"'"' "$greeting"'
printf '%s\n' "$greeting"
__ush_at 21 'match greeting {'
if [ "${greeting}" = 'hello' ]; then
  __ush_at 22 '"hello" => $ printf '"'"'%s\n'"'"' matched'
printf '%s\n' matched
elif :; then
  __ush_at 23 '_ => print "fallback"'
  printf '%s\n' 'fallback'
fi
__ush_at 26 'print page'
printf '%s\n' "${page}"
__ush_at 27 'print $ page.starts_with("<div>")'
__ush_value_3=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__starts_with "${page}" '<div>')
printf '%s\n' "${__ush_value_3}"
__ush_at 28 'print $ page.ends_with("</div>")'
__ush_value_4=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__ends_with "${page}" '</div>')
printf '%s\n' "${__ush_value_4}"
__ush_at 29 'print $ get_or "USH_STD_IMPORT" "fallback"'
__ush_value_5="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get_or 'USH_STD_IMPORT' 'fallback')"
printf '%s\n' "${__ush_value_5}"
__ush_at 30 'print $ join "/tmp/" "/ush"'
__ush_value_6="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__join '/tmp/' '/ush')"
printf '%s\n' "${__ush_value_6}"
__ush_at 31 'print $ replace "hello world" "world" "ush"'
__ush_value_7="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__replace 'hello world' 'world' 'ush')"
printf '%s\n' "${__ush_value_7}"
__ush_at 32 'print $ starts_with "ush" "u"'
__ush_value_8="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__starts_with 'ush' 'u')"
printf '%s\n' "${__ush_value_8}"
__ush_at 33 'print $ capture "printf '"'"'%s\n'"'"' hello"'
__ush_value_9="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__capture 'printf '"'"'%s\n'"'"' hello')"
printf '%s\n' "${__ush_value_9}"
__ush_at 34 'print $ status "exit 7"'
__ush_value_10="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__status 'exit 7')"
printf '%s\n' $((__ush_value_10))
__ush_at 35 'print $ regex_capture("release-v0.3.4", "v([0-9.]+)", 1)'
__ush_value_11="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__regex__capture 'release-v0.3.4' 'v([0-9.]+)' 1)"
printf '%s\n' "${__ush_value_11}"
__ush_at 36 'print $ path.read_text()'
__ush_value_12=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${path}")
printf '%s\n' "${__ush_value_12}"
__ush_at 37 'shell "printf '"'"'%s\n'"'"' from-shell"'
printf '%s\n' from-shell
__ush_at 38 'path.remove()'
__ush_value_13=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__remove "${path}")
