#!/bin/sh
# Generated from 'test_install.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: test_install.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'test_install.ush' --checked-in -o 'test_install.sh'
set -eu

__ush_lf='
'
__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'test_install.ush'
__ush_origin='test_install.ush'

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
  -h|--help) printf '%s' 'test_install.ush

Usage:
  test_install.ush [--help]
  test_install.ush [--man [ITEM]]
  test_install.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
test_install.ush

SYNOPSIS
test_install.ush [--help]
test_install.ush [--man [ITEM]]
test_install.ush [--complete [PREFIX]]
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

ush_fn_std__env__set() {
  __ush_std_env_validate "$1" || return "$?"
  export "$1=$2"
}

ush_fn_std__env__unset() {
  __ush_std_env_validate "$1" || return "$?"
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

ush_fn_std__fs__is_file() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  if [ -f "$__ush_path" ]; then printf '%s' 'true'; else printf '%s' 'false'; fi
}

ush_fn_std__fs__mkdir_p() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  mkdir -p "$__ush_path"
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

ush_fn_std__path__cwd() {
  pwd
}

ush_fn_std__path__source_root() {
  if [ -n "${__ush_source_dir:-}" ]; then
    printf '%s' "$__ush_source_dir"
  else
    pwd
  fi
}

ush_fn_std__path__resolve() {
  __ush_std_path_resolve "$1"
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

ush_fn_std__test() {
  __ush_test=0
  # Preserve test's status 2 (invalid arguments), distinct from false/status 1.
  # shellcheck disable=SC2319
  test "$@" || __ush_test=$?
  if [ "$__ush_test" -eq 0 ]; then printf true; else
    if [ "$__ush_test" -eq 1 ]; then printf false; else return "$__ush_test"; fi
  fi
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

ush_fn_std__io__eprint() {
  printf '%s\n' "$1" >&2
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

ush_fn_std__process__redirect() {
  if (ush_fn_std__process__exec "$1" "$2") > "$3" 2> "$4"; then printf 0; else printf '%s' "$?"; fi
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

ush_fn_std__string__contains() {
  case $1 in *"$2"*) printf true ;; *) printf false ;; esac
}

# raises: unknown
ush_fn_target() {
  __ush_at 12 'let os = match uname -s {'
  __ush_value_1="$(__ush_return_path='' __ush_capture_return='1' command uname '-s')" || return "$?"
  __ush_match_0="${__ush_value_1}"
  case "${__ush_match_0}" in
    'Linux')
      __ush_at 13 '"Linux" => "unknown-linux-gnu"'
      __ush_binding_12_os='unknown-linux-gnu'
      ;;
    'Darwin')
      __ush_at 14 '"Darwin" => "apple-darwin"'
      __ush_binding_12_os='apple-darwin'
      ;;
    *)
      other="${__ush_match_0-}"
      __ush_at 15 'other => fail 1 ("unsupported test OS: " + other)'
      __ush_status=1
      if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
      __ush_report "$__ush_status" 'unsupported test OS: '"${other}"
      return "$__ush_status"
      ;;
  esac
  # shellcheck disable=SC2034
  os="${__ush_binding_12_os}"
  __ush_at 17 'let arch = match uname -m {'
  __ush_value_3="$(__ush_return_path='' __ush_capture_return='1' command uname '-m')" || return "$?"
  __ush_match_2="${__ush_value_3}"
  case "${__ush_match_2}" in
    'arm64')
      __ush_at 18 '"arm64" => "aarch64"'
      __ush_binding_17_arch='aarch64'
      ;;
    'amd64')
      __ush_at 19 '"amd64" => "x86_64"'
      __ush_binding_17_arch='x86_64'
      ;;
    *)
      other="${__ush_match_2-}"
      __ush_at 20 'other => other'
      __ush_binding_17_arch="${other}"
      ;;
  esac
  # shellcheck disable=SC2034
  arch="${__ush_binding_17_arch}"
  __ush_at 22 'arch + "-" + os'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_binding_17_arch}"'-'"${__ush_binding_12_os}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_binding_17_arch}"'-'"${__ush_binding_12_os}" >&4
  fi
  return 0
}

