#!/bin/sh
# Generated from 'install.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: install.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'install.ush' --checked-in -o 'install.sh'
set -eu

__ush_lf='
'
__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'install.ush'
__ush_origin='install.ush'

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

ush_fn_std__env__contains() {
  __ush_std_env_validate "$1" || return "$?"
  __ush_present=''
  eval "__ush_present=\${$1+x}"
  if [ "$__ush_present" = x ]; then printf true; else printf false; fi
}

ush_fn_std__fs__read_text() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  cat "$__ush_path"
}

ush_fn_std__fs__append_text() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  printf '%s' "$2" >> "$__ush_path"
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

ush_fn_std__path__resolve() {
  __ush_std_path_resolve "$1"
}

ush_fn_std__path__basename() {
  basename "$(__ush_std_path_resolve "$1")"
}

ush_fn_std__command__exists() {
  if command -v "$1" >/dev/null 2>&1; then printf '%s' 'true'; else printf '%s' 'false'; fi
}

ush_fn_std__string__starts_with() {
  VALUE="$1" PREFIX="$2" awk 'BEGIN { value = ENVIRON["VALUE"]; prefix = ENVIRON["PREFIX"]; if (substr(value, 1, length(prefix)) == prefix) { printf "true" } else { printf "false" } }'
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

__ush_std_encode_args() {
  for __ush_arg do
    printf x
    printf '%s' "$__ush_arg" | od -A n -v -t x1 | tr -d ' \n'
    printf ' '
  done
}

ush_fn_std__process__args() {
  # Keep the encoder linked even when args is the only process helper used.
  if [ "${__ush_argv+x}" != x ]; then __ush_std_encode_args; else printf '%s' "$__ush_argv"; fi
}

ush_fn_std__process__exit() {
  case $1 in ''|*[!0-9]*) return 1 ;; esac
  [ "$1" -le 255 ] || return 1
  exit "$1"
}

ush_fn_std__io__eprint() {
  printf '%s\n' "$1" >&2
}

ush_fn_std__fs__temp_dir() {
  mktemp -d "$__ush_task_dir/tmp.XXXXXXXXXX"
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

ush_fn_std__string__contains() {
  case $1 in *"$2"*) printf true ;; *) printf false ;; esac
}

__ush_argv="$(__ush_std_encode_args "$@")"
ush_fn_option() {
  __ush_fn_option_arg_0="${1}"
  # shellcheck disable=SC2034
  name="${__ush_fn_option_arg_0}"
  __ush_fn_option_arg_1="${2}"
  # shellcheck disable=SC2034
  legacy="${__ush_fn_option_arg_1}"
  __ush_fn_option_arg_2="${3}"
  # shellcheck disable=SC2034
  fallback="${__ush_fn_option_arg_2}"
  __ush_at 11 'let mut value = get $name'
  __ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get "${__ush_fn_option_arg_0}")" || return "$?"
  __ush_binding_11_value="${__ush_value_0}"
  # shellcheck disable=SC2034
  value="${__ush_binding_11_value}"
  __ush_at 12 'if value != "" {'
  if {
  [ "$(if [ "${__ush_binding_11_value}" != '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 13 'return value'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${__ush_binding_11_value}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${__ush_binding_11_value}" >&4
    fi
    return 0
  fi
  __ush_at 15 'value = get $legacy'
  __ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get "${__ush_fn_option_arg_1}")" || return "$?"
  __ush_binding_11_value="${__ush_value_1}"
  # shellcheck disable=SC2034
  value="${__ush_binding_11_value}"
  __ush_at 16 'if value != "" {'
  if {
  [ "$(if [ "${__ush_binding_11_value}" != '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 17 'return value'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${__ush_binding_11_value}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${__ush_binding_11_value}" >&4
    fi
    return 0
  fi
  __ush_at 19 'fallback'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_fn_option_arg_2}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_fn_option_arg_2}" >&4
  fi
  return 0
}

ush_fn_usage() {
  __ush_at 23 'echo "usage: install.sh [--version VERSION] [--bin-dir DIR] [--prefix DIR]"'
  printf '%s\n' 'usage: install.sh [--version VERSION] [--bin-dir DIR] [--prefix DIR]' || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 24 'echo "                  [--download-url URL] [--checksum-url URL] [--no-modify-path]"'
  printf '%s\n' '                  [--download-url URL] [--checksum-url URL] [--no-modify-path]' || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 25 'echo "Install ush and ush_lsp. Example: sh install.sh --version v0.13.0"'
  printf '%s\n' 'Install ush and ush_lsp. Example: sh install.sh --version v0.13.0' || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
}

# raises: unknown
ush_fn_need() {
  __ush_fn_need_arg_0="${1}"
  # shellcheck disable=SC2034
  command="${__ush_fn_need_arg_0}"
  __ush_at 29 'if exists $command == false {'
  if {
  __ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__exists "${__ush_fn_need_arg_0}")" || return "$?"
  [ "$(if [ "${__ush_value_2}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 30 'fail 1 ("install.sh: " + command + " is required")'
    __ush_status=1
    if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
    __ush_report "$__ush_status" 'install.sh: '"${__ush_fn_need_arg_0}"' is required'
    return "$__ush_status"
  fi
}

# raises: unknown
ush_fn_target() {
  __ush_at 35 'let mut os = uname -s'
  __ush_value_3="$(__ush_return_path='' __ush_capture_return='1' command uname '-s')" || return "$?"
  __ush_binding_35_os="${__ush_value_3}"
  # shellcheck disable=SC2034
  os="${__ush_binding_35_os}"
  __ush_at 36 'let mut arch = uname -m'
  __ush_value_4="$(__ush_return_path='' __ush_capture_return='1' command uname '-m')" || return "$?"
  __ush_binding_36_arch="${__ush_value_4}"
  # shellcheck disable=SC2034
  arch="${__ush_binding_36_arch}"
  __ush_at 37 'match os {'
  case "${__ush_binding_35_os}" in
    'Linux')
      __ush_at 38 '"Linux" => os = "unknown-linux-gnu"'
      __ush_binding_35_os='unknown-linux-gnu'
      # shellcheck disable=SC2034
      os="${__ush_binding_35_os}"
      ;;
    'Darwin')
      __ush_at 39 '"Darwin" => os = "apple-darwin"'
      __ush_binding_35_os='apple-darwin'
      # shellcheck disable=SC2034
      os="${__ush_binding_35_os}"
      ;;
    *)
      __ush_at 40 '_ => fail 1 ("install.sh: unsupported operating system: " + os)'
      __ush_status=1
      if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
      __ush_report "$__ush_status" 'install.sh: unsupported operating system: '"${__ush_binding_35_os}"
      return "$__ush_status"
      ;;
  esac
  __ush_at 42 'match arch {'
  case "${__ush_binding_36_arch}" in
    'x86_64')
      __ush_at 43 '"x86_64" => arch = "x86_64"'
      __ush_binding_36_arch='x86_64'
      # shellcheck disable=SC2034
      arch="${__ush_binding_36_arch}"
      ;;
    'amd64')
      __ush_at 44 '"amd64" => arch = "x86_64"'
      __ush_binding_36_arch='x86_64'
      # shellcheck disable=SC2034
      arch="${__ush_binding_36_arch}"
      ;;
    'arm64')
      __ush_at 45 '"arm64" => arch = "aarch64"'
      __ush_binding_36_arch='aarch64'
      # shellcheck disable=SC2034
      arch="${__ush_binding_36_arch}"
      ;;
    'aarch64')
      __ush_at 46 '"aarch64" => arch = "aarch64"'
      __ush_binding_36_arch='aarch64'
      # shellcheck disable=SC2034
      arch="${__ush_binding_36_arch}"
      ;;
    *)
      __ush_at 47 '_ => fail 1 ("install.sh: unsupported architecture: " + arch)'
      __ush_status=1
      if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
      __ush_report "$__ush_status" 'install.sh: unsupported architecture: '"${__ush_binding_36_arch}"
      return "$__ush_status"
      ;;
  esac
  __ush_at 49 'arch + "-" + os'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_binding_36_arch}"'-'"${__ush_binding_35_os}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_binding_36_arch}"'-'"${__ush_binding_35_os}" >&4
  fi
  return 0
}

