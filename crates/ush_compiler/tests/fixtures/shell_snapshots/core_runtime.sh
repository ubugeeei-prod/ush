#!/bin/sh
set -eu

__ush_jobs=''
__ush_task_seq='0'
__ush_task_files=''
__ush_tmp_seq='0'

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/core_runtime.ush'

__ush_print_help() {
  cat <<'__USH_DOC__'
core_runtime.ush

Usage:
  core_runtime.ush [--help]
  core_runtime.ush [--man [ITEM]]
  core_runtime.ush [--complete [PREFIX]]
__USH_DOC__
}

__ush_print_man() {
  case "${1:-}" in
    '')
      cat <<'__USH_MAIN__'
NAME
core_runtime.ush

SYNOPSIS
core_runtime.ush [--help]
core_runtime.ush [--man [ITEM]]
core_runtime.ush [--complete [PREFIX]]
__USH_MAIN__
      ;;
    *)
      printf '%s\n' "No documented item named: ${1}" >&2
      return 1
      ;;
  esac
}

__ush_complete() {
  __ush_prefix="${1:-}"
  while IFS= read -r __ush_item; do
    case "$__ush_item" in
      "$__ush_prefix"*) printf '%s\n' "$__ush_item" ;;
    esac
  done <<'__USH_COMPLETE__'
--complete
--help
--man
-h
complete
man
__USH_COMPLETE__
}

case "${1:-}" in
  -h|--help)
    __ush_print_help
    exit 0
    ;;
  man|--man)
    shift
    __ush_print_man "${1:-}"
    exit $?
    ;;
  complete|--complete)
    shift
    __ush_complete "${1:-}"
    exit 0
    ;;
esac

__ush_std_env_validate() {
  case "$1" in
    ''|[0-9]*|*[!A-Za-z0-9_]*)
      printf '%s\n' "ush std::env: invalid env name: $1" >&2
      return 1
      ;;
  esac
}

ush_fn_std__env__get() {
  __ush_std_env_validate "$1"
  eval "printf '%s' \"\${$1-}\""
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

ush_fn_std__env__unset() {
  __ush_std_env_validate "$1"
  unset "$1"
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

ush_fn_std__fs__move() {
  __ush_from=$(ush_fn_std__path__resolve "$1")
  __ush_to=$(ush_fn_std__path__resolve "$2")
  mv "$__ush_from" "$__ush_to"
}

ush_fn_std__fs__copy() {
  __ush_from=$(ush_fn_std__path__resolve "$1")
  __ush_to=$(ush_fn_std__path__resolve "$2")
  cp "$__ush_from" "$__ush_to"
}

ush_fn_std__fs__exists() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  if [ -e "$__ush_path" ]; then printf '%s' 'true'; else printf '%s' 'false'; fi
}

ush_fn_std__fs__is_file() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  if [ -f "$__ush_path" ]; then printf '%s' 'true'; else printf '%s' 'false'; fi
}

ush_fn_std__fs__is_dir() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  if [ -d "$__ush_path" ]; then printf '%s' 'true'; else printf '%s' 'false'; fi
}

ush_fn_std__fs__mkdir_p() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  mkdir -p "$__ush_path"
}

ush_fn_std__fs__tmpfile() {
  __ush_tmp_seq=$((__ush_tmp_seq + 1))
  __ush_tmp_path="${TMPDIR:-/tmp}/ush.$$.$__ush_tmp_seq.tmp"
  : > "$__ush_tmp_path"
  printf '%s' "$__ush_tmp_path"
}

ush_fn_std__fs__sha256() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$__ush_path" | awk '{print $1}'
    return 0
  fi
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$__ush_path" | awk '{print $1}'
    return 0
  fi
  if command -v openssl >/dev/null 2>&1; then
    openssl dgst -sha256 "$__ush_path" | awk '{print $NF}'
    return 0
  fi
  printf '%s\n' 'ush std::fs::sha256: no hash tool available' >&2
  return 1
}