# raises: unknown
ush_fn_fixture_binary() {
  __ush_at 26 'for directory in split (get PATH) ":" {'
  __ush_value_5="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'PATH')" || return "$?"
  __ush_packed_6="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__split "${__ush_value_5}" ':')" || return "$?"
  __ush_match_4__len=0
  ush_fn_std__unpack __ush_match_4 "${__ush_packed_6}"
  __ush_for_list_index_7=0
  while [ "${__ush_for_list_index_7}" -lt "${__ush_match_4__len}" ]; do
    __ush_for_value_8=''
    eval "__ush_for_value_8=\"\${__ush_match_4__${__ush_for_list_index_7}}\""
    directory="${__ush_for_value_8}"
    __ush_for_list_index_7=$((__ush_for_list_index_7 + 1))
    __ush_at 27 'let candidate = directory + "/true"'
    __ush_binding_27_candidate="${directory}"'/true'
    # shellcheck disable=SC2034
    candidate="${__ush_binding_27_candidate}"
    __ush_at 28 'if test -f $candidate && test -x $candidate {'
    if {
    {
    __ush_value_9="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-f' "${__ush_binding_27_candidate}")" || return "$?"
    [ "${__ush_value_9}" = 'true' ]
    } && {
    __ush_value_10="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-x' "${__ush_binding_27_candidate}")" || return "$?"
    [ "${__ush_value_10}" = 'true' ]
    }
    }; then
      __ush_at 29 'return candidate'
      if [ -n "${__ush_return_path:-}" ]; then
        printf '%s' "${__ush_binding_27_candidate}" > "$__ush_return_path"
      elif [ "${__ush_capture_return:-0}" = '1' ]; then
        printf '%s' "${__ush_binding_27_candidate}" >&4
      fi
      return 0
    fi
  done
  __ush_at 32 'fail 1 "test_install: the external true utility is required on PATH"'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'test_install: the external true utility is required on PATH'
  return "$__ush_status"
}

# raises: unknown
ush_fn_archive() {
  __ush_fn_archive_arg_0="${1}"
  # shellcheck disable=SC2034
  work="${__ush_fn_archive_arg_0}"
  __ush_fn_archive_arg_1="${2}"
  # shellcheck disable=SC2034
  target="${__ush_fn_archive_arg_1}"
  __ush_fn_archive_arg_2="${3}"
  # shellcheck disable=SC2034
  fixture="${__ush_fn_archive_arg_2}"
  __ush_at 36 'let package = work + "/ush-" + target'
  __ush_binding_36_package="${__ush_fn_archive_arg_0}"'/ush-'"${__ush_fn_archive_arg_1}"
  # shellcheck disable=SC2034
  package="${__ush_binding_36_package}"
  __ush_at 37 'mkdir_p $package'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__mkdir_p "${__ush_binding_36_package}" ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 39 'cp $fixture (package + "/ush")'
  command cp "${__ush_fn_archive_arg_2}" "${__ush_binding_36_package}"'/ush' || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 40 'cp $fixture (package + "/ush_lsp")'
  command cp "${__ush_fn_archive_arg_2}" "${__ush_binding_36_package}"'/ush_lsp' || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 41 'tar -czf (work + "/ush.tar.gz") -C $work ("ush-" + target)'
  command tar '-czf' "${__ush_fn_archive_arg_0}"'/ush.tar.gz' '-C' "${__ush_fn_archive_arg_0}" 'ush-'"${__ush_fn_archive_arg_1}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 42 'write_text (work + "/sha256sums.txt") (sha256 (work + "/ush.tar.gz") + "  ush-" + target + ".tar.gz\n")'
  __ush_value_11="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__sha256 "${__ush_fn_archive_arg_0}"'/ush.tar.gz')" || return "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__write_text "${__ush_fn_archive_arg_0}"'/sha256sums.txt' "${__ush_value_11}"'  ush-'"${__ush_fn_archive_arg_1}"'.tar.gz'"${__ush_lf}"'' ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
}