ush_fn_path_contains() {
  __ush_fn_path_contains_arg_0="${1}"
  # shellcheck disable=SC2034
  directory="${__ush_fn_path_contains_arg_0}"
  __ush_at 53 'contains (":" + get PATH + ":") (":" + directory + ":")'
  __ush_value_5="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'PATH')" || return "$?"
  __ush_value_6="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains ':'"${__ush_value_5}"':' ':'"${__ush_fn_path_contains_arg_0}"':')" || return "$?"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_value_6}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_value_6}" >&4
  fi
  return 0
}

# raises: unknown
ush_fn_bin_directory() {
  __ush_fn_bin_directory_arg_0="${1}"
  # shellcheck disable=SC2034
  explicit="${__ush_fn_bin_directory_arg_0}"
  __ush_fn_bin_directory_arg_1="${2}"
  # shellcheck disable=SC2034
  prefix="${__ush_fn_bin_directory_arg_1}"
  __ush_at 57 'if explicit != "" {'
  if {
  [ "$(if [ "${__ush_fn_bin_directory_arg_0}" != '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 58 'return explicit'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${__ush_fn_bin_directory_arg_0}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${__ush_fn_bin_directory_arg_0}" >&4
    fi
    return 0
  fi
  __ush_at 60 'if prefix != "" {'
  if {
  [ "$(if [ "${__ush_fn_bin_directory_arg_1}" != '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 61 'return prefix + "/bin"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${__ush_fn_bin_directory_arg_1}"'/bin' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${__ush_fn_bin_directory_arg_1}"'/bin' >&4
    fi
    return 0
  fi
  __ush_at 63 'let home = get HOME'
  __ush_value_7="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'HOME')" || return "$?"
  __ush_binding_63_home="${__ush_value_7}"
  # shellcheck disable=SC2034
  home="${__ush_binding_63_home}"
  __ush_at 64 'for candidate in [home + "/.local/bin", home + "/bin"] {'
  __ush_match_8__len='2'
  # shellcheck disable=SC2034
  __ush_match_8__0="${__ush_binding_63_home}"'/.local/bin'
  # shellcheck disable=SC2034
  __ush_match_8__1="${__ush_binding_63_home}"'/bin'
  __ush_for_list_index_9=0
  while [ "${__ush_for_list_index_9}" -lt "${__ush_match_8__len}" ]; do
    __ush_for_value_10=''
    eval "__ush_for_value_10=\"\${__ush_match_8__${__ush_for_list_index_9}}\""
    candidate="${__ush_for_value_10}"
    __ush_for_list_index_9=$((__ush_for_list_index_9 + 1))
    __ush_at 65 'if path_contains $candidate {'
    if {
    __ush_value_11="$(__ush_return_path='' __ush_capture_return='1' ush_fn_path_contains "${candidate}" 4>&1 >&3)" || return "$?"
    [ "${__ush_value_11}" = 'true' ]
    }; then
      __ush_at 66 'return candidate'
      if [ -n "${__ush_return_path:-}" ]; then
        printf '%s' "${candidate}" > "$__ush_return_path"
      elif [ "${__ush_capture_return:-0}" = '1' ]; then
        printf '%s' "${candidate}" >&4
      fi
      return 0
    fi
  done
  __ush_at 69 'for candidate in split (get PATH) ":" {'
  __ush_value_13="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'PATH')" || return "$?"
  __ush_packed_14="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__split "${__ush_value_13}" ':')" || return "$?"
  __ush_match_12__len=0
  ush_fn_std__unpack __ush_match_12 "${__ush_packed_14}"
  __ush_for_list_index_15=0
  while [ "${__ush_for_list_index_15}" -lt "${__ush_match_12__len}" ]; do
    __ush_for_value_16=''
    eval "__ush_for_value_16=\"\${__ush_match_12__${__ush_for_list_index_15}}\""
    candidate="${__ush_for_value_16}"
    __ush_for_list_index_15=$((__ush_for_list_index_15 + 1))
    __ush_at 70 'if starts_with $candidate (home + "/") && test -d $candidate && test -w $candidate {'
    if {
    {
    __ush_value_17="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__starts_with "${candidate}" "${__ush_binding_63_home}"'/')" || return "$?"
    [ "${__ush_value_17}" = 'true' ]
    } && {
    __ush_value_18="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-d' "${candidate}")" || return "$?"
    [ "${__ush_value_18}" = 'true' ]
    } && {
    __ush_value_19="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-w' "${candidate}")" || return "$?"
    [ "${__ush_value_19}" = 'true' ]
    }
    }; then
      __ush_at 71 'return candidate'
      if [ -n "${__ush_return_path:-}" ]; then
        printf '%s' "${candidate}" > "$__ush_return_path"
      elif [ "${__ush_capture_return:-0}" = '1' ]; then
        printf '%s' "${candidate}" >&4
      fi
      return 0
    fi
  done
  __ush_at 74 'home + "/.local/bin"'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_binding_63_home}"'/.local/bin' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_binding_63_home}"'/.local/bin' >&4
  fi
  return 0
}

ush_fn_profile() {
  __ush_at 78 'let home = get HOME'
  __ush_value_20="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'HOME')" || return "$?"
  __ush_binding_78_home="${__ush_value_20}"
  # shellcheck disable=SC2034
  home="${__ush_binding_78_home}"
  __ush_at 79 'let name = std::path::basename (get SHELL)'
  __ush_value_21="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'SHELL')" || return "$?"
  __ush_value_22="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__basename "${__ush_value_21}")" || return "$?"
  __ush_binding_79_name="${__ush_value_22}"
  # shellcheck disable=SC2034
  name="${__ush_binding_79_name}"
  __ush_at 80 'if name == "zsh" {'
  if {
  [ "$(if [ "${__ush_binding_79_name}" = 'zsh' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 81 'return home + "/.zshrc"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${__ush_binding_78_home}"'/.zshrc' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${__ush_binding_78_home}"'/.zshrc' >&4
    fi
    return 0
  fi
  __ush_at 83 'if name == "ksh" {'
  if {
  [ "$(if [ "${__ush_binding_79_name}" = 'ksh' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 84 'return home + "/.kshrc"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${__ush_binding_78_home}"'/.kshrc' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${__ush_binding_78_home}"'/.kshrc' >&4
    fi
    return 0
  fi
  __ush_at 86 'if name == "fish" {'
  if {
  [ "$(if [ "${__ush_binding_79_name}" = 'fish' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 87 'return ""'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' '' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' '' >&4
    fi
    return 0
  fi
  __ush_at 89 'if name == "bash" {'
  if {
  [ "$(if [ "${__ush_binding_79_name}" = 'bash' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 90 'if is_file (home + "/.bashrc") || is_file (home + "/.bash_profile") == false {'
    if {
    {
    __ush_value_23="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${__ush_binding_78_home}"'/.bashrc')" || return "$?"
    [ "${__ush_value_23}" = 'true' ]
    } || {
    __ush_value_24="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${__ush_binding_78_home}"'/.bash_profile')" || return "$?"
    [ "$(if [ "${__ush_value_24}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }
    }; then
      __ush_at 91 'return home + "/.bashrc"'
      if [ -n "${__ush_return_path:-}" ]; then
        printf '%s' "${__ush_binding_78_home}"'/.bashrc' > "$__ush_return_path"
      elif [ "${__ush_capture_return:-0}" = '1' ]; then
        printf '%s' "${__ush_binding_78_home}"'/.bashrc' >&4
      fi
      return 0
    fi
    __ush_at 93 'return home + "/.bash_profile"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${__ush_binding_78_home}"'/.bash_profile' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${__ush_binding_78_home}"'/.bash_profile' >&4
    fi
    return 0
  fi
  __ush_at 95 'home + "/.profile"'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_binding_78_home}"'/.profile' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_binding_78_home}"'/.profile' >&4
  fi
  return 0
}

ush_fn_escaped() {
  __ush_fn_escaped_arg_0="${1}"
  # shellcheck disable=SC2034
  value="${__ush_fn_escaped_arg_0}"
  __ush_at 100 'let mut value = replace $value "\\" "\\\\"'
  __ush_value_25="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__replace "${__ush_fn_escaped_arg_0}" '\' '\\')" || return "$?"
  __ush_binding_100_value="${__ush_value_25}"
  # shellcheck disable=SC2034
  value="${__ush_binding_100_value}"
  __ush_at 101 'value = replace $value "\"" "\\\""'
  __ush_value_26="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__replace "${__ush_binding_100_value}" '"' '\"')" || return "$?"
  __ush_binding_100_value="${__ush_value_26}"
  # shellcheck disable=SC2034
  value="${__ush_binding_100_value}"
  __ush_at 102 'value = replace $value "$" "\\$"'
  __ush_value_27="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__replace "${__ush_binding_100_value}" '$' '\$')" || return "$?"
  __ush_binding_100_value="${__ush_value_27}"
  # shellcheck disable=SC2034
  value="${__ush_binding_100_value}"
  __ush_at 103 'replace $value "`" "\\`"'
  __ush_value_28="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__replace "${__ush_binding_100_value}" '`' '\`')" || return "$?"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_value_28}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_value_28}" >&4
  fi
  return 0
}

ush_fn_path_line() {
  __ush_fn_path_line_arg_0="${1}"
  # shellcheck disable=SC2034
  directory="${__ush_fn_path_line_arg_0}"
  __ush_at 107 'let home = get HOME'
  __ush_value_29="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'HOME')" || return "$?"
  __ush_binding_107_home="${__ush_value_29}"
  # shellcheck disable=SC2034
  home="${__ush_binding_107_home}"
  __ush_at 108 'if starts_with $directory (home + "/") {'
  if {
  __ush_value_30="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__starts_with "${__ush_fn_path_line_arg_0}" "${__ush_binding_107_home}"'/')" || return "$?"
  [ "${__ush_value_30}" = 'true' ]
  }; then
    __ush_at 109 'return "export PATH=\"$HOME/" + escaped (trim_prefix $directory (home + "/")) + ":$PATH\""'
    __ush_value_31="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${__ush_fn_path_line_arg_0}" "${__ush_binding_107_home}"'/')" || return "$?"
    __ush_value_32="$(__ush_return_path='' __ush_capture_return='1' ush_fn_escaped "${__ush_value_31}" 4>&1 >&3)" || return "$?"
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'export PATH="$HOME/'"${__ush_value_32}"':$PATH"' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'export PATH="$HOME/'"${__ush_value_32}"':$PATH"' >&4
    fi
    return 0
  fi
  __ush_at 111 '"export PATH=\"" + escaped $directory + ":$PATH\""'
  __ush_value_33="$(__ush_return_path='' __ush_capture_return='1' ush_fn_escaped "${__ush_fn_path_line_arg_0}" 4>&1 >&3)" || return "$?"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' 'export PATH="'"${__ush_value_33}"':$PATH"' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' 'export PATH="'"${__ush_value_33}"':$PATH"' >&4
  fi
  return 0
}

# raises: unknown
ush_fn_ensure_path() {
  __ush_fn_ensure_path_arg_0="${1}"
  # shellcheck disable=SC2034
  directory="${__ush_fn_ensure_path_arg_0}"
  __ush_fn_ensure_path_arg_1="${2}"
  # shellcheck disable=SC2034
  automatic="${__ush_fn_ensure_path_arg_1}"
  __ush_at 115 'if path_contains $directory {'
  if {
  __ush_value_34="$(__ush_return_path='' __ush_capture_return='1' ush_fn_path_contains "${__ush_fn_ensure_path_arg_0}" 4>&1 >&3)" || return "$?"
  [ "${__ush_value_34}" = 'true' ]
  }; then
    __ush_at 116 'return "ready"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'ready' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'ready' >&4
    fi
    return 0
  fi
  __ush_at 118 'if automatic == false {'
  if {
  [ "$(if [ "${__ush_fn_ensure_path_arg_1}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 119 'return "manual"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'manual' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'manual' >&4
    fi
    return 0
  fi
  __ush_at 121 'let file = profile'
  __ush_value_35="$(__ush_return_path='' __ush_capture_return='1' ush_fn_profile 4>&1 >&3)" || return "$?"
  __ush_binding_121_file="${__ush_value_35}"
  # shellcheck disable=SC2034
  file="${__ush_binding_121_file}"
  __ush_at 122 'if file == "" {'
  if {
  [ "$(if [ "${__ush_binding_121_file}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 123 'return "manual"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'manual' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'manual' >&4
    fi
    return 0
  fi
  __ush_at 125 'if is_dir $file {'
  if {
  __ush_value_36="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_dir "${__ush_binding_121_file}")" || return "$?"
  [ "${__ush_value_36}" = 'true' ]
  }; then
    __ush_at 126 'fail 1 ("install.sh: shell profile is a directory: " + file)'
    __ush_status=1
    if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
    __ush_report "$__ush_status" 'install.sh: shell profile is a directory: '"${__ush_binding_121_file}"
    return "$__ush_status"
  fi
  __ush_at 128 'let line = path_line $directory'
  __ush_value_37="$(__ush_return_path='' __ush_capture_return='1' ush_fn_path_line "${__ush_fn_ensure_path_arg_0}" 4>&1 >&3)" || return "$?"
  __ush_binding_128_line="${__ush_value_37}"
  # shellcheck disable=SC2034
  line="${__ush_binding_128_line}"
  __ush_at 129 'if is_file $file && contains (read_text $file) $line {'
  if {
  {
  __ush_value_38="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${__ush_binding_121_file}")" || return "$?"
  [ "${__ush_value_38}" = 'true' ]
  } && {
  __ush_value_39="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_121_file}")" || return "$?"
  __ush_value_40="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_39}" "${__ush_binding_128_line}")" || return "$?"
  [ "${__ush_value_40}" = 'true' ]
  }
  }; then
    __ush_at 130 'return file'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${__ush_binding_121_file}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${__ush_binding_121_file}" >&4
    fi
    return 0
  fi
  __ush_at 132 'append_text $file ("\n# Added by ush install.sh\n" + line + "\n")?'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__append_text "${__ush_binding_121_file}" ''"${__ush_lf}"'# Added by ush install.sh'"${__ush_lf}"''"${__ush_binding_128_line}"''"${__ush_lf}"'' ) || return "$?"
  __ush_at 133 'file'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_binding_121_file}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_binding_121_file}" >&4
  fi
  return 0
}

# raises: unknown
ush_fn_fetch() {
  __ush_fn_fetch_arg_0="${1}"
  # shellcheck disable=SC2034
  url="${__ush_fn_fetch_arg_0}"
  __ush_fn_fetch_arg_1="${2}"
  # shellcheck disable=SC2034
  output="${__ush_fn_fetch_arg_1}"
  __ush_at 137 'if starts_with $url file:// {'
  if {
  __ush_value_41="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__starts_with "${__ush_fn_fetch_arg_0}" 'file://')" || return "$?"
  [ "${__ush_value_41}" = 'true' ]
  }; then
    __ush_at 138 'if exists curl {'
    if {
    __ush_value_42="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__exists 'curl')" || return "$?"
    [ "${__ush_value_42}" = 'true' ]
    }; then
      __ush_at 139 'curl -fsSL --proto "=file" $url -o $output'
      command curl '-fsSL' '--proto' '=file' "${__ush_fn_fetch_arg_0}" '-o' "${__ush_fn_fetch_arg_1}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
    else
      __ush_at 141 'let mut local_path = trim_prefix $url file://'
      __ush_value_43="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${__ush_fn_fetch_arg_0}" 'file://')" || return "$?"
      __ush_binding_141_local_path="${__ush_value_43}"
      # shellcheck disable=SC2034
      local_path="${__ush_binding_141_local_path}"
      __ush_at 142 'if starts_with $local_path localhost/ {'
      if {
      __ush_value_44="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__starts_with "${__ush_binding_141_local_path}" 'localhost/')" || return "$?"
      [ "${__ush_value_44}" = 'true' ]
      }; then
        __ush_at 143 'local_path = trim_prefix $local_path localhost'
        __ush_value_45="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${__ush_binding_141_local_path}" 'localhost')" || return "$?"
        __ush_binding_141_local_path="${__ush_value_45}"
        # shellcheck disable=SC2034
        local_path="${__ush_binding_141_local_path}"
      fi
      __ush_at 145 'if starts_with $local_path / == false || contains $local_path "%" {'
      if {
      {
      __ush_value_46="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__starts_with "${__ush_binding_141_local_path}" '/')" || return "$?"
      [ "$(if [ "${__ush_value_46}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
      } || {
      __ush_value_47="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_binding_141_local_path}" '%')" || return "$?"
      [ "${__ush_value_47}" = 'true' ]
      }
      }; then
        __ush_at 146 'fail 1 "install.sh: encoded or nonlocal file URLs require curl"'
        __ush_status=1
        if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
        __ush_report "$__ush_status" 'install.sh: encoded or nonlocal file URLs require curl'
        return "$__ush_status"
      fi
      __ush_at 148 'cp $local_path $output'
      command cp "${__ush_binding_141_local_path}" "${__ush_fn_fetch_arg_1}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
    fi
  else
    __ush_at 150 '} else if exists curl {'
    if {
    __ush_value_48="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__exists 'curl')" || return "$?"
    [ "${__ush_value_48}" = 'true' ]
    }; then
      __ush_at 151 'curl -fsSL --proto "=https" --tlsv1.2 --retry "3" --retry-delay "2" $url -o $output'
      command curl '-fsSL' '--proto' '=https' '--tlsv1.2' '--retry' '3' '--retry-delay' '2' "${__ush_fn_fetch_arg_0}" '-o' "${__ush_fn_fetch_arg_1}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
    else
      __ush_at 152 '} else if exists wget {'
      if {
      __ush_value_49="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__exists 'wget')" || return "$?"
      [ "${__ush_value_49}" = 'true' ]
      }; then
        __ush_at 153 'wget -q --https-only --secure-protocol=TLSv1_2 --tries=3 -O $output $url'
        command wget '-q' '--https-only' '--secure-protocol=TLSv1_2' '--tries=3' '-O' "${__ush_fn_fetch_arg_1}" "${__ush_fn_fetch_arg_0}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
      else
        __ush_at 155 'fail 1 "install.sh: curl or wget is required"'
        __ush_status=1
        if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
        __ush_report "$__ush_status" 'install.sh: curl or wget is required'
        return "$__ush_status"
      fi
    fi
  fi
  __ush_at 157 '()'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' '' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' '' >&4
  fi
  return 0
}

ush_fn_release_url() {
  __ush_fn_release_url_arg_0="${1}"
  # shellcheck disable=SC2034
  repo="${__ush_fn_release_url_arg_0}"
  __ush_fn_release_url_arg_1="${2}"
  # shellcheck disable=SC2034
  version="${__ush_fn_release_url_arg_1}"
  __ush_fn_release_url_arg_2="${3}"
  # shellcheck disable=SC2034
  asset="${__ush_fn_release_url_arg_2}"
  __ush_at 161 'if version == "latest" {'
  if {
  [ "$(if [ "${__ush_fn_release_url_arg_1}" = 'latest' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 162 'return "https://github.com/" + repo + "/releases/latest/download/" + asset'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'https://github.com/'"${__ush_fn_release_url_arg_0}"'/releases/latest/download/'"${__ush_fn_release_url_arg_2}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'https://github.com/'"${__ush_fn_release_url_arg_0}"'/releases/latest/download/'"${__ush_fn_release_url_arg_2}" >&4
    fi
    return 0
  fi
  __ush_at 164 '"https://github.com/" + repo + "/releases/download/" + version + "/" + asset'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' 'https://github.com/'"${__ush_fn_release_url_arg_0}"'/releases/download/'"${__ush_fn_release_url_arg_1}"'/'"${__ush_fn_release_url_arg_2}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' 'https://github.com/'"${__ush_fn_release_url_arg_0}"'/releases/download/'"${__ush_fn_release_url_arg_1}"'/'"${__ush_fn_release_url_arg_2}" >&4
  fi
  return 0
}

# raises: unknown
ush_fn_expected_checksum() {
  __ush_fn_expected_checksum_arg_0="${1}"
  # shellcheck disable=SC2034
  file="${__ush_fn_expected_checksum_arg_0}"
  __ush_fn_expected_checksum_arg_1="${2}"
  # shellcheck disable=SC2034
  asset="${__ush_fn_expected_checksum_arg_1}"
  __ush_at 168 'for line in lines (read_text $file) {'
  __ush_value_51="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_fn_expected_checksum_arg_0}")" || return "$?"
  __ush_packed_52="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__lines "${__ush_value_51}")" || return "$?"
  __ush_match_50__len=0
  ush_fn_std__unpack __ush_match_50 "${__ush_packed_52}"
  __ush_for_list_index_53=0
  while [ "${__ush_for_list_index_53}" -lt "${__ush_match_50__len}" ]; do
    __ush_for_value_54=''
    eval "__ush_for_value_54=\"\${__ush_match_50__${__ush_for_list_index_53}}\""
    line="${__ush_for_value_54}"
    __ush_for_list_index_53=$((__ush_for_list_index_53 + 1))
    __ush_at 169 'let mut field = 0'
    __ush_binding_169_field=0
    # shellcheck disable=SC2034
    field="${__ush_binding_169_field}"
    __ush_at 170 'let mut digest = ""'
    __ush_binding_170_digest=''
    # shellcheck disable=SC2034
    digest="${__ush_binding_170_digest}"
    __ush_at 171 'for word in split (replace $line "\t" " ") " " {'
    __ush_value_56="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__replace "${line}" '	' ' ')" || return "$?"
    __ush_packed_57="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__split "${__ush_value_56}" ' ')" || return "$?"
    __ush_match_55__len=0
    ush_fn_std__unpack __ush_match_55 "${__ush_packed_57}"
    __ush_for_list_index_58=0
    while [ "${__ush_for_list_index_58}" -lt "${__ush_match_55__len}" ]; do
      __ush_for_value_59=''
      eval "__ush_for_value_59=\"\${__ush_match_55__${__ush_for_list_index_58}}\""
      word="${__ush_for_value_59}"
      __ush_for_list_index_58=$((__ush_for_list_index_58 + 1))
      __ush_at 172 'if word != "" {'
      if {
      [ "$(if [ "${word}" != '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
      }; then
        __ush_at 173 'if field == 0 {'
        if {
        [ "$(if [ "${__ush_binding_169_field}" -eq 0 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        }; then
          __ush_at 174 'digest = word'
          __ush_binding_170_digest="${word}"
          # shellcheck disable=SC2034
          digest="${__ush_binding_170_digest}"
        else
          __ush_at 175 '} else if field == 1 && trim_prefix $word * == asset {'
          if {
          {
          [ "$(if [ "${__ush_binding_169_field}" -eq 1 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
          } && {
          __ush_value_60="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${word}" '*')" || return "$?"
          [ "$(if [ "${__ush_value_60}" = "${__ush_fn_expected_checksum_arg_1}" ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
          }
          }; then
            __ush_at 176 'return digest'
            if [ -n "${__ush_return_path:-}" ]; then
              printf '%s' "${__ush_binding_170_digest}" > "$__ush_return_path"
            elif [ "${__ush_capture_return:-0}" = '1' ]; then
              printf '%s' "${__ush_binding_170_digest}" >&4
            fi
            return 0
          fi
        fi
        __ush_at 178 'field = field + 1'
        __ush_binding_169_field=$((__ush_binding_169_field + 1))
        # shellcheck disable=SC2034
        field="${__ush_binding_169_field}"
      fi
    done
  done
  __ush_at 182 'fail 1 ("install.sh: could not find checksum for " + asset)'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'install.sh: could not find checksum for '"${__ush_fn_expected_checksum_arg_1}"
  return "$__ush_status"
}

__ush_at 185 'let mut repo = option USH_REPO UBSH_REPO ubugeeei/ush'
__ush_value_61="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_REPO' 'UBSH_REPO' 'ubugeeei/ush' 4>&1 >&3)" || exit "$?"
__ush_binding_185_repo="${__ush_value_61}"
# shellcheck disable=SC2034
repo="${__ush_binding_185_repo}"
__ush_at 186 'repo = trim_suffix (trim_prefix (trim_prefix (trim_prefix $repo https://github.com/) http://github.com/) "git@github.com:") .git'
__ush_value_62="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${__ush_binding_185_repo}" 'https://github.com/')" || exit "$?"
__ush_value_63="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${__ush_value_62}" 'http://github.com/')" || exit "$?"
__ush_value_64="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${__ush_value_63}" 'git@github.com:')" || exit "$?"
__ush_value_65="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_suffix "${__ush_value_64}" '.git')" || exit "$?"
__ush_binding_185_repo="${__ush_value_65}"
# shellcheck disable=SC2034
repo="${__ush_binding_185_repo}"
__ush_at 187 'let mut version = option USH_VERSION UBSH_VERSION latest'
__ush_value_66="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_VERSION' 'UBSH_VERSION' 'latest' 4>&1 >&3)" || exit "$?"
__ush_binding_187_version="${__ush_value_66}"
# shellcheck disable=SC2034
version="${__ush_binding_187_version}"
__ush_at 188 'let mut prefix = option USH_PREFIX UBSH_PREFIX ""'
__ush_value_67="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_PREFIX' 'UBSH_PREFIX' '' 4>&1 >&3)" || exit "$?"
__ush_binding_188_prefix="${__ush_value_67}"
# shellcheck disable=SC2034
prefix="${__ush_binding_188_prefix}"
__ush_at 189 'let mut directory = option USH_BIN_DIR UBSH_BIN_DIR ""'
__ush_value_68="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_BIN_DIR' 'UBSH_BIN_DIR' '' 4>&1 >&3)" || exit "$?"
__ush_binding_189_directory="${__ush_value_68}"
# shellcheck disable=SC2034
directory="${__ush_binding_189_directory}"
__ush_at 190 'let mut download_url = option USH_DOWNLOAD_URL UBSH_DOWNLOAD_URL ""'
__ush_value_69="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_DOWNLOAD_URL' 'UBSH_DOWNLOAD_URL' '' 4>&1 >&3)" || exit "$?"
__ush_binding_190_download_url="${__ush_value_69}"
# shellcheck disable=SC2034
download_url="${__ush_binding_190_download_url}"
__ush_at 191 'let mut checksum_url = option USH_CHECKSUM_URL UBSH_CHECKSUM_URL ""'
__ush_value_70="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_CHECKSUM_URL' 'UBSH_CHECKSUM_URL' '' 4>&1 >&3)" || exit "$?"
__ush_binding_191_checksum_url="${__ush_value_70}"
# shellcheck disable=SC2034
checksum_url="${__ush_binding_191_checksum_url}"
__ush_at 192 'let mut automatic = option USH_AUTO_PATH UBSH_AUTO_PATH "1"'
__ush_value_71="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_AUTO_PATH' 'UBSH_AUTO_PATH' '1' 4>&1 >&3)" || exit "$?"
__ush_binding_192_automatic="${__ush_value_71}"
# shellcheck disable=SC2034
automatic="${__ush_binding_192_automatic}"
__ush_at 193 'let mut pending = ""'
__ush_binding_193_pending=''
# shellcheck disable=SC2034
pending="${__ush_binding_193_pending}"
__ush_at 194 'for value in args {'
__ush_packed_73="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__process__args)" || exit "$?"
__ush_match_72__len=0
ush_fn_std__unpack __ush_match_72 "${__ush_packed_73}"
__ush_for_list_index_74=0
while [ "${__ush_for_list_index_74}" -lt "${__ush_match_72__len}" ]; do
  __ush_for_value_75=''
  eval "__ush_for_value_75=\"\${__ush_match_72__${__ush_for_list_index_74}}\""
  value="${__ush_for_value_75}"
  __ush_for_list_index_74=$((__ush_for_list_index_74 + 1))
  __ush_at 195 'if pending != "" {'
  if {
  [ "$(if [ "${__ush_binding_193_pending}" != '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 196 'match pending {'
    case "${__ush_binding_193_pending}" in
      '--version')
        __ush_at 197 '"--version" => version = value'
        __ush_binding_187_version="${value}"
        # shellcheck disable=SC2034
        version="${__ush_binding_187_version}"
        ;;
      '--prefix')
        __ush_at 198 '"--prefix" => prefix = value'
        __ush_binding_188_prefix="${value}"
        # shellcheck disable=SC2034
        prefix="${__ush_binding_188_prefix}"
        ;;
      '--bin-dir')
        __ush_at 199 '"--bin-dir" => directory = value'
        __ush_binding_189_directory="${value}"
        # shellcheck disable=SC2034
        directory="${__ush_binding_189_directory}"
        ;;
      '--download-url')
        __ush_at 200 '"--download-url" => download_url = value'
        __ush_binding_190_download_url="${value}"
        # shellcheck disable=SC2034
        download_url="${__ush_binding_190_download_url}"
        ;;
      '--checksum-url')
        __ush_at 201 '"--checksum-url" => checksum_url = value'
        __ush_binding_191_checksum_url="${value}"
        # shellcheck disable=SC2034
        checksum_url="${__ush_binding_191_checksum_url}"
        ;;
      *)
        __ush_at 202 '_ => fail 1 "install.sh: invalid option state"'
        __ush_status=1
        if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
        __ush_report "$__ush_status" 'install.sh: invalid option state'
        exit "$__ush_status"
        ;;
    esac
    __ush_at 204 'pending = ""'
    __ush_binding_193_pending=''
    # shellcheck disable=SC2034
    pending="${__ush_binding_193_pending}"
  else
    __ush_at 206 'if value == "--help" || value == "-h" {'
    if {
    {
    [ "$(if [ "${value}" = '--help' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    } || {
    [ "$(if [ "${value}" = '-h' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }
    }; then
      __ush_at 207 'usage'
      ( __ush_capture_return='0' __ush_return_path=''; ush_fn_usage )
      __ush_at 208 'exit 0'
      ush_fn_std__process__exit 0
    else
      __ush_at 209 '} else if value == "--no-modify-path" {'
      if {
      [ "$(if [ "${value}" = '--no-modify-path' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
      }; then
        __ush_at 210 'automatic = "0"'
        __ush_binding_192_automatic='0'
        # shellcheck disable=SC2034
        automatic="${__ush_binding_192_automatic}"
      else
        __ush_at 211 '} else if value == "--version" || value == "--prefix" || value == "--bin-dir" || value == "--download-url" || value == "--checksum-url" {'
        if {
        {
        [ "$(if [ "${value}" = '--version' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        } || {
        [ "$(if [ "${value}" = '--prefix' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        } || {
        [ "$(if [ "${value}" = '--bin-dir' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        } || {
        [ "$(if [ "${value}" = '--download-url' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        } || {
        [ "$(if [ "${value}" = '--checksum-url' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        }
        }; then
          __ush_at 212 'pending = value'
          __ush_binding_193_pending="${value}"
          # shellcheck disable=SC2034
          pending="${__ush_binding_193_pending}"
        else
          __ush_at 214 'fail 1 ("install.sh: unknown option: " + value)'
          __ush_status=1
          if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
          __ush_report "$__ush_status" 'install.sh: unknown option: '"${value}"
          exit "$__ush_status"
        fi
      fi
    fi
  fi
done
__ush_at 218 'if pending != "" {'
if {
[ "$(if [ "${__ush_binding_193_pending}" != '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 219 'fail 1 ("install.sh: " + pending + " requires a value")'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'install.sh: '"${__ush_binding_193_pending}"' requires a value'
  exit "$__ush_status"
fi
__ush_at 221 'if std::env::contains USH_BRANCH || std::env::contains UBSH_BRANCH {'
if {
{
__ush_value_76="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__contains 'USH_BRANCH')" || exit "$?"
[ "${__ush_value_76}" = 'true' ]
} || {
__ush_value_77="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__contains 'UBSH_BRANCH')" || exit "$?"
[ "${__ush_value_77}" = 'true' ]
}
}; then
  __ush_at 222 'fail 1 "install.sh: USH_BRANCH/UBSH_BRANCH is no longer supported; use USH_VERSION or build from source"'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'install.sh: USH_BRANCH/UBSH_BRANCH is no longer supported; use USH_VERSION or build from source'
  exit "$__ush_status"
fi
__ush_at 224 'need tar'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_need 'tar' )
__ush_at 225 'need install'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_need 'install' )
__ush_at 226 'let target = target'
__ush_value_78="$(__ush_return_path='' __ush_capture_return='1' ush_fn_target 4>&1 >&3)" || exit "$?"
__ush_binding_226_target="${__ush_value_78}"
# shellcheck disable=SC2034
target="${__ush_binding_226_target}"
__ush_at 227 'directory = bin_directory $directory $prefix'
__ush_value_79="$(__ush_return_path='' __ush_capture_return='1' ush_fn_bin_directory "${__ush_binding_189_directory}" "${__ush_binding_188_prefix}" 4>&1 >&3)" || exit "$?"
__ush_binding_189_directory="${__ush_value_79}"
# shellcheck disable=SC2034
directory="${__ush_binding_189_directory}"
__ush_at 228 'if starts_with $directory / == false {'
if {
__ush_value_80="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__starts_with "${__ush_binding_189_directory}" '/')" || exit "$?"
[ "$(if [ "${__ush_value_80}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 229 'directory = std::path::cwd + "/" + directory'
  __ush_value_81="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__cwd)" || exit "$?"
  __ush_binding_189_directory="${__ush_value_81}"'/'"${__ush_binding_189_directory}"
  # shellcheck disable=SC2034
  directory="${__ush_binding_189_directory}"
fi
__ush_at 231 'let asset = "ush-" + target + ".tar.gz"'
__ush_binding_231_asset='ush-'"${__ush_binding_226_target}"'.tar.gz'
# shellcheck disable=SC2034
asset="${__ush_binding_231_asset}"
__ush_at 232 'if download_url == "" {'
if {
[ "$(if [ "${__ush_binding_190_download_url}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 233 'download_url = release_url $repo $version $asset'
  __ush_value_82="$(__ush_return_path='' __ush_capture_return='1' ush_fn_release_url "${__ush_binding_185_repo}" "${__ush_binding_187_version}" "${__ush_binding_231_asset}" 4>&1 >&3)" || exit "$?"
  __ush_binding_190_download_url="${__ush_value_82}"
  # shellcheck disable=SC2034
  download_url="${__ush_binding_190_download_url}"
fi
__ush_at 235 'if checksum_url == "" {'
if {
[ "$(if [ "${__ush_binding_191_checksum_url}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 236 'checksum_url = release_url $repo $version sha256sums.txt'
  __ush_value_83="$(__ush_return_path='' __ush_capture_return='1' ush_fn_release_url "${__ush_binding_185_repo}" "${__ush_binding_187_version}" 'sha256sums.txt' 4>&1 >&3)" || exit "$?"
  __ush_binding_191_checksum_url="${__ush_value_83}"
  # shellcheck disable=SC2034
  checksum_url="${__ush_binding_191_checksum_url}"
fi
__ush_at 238 'let work = temp_dir'
__ush_task_init
__ush_at 238 'let work = temp_dir'
__ush_value_84="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
__ush_binding_238_work="${__ush_value_84}"
# shellcheck disable=SC2034
work="${__ush_binding_238_work}"
__ush_at 239 'let archive = work + "/ush.tar.gz"'
__ush_binding_239_archive="${__ush_binding_238_work}"'/ush.tar.gz'
# shellcheck disable=SC2034
archive="${__ush_binding_239_archive}"
__ush_at 240 'let sums = work + "/sha256sums.txt"'
__ush_binding_240_sums="${__ush_binding_238_work}"'/sha256sums.txt'
# shellcheck disable=SC2034
sums="${__ush_binding_240_sums}"
__ush_at 241 'eprint ("install.sh: downloading " + download_url)'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint 'install.sh: downloading '"${__ush_binding_190_download_url}" )
__ush_at 242 'fetch $download_url $archive'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_fetch "${__ush_binding_190_download_url}" "${__ush_binding_239_archive}" )
__ush_at 243 'eprint ("install.sh: downloading " + checksum_url)'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint 'install.sh: downloading '"${__ush_binding_191_checksum_url}" )
__ush_at 244 'fetch $checksum_url $sums'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_fetch "${__ush_binding_191_checksum_url}" "${__ush_binding_240_sums}" )
__ush_at 245 'let expected = expected_checksum $sums $asset'
__ush_value_85="$(__ush_return_path='' __ush_capture_return='1' ush_fn_expected_checksum "${__ush_binding_240_sums}" "${__ush_binding_231_asset}" 4>&1 >&3)" || exit "$?"
__ush_binding_245_expected="${__ush_value_85}"
# shellcheck disable=SC2034
expected="${__ush_binding_245_expected}"
__ush_at 246 'let actual = sha256 $archive?'
__ush_value_86="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__sha256 "${__ush_binding_239_archive}")" || exit "$?"
__ush_binding_246_actual="${__ush_value_86}"
# shellcheck disable=SC2034
actual="${__ush_binding_246_actual}"
__ush_at 247 'if actual != expected {'
if {
[ "$(if [ "${__ush_binding_246_actual}" != "${__ush_binding_245_expected}" ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 248 'fail 1 ("install.sh: checksum verification failed for " + asset)'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'install.sh: checksum verification failed for '"${__ush_binding_231_asset}"
  exit "$__ush_status"
fi
__ush_at 250 'eprint ("install.sh: verified checksum for " + asset)'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint 'install.sh: verified checksum for '"${__ush_binding_231_asset}" )
__ush_at 251 'let unpack = work + "/unpack"'
__ush_binding_251_unpack="${__ush_binding_238_work}"'/unpack'
# shellcheck disable=SC2034
unpack="${__ush_binding_251_unpack}"
__ush_at 252 'mkdir_p $unpack'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__mkdir_p "${__ush_binding_251_unpack}" )
__ush_at 253 'tar -xzf $archive -C $unpack'
command tar '-xzf' "${__ush_binding_239_archive}" '-C' "${__ush_binding_251_unpack}"
__ush_at 254 'let mut package = unpack + "/ush-" + target'
__ush_binding_254_package="${__ush_binding_251_unpack}"'/ush-'"${__ush_binding_226_target}"
# shellcheck disable=SC2034
package="${__ush_binding_254_package}"
__ush_at 255 'if is_file (package + "/ush") == false && is_file (unpack + "/ush") {'
if {
{
__ush_value_87="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${__ush_binding_254_package}"'/ush')" || exit "$?"
[ "$(if [ "${__ush_value_87}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
} && {
__ush_value_88="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${__ush_binding_251_unpack}"'/ush')" || exit "$?"
[ "${__ush_value_88}" = 'true' ]
}
}; then
  __ush_at 256 'package = unpack'
  __ush_binding_254_package="${__ush_binding_251_unpack}"
  # shellcheck disable=SC2034
  package="${__ush_binding_254_package}"
fi
__ush_at 258 'if is_file (package + "/ush") == false {'
if {
__ush_value_89="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${__ush_binding_254_package}"'/ush')" || exit "$?"
[ "$(if [ "${__ush_value_89}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 259 'fail 1 "install.sh: release archive did not contain ush"'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'install.sh: release archive did not contain ush'
  exit "$__ush_status"
fi
__ush_at 261 'mkdir_p $directory'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__mkdir_p "${__ush_binding_189_directory}" )
__ush_at 262 'install -m "755" (package + "/ush") (directory + "/ush")'
command install '-m' '755' "${__ush_binding_254_package}"'/ush' "${__ush_binding_189_directory}"'/ush'
__ush_at 263 'if is_file (package + "/ush_lsp") {'
if {
__ush_value_90="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${__ush_binding_254_package}"'/ush_lsp')" || exit "$?"
[ "${__ush_value_90}" = 'true' ]
}; then
  __ush_at 264 'install -m "755" (package + "/ush_lsp") (directory + "/ush_lsp")'
  command install '-m' '755' "${__ush_binding_254_package}"'/ush_lsp' "${__ush_binding_189_directory}"'/ush_lsp'
  __ush_at 265 'echo ("ush_lsp installed to " + directory + "/ush_lsp")'
  printf '%s\n' 'ush_lsp installed to '"${__ush_binding_189_directory}"'/ush_lsp'
fi
__ush_at 267 'let path_status = ensure_path $directory (automatic != "0")?'
__ush_value_91="$(__ush_return_path='' __ush_capture_return='1' ush_fn_ensure_path "${__ush_binding_189_directory}" "$(if [ "${__ush_binding_192_automatic}" != '0' ]; then printf '%s' true; else printf '%s' false; fi)" 4>&1 >&3)" || exit "$?"
__ush_binding_267_path_status="${__ush_value_91}"
# shellcheck disable=SC2034
path_status="${__ush_binding_267_path_status}"
__ush_at 268 'echo ("ush installed to " + directory + "/ush")'
printf '%s\n' 'ush installed to '"${__ush_binding_189_directory}"'/ush'
__ush_at 269 'if path_status == "ready" {'
if {
[ "$(if [ "${__ush_binding_267_path_status}" = 'ready' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 270 'echo "ush is already on PATH. Try: ush --version"'
  printf '%s\n' 'ush is already on PATH. Try: ush --version'
else
  __ush_at 271 '} else if path_status == "manual" {'
  if {
  [ "$(if [ "${__ush_binding_267_path_status}" = 'manual' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 272 'echo "add this to your shell config, then restart your shell:"'
    printf '%s\n' 'add this to your shell config, then restart your shell:'
    __ush_at 273 'echo (path_line $directory)'
    __ush_value_92="$(__ush_return_path='' __ush_capture_return='1' ush_fn_path_line "${__ush_binding_189_directory}" 4>&1 >&3)" || exit "$?"
    printf '%s\n' "${__ush_value_92}"
  else
    __ush_at 275 'echo ("added " + directory + " to PATH in " + path_status)'
    printf '%s\n' 'added '"${__ush_binding_189_directory}"' to PATH in '"${__ush_binding_267_path_status}"
    __ush_at 276 'echo ("restart your shell or run: . \"" + path_status + "\"")'
    printf '%s\n' 'restart your shell or run: . "'"${__ush_binding_267_path_status}"'"'
  fi
fi
__ush_at 278 'echo ("quick check: " + directory + "/ush --version")'
printf '%s\n' 'quick check: '"${__ush_binding_189_directory}"'/ush --version'