ush_fn_std__fs__mime_type() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  if ! command -v file >/dev/null 2>&1; then
    printf '%s' 'application/octet-stream'
    return 0
  fi
  if file -b --mime-type "$__ush_path" >/dev/null 2>&1; then
    file -b --mime-type "$__ush_path"
    return 0
  fi
  file -I "$__ush_path" | awk -F': ' '{print $2}' | awk -F';' '{print $1}'
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

__ush_std_path_dirname_ref() {
  __ush_value=$1
  case "$__ush_value" in
    __ush_pathref__:cwd:*)
      printf '__ush_pathref__:cwd:%s' "$(dirname "${__ush_value#__ush_pathref__:cwd:}")"
      ;;
    __ush_pathref__:source:*)
      printf '__ush_pathref__:source:%s' "$(dirname "${__ush_value#__ush_pathref__:source:}")"
      ;;
    *) dirname "$__ush_value" ;;
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

ush_fn_std__path__cwd() {
  pwd
}

ush_fn_std__path__home() {
  printf '%s' "${HOME:-}"
}

ush_fn_std__path__source_root() {
  if [ -n "${__ush_source_dir:-}" ]; then
    printf '%s' "$__ush_source_dir"
  else
    pwd
  fi
}

ush_fn_std__path__tmpfile() {
  __ush_tmp_seq=$((__ush_tmp_seq + 1))
  __ush_tmp_path="${TMPDIR:-/tmp}/ush.$$.$__ush_tmp_seq.tmp"
  : > "$__ush_tmp_path"
  printf '%s' "$__ush_tmp_path"
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

ush_fn_std__path__dirname() {
  __ush_std_path_dirname_ref "$1"
}

ush_fn_std__path__basename() {
  basename "$(__ush_std_path_resolve "$1")"
}

ush_fn_std__path__exists() {
  __ush_path=$(__ush_std_path_resolve "$1")
  if [ -e "$__ush_path" ]; then printf '%s' 'true'; else printf '%s' 'false'; fi
}

ush_fn_std__path__is_file() {
  __ush_path=$(__ush_std_path_resolve "$1")
  if [ -f "$__ush_path" ]; then printf '%s' 'true'; else printf '%s' 'false'; fi
}

ush_fn_std__path__is_dir() {
  __ush_path=$(__ush_std_path_resolve "$1")
  if [ -d "$__ush_path" ]; then printf '%s' 'true'; else printf '%s' 'false'; fi
}

ush_fn_std__path__mkdir_p() {
  mkdir -p "$(__ush_std_path_resolve "$1")"
}

ush_fn_std__path__prepend_env() {
  __ush_std_env_validate "$1"
  __ush_segment=$(__ush_std_path_resolve "$2")
  __ush_current="$(ush_fn_std__env__get "$1")"
  if [ -n "$__ush_current" ]; then
    export "$1=$__ush_segment:$__ush_current"
  else
    export "$1=$__ush_segment"
  fi
}

ush_fn_std__command__exists() {
  if command -v "$1" >/dev/null 2>&1; then printf '%s' 'true'; else printf '%s' 'false'; fi
}

ush_fn_std__command__capture() {
  /bin/sh -c "$1"
}

ush_fn_std__command__capture_stderr() {
  /bin/sh -c "$1" 2>&1 1>/dev/null
}

ush_fn_std__command__status() {
  if /bin/sh -c "$1" >/dev/null 2>/dev/null; then
    __ush_status=0
  else
    __ush_status=$?
  fi
  printf '%s' "$__ush_status"
}

ush_fn_std__command__run() {
  /bin/sh -c "$1"
}

ush_fn_std__regex__is_match() {
  VALUE="$1" PATTERN="$2" awk 'BEGIN { value = ENVIRON["VALUE"]; pattern = ENVIRON["PATTERN"]; if (value ~ pattern) { printf "true" } else { printf "false" } }'
}

ush_fn_std__regex__find() {
  VALUE="$1" PATTERN="$2" awk 'BEGIN { value = ENVIRON["VALUE"]; pattern = ENVIRON["PATTERN"]; if (match(value, pattern)) { printf "%s", substr(value, RSTART, RLENGTH) } }'
}

ush_fn_std__regex__replace() {
  VALUE="$1" PATTERN="$2" REPLACEMENT="$3" awk 'BEGIN { value = ENVIRON["VALUE"]; pattern = ENVIRON["PATTERN"]; replacement = ENVIRON["REPLACEMENT"]; gsub(pattern, replacement, value); printf "%s", value }'
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

ush_fn_std__string__trim_prefix() {
  VALUE="$1" PREFIX="$2" awk 'BEGIN { value = ENVIRON["VALUE"]; prefix = ENVIRON["PREFIX"]; if (substr(value, 1, length(prefix)) == prefix) { value = substr(value, length(prefix) + 1) } printf "%s", value }'
}

ush_fn_std__string__trim_suffix() {
  VALUE="$1" SUFFIX="$2" awk 'BEGIN { value = ENVIRON["VALUE"]; suffix = ENVIRON["SUFFIX"]; if (length(suffix) > 0 && length(value) >= length(suffix) && substr(value, length(value) - length(suffix) + 1) == suffix) { value = substr(value, 1, length(value) - length(suffix)) } printf "%s", value }'
}

ush_fn_greet() {
  __ush_fn_greet_arg_0="$1"
  __ush_fn_greet_arg_1="$2"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "$(printf '%s' "${__ush_fn_greet_arg_0}" ':' "${__ush_fn_greet_arg_1}")" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "$(printf '%s' "${__ush_fn_greet_arg_0}" ':' "${__ush_fn_greet_arg_1}")"
  fi
  return 0
}

ush_fn_wrap() {
  __ush_fn_wrap_arg_0="$1"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "$(printf '%s' '<' "${__ush_fn_wrap_arg_0}" '>')" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "$(printf '%s' '<' "${__ush_fn_wrap_arg_0}" '>')"
  fi
  return 0
}

wrapped__tag='Envelope::Wrap'
wrapped__0__tag='Response::Ok'
wrapped__0__0='done'
__ush_match_0__tag="${wrapped__tag}"
case "${wrapped__tag}" in
  'Envelope::Wrap')
__ush_match_0__0__tag="${wrapped__0__tag}"
case "${wrapped__0__tag}" in
  'Response::Ok')