__ush_at 45 'let installer = resolve (source_root + "/../install.sh")'
__ush_value_12="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__source_root)" || exit "$?"
__ush_value_13="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__resolve "${__ush_value_12}"'/../install.sh')" || exit "$?"
__ush_binding_45_installer="${__ush_value_13}"
# shellcheck disable=SC2034
installer="${__ush_binding_45_installer}"
__ush_at 46 'let target = target'
__ush_value_14="$(__ush_return_path='' __ush_capture_return='1' ush_fn_target 4>&1 >&3)" || exit "$?"
__ush_binding_46_target="${__ush_value_14}"
# shellcheck disable=SC2034
target="${__ush_binding_46_target}"
__ush_at 47 'let tools_path = get PATH'
__ush_value_15="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'PATH')" || exit "$?"
__ush_binding_47_tools_path="${__ush_value_15}"
# shellcheck disable=SC2034
tools_path="${__ush_binding_47_tools_path}"
__ush_at 48 'let fixture = fixture_binary'
__ush_value_16="$(__ush_return_path='' __ush_capture_return='1' ush_fn_fixture_binary 4>&1 >&3)" || exit "$?"
__ush_binding_48_fixture="${__ush_value_16}"
# shellcheck disable=SC2034
fixture="${__ush_binding_48_fixture}"
__ush_at 49 'let root = std::path::cwd'
__ush_value_17="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__cwd)" || exit "$?"
__ush_binding_49_root="${__ush_value_17}"
# shellcheck disable=SC2034
root="${__ush_binding_49_root}"
__ush_at 50 'for name in ["USH_PREFIX", "UBSH_PREFIX", "USH_BIN_DIR", "UBSH_BIN_DIR", "USH_VERSION", "UBSH_VERSION", "USH_BRANCH", "UBSH_BRANCH", "UBSH_AUTO_PATH"] {'
__ush_match_18__len='9'
# shellcheck disable=SC2034
__ush_match_18__0='USH_PREFIX'
# shellcheck disable=SC2034
__ush_match_18__1='UBSH_PREFIX'
# shellcheck disable=SC2034
__ush_match_18__2='USH_BIN_DIR'
# shellcheck disable=SC2034
__ush_match_18__3='UBSH_BIN_DIR'
# shellcheck disable=SC2034
__ush_match_18__4='USH_VERSION'
# shellcheck disable=SC2034
__ush_match_18__5='UBSH_VERSION'
# shellcheck disable=SC2034
__ush_match_18__6='USH_BRANCH'
# shellcheck disable=SC2034
__ush_match_18__7='UBSH_BRANCH'
# shellcheck disable=SC2034
__ush_match_18__8='UBSH_AUTO_PATH'
__ush_for_list_index_19=0
while [ "${__ush_for_list_index_19}" -lt "${__ush_match_18__len}" ]; do
  __ush_for_value_20=''
  eval "__ush_for_value_20=\"\${__ush_match_18__${__ush_for_list_index_19}}\""
  name="${__ush_for_value_20}"
  __ush_for_list_index_19=$((__ush_for_list_index_19 + 1))
  __ush_at 51 'unset $name'
  ush_fn_std__env__unset "${name}"
