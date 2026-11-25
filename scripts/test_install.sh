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
  __ush_at 12 'let mut os = uname "-s"'
  __ush_value_0="$(__ush_return_path='' __ush_capture_return='1' command uname '-s')" || return "$?"
  __ush_binding_12_os="${__ush_value_0}"
  # shellcheck disable=SC2034
  os="${__ush_binding_12_os}"
  __ush_at 13 'let mut arch = uname "-m"'
  __ush_value_1="$(__ush_return_path='' __ush_capture_return='1' command uname '-m')" || return "$?"
  __ush_binding_13_arch="${__ush_value_1}"
  # shellcheck disable=SC2034
  arch="${__ush_binding_13_arch}"
  __ush_at 14 'if os == "Linux" {'
  if {
  [ "$(if [ "${__ush_binding_12_os}" = 'Linux' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 15 'os = "unknown-linux-gnu"'
    __ush_binding_12_os='unknown-linux-gnu'
    # shellcheck disable=SC2034
    os="${__ush_binding_12_os}"
  else
    __ush_at 17 'else if os == "Darwin" {'
    if {
    [ "$(if [ "${__ush_binding_12_os}" = 'Darwin' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }; then
      __ush_at 18 'os = "apple-darwin"'
      __ush_binding_12_os='apple-darwin'
      # shellcheck disable=SC2034
      os="${__ush_binding_12_os}"
    else
      __ush_at 21 'fail 1 ("unsupported test OS: " + os)'
      __ush_status=1
      if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
      __ush_report "$__ush_status" 'unsupported test OS: '"${__ush_binding_12_os}"
      return "$__ush_status"
    fi
  fi
  __ush_at 23 'if arch == "arm64" {'
  if {
  [ "$(if [ "${__ush_binding_13_arch}" = 'arm64' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 24 'arch = "aarch64"'
    __ush_binding_13_arch='aarch64'
    # shellcheck disable=SC2034
    arch="${__ush_binding_13_arch}"
  else
    __ush_at 26 'else if arch == "amd64" {'
    if {
    [ "$(if [ "${__ush_binding_13_arch}" = 'amd64' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }; then
      __ush_at 27 'arch = "x86_64"'
      __ush_binding_13_arch='x86_64'
      # shellcheck disable=SC2034
      arch="${__ush_binding_13_arch}"
    fi
  fi
  __ush_at 29 'arch + "-" + os'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_binding_13_arch}"'-'"${__ush_binding_12_os}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_binding_13_arch}"'-'"${__ush_binding_12_os}" >&4
  fi
  return 0
}

# raises: unknown
ush_fn_fixture_binary() {
  __ush_at 33 'for directory in split (get "PATH") ":" {'
  __ush_value_3="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'PATH')" || return "$?"
  __ush_packed_4="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__split "${__ush_value_3}" ':')" || return "$?"
  __ush_match_2__len=0
  ush_fn_std__unpack __ush_match_2 "${__ush_packed_4}"
  __ush_for_list_index_5=0
  while [ "${__ush_for_list_index_5}" -lt "${__ush_match_2__len}" ]; do
    __ush_for_value_6=''
    eval "__ush_for_value_6=\"\${__ush_match_2__${__ush_for_list_index_5}}\""
    directory="${__ush_for_value_6}"
    __ush_for_list_index_5=$((__ush_for_list_index_5 + 1))
    __ush_at 34 'let candidate = directory + "/true"'
    __ush_binding_34_candidate="${directory}"'/true'
    # shellcheck disable=SC2034
    candidate="${__ush_binding_34_candidate}"
    __ush_at 35 'if test "-f" candidate && test "-x" candidate {'
    if {
    {
    __ush_value_7="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-f' "${__ush_binding_34_candidate}")" || return "$?"
    [ "${__ush_value_7}" = 'true' ]
    } && {
    __ush_value_8="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-x' "${__ush_binding_34_candidate}")" || return "$?"
    [ "${__ush_value_8}" = 'true' ]
    }
    }; then
      __ush_at 36 'return candidate'
      if [ -n "${__ush_return_path:-}" ]; then
        printf '%s' "${__ush_binding_34_candidate}" > "$__ush_return_path"
      elif [ "${__ush_capture_return:-0}" = '1' ]; then
        printf '%s' "${__ush_binding_34_candidate}" >&4
      fi
      return 0
    fi
  done
  __ush_at 39 'fail 1 "test_install: the external true utility is required on PATH"'
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
  __ush_at 43 'let package = work + "/ush-" + target'
  __ush_binding_43_package="${__ush_fn_archive_arg_0}"'/ush-'"${__ush_fn_archive_arg_1}"
  # shellcheck disable=SC2034
  package="${__ush_binding_43_package}"
  __ush_at 44 'mkdir_p package'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__mkdir_p "${__ush_binding_43_package}" ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 46 'cp fixture (package + "/ush")'
  command cp "${__ush_fn_archive_arg_2}" "${__ush_binding_43_package}"'/ush' || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 47 'cp fixture (package + "/ush_lsp")'
  command cp "${__ush_fn_archive_arg_2}" "${__ush_binding_43_package}"'/ush_lsp' || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 48 'tar "-czf" (work + "/ush.tar.gz") "-C" work ("ush-" + target)'
  command tar '-czf' "${__ush_fn_archive_arg_0}"'/ush.tar.gz' '-C' "${__ush_fn_archive_arg_0}" 'ush-'"${__ush_fn_archive_arg_1}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 49 'write_text (work + "/sha256sums.txt") (sha256 (work + "/ush.tar.gz") + "  ush-" + target + ".tar.gz\n")'
  __ush_value_9="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__sha256 "${__ush_fn_archive_arg_0}"'/ush.tar.gz')" || return "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__write_text "${__ush_fn_archive_arg_0}"'/sha256sums.txt' "${__ush_value_9}"'  ush-'"${__ush_fn_archive_arg_1}"'.tar.gz'"${__ush_lf}"'' ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
}

__ush_at 52 'let installer = resolve (source_root + "/../install.sh")'
__ush_value_10="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__source_root)" || exit "$?"
__ush_value_11="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__resolve "${__ush_value_10}"'/../install.sh')" || exit "$?"
__ush_binding_52_installer="${__ush_value_11}"
# shellcheck disable=SC2034
installer="${__ush_binding_52_installer}"
__ush_at 53 'let target = target'
__ush_value_12="$(__ush_return_path='' __ush_capture_return='1' ush_fn_target 4>&1 >&3)" || exit "$?"
__ush_binding_53_target="${__ush_value_12}"
# shellcheck disable=SC2034
target="${__ush_binding_53_target}"
__ush_at 54 'let tools_path = get "PATH"'
__ush_value_13="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'PATH')" || exit "$?"
__ush_binding_54_tools_path="${__ush_value_13}"
# shellcheck disable=SC2034
tools_path="${__ush_binding_54_tools_path}"
__ush_at 55 'let fixture = fixture_binary'
__ush_value_14="$(__ush_return_path='' __ush_capture_return='1' ush_fn_fixture_binary 4>&1 >&3)" || exit "$?"
__ush_binding_55_fixture="${__ush_value_14}"
# shellcheck disable=SC2034
fixture="${__ush_binding_55_fixture}"
__ush_at 56 'let root = std::path::cwd'
__ush_value_15="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__cwd)" || exit "$?"
__ush_binding_56_root="${__ush_value_15}"
# shellcheck disable=SC2034
root="${__ush_binding_56_root}"
__ush_at 57 'for name in ["USH_PREFIX", "UBSH_PREFIX", "USH_BIN_DIR", "UBSH_BIN_DIR", "USH_VERSION", "UBSH_VERSION", "USH_BRANCH", "UBSH_BRANCH", "UBSH_AUTO_PATH"] {'
__ush_match_16__len='9'
# shellcheck disable=SC2034
__ush_match_16__0='USH_PREFIX'
# shellcheck disable=SC2034
__ush_match_16__1='UBSH_PREFIX'
# shellcheck disable=SC2034
__ush_match_16__2='USH_BIN_DIR'
# shellcheck disable=SC2034
__ush_match_16__3='UBSH_BIN_DIR'
# shellcheck disable=SC2034
__ush_match_16__4='USH_VERSION'
# shellcheck disable=SC2034
__ush_match_16__5='UBSH_VERSION'
# shellcheck disable=SC2034
__ush_match_16__6='USH_BRANCH'
# shellcheck disable=SC2034
__ush_match_16__7='UBSH_BRANCH'
# shellcheck disable=SC2034
__ush_match_16__8='UBSH_AUTO_PATH'
__ush_for_list_index_17=0
while [ "${__ush_for_list_index_17}" -lt "${__ush_match_16__len}" ]; do
  __ush_for_value_18=''
  eval "__ush_for_value_18=\"\${__ush_match_16__${__ush_for_list_index_17}}\""
  name="${__ush_for_value_18}"
  __ush_for_list_index_17=$((__ush_for_list_index_17 + 1))
  __ush_at 58 'unset name'
  ush_fn_std__env__unset "${name}"
done
__ush_at 60 'for scenario in ["existing", "automatic", "disabled", "flags", "prefix", "relative-prefix", "empty-env", "localhost", "checksum-name", "checksum-content", "profile-failure", "spaces"] {'
__ush_task_init
__ush_at 60 'for scenario in ["existing", "automatic", "disabled", "flags", "prefix", "relative-prefix", "empty-env", "localhost", "checksum-name", "checksum-content", "profile-failure", "spaces"] {'
__ush_match_19__len='12'
# shellcheck disable=SC2034
__ush_match_19__0='existing'
# shellcheck disable=SC2034
__ush_match_19__1='automatic'
# shellcheck disable=SC2034
__ush_match_19__2='disabled'
# shellcheck disable=SC2034
__ush_match_19__3='flags'
# shellcheck disable=SC2034
__ush_match_19__4='prefix'
# shellcheck disable=SC2034
__ush_match_19__5='relative-prefix'
# shellcheck disable=SC2034
__ush_match_19__6='empty-env'
# shellcheck disable=SC2034
__ush_match_19__7='localhost'
# shellcheck disable=SC2034
__ush_match_19__8='checksum-name'
# shellcheck disable=SC2034
__ush_match_19__9='checksum-content'
# shellcheck disable=SC2034
__ush_match_19__10='profile-failure'
# shellcheck disable=SC2034
__ush_match_19__11='spaces'
__ush_for_list_index_20=0
while [ "${__ush_for_list_index_20}" -lt "${__ush_match_19__len}" ]; do
  __ush_for_value_21=''
  eval "__ush_for_value_21=\"\${__ush_match_19__${__ush_for_list_index_20}}\""
  scenario="${__ush_for_value_21}"
  __ush_for_list_index_20=$((__ush_for_list_index_20 + 1))
  __ush_at 61 'let work = temp_dir'
  __ush_value_22="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
  __ush_binding_61_work="${__ush_value_22}"
  # shellcheck disable=SC2034
  work="${__ush_binding_61_work}"
  __ush_at 62 'archive work target fixture'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_archive "${__ush_binding_61_work}" "${__ush_binding_53_target}" "${__ush_binding_55_fixture}" )
  __ush_at 63 'let mut home = work + "/home"'
  __ush_binding_63_home="${__ush_binding_61_work}"'/home'
  # shellcheck disable=SC2034
  home="${__ush_binding_63_home}"
  __ush_at 64 'if scenario == "spaces" {'
  if {
  [ "$(if [ "${scenario}" = 'spaces' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 65 'home = work + "/home with spaces '"'"'$value"'
    __ush_binding_63_home="${__ush_binding_61_work}"'/home with spaces '"'"'$value'
    # shellcheck disable=SC2034
    home="${__ush_binding_63_home}"
  fi
  __ush_at 67 'mkdir_p home'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__mkdir_p "${__ush_binding_63_home}" )
  __ush_at 68 'set "HOME" home'
  ush_fn_std__env__set 'HOME' "${__ush_binding_63_home}"
  __ush_at 69 'set "PATH" tools_path'
  ush_fn_std__env__set 'PATH' "${__ush_binding_54_tools_path}"
  __ush_at 70 'set "SHELL" "/bin/zsh"'
  ush_fn_std__env__set 'SHELL' '/bin/zsh'
  __ush_at 71 'set "USH_AUTO_PATH" "1"'
  ush_fn_std__env__set 'USH_AUTO_PATH' '1'
  __ush_at 72 'unset "UBSH_AUTO_PATH"'
  ush_fn_std__env__unset 'UBSH_AUTO_PATH'
  __ush_at 73 'set "USH_DOWNLOAD_URL" ("file://" + work + "/ush.tar.gz")'
  ush_fn_std__env__set 'USH_DOWNLOAD_URL' 'file://'"${__ush_binding_61_work}"'/ush.tar.gz'
  __ush_at 74 'set "USH_CHECKSUM_URL" ("file://" + work + "/sha256sums.txt")'
  ush_fn_std__env__set 'USH_CHECKSUM_URL' 'file://'"${__ush_binding_61_work}"'/sha256sums.txt'
  __ush_at 75 'let mut directory = home + "/.local/bin"'
  __ush_binding_75_directory="${__ush_binding_63_home}"'/.local/bin'
  # shellcheck disable=SC2034
  directory="${__ush_binding_75_directory}"
  __ush_at 76 'let mut arguments = [installer]'
  __ush_binding_76_arguments__len='1'
  # shellcheck disable=SC2034
  __ush_binding_76_arguments__0="${__ush_binding_52_installer}"
  __ush_at 77 'let mut expected = 0'
  __ush_binding_77_expected=0
  # shellcheck disable=SC2034
  expected="${__ush_binding_77_expected}"
  __ush_at 78 'if scenario == "existing" {'
  if {
  [ "$(if [ "${scenario}" = 'existing' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 79 'directory = home + "/bin"'
    __ush_binding_75_directory="${__ush_binding_63_home}"'/bin'
    # shellcheck disable=SC2034
    directory="${__ush_binding_75_directory}"
    __ush_at 80 'mkdir_p directory'
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__mkdir_p "${__ush_binding_75_directory}" )
    __ush_at 81 'set "PATH" (directory + ":" + tools_path)'
    ush_fn_std__env__set 'PATH' "${__ush_binding_75_directory}"':'"${__ush_binding_54_tools_path}"
  else
    __ush_at 83 'else if scenario == "disabled" {'
    if {
    [ "$(if [ "${scenario}" = 'disabled' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }; then
      __ush_at 84 'set "USH_AUTO_PATH" "0"'
      ush_fn_std__env__set 'USH_AUTO_PATH' '0'
    else
      __ush_at 86 'else if scenario == "flags" {'
      if {
      [ "$(if [ "${scenario}" = 'flags' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
      }; then
        __ush_at 87 'set "USH_DOWNLOAD_URL" ("file://" + work + "/missing-archive")'
        ush_fn_std__env__set 'USH_DOWNLOAD_URL' 'file://'"${__ush_binding_61_work}"'/missing-archive'
        __ush_at 88 'set "USH_CHECKSUM_URL" ("file://" + work + "/missing-sums")'
        ush_fn_std__env__set 'USH_CHECKSUM_URL' 'file://'"${__ush_binding_61_work}"'/missing-sums'
        __ush_at 89 'directory = home + "/tools/bin"'
        __ush_binding_75_directory="${__ush_binding_63_home}"'/tools/bin'
        # shellcheck disable=SC2034
        directory="${__ush_binding_75_directory}"
        __ush_at 90 'arguments = [installer, "--bin-dir", directory, "--no-modify-path", "--download-url", "file://" + work + "/ush.tar.gz", "--checksum-url", "file://" + work + "/sha256sums.txt"]'
        __ush_assignment_23__len='8'
        # shellcheck disable=SC2034
        __ush_assignment_23__0="${__ush_binding_52_installer}"
        # shellcheck disable=SC2034
        __ush_assignment_23__1='--bin-dir'
        # shellcheck disable=SC2034
        __ush_assignment_23__2="${__ush_binding_75_directory}"
        # shellcheck disable=SC2034
        __ush_assignment_23__3='--no-modify-path'
        # shellcheck disable=SC2034
        __ush_assignment_23__4='--download-url'
        # shellcheck disable=SC2034
        __ush_assignment_23__5='file://'"${__ush_binding_61_work}"'/ush.tar.gz'
        # shellcheck disable=SC2034
        __ush_assignment_23__6='--checksum-url'
        # shellcheck disable=SC2034
        __ush_assignment_23__7='file://'"${__ush_binding_61_work}"'/sha256sums.txt'
        __ush_binding_76_arguments__len="${__ush_assignment_23__len}"
        __ush_copy_index_24=0
        while [ "$__ush_copy_index_24" -lt "${__ush_assignment_23__len}" ]; do
          eval "__ush_copy_value_25=\"\${__ush_assignment_23__$__ush_copy_index_24}\""
          eval "__ush_binding_76_arguments__${__ush_copy_index_24}=\$__ush_copy_value_25"
          __ush_copy_index_24=$((__ush_copy_index_24 + 1))
        done
      else
        __ush_at 92 'else if scenario == "prefix" {'
        if {
        [ "$(if [ "${scenario}" = 'prefix' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        }; then
          __ush_at 93 'directory = home + "/prefix/bin"'
          __ush_binding_75_directory="${__ush_binding_63_home}"'/prefix/bin'
          # shellcheck disable=SC2034
          directory="${__ush_binding_75_directory}"
          __ush_at 94 'arguments = [installer, "--prefix", home + "/prefix", "--no-modify-path"]'
          __ush_assignment_26__len='4'
          # shellcheck disable=SC2034
          __ush_assignment_26__0="${__ush_binding_52_installer}"
          # shellcheck disable=SC2034
          __ush_assignment_26__1='--prefix'
          # shellcheck disable=SC2034
          __ush_assignment_26__2="${__ush_binding_63_home}"'/prefix'
          # shellcheck disable=SC2034
          __ush_assignment_26__3='--no-modify-path'
          __ush_binding_76_arguments__len="${__ush_assignment_26__len}"
          __ush_copy_index_27=0
          while [ "$__ush_copy_index_27" -lt "${__ush_assignment_26__len}" ]; do
            eval "__ush_copy_value_28=\"\${__ush_assignment_26__$__ush_copy_index_27}\""
            eval "__ush_binding_76_arguments__${__ush_copy_index_27}=\$__ush_copy_value_28"
            __ush_copy_index_27=$((__ush_copy_index_27 + 1))
          done
        else
          __ush_at 96 'else if scenario == "relative-prefix" {'
          if {
          [ "$(if [ "${scenario}" = 'relative-prefix' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
          }; then
            __ush_at 97 'cd home'
            command cd "${__ush_binding_63_home}"
            __ush_at 98 'directory = home + "/tools/bin"'
            __ush_binding_75_directory="${__ush_binding_63_home}"'/tools/bin'
            # shellcheck disable=SC2034
            directory="${__ush_binding_75_directory}"
            __ush_at 99 'arguments = [installer, "--prefix", "tools"]'
            __ush_assignment_29__len='3'
            # shellcheck disable=SC2034
            __ush_assignment_29__0="${__ush_binding_52_installer}"
            # shellcheck disable=SC2034
            __ush_assignment_29__1='--prefix'
            # shellcheck disable=SC2034
            __ush_assignment_29__2='tools'
            __ush_binding_76_arguments__len="${__ush_assignment_29__len}"
            __ush_copy_index_30=0
            while [ "$__ush_copy_index_30" -lt "${__ush_assignment_29__len}" ]; do
              eval "__ush_copy_value_31=\"\${__ush_assignment_29__$__ush_copy_index_30}\""
              eval "__ush_binding_76_arguments__${__ush_copy_index_30}=\$__ush_copy_value_31"
              __ush_copy_index_30=$((__ush_copy_index_30 + 1))
            done
          else
            __ush_at 101 'else if scenario == "empty-env" {'
            if {
            [ "$(if [ "${scenario}" = 'empty-env' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
            }; then
              __ush_at 102 'set "USH_AUTO_PATH" ""'
              ush_fn_std__env__set 'USH_AUTO_PATH' ''
              __ush_at 103 'set "UBSH_AUTO_PATH" "0"'
              ush_fn_std__env__set 'UBSH_AUTO_PATH' '0'
            else
              __ush_at 105 'else if scenario == "localhost" {'
              if {
              [ "$(if [ "${scenario}" = 'localhost' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
              }; then
                __ush_at 106 'set "USH_DOWNLOAD_URL" ("file://localhost" + work + "/ush.tar.gz")'
                ush_fn_std__env__set 'USH_DOWNLOAD_URL' 'file://localhost'"${__ush_binding_61_work}"'/ush.tar.gz'
                __ush_at 107 'arguments = [installer, "--no-modify-path"]'
                __ush_assignment_32__len='2'
                # shellcheck disable=SC2034
                __ush_assignment_32__0="${__ush_binding_52_installer}"
                # shellcheck disable=SC2034
                __ush_assignment_32__1='--no-modify-path'
                __ush_binding_76_arguments__len="${__ush_assignment_32__len}"
                __ush_copy_index_33=0
                while [ "$__ush_copy_index_33" -lt "${__ush_assignment_32__len}" ]; do
                  eval "__ush_copy_value_34=\"\${__ush_assignment_32__$__ush_copy_index_33}\""
                  eval "__ush_binding_76_arguments__${__ush_copy_index_33}=\$__ush_copy_value_34"
                  __ush_copy_index_33=$((__ush_copy_index_33 + 1))
                done
              else
                __ush_at 109 'else if scenario == "checksum-name" {'
                if {
                [ "$(if [ "${scenario}" = 'checksum-name' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
                }; then
                  __ush_at 110 'write_text (work + "/sha256sums.txt") (sha256 (work + "/ush.tar.gz") + "  ush-" + target + ".tarXgz\n")'
                  __ush_value_35="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__sha256 "${__ush_binding_61_work}"'/ush.tar.gz')" || exit "$?"
                  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__write_text "${__ush_binding_61_work}"'/sha256sums.txt' "${__ush_value_35}"'  ush-'"${__ush_binding_53_target}"'.tarXgz'"${__ush_lf}"'' )
                  __ush_at 111 'expected = 1'
                  __ush_binding_77_expected=1
                  # shellcheck disable=SC2034
                  expected="${__ush_binding_77_expected}"
                else
                  __ush_at 113 'else if scenario == "checksum-content" {'
                  if {
                  [ "$(if [ "${scenario}" = 'checksum-content' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
                  }; then
                    __ush_at 114 'write_text (work + "/sha256sums.txt") ("0000000000000000000000000000000000000000000000000000000000000000  ush-" + target + ".tar.gz\n")'
                    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__write_text "${__ush_binding_61_work}"'/sha256sums.txt' '0000000000000000000000000000000000000000000000000000000000000000  ush-'"${__ush_binding_53_target}"'.tar.gz'"${__ush_lf}"'' )
                    __ush_at 115 'expected = 1'
                    __ush_binding_77_expected=1
                    # shellcheck disable=SC2034
                    expected="${__ush_binding_77_expected}"
                  else
                    __ush_at 117 'else if scenario == "profile-failure" {'
                    if {
                    [ "$(if [ "${scenario}" = 'profile-failure' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
                    }; then
                      __ush_at 118 'mkdir_p (home + "/.zshrc")'
                      ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__mkdir_p "${__ush_binding_63_home}"'/.zshrc' )
                      __ush_at 119 'expected = 1'
                      __ush_binding_77_expected=1
                      # shellcheck disable=SC2034
                      expected="${__ush_binding_77_expected}"
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
  __ush_at 121 'let code = redirect "sh" arguments (work + "/out") (work + "/err")'
  __ush_match_36__len="${__ush_binding_76_arguments__len}"
  __ush_copy_index_37=0
  while [ "$__ush_copy_index_37" -lt "${__ush_binding_76_arguments__len}" ]; do
    eval "__ush_copy_value_38=\"\${__ush_binding_76_arguments__$__ush_copy_index_37}\""
    eval "__ush_match_36__${__ush_copy_index_37}=\$__ush_copy_value_38"
    __ush_copy_index_37=$((__ush_copy_index_37 + 1))
  done
  __ush_value_40="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect 'sh' "$(ush_fn_std__pack __ush_match_36)" "${__ush_binding_61_work}"'/out' "${__ush_binding_61_work}"'/err')" || exit "$?"
  __ush_binding_121_code="${__ush_value_40}"
  # shellcheck disable=SC2034
  code="${__ush_binding_121_code}"
  __ush_at 122 'if code != expected {'
  if {
  [ "$(if [ "${__ush_binding_121_code}" -ne "${__ush_binding_77_expected}" ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 123 'eprint (read_text (work + "/out"))'
    __ush_value_41="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_61_work}"'/out')" || exit "$?"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint "${__ush_value_41}" )
    __ush_at 124 'eprint (read_text (work + "/err"))'
    __ush_value_42="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_61_work}"'/err')" || exit "$?"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint "${__ush_value_42}" )
  fi
  __ush_at 126 'assert_eq code expected'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_binding_121_code}" "${__ush_binding_77_expected}" )
  __ush_at 127 'if expected == 0 {'
  if {
  [ "$(if [ "${__ush_binding_77_expected}" -eq 0 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 128 'assert (test "-x" (directory + "/ush"))'
    __ush_value_43="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-x' "${__ush_binding_75_directory}"'/ush')" || exit "$?"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_43}" )
    __ush_at 129 'assert (test "-x" (directory + "/ush_lsp"))'
    __ush_value_44="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-x' "${__ush_binding_75_directory}"'/ush_lsp')" || exit "$?"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_44}" )
    __ush_at 130 'assert (contains (read_text (work + "/out")) ("ush installed to " + directory + "/ush"))'
    __ush_value_45="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_61_work}"'/out')" || exit "$?"
    __ush_value_46="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_45}" 'ush installed to '"${__ush_binding_75_directory}"'/ush')" || exit "$?"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_46}" )
    __ush_at 131 'if scenario == "automatic" || scenario == "spaces" {'
    if {
    {
    [ "$(if [ "${scenario}" = 'automatic' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    } || {
    [ "$(if [ "${scenario}" = 'spaces' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }
    }; then
      __ush_at 132 'assert (contains (read_text (home + "/.zshrc")) "export PATH=\"$HOME/.local/bin:$PATH\"")'
      __ush_value_47="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_63_home}"'/.zshrc')" || exit "$?"
      __ush_value_48="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_47}" 'export PATH="$HOME/.local/bin:$PATH"')" || exit "$?"
      ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_48}" )
      __ush_at 133 'let original = read_text (home + "/.zshrc")'
      __ush_value_49="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_63_home}"'/.zshrc')" || exit "$?"
      __ush_binding_133_original="${__ush_value_49}"
      # shellcheck disable=SC2034
      original="${__ush_binding_133_original}"
      __ush_at 134 'assert_eq (redirect "sh" arguments (work + "/out") (work + "/err")) 0'
      __ush_match_50__len="${__ush_binding_76_arguments__len}"
      __ush_copy_index_51=0
      while [ "$__ush_copy_index_51" -lt "${__ush_binding_76_arguments__len}" ]; do
        eval "__ush_copy_value_52=\"\${__ush_binding_76_arguments__$__ush_copy_index_51}\""
        eval "__ush_match_50__${__ush_copy_index_51}=\$__ush_copy_value_52"
        __ush_copy_index_51=$((__ush_copy_index_51 + 1))
      done
      __ush_value_54="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect 'sh' "$(ush_fn_std__pack __ush_match_50)" "${__ush_binding_61_work}"'/out' "${__ush_binding_61_work}"'/err')" || exit "$?"
      ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_54}" 0 )
      __ush_at 135 'assert_eq (read_text (home + "/.zshrc")) original'
      __ush_value_55="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_63_home}"'/.zshrc')" || exit "$?"
      ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_55}" "${__ush_binding_133_original}" )
    else
      __ush_at 137 'else if scenario == "relative-prefix" {'
      if {
      [ "$(if [ "${scenario}" = 'relative-prefix' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
      }; then
        __ush_at 138 'assert (contains (read_text (home + "/.zshrc")) "export PATH=\"$HOME/tools/bin:$PATH\"")'
        __ush_value_56="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_63_home}"'/.zshrc')" || exit "$?"
        __ush_value_57="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_56}" 'export PATH="$HOME/tools/bin:$PATH"')" || exit "$?"
        ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_57}" )
      else
        __ush_at 141 'assert_eq (is_file (home + "/.zshrc")) false'
        __ush_value_58="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${__ush_binding_63_home}"'/.zshrc')" || exit "$?"
        ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_58}" 'false' )
      fi
    fi
  else
    __ush_at 144 'else if scenario == "checksum-name" || scenario == "checksum-content" {'
    if {
    {
    [ "$(if [ "${scenario}" = 'checksum-name' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    } || {
    [ "$(if [ "${scenario}" = 'checksum-content' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }
    }; then
      __ush_at 145 'assert_eq (is_file (directory + "/ush")) false'
      __ush_value_59="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${__ush_binding_75_directory}"'/ush')" || exit "$?"
      ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_59}" 'false' )
    fi
  fi
  __ush_at 147 'print "installer: " + scenario + " passed"'
  printf '%s\n' 'installer: '"${scenario}"' passed'
  __ush_at 148 'cd root'
  command cd "${__ush_binding_56_root}"
done
__ush_at 150 'let work = temp_dir'
__ush_value_60="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
__ush_binding_150_work="${__ush_value_60}"
# shellcheck disable=SC2034
work="${__ush_binding_150_work}"
__ush_at 151 'assert_eq (redirect "sh" [installer, "--version"] (work + "/out") (work + "/err")) 1'
__ush_match_61__len='2'
# shellcheck disable=SC2034
__ush_match_61__0="${__ush_binding_52_installer}"
# shellcheck disable=SC2034
__ush_match_61__1='--version'
__ush_value_63="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect 'sh' "$(ush_fn_std__pack __ush_match_61)" "${__ush_binding_150_work}"'/out' "${__ush_binding_150_work}"'/err')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_63}" 1 )
__ush_at 152 'assert (contains (read_text (work + "/err")) "--version requires a value")'
__ush_value_64="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_150_work}"'/err')" || exit "$?"
__ush_value_65="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_64}" '--version requires a value')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_65}" )
__ush_at 153 'assert_eq (redirect "sh" [installer, "--unknown"] (work + "/out") (work + "/err")) 1'
__ush_match_66__len='2'
# shellcheck disable=SC2034
__ush_match_66__0="${__ush_binding_52_installer}"
# shellcheck disable=SC2034
__ush_match_66__1='--unknown'
__ush_value_68="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect 'sh' "$(ush_fn_std__pack __ush_match_66)" "${__ush_binding_150_work}"'/out' "${__ush_binding_150_work}"'/err')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_68}" 1 )
__ush_at 154 'for branch in ["", "__unset__"] {'
__ush_match_69__len='2'
# shellcheck disable=SC2034
__ush_match_69__0=''
# shellcheck disable=SC2034
__ush_match_69__1='__unset__'
__ush_for_list_index_70=0
while [ "${__ush_for_list_index_70}" -lt "${__ush_match_69__len}" ]; do
  __ush_for_value_71=''
  eval "__ush_for_value_71=\"\${__ush_match_69__${__ush_for_list_index_70}}\""
  branch="${__ush_for_value_71}"
  __ush_for_list_index_70=$((__ush_for_list_index_70 + 1))
  __ush_at 155 'set "USH_BRANCH" branch'
  ush_fn_std__env__set 'USH_BRANCH' "${branch}"
  __ush_at 156 'assert_eq (redirect "sh" [installer] (work + "/out") (work + "/err")) 1'
  __ush_match_72__len='1'
  # shellcheck disable=SC2034
  __ush_match_72__0="${__ush_binding_52_installer}"
  __ush_value_74="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect 'sh' "$(ush_fn_std__pack __ush_match_72)" "${__ush_binding_150_work}"'/out' "${__ush_binding_150_work}"'/err')" || exit "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_74}" 1 )
  __ush_at 157 'assert (contains (read_text (work + "/err")) "USH_BRANCH/UBSH_BRANCH is no longer supported")'
  __ush_value_75="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_150_work}"'/err')" || exit "$?"
  __ush_value_76="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_75}" 'USH_BRANCH/UBSH_BRANCH is no longer supported')" || exit "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert "${__ush_value_76}" )
done
__ush_at 159 'unset "USH_BRANCH"'
ush_fn_std__env__unset 'USH_BRANCH'
__ush_at 160 'set "TMPDIR" (work + "/does-not-exist")'
ush_fn_std__env__set 'TMPDIR' "${__ush_binding_150_work}"'/does-not-exist'
__ush_at 161 'assert_eq (redirect "sh" [installer, "--help"] (work + "/out") (work + "/err")) 0'
__ush_match_77__len='2'
# shellcheck disable=SC2034
__ush_match_77__0="${__ush_binding_52_installer}"
# shellcheck disable=SC2034
__ush_match_77__1='--help'
__ush_value_79="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect 'sh' "$(ush_fn_std__pack __ush_match_77)" "${__ush_binding_150_work}"'/out' "${__ush_binding_150_work}"'/err')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_79}" 0 )