__ush_match_0__0__0="${wrapped__0__0}"
    ;;
  'Response::Err')
__ush_match_0__0__0="${wrapped__0__0}"
    ;;
esac
    ;;
  'Envelope::Empty')
    ;;
esac
if [ "${__ush_match_0__tag}" = 'Envelope::Wrap' ] && [ "${__ush_match_0__0__tag}" = 'Response::Ok' ]; then
  message="${__ush_match_0__0__0-}"
  __ush_value_1="$(__ush_capture_return='1' ush_fn_wrap "${message}")"
  printf '%s\n' "${__ush_value_1}"
elif :; then
  printf '%s\n' 'fallback'
fi
__ush_value_2="$(__ush_capture_return='1' ush_fn_greet 'hello' 2)"
value="${__ush_value_2}"
printf '%s\n' "${value}"
__ush_value_3="$(__ush_capture_return='1' ush_fn_greet 'team' 3)"
__ush_value_4="$(__ush_capture_return='1' ush_fn_wrap "${__ush_value_3}")"
printf '%s\n' "${__ush_value_4}"

if [ -n "$__ush_jobs" ]; then
  for __ush_job in $__ush_jobs; do
    wait "$__ush_job" 2>/dev/null || true
  done
fi
if [ -n "$__ush_task_files" ]; then
  for __ush_task_file in $__ush_task_files; do
    rm -f "$__ush_task_file"
  done
fi