done
__ush_at 53 'for scenario in ["existing", "automatic", "disabled", "flags", "prefix", "relative-prefix", "empty-env", "localhost", "checksum-name", "checksum-content", "profile-failure", "spaces"] {'
__ush_task_init
__ush_at 53 'for scenario in ["existing", "automatic", "disabled", "flags", "prefix", "relative-prefix", "empty-env", "localhost", "checksum-name", "checksum-content", "profile-failure", "spaces"] {'
__ush_match_21__len='12'
# shellcheck disable=SC2034
__ush_match_21__0='existing'
# shellcheck disable=SC2034
__ush_match_21__1='automatic'
# shellcheck disable=SC2034
__ush_match_21__2='disabled'
# shellcheck disable=SC2034
__ush_match_21__3='flags'
# shellcheck disable=SC2034
__ush_match_21__4='prefix'
# shellcheck disable=SC2034
__ush_match_21__5='relative-prefix'
# shellcheck disable=SC2034
__ush_match_21__6='empty-env'
# shellcheck disable=SC2034
__ush_match_21__7='localhost'
# shellcheck disable=SC2034
__ush_match_21__8='checksum-name'
# shellcheck disable=SC2034
__ush_match_21__9='checksum-content'
# shellcheck disable=SC2034
__ush_match_21__10='profile-failure'
# shellcheck disable=SC2034
__ush_match_21__11='spaces'
__ush_for_list_index_22=0
while [ "${__ush_for_list_index_22}" -lt "${__ush_match_21__len}" ]; do
  __ush_for_value_23=''
  eval "__ush_for_value_23=\"\${__ush_match_21__${__ush_for_list_index_22}}\""
  scenario="${__ush_for_value_23}"
  __ush_for_list_index_22=$((__ush_for_list_index_22 + 1))
  __ush_at 54 'let work = temp_dir'
  __ush_value_24="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
  __ush_binding_54_work="${__ush_value_24}"
  # shellcheck disable=SC2034
  work="${__ush_binding_54_work}"
  __ush_at 55 'archive $work $target $fixture'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_archive "${__ush_binding_54_work}" "${__ush_binding_46_target}" "${__ush_binding_48_fixture}" )
  __ush_at 56 'let mut home = work + "/home"'
  __ush_binding_56_home="${__ush_binding_54_work}"'/home'
  # shellcheck disable=SC2034
  home="${__ush_binding_56_home}"
  __ush_at 57 'if scenario == "spaces" {'
  if {
  [ "$(if [ "${scenario}" = 'spaces' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 58 'home = work + "/home with spaces '"'"'$value"'
    __ush_binding_56_home="${__ush_binding_54_work}"'/home with spaces '"'"'$value'
    # shellcheck disable=SC2034
    home="${__ush_binding_56_home}"
  fi
  __ush_at 60 'mkdir_p $home'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__mkdir_p "${__ush_binding_56_home}" )
  __ush_at 61 'set HOME $home'
  ush_fn_std__env__set 'HOME' "${__ush_binding_56_home}"
  __ush_at 62 'set PATH $tools_path'
  ush_fn_std__env__set 'PATH' "${__ush_binding_47_tools_path}"
  __ush_at 63 'set SHELL /bin/zsh'
  ush_fn_std__env__set 'SHELL' '/bin/zsh'
  __ush_at 64 'set USH_AUTO_PATH "1"'
  ush_fn_std__env__set 'USH_AUTO_PATH' '1'
  __ush_at 65 'unset UBSH_AUTO_PATH'
  ush_fn_std__env__unset 'UBSH_AUTO_PATH'
  __ush_at 66 'set USH_DOWNLOAD_URL ("file://" + work + "/ush.tar.gz")'
  ush_fn_std__env__set 'USH_DOWNLOAD_URL' 'file://'"${__ush_binding_54_work}"'/ush.tar.gz'
  __ush_at 67 'set USH_CHECKSUM_URL ("file://" + work + "/sha256sums.txt")'
  ush_fn_std__env__set 'USH_CHECKSUM_URL' 'file://'"${__ush_binding_54_work}"'/sha256sums.txt'
  __ush_at 68 'let mut directory = home + "/.local/bin"'
  __ush_binding_68_directory="${__ush_binding_56_home}"'/.local/bin'
  # shellcheck disable=SC2034
  directory="${__ush_binding_68_directory}"
  __ush_at 69 'let mut arguments = [installer]'
  __ush_binding_69_arguments__len='1'
  # shellcheck disable=SC2034
  __ush_binding_69_arguments__0="${__ush_binding_45_installer}"
  __ush_at 70 'let mut expected = 0'
  __ush_binding_70_expected=0
  # shellcheck disable=SC2034
  expected="${__ush_binding_70_expected}"
  __ush_at 71 'if scenario == "existing" {'
  if {
  [ "$(if [ "${scenario}" = 'existing' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 72 'directory = home + "/bin"'
    __ush_binding_68_directory="${__ush_binding_56_home}"'/bin'
    # shellcheck disable=SC2034
    directory="${__ush_binding_68_directory}"
    __ush_at 73 'mkdir_p $directory'
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__mkdir_p "${__ush_binding_68_directory}" )
    __ush_at 74 'set PATH (directory + ":" + tools_path)'
    ush_fn_std__env__set 'PATH' "${__ush_binding_68_directory}"':'"${__ush_binding_47_tools_path}"
  else
    __ush_at 75 '} else if scenario == "disabled" {'
    if {
    [ "$(if [ "${scenario}" = 'disabled' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }; then
      __ush_at 76 'set USH_AUTO_PATH "0"'
      ush_fn_std__env__set 'USH_AUTO_PATH' '0'
    else
      __ush_at 77 '} else if scenario == "flags" {'
      if {
      [ "$(if [ "${scenario}" = 'flags' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
      }; then
        __ush_at 78 'set USH_DOWNLOAD_URL ("file://" + work + "/missing-archive")'
        ush_fn_std__env__set 'USH_DOWNLOAD_URL' 'file://'"${__ush_binding_54_work}"'/missing-archive'
        __ush_at 79 'set USH_CHECKSUM_URL ("file://" + work + "/missing-sums")'
        ush_fn_std__env__set 'USH_CHECKSUM_URL' 'file://'"${__ush_binding_54_work}"'/missing-sums'
        __ush_at 80 'directory = home + "/tools/bin"'
        __ush_binding_68_directory="${__ush_binding_56_home}"'/tools/bin'
        # shellcheck disable=SC2034
        directory="${__ush_binding_68_directory}"
        __ush_at 81 'arguments = [installer, "--bin-dir", directory, "--no-modify-path", "--download-url", "file://" + work + "/ush.tar.gz", "--checksum-url", "file://" + work + "/sha256sums.txt"]'
        __ush_assignment_25__len='8'
        # shellcheck disable=SC2034
        __ush_assignment_25__0="${__ush_binding_45_installer}"
        # shellcheck disable=SC2034
        __ush_assignment_25__1='--bin-dir'
        # shellcheck disable=SC2034
        __ush_assignment_25__2="${__ush_binding_68_directory}"
        # shellcheck disable=SC2034
        __ush_assignment_25__3='--no-modify-path'
        # shellcheck disable=SC2034
        __ush_assignment_25__4='--download-url'
        # shellcheck disable=SC2034
        __ush_assignment_25__5='file://'"${__ush_binding_54_work}"'/ush.tar.gz'
        # shellcheck disable=SC2034
        __ush_assignment_25__6='--checksum-url'
        # shellcheck disable=SC2034
        __ush_assignment_25__7='file://'"${__ush_binding_54_work}"'/sha256sums.txt'
        __ush_binding_69_arguments__len="${__ush_assignment_25__len}"
        __ush_copy_index_26=0
        while [ "$__ush_copy_index_26" -lt "${__ush_assignment_25__len}" ]; do
          eval "__ush_copy_value_27=\"\${__ush_assignment_25__$__ush_copy_index_26}\""
          eval "__ush_binding_69_arguments__${__ush_copy_index_26}=\$__ush_copy_value_27"
          __ush_copy_index_26=$((__ush_copy_index_26 + 1))
        done
      else
        __ush_at 82 '} else if scenario == "prefix" {'
        if {
        [ "$(if [ "${scenario}" = 'prefix' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        }; then
          __ush_at 83 'directory = home + "/prefix/bin"'
          __ush_binding_68_directory="${__ush_binding_56_home}"'/prefix/bin'
          # shellcheck disable=SC2034
          directory="${__ush_binding_68_directory}"
          __ush_at 84 'arguments = [installer, "--prefix", home + "/prefix", "--no-modify-path"]'
          __ush_assignment_28__len='4'
          # shellcheck disable=SC2034
          __ush_assignment_28__0="${__ush_binding_45_installer}"
          # shellcheck disable=SC2034
          __ush_assignment_28__1='--prefix'
          # shellcheck disable=SC2034
          __ush_assignment_28__2="${__ush_binding_56_home}"'/prefix'
          # shellcheck disable=SC2034
          __ush_assignment_28__3='--no-modify-path'
          __ush_binding_69_arguments__len="${__ush_assignment_28__len}"
          __ush_copy_index_29=0
          while [ "$__ush_copy_index_29" -lt "${__ush_assignment_28__len}" ]; do
            eval "__ush_copy_value_30=\"\${__ush_assignment_28__$__ush_copy_index_29}\""
            eval "__ush_binding_69_arguments__${__ush_copy_index_29}=\$__ush_copy_value_30"
            __ush_copy_index_29=$((__ush_copy_index_29 + 1))
          done
        else
          __ush_at 85 '} else if scenario == "relative-prefix" {'
          if {
          [ "$(if [ "${scenario}" = 'relative-prefix' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
          }; then
            __ush_at 86 'cd $home'
            command cd "${__ush_binding_56_home}"
            __ush_at 87 'directory = home + "/tools/bin"'
            __ush_binding_68_directory="${__ush_binding_56_home}"'/tools/bin'
            # shellcheck disable=SC2034
            directory="${__ush_binding_68_directory}"
            __ush_at 88 'arguments = [installer, "--prefix", "tools"]'
            __ush_assignment_31__len='3'
            # shellcheck disable=SC2034
            __ush_assignment_31__0="${__ush_binding_45_installer}"
            # shellcheck disable=SC2034
            __ush_assignment_31__1='--prefix'
            # shellcheck disable=SC2034
            __ush_assignment_31__2='tools'
            __ush_binding_69_arguments__len="${__ush_assignment_31__len}"
            __ush_copy_index_32=0
            while [ "$__ush_copy_index_32" -lt "${__ush_assignment_31__len}" ]; do
              eval "__ush_copy_value_33=\"\${__ush_assignment_31__$__ush_copy_index_32}\""
              eval "__ush_binding_69_arguments__${__ush_copy_index_32}=\$__ush_copy_value_33"
              __ush_copy_index_32=$((__ush_copy_index_32 + 1))
            done
          else
            __ush_at 89 '} else if scenario == "empty-env" {'
            if {
            [ "$(if [ "${scenario}" = 'empty-env' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
            }; then
              __ush_at 90 'set USH_AUTO_PATH ""'
              ush_fn_std__env__set 'USH_AUTO_PATH' ''
              __ush_at 91 'set UBSH_AUTO_PATH "0"'
              ush_fn_std__env__set 'UBSH_AUTO_PATH' '0'
            else
              __ush_at 92 '} else if scenario == "localhost" {'
              if {
              [ "$(if [ "${scenario}" = 'localhost' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
              }; then
                __ush_at 93 'set USH_DOWNLOAD_URL ("file://localhost" + work + "/ush.tar.gz")'
                ush_fn_std__env__set 'USH_DOWNLOAD_URL' 'file://localhost'"${__ush_binding_54_work}"'/ush.tar.gz'
                __ush_at 94 'arguments = [installer, "--no-modify-path"]'
                __ush_assignment_34__len='2'
                # shellcheck disable=SC2034
                __ush_assignment_34__0="${__ush_binding_45_installer}"
                # shellcheck disable=SC2034
                __ush_assignment_34__1='--no-modify-path'
                __ush_binding_69_arguments__len="${__ush_assignment_34__len}"
                __ush_copy_index_35=0
                while [ "$__ush_copy_index_35" -lt "${__ush_assignment_34__len}" ]; do
                  eval "__ush_copy_value_36=\"\${__ush_assignment_34__$__ush_copy_index_35}\""
                  eval "__ush_binding_69_arguments__${__ush_copy_index_35}=\$__ush_copy_value_36"
                  __ush_copy_index_35=$((__ush_copy_index_35 + 1))
                done
              else
                __ush_at 95 '} else if scenario == "checksum-name" {'
                if {
                [ "$(if [ "${scenario}" = 'checksum-name' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
                }; then
                  __ush_at 96 'write_text (work + "/sha256sums.txt") (sha256 (work + "/ush.tar.gz") + "  ush-" + target + ".tarXgz\n")'
                  __ush_value_37="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__sha256 "${__ush_binding_54_work}"'/ush.tar.gz')" || exit "$?"
                  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__write_text "${__ush_binding_54_work}"'/sha256sums.txt' "${__ush_value_37}"'  ush-'"${__ush_binding_46_target}"'.tarXgz'"${__ush_lf}"'' )
                  __ush_at 97 'expected = 1'
                  __ush_binding_70_expected=1
                  # shellcheck disable=SC2034
                  expected="${__ush_binding_70_expected}"
                else
                  __ush_at 98 '} else if scenario == "checksum-content" {'
                  if {
                  [ "$(if [ "${scenario}" = 'checksum-content' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
                  }; then
                    __ush_at 99 'write_text (work + "/sha256sums.txt") ("0000000000000000000000000000000000000000000000000000000000000000  ush-" + target + ".tar.gz\n")'
                    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__write_text "${__ush_binding_54_work}"'/sha256sums.txt' '0000000000000000000000000000000000000000000000000000000000000000  ush-'"${__ush_binding_46_target}"'.tar.gz'"${__ush_lf}"'' )
                    __ush_at 100 'expected = 1'
                    __ush_binding_70_expected=1
                    # shellcheck disable=SC2034
                    expected="${__ush_binding_70_expected}"
                  else
                    __ush_at 101 '} else if scenario == "profile-failure" {'
                    if {
                    [ "$(if [ "${scenario}" = 'profile-failure' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
                    }; then
                      __ush_at 102 'mkdir_p (home + "/.zshrc")'
                      ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__mkdir_p "${__ush_binding_56_home}"'/.zshrc' )
                      __ush_at 103 'expected = 1'
                      __ush_binding_70_expected=1
                      # shellcheck disable=SC2034
                      expected="${__ush_binding_70_expected}"
                    fi
                  fi
                fi
              fi
            fi
          fi
        fi
      fi
    fi
  fi
  __ush_at 105 'let code = redirect sh $arguments (work + "/out") (work + "/err")'
  __ush_match_38__len="${__ush_binding_69_arguments__len}"
  __ush_copy_index_39=0
  while [ "$__ush_copy_index_39" -lt "${__ush_binding_69_arguments__len}" ]; do
    eval "__ush_copy_value_40=\"\${__ush_binding_69_arguments__$__ush_copy_index_39}\""
    eval "__ush_match_38__${__ush_copy_index_39}=\$__ush_copy_value_40"
    __ush_copy_index_39=$((__ush_copy_index_39 + 1))
  done
  __ush_value_42="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect 'sh' "$(ush_fn_std__pack __ush_match_38)" "${__ush_binding_54_work}"'/out' "${__ush_binding_54_work}"'/err')" || exit "$?"
  __ush_binding_105_code="${__ush_value_42}"
  # shellcheck disable=SC2034
  code="${__ush_binding_105_code}"
  __ush_at 106 'if code != expected {'
  if {
  [ "$(if [ "${__ush_binding_105_code}" -ne "${__ush_binding_70_expected}" ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 107 'eprint (read_text (work + "/out"))'
    __ush_value_43="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_54_work}"'/out')" || exit "$?"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint "${__ush_value_43}" )
    __ush_at 108 'eprint (read_text (work + "/err"))'
    __ush_value_44="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_54_work}"'/err')" || exit "$?"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint "${__ush_value_44}" )
  fi
  __ush_at 110 'assert_eq $code $expected'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_binding_105_code}" "${__ush_binding_70_expected}" )
  __ush_at 111 'if expected == 0 {'
  if {
  [ "$(if [ "${__ush_binding_70_expected}" -eq 0 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 112 'assert (test -x (directory + "/ush"))'
    __ush_value_45="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-x' "${__ush_binding_68_directory}"'/ush')" || exit "$?"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_45}" )
    __ush_at 113 'assert (test -x (directory + "/ush_lsp"))'
    __ush_value_46="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-x' "${__ush_binding_68_directory}"'/ush_lsp')" || exit "$?"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_46}" )
    __ush_at 114 'assert (contains (read_text (work + "/out")) ("ush installed to " + directory + "/ush"))'
    __ush_value_47="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_54_work}"'/out')" || exit "$?"
    __ush_value_48="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_47}" 'ush installed to '"${__ush_binding_68_directory}"'/ush')" || exit "$?"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_48}" )
    __ush_at 115 'if scenario == "automatic" || scenario == "spaces" {'
    if {
    {
    [ "$(if [ "${scenario}" = 'automatic' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    } || {
    [ "$(if [ "${scenario}" = 'spaces' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }
    }; then
      __ush_at 116 'assert (contains (read_text (home + "/.zshrc")) "export PATH=\"$HOME/.local/bin:$PATH\"")'
      __ush_value_49="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_56_home}"'/.zshrc')" || exit "$?"
      __ush_value_50="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_49}" 'export PATH="$HOME/.local/bin:$PATH"')" || exit "$?"
      ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_50}" )
      __ush_at 117 'let original = read_text (home + "/.zshrc")'
      __ush_value_51="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_56_home}"'/.zshrc')" || exit "$?"
      __ush_binding_117_original="${__ush_value_51}"
      # shellcheck disable=SC2034
      original="${__ush_binding_117_original}"
      __ush_at 118 'assert_eq (redirect sh $arguments (work + "/out") (work + "/err")) 0'
      __ush_match_52__len="${__ush_binding_69_arguments__len}"
      __ush_copy_index_53=0
      while [ "$__ush_copy_index_53" -lt "${__ush_binding_69_arguments__len}" ]; do
        eval "__ush_copy_value_54=\"\${__ush_binding_69_arguments__$__ush_copy_index_53}\""
        eval "__ush_match_52__${__ush_copy_index_53}=\$__ush_copy_value_54"
        __ush_copy_index_53=$((__ush_copy_index_53 + 1))
      done
      __ush_value_56="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect 'sh' "$(ush_fn_std__pack __ush_match_52)" "${__ush_binding_54_work}"'/out' "${__ush_binding_54_work}"'/err')" || exit "$?"
      ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_56}" 0 )
      __ush_at 119 'assert_eq (read_text (home + "/.zshrc")) $original'
      __ush_value_57="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_56_home}"'/.zshrc')" || exit "$?"
      ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_57}" "${__ush_binding_117_original}" )
    else
      __ush_at 120 '} else if scenario == "relative-prefix" {'
      if {
      [ "$(if [ "${scenario}" = 'relative-prefix' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
      }; then
        __ush_at 121 'assert (contains (read_text (home + "/.zshrc")) "export PATH=\"$HOME/tools/bin:$PATH\"")'
        __ush_value_58="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_56_home}"'/.zshrc')" || exit "$?"
        __ush_value_59="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_58}" 'export PATH="$HOME/tools/bin:$PATH"')" || exit "$?"
        ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_59}" )
      else
        __ush_at 123 'assert_eq (is_file (home + "/.zshrc")) false'
        __ush_value_60="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${__ush_binding_56_home}"'/.zshrc')" || exit "$?"
        ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_60}" 'false' )
      fi
    fi
  else
    __ush_at 125 '} else if scenario == "checksum-name" || scenario == "checksum-content" {'
    if {
    {
    [ "$(if [ "${scenario}" = 'checksum-name' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    } || {
    [ "$(if [ "${scenario}" = 'checksum-content' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }
    }; then
      __ush_at 126 'assert_eq (is_file (directory + "/ush")) false'
      __ush_value_61="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${__ush_binding_68_directory}"'/ush')" || exit "$?"
      ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_61}" 'false' )
    fi
  fi
  __ush_at 128 'echo ("installer: " + scenario + " passed")'
  printf '%s\n' 'installer: '"${scenario}"' passed'
  __ush_at 129 'cd $root'
  command cd "${__ush_binding_49_root}"
done
__ush_at 131 'let work = temp_dir'
__ush_value_62="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
__ush_binding_131_work="${__ush_value_62}"
# shellcheck disable=SC2034
work="${__ush_binding_131_work}"
__ush_at 132 'assert_eq (redirect sh [installer, "--version"] (work + "/out") (work + "/err")) 1'
__ush_match_63__len='2'
# shellcheck disable=SC2034
__ush_match_63__0="${__ush_binding_45_installer}"
# shellcheck disable=SC2034
__ush_match_63__1='--version'
__ush_value_65="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect 'sh' "$(ush_fn_std__pack __ush_match_63)" "${__ush_binding_131_work}"'/out' "${__ush_binding_131_work}"'/err')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_65}" 1 )
__ush_at 133 'assert (contains (read_text (work + "/err")) "--version requires a value")'
__ush_value_66="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_131_work}"'/err')" || exit "$?"
__ush_value_67="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_66}" '--version requires a value')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_67}" )
__ush_at 134 'assert_eq (redirect sh [installer, "--unknown"] (work + "/out") (work + "/err")) 1'
__ush_match_68__len='2'
# shellcheck disable=SC2034
__ush_match_68__0="${__ush_binding_45_installer}"
# shellcheck disable=SC2034
__ush_match_68__1='--unknown'
__ush_value_70="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect 'sh' "$(ush_fn_std__pack __ush_match_68)" "${__ush_binding_131_work}"'/out' "${__ush_binding_131_work}"'/err')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_70}" 1 )
__ush_at 135 'for branch in ["", "__unset__"] {'
__ush_match_71__len='2'
# shellcheck disable=SC2034
__ush_match_71__0=''
# shellcheck disable=SC2034
__ush_match_71__1='__unset__'
__ush_for_list_index_72=0
while [ "${__ush_for_list_index_72}" -lt "${__ush_match_71__len}" ]; do
  __ush_for_value_73=''
  eval "__ush_for_value_73=\"\${__ush_match_71__${__ush_for_list_index_72}}\""
  branch="${__ush_for_value_73}"
  __ush_for_list_index_72=$((__ush_for_list_index_72 + 1))
  __ush_at 136 'set USH_BRANCH $branch'
  ush_fn_std__env__set 'USH_BRANCH' "${branch}"
  __ush_at 137 'assert_eq (redirect sh [installer] (work + "/out") (work + "/err")) 1'
  __ush_match_74__len='1'
  # shellcheck disable=SC2034
  __ush_match_74__0="${__ush_binding_45_installer}"
  __ush_value_76="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect 'sh' "$(ush_fn_std__pack __ush_match_74)" "${__ush_binding_131_work}"'/out' "${__ush_binding_131_work}"'/err')" || exit "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_76}" 1 )
  __ush_at 138 'assert (contains (read_text (work + "/err")) "USH_BRANCH/UBSH_BRANCH is no longer supported")'
  __ush_value_77="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_131_work}"'/err')" || exit "$?"
  __ush_value_78="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_77}" 'USH_BRANCH/UBSH_BRANCH is no longer supported')" || exit "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_78}" )
done
__ush_at 140 'unset USH_BRANCH'
ush_fn_std__env__unset 'USH_BRANCH'
__ush_at 141 'set TMPDIR (work + "/does-not-exist")'
ush_fn_std__env__set 'TMPDIR' "${__ush_binding_131_work}"'/does-not-exist'
__ush_at 142 'assert_eq (redirect sh [installer, "--help"] (work + "/out") (work + "/err")) 0'
__ush_match_79__len='2'
# shellcheck disable=SC2034
__ush_match_79__0="${__ush_binding_45_installer}"
# shellcheck disable=SC2034
__ush_match_79__1='--help'
__ush_value_81="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect 'sh' "$(ush_fn_std__pack __ush_match_79)" "${__ush_binding_131_work}"'/out' "${__ush_binding_131_work}"'/err')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_81}" 0 )
