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
__ush_task_seq=0
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
__ush_join() {
  __ush_exit=$1
  __ush_report "$__ush_exit"
  for __ush_pid in $__ush_jobs; do
    wait "$__ush_pid" || { __ush_child=$?; [ "$__ush_exit" -ne 0 ] || __ush_exit=$__ush_child; }
  done
}
__ush_task_scope() {
  __ush_jobs=''
  __ush_completion=${1:-}
  trap '__ush_end_scope "$?"' 0
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
  rm -rf "$__ush_task_dir"
  exit "$__ush_exit"
}
trap '__ush_finish "$?"' 0
trap 'exit 129' HUP
trap 'exit 130' INT
trap 'exit 143' TERM

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

ush_fn_std__env__get_or() {
  __ush_std_env_validate "$1" || return "$?"
  __ush_default=$2
  eval "printf '%s' \"\${$1-\${__ush_default}}\""
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
  __ush_at 11 'get_or name (get_or legacy fallback)'
  __ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get_or "${__ush_fn_option_arg_1}" "${__ush_fn_option_arg_2}")"
  __ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get_or "${__ush_fn_option_arg_0}" "${__ush_value_0}")"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_value_1}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_value_1}" >&4
  fi
  return 0
}

ush_fn_usage() {
  __ush_at 15 'print "usage: install.sh [--version VERSION] [--bin-dir DIR] [--prefix DIR]"'
  printf '%s\n' 'usage: install.sh [--version VERSION] [--bin-dir DIR] [--prefix DIR]'
  __ush_at 16 'print "                  [--download-url URL] [--checksum-url URL] [--no-modify-path]"'
  printf '%s\n' '                  [--download-url URL] [--checksum-url URL] [--no-modify-path]'
  __ush_at 17 'print "Install ush and ush_lsp. Example: sh install.sh --version v0.13.0"'
  printf '%s\n' 'Install ush and ush_lsp. Example: sh install.sh --version v0.13.0'
}

# raises: unknown
ush_fn_need() {
  __ush_fn_need_arg_0="${1}"
  # shellcheck disable=SC2034
  command="${__ush_fn_need_arg_0}"
  __ush_at 21 'if exists command == false {'
  if {
  __ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__exists "${__ush_fn_need_arg_0}")"
  [ "$(if [ "${__ush_value_2}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 22 'fail 1 ("install.sh: " + command + " is required")'
    __ush_status=1
    if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
    __ush_report "$__ush_status" 'install.sh: '"${__ush_fn_need_arg_0}"' is required'
    return "$__ush_status"
  fi
}

# raises: unknown
ush_fn_target() {
  __ush_at 27 'let os = uname "-s"'
  __ush_value_3="$(__ush_return_path='' __ush_capture_return='1' command uname '-s')"
  os="${__ush_value_3}"
  __ush_at 28 'let arch = uname "-m"'
  __ush_value_4="$(__ush_return_path='' __ush_capture_return='1' command uname '-m')"
  arch="${__ush_value_4}"
  __ush_at 29 'match os {'
  if [ "${os}" = 'Linux' ]; then
    __ush_at 30 '"Linux" => let os = "unknown-linux-gnu"'
    os='unknown-linux-gnu'
  elif [ "${os}" = 'Darwin' ]; then
    __ush_at 31 '"Darwin" => let os = "apple-darwin"'
    os='apple-darwin'
  elif :; then
    __ush_at 32 '_ => fail 1 ("install.sh: unsupported operating system: " + os)'
    __ush_status=1
    if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
    __ush_report "$__ush_status" 'install.sh: unsupported operating system: '"${os}"
    return "$__ush_status"
  fi
  __ush_at 34 'match arch {'
  if [ "${arch}" = 'x86_64' ]; then
    __ush_at 35 '"x86_64" => let arch = "x86_64"'
    arch='x86_64'
  elif [ "${arch}" = 'amd64' ]; then
    __ush_at 36 '"amd64" => let arch = "x86_64"'
    arch='x86_64'
  elif [ "${arch}" = 'arm64' ]; then
    __ush_at 37 '"arm64" => let arch = "aarch64"'
    arch='aarch64'
  elif [ "${arch}" = 'aarch64' ]; then
    __ush_at 38 '"aarch64" => let arch = "aarch64"'
    arch='aarch64'
  elif :; then
    __ush_at 39 '_ => fail 1 ("install.sh: unsupported architecture: " + arch)'
    __ush_status=1
    if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
    __ush_report "$__ush_status" 'install.sh: unsupported architecture: '"${arch}"
    return "$__ush_status"
  fi
  __ush_at 41 'arch + "-" + os'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${arch}"'-'"${os}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${arch}"'-'"${os}" >&4
  fi
  return 0
}

ush_fn_path_contains() {
  __ush_fn_path_contains_arg_0="${1}"
  # shellcheck disable=SC2034
  directory="${__ush_fn_path_contains_arg_0}"
  __ush_at 45 'contains (":" + get "PATH" + ":") (":" + directory + ":")'
  __ush_value_5="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'PATH')"
  __ush_value_6="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains ':'"${__ush_value_5}"':' ':'"${__ush_fn_path_contains_arg_0}"':')"
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
  __ush_at 49 'if explicit != "" {'
  if {
  [ "$(if [ "${__ush_fn_bin_directory_arg_0}" != '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 50 'return explicit'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${__ush_fn_bin_directory_arg_0}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${__ush_fn_bin_directory_arg_0}" >&4
    fi
    return 0
  fi
  __ush_at 52 'if prefix != "" {'
  if {
  [ "$(if [ "${__ush_fn_bin_directory_arg_1}" != '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 53 'return prefix + "/bin"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${__ush_fn_bin_directory_arg_1}"'/bin' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${__ush_fn_bin_directory_arg_1}"'/bin' >&4
    fi
    return 0
  fi
  __ush_at 55 'let home = get "HOME"'
  __ush_value_7="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'HOME')"
  home="${__ush_value_7}"
  __ush_at 56 'for candidate in [home + "/.local/bin", home + "/bin"] {'
  __ush_match_8__len='2'
  # shellcheck disable=SC2034
  __ush_match_8__0="${home}"'/.local/bin'
  # shellcheck disable=SC2034
  __ush_match_8__1="${home}"'/bin'
  __ush_for_list_index_9=0
  while [ "${__ush_for_list_index_9}" -lt "${__ush_match_8__len}" ]; do
    __ush_for_value_10=''
    eval "__ush_for_value_10=\"\${__ush_match_8__${__ush_for_list_index_9}}\""
    candidate="${__ush_for_value_10}"
    __ush_for_list_index_9=$((__ush_for_list_index_9 + 1))
    __ush_at 57 'if path_contains candidate {'
    if {
    __ush_value_11="$(__ush_return_path='' __ush_capture_return='1' ush_fn_path_contains "${candidate}" 4>&1 >&3)"
    [ "${__ush_value_11}" = 'true' ]
    }; then
      __ush_at 58 'return candidate'
      if [ -n "${__ush_return_path:-}" ]; then
        printf '%s' "${candidate}" > "$__ush_return_path"
      elif [ "${__ush_capture_return:-0}" = '1' ]; then
        printf '%s' "${candidate}" >&4
      fi
      return 0
    fi
  done
  __ush_at 61 'for candidate in split (get "PATH") ":" {'
  __ush_value_13="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'PATH')"
  __ush_packed_14="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__split "${__ush_value_13}" ':')" || return "$?"
  __ush_match_12__len=0
  ush_fn_std__unpack __ush_match_12 "${__ush_packed_14}"
  __ush_for_list_index_15=0
  while [ "${__ush_for_list_index_15}" -lt "${__ush_match_12__len}" ]; do
    __ush_for_value_16=''
    eval "__ush_for_value_16=\"\${__ush_match_12__${__ush_for_list_index_15}}\""
    candidate="${__ush_for_value_16}"
    __ush_for_list_index_15=$((__ush_for_list_index_15 + 1))
    __ush_at 62 'if starts_with candidate (home + "/") && test "-d" candidate && test "-w" candidate {'
    if {
    __ush_value_17="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__starts_with "${candidate}" "${home}"'/')"
    [ "${__ush_value_17}" = 'true' ]
    } && {
    __ush_value_18="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-d' "${candidate}")"
    [ "${__ush_value_18}" = 'true' ]
    } && {
    __ush_value_19="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-w' "${candidate}")"
    [ "${__ush_value_19}" = 'true' ]
    }; then
      __ush_at 63 'return candidate'
      if [ -n "${__ush_return_path:-}" ]; then
        printf '%s' "${candidate}" > "$__ush_return_path"
      elif [ "${__ush_capture_return:-0}" = '1' ]; then
        printf '%s' "${candidate}" >&4
      fi
      return 0
    fi
  done
  __ush_at 66 'home + "/.local/bin"'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${home}"'/.local/bin' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${home}"'/.local/bin' >&4
  fi
  return 0
}

ush_fn_profile() {
  __ush_at 70 'let home = get "HOME"'
  __ush_value_20="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'HOME')"
  home="${__ush_value_20}"
  __ush_at 71 'let name = std::path::basename (get "SHELL")'
  __ush_value_21="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'SHELL')"
  __ush_value_22="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__basename "${__ush_value_21}")"
  name="${__ush_value_22}"
  __ush_at 72 'if name == "zsh" {'
  if {
  [ "$(if [ "${name}" = 'zsh' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 73 'return home + "/.zshrc"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${home}"'/.zshrc' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${home}"'/.zshrc' >&4
    fi
    return 0
  fi
  __ush_at 75 'if name == "ksh" {'
  if {
  [ "$(if [ "${name}" = 'ksh' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 76 'return home + "/.kshrc"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${home}"'/.kshrc' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${home}"'/.kshrc' >&4
    fi
    return 0
  fi
  __ush_at 78 'if name == "fish" {'
  if {
  [ "$(if [ "${name}" = 'fish' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 79 'return ""'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' '' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' '' >&4
    fi
    return 0
  fi
  __ush_at 81 'if name == "bash" {'
  if {
  [ "$(if [ "${name}" = 'bash' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 82 'if is_file (home + "/.bashrc") || is_file (home + "/.bash_profile") == false {'
    if {
    __ush_value_23="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${home}"'/.bashrc')"
    [ "${__ush_value_23}" = 'true' ]
    } || {
    __ush_value_24="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${home}"'/.bash_profile')"
    [ "$(if [ "${__ush_value_24}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }; then
      __ush_at 83 'return home + "/.bashrc"'
      if [ -n "${__ush_return_path:-}" ]; then
        printf '%s' "${home}"'/.bashrc' > "$__ush_return_path"
      elif [ "${__ush_capture_return:-0}" = '1' ]; then
        printf '%s' "${home}"'/.bashrc' >&4
      fi
      return 0
    fi
    __ush_at 85 'return home + "/.bash_profile"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${home}"'/.bash_profile' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${home}"'/.bash_profile' >&4
    fi
    return 0
  fi
  __ush_at 87 'home + "/.profile"'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${home}"'/.profile' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${home}"'/.profile' >&4
  fi
  return 0
}

ush_fn_escaped() {
  __ush_fn_escaped_arg_0="${1}"
  # shellcheck disable=SC2034
  value="${__ush_fn_escaped_arg_0}"
  __ush_at 92 'let value = replace value "\\" "\\\\"'
  __ush_value_25="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__replace "${__ush_fn_escaped_arg_0}" '\' '\\')"
  value="${__ush_value_25}"
  __ush_at 93 'let value = replace value "\"" "\\\""'
  __ush_value_26="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__replace "${value}" '"' '\"')"
  value="${__ush_value_26}"
  __ush_at 94 'let value = replace value "$" "\\$"'
  __ush_value_27="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__replace "${value}" '$' '\$')"
  value="${__ush_value_27}"
  __ush_at 95 'replace value "`" "\\`"'
  __ush_value_28="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__replace "${value}" '`' '\`')"
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
  __ush_at 99 'let home = get "HOME"'
  __ush_value_29="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'HOME')"
  home="${__ush_value_29}"
  __ush_at 100 'if starts_with directory (home + "/") {'
  if {
  __ush_value_30="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__starts_with "${__ush_fn_path_line_arg_0}" "${home}"'/')"
  [ "${__ush_value_30}" = 'true' ]
  }; then
    __ush_at 101 'return "export PATH=\"$HOME/" + escaped (trim_prefix directory (home + "/")) + ":$PATH\""'
    __ush_value_31="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${__ush_fn_path_line_arg_0}" "${home}"'/')"
    __ush_value_32="$(__ush_return_path='' __ush_capture_return='1' ush_fn_escaped "${__ush_value_31}" 4>&1 >&3)"
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'export PATH="$HOME/'"${__ush_value_32}"':$PATH"' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'export PATH="$HOME/'"${__ush_value_32}"':$PATH"' >&4
    fi
    return 0
  fi
  __ush_at 103 '"export PATH=\"" + escaped directory + ":$PATH\""'
  __ush_value_33="$(__ush_return_path='' __ush_capture_return='1' ush_fn_escaped "${__ush_fn_path_line_arg_0}" 4>&1 >&3)"
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
  __ush_at 107 'if path_contains directory {'
  if {
  __ush_value_34="$(__ush_return_path='' __ush_capture_return='1' ush_fn_path_contains "${__ush_fn_ensure_path_arg_0}" 4>&1 >&3)"
  [ "${__ush_value_34}" = 'true' ]
  }; then
    __ush_at 108 'return "ready"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'ready' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'ready' >&4
    fi
    return 0
  fi
  __ush_at 110 'if automatic == false {'
  if {
  [ "$(if [ "${__ush_fn_ensure_path_arg_1}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 111 'return "manual"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'manual' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'manual' >&4
    fi
    return 0
  fi
  __ush_at 113 'let file = profile'
  __ush_value_35="$(__ush_return_path='' __ush_capture_return='1' ush_fn_profile 4>&1 >&3)"
  file="${__ush_value_35}"
  __ush_at 114 'if file == "" {'
  if {
  [ "$(if [ "${file}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 115 'return "manual"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'manual' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'manual' >&4
    fi
    return 0
  fi
  __ush_at 117 'if is_dir file {'
  if {
  __ush_value_36="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_dir "${file}")"
  [ "${__ush_value_36}" = 'true' ]
  }; then
    __ush_at 118 'fail 1 ("install.sh: shell profile is a directory: " + file)'
    __ush_status=1
    if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
    __ush_report "$__ush_status" 'install.sh: shell profile is a directory: '"${file}"
    return "$__ush_status"
  fi
  __ush_at 120 'let line = path_line directory'
  __ush_value_37="$(__ush_return_path='' __ush_capture_return='1' ush_fn_path_line "${__ush_fn_ensure_path_arg_0}" 4>&1 >&3)"
  line="${__ush_value_37}"
  __ush_at 121 'if is_file file && contains (read_text file) line {'
  if {
  __ush_value_38="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${file}")"
  [ "${__ush_value_38}" = 'true' ]
  } && {
  __ush_value_39="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${file}")"
  __ush_value_40="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${__ush_value_39}" "${line}")"
  [ "${__ush_value_40}" = 'true' ]
  }; then
    __ush_at 122 'return file'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${file}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${file}" >&4
    fi
    return 0
  fi
  __ush_at 124 'append_text file ("\n# Added by ush install.sh\n" + line + "\n")?'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__append_text "${file}" ''"${__ush_lf}"'# Added by ush install.sh'"${__ush_lf}"''"${line}"''"${__ush_lf}"'' ) || return "$?"
  __ush_at 125 'file'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${file}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${file}" >&4
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
  __ush_at 129 'if starts_with url "file://" {'
  if {
  __ush_value_41="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__starts_with "${__ush_fn_fetch_arg_0}" 'file://')"
  [ "${__ush_value_41}" = 'true' ]
  }; then
    __ush_at 130 'cp (trim_prefix url "file://") output'
    __ush_value_42="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${__ush_fn_fetch_arg_0}" 'file://')"
    command cp "${__ush_value_42}" "${__ush_fn_fetch_arg_1}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  else
    __ush_at 132 'else if exists "curl" {'
    if {
    __ush_value_43="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__exists 'curl')"
    [ "${__ush_value_43}" = 'true' ]
    }; then
      __ush_at 133 'curl "-fsSL" "--proto" "=https" "--tlsv1.2" "--retry" "3" "--retry-delay" "2" url "-o" output'
      command curl '-fsSL' '--proto' '=https' '--tlsv1.2' '--retry' '3' '--retry-delay' '2' "${__ush_fn_fetch_arg_0}" '-o' "${__ush_fn_fetch_arg_1}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
    else
      __ush_at 135 'else if exists "wget" {'
      if {
      __ush_value_44="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__exists 'wget')"
      [ "${__ush_value_44}" = 'true' ]
      }; then
        __ush_at 136 'wget "-q" "--https-only" "--secure-protocol=TLSv1_2" "--tries=3" "-O" output url'
        command wget '-q' '--https-only' '--secure-protocol=TLSv1_2' '--tries=3' '-O' "${__ush_fn_fetch_arg_1}" "${__ush_fn_fetch_arg_0}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
      else
        __ush_at 139 'fail 1 "install.sh: curl or wget is required"'
        __ush_status=1
        if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
        __ush_report "$__ush_status" 'install.sh: curl or wget is required'
        return "$__ush_status"
      fi
    fi
  fi
  __ush_at 141 '()'
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
  __ush_at 145 'if version == "latest" {'
  if {
  [ "$(if [ "${__ush_fn_release_url_arg_1}" = 'latest' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 146 'return "https://github.com/" + repo + "/releases/latest/download/" + asset'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'https://github.com/'"${__ush_fn_release_url_arg_0}"'/releases/latest/download/'"${__ush_fn_release_url_arg_2}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'https://github.com/'"${__ush_fn_release_url_arg_0}"'/releases/latest/download/'"${__ush_fn_release_url_arg_2}" >&4
    fi
    return 0
  fi
  __ush_at 148 '"https://github.com/" + repo + "/releases/download/" + version + "/" + asset'
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
  __ush_at 152 'for line in lines (read_text file) {'
  __ush_value_46="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_fn_expected_checksum_arg_0}")"
  __ush_packed_47="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__lines "${__ush_value_46}")" || return "$?"
  __ush_match_45__len=0
  ush_fn_std__unpack __ush_match_45 "${__ush_packed_47}"
  __ush_for_list_index_48=0
  while [ "${__ush_for_list_index_48}" -lt "${__ush_match_45__len}" ]; do
    __ush_for_value_49=''
    eval "__ush_for_value_49=\"\${__ush_match_45__${__ush_for_list_index_48}}\""
    line="${__ush_for_value_49}"
    __ush_for_list_index_48=$((__ush_for_list_index_48 + 1))
    __ush_at 153 'let field = 0'
    field=0
    __ush_at 154 'let digest = ""'
    digest=''
    __ush_at 155 'for word in split (replace line "\t" " ") " " {'
    __ush_value_51="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__replace "${line}" '	' ' ')"
    __ush_packed_52="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__split "${__ush_value_51}" ' ')" || return "$?"
    __ush_match_50__len=0
    ush_fn_std__unpack __ush_match_50 "${__ush_packed_52}"
    __ush_for_list_index_53=0
    while [ "${__ush_for_list_index_53}" -lt "${__ush_match_50__len}" ]; do
      __ush_for_value_54=''
      eval "__ush_for_value_54=\"\${__ush_match_50__${__ush_for_list_index_53}}\""
      word="${__ush_for_value_54}"
      __ush_for_list_index_53=$((__ush_for_list_index_53 + 1))
      __ush_at 156 'if word != "" {'
      if {
      [ "$(if [ "${word}" != '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
      }; then
        __ush_at 157 'if field == 0 {'
        if {
        [ "$(if [ $((field)) -eq 0 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        }; then
          __ush_at 158 'let digest = word'
          digest="${word}"
        else
          __ush_at 160 'else if field == 1 && trim_prefix word "*" == asset {'
          if {
          [ "$(if [ $((field)) -eq 1 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
          } && {
          __ush_value_55="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${word}" '*')"
          [ "$(if [ "${__ush_value_55}" = "${__ush_fn_expected_checksum_arg_1}" ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
          }; then
            __ush_at 161 'return digest'
            if [ -n "${__ush_return_path:-}" ]; then
              printf '%s' "${digest}" > "$__ush_return_path"
            elif [ "${__ush_capture_return:-0}" = '1' ]; then
              printf '%s' "${digest}" >&4
            fi
            return 0
          fi
        fi
        __ush_at 163 'let field = field + 1'
        field=$((field + 1))
      fi
    done
  done
  __ush_at 167 'fail 1 ("install.sh: could not find checksum for " + asset)'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'install.sh: could not find checksum for '"${__ush_fn_expected_checksum_arg_1}"
  return "$__ush_status"
}

__ush_at 170 'let repo = option "USH_REPO" "UBSH_REPO" "ubugeeei/ush"'
__ush_value_56="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_REPO' 'UBSH_REPO' 'ubugeeei/ush' 4>&1 >&3)"
repo="${__ush_value_56}"
__ush_at 171 'let repo = trim_suffix (trim_prefix (trim_prefix (trim_prefix repo "https://github.com/") "http://github.com/") "git@github.com:") ".git"'
__ush_value_57="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${repo}" 'https://github.com/')"
__ush_value_58="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${__ush_value_57}" 'http://github.com/')"
__ush_value_59="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_prefix "${__ush_value_58}" 'git@github.com:')"
__ush_value_60="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_suffix "${__ush_value_59}" '.git')"
repo="${__ush_value_60}"
__ush_at 172 'let version = option "USH_VERSION" "UBSH_VERSION" "latest"'
__ush_value_61="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_VERSION' 'UBSH_VERSION' 'latest' 4>&1 >&3)"
version="${__ush_value_61}"
__ush_at 173 'let prefix = option "USH_PREFIX" "UBSH_PREFIX" ""'
__ush_value_62="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_PREFIX' 'UBSH_PREFIX' '' 4>&1 >&3)"
prefix="${__ush_value_62}"
__ush_at 174 'let directory = option "USH_BIN_DIR" "UBSH_BIN_DIR" ""'
__ush_value_63="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_BIN_DIR' 'UBSH_BIN_DIR' '' 4>&1 >&3)"
directory="${__ush_value_63}"
__ush_at 175 'let download_url = option "USH_DOWNLOAD_URL" "UBSH_DOWNLOAD_URL" ""'
__ush_value_64="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_DOWNLOAD_URL' 'UBSH_DOWNLOAD_URL' '' 4>&1 >&3)"
download_url="${__ush_value_64}"
__ush_at 176 'let checksum_url = option "USH_CHECKSUM_URL" "UBSH_CHECKSUM_URL" ""'
__ush_value_65="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_CHECKSUM_URL' 'UBSH_CHECKSUM_URL' '' 4>&1 >&3)"
checksum_url="${__ush_value_65}"
__ush_at 177 'let automatic = option "USH_AUTO_PATH" "UBSH_AUTO_PATH" "1"'
__ush_value_66="$(__ush_return_path='' __ush_capture_return='1' ush_fn_option 'USH_AUTO_PATH' 'UBSH_AUTO_PATH' '1' 4>&1 >&3)"
automatic="${__ush_value_66}"
__ush_at 178 'let pending = ""'
pending=''
__ush_at 179 'for value in args {'
__ush_packed_68="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__process__args)" || exit "$?"
__ush_match_67__len=0
ush_fn_std__unpack __ush_match_67 "${__ush_packed_68}"
__ush_for_list_index_69=0
while [ "${__ush_for_list_index_69}" -lt "${__ush_match_67__len}" ]; do
  __ush_for_value_70=''
  eval "__ush_for_value_70=\"\${__ush_match_67__${__ush_for_list_index_69}}\""
  value="${__ush_for_value_70}"
  __ush_for_list_index_69=$((__ush_for_list_index_69 + 1))
  __ush_at 180 'if pending != "" {'
  if {
  [ "$(if [ "${pending}" != '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 181 'match pending {'
    if [ "${pending}" = '--version' ]; then
      __ush_at 182 '"--version" => let version = value'
      version="${value}"
    elif [ "${pending}" = '--prefix' ]; then
      __ush_at 183 '"--prefix" => let prefix = value'
      prefix="${value}"
    elif [ "${pending}" = '--bin-dir' ]; then
      __ush_at 184 '"--bin-dir" => let directory = value'
      directory="${value}"
    elif [ "${pending}" = '--download-url' ]; then
      __ush_at 185 '"--download-url" => let download_url = value'
      download_url="${value}"
    elif [ "${pending}" = '--checksum-url' ]; then
      __ush_at 186 '"--checksum-url" => let checksum_url = value'
      checksum_url="${value}"
    elif :; then
      __ush_at 187 '_ => fail 1 "install.sh: invalid option state"'
      __ush_status=1
      if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
      __ush_report "$__ush_status" 'install.sh: invalid option state'
      exit "$__ush_status"
    fi
    __ush_at 189 'let pending = ""'
    pending=''
  else
    __ush_at 192 'if value == "--help" || value == "-h" {'
    if {
    [ "$(if [ "${value}" = '--help' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    } || {
    [ "$(if [ "${value}" = '-h' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }; then
      __ush_at 193 'usage'
      ( __ush_capture_return='0' __ush_return_path=''; ush_fn_usage )
      __ush_at 194 'exit 0'
      ush_fn_std__process__exit 0
    else
      __ush_at 196 'else if value == "--no-modify-path" {'
      if {
      [ "$(if [ "${value}" = '--no-modify-path' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
      }; then
        __ush_at 197 'let automatic = "0"'
        automatic='0'
      else
        __ush_at 199 'else if value == "--version" || value == "--prefix" || value == "--bin-dir" || value == "--download-url" || value == "--checksum-url" {'
        if {
        [ "$(if [ "${value}" = '--version' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        } || {
        [ "$(if [ "${value}" = '--prefix' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        } || {
        [ "$(if [ "${value}" = '--bin-dir' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        } || {
        [ "$(if [ "${value}" = '--download-url' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        } || {
        [ "$(if [ "${value}" = '--checksum-url' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        }; then
          __ush_at 200 'let pending = value'
          pending="${value}"
        else
          __ush_at 203 'fail 1 ("install.sh: unknown option: " + value)'
          __ush_status=1
          if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
          __ush_report "$__ush_status" 'install.sh: unknown option: '"${value}"
          exit "$__ush_status"
        fi
      fi
    fi
  fi
done
__ush_at 207 'if pending != "" {'
if {
[ "$(if [ "${pending}" != '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 208 'fail 1 ("install.sh: " + pending + " requires a value")'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'install.sh: '"${pending}"' requires a value'
  exit "$__ush_status"
fi
__ush_at 210 'if get_or "USH_BRANCH" "__unset__" != "__unset__" || get_or "UBSH_BRANCH" "__unset__" != "__unset__" {'
if {
__ush_value_71="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get_or 'USH_BRANCH' '__unset__')"
[ "$(if [ "${__ush_value_71}" != '__unset__' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
} || {
__ush_value_72="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get_or 'UBSH_BRANCH' '__unset__')"
[ "$(if [ "${__ush_value_72}" != '__unset__' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 211 'fail 1 "install.sh: USH_BRANCH/UBSH_BRANCH is no longer supported; use USH_VERSION or build from source"'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'install.sh: USH_BRANCH/UBSH_BRANCH is no longer supported; use USH_VERSION or build from source'
  exit "$__ush_status"
fi
__ush_at 213 'need "tar"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_need 'tar' )
__ush_at 214 'need "install"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_need 'install' )
__ush_at 215 'let target = target'
__ush_value_73="$(__ush_return_path='' __ush_capture_return='1' ush_fn_target 4>&1 >&3)"
target="${__ush_value_73}"
__ush_at 216 'let directory = bin_directory directory prefix'
__ush_value_74="$(__ush_return_path='' __ush_capture_return='1' ush_fn_bin_directory "${directory}" "${prefix}" 4>&1 >&3)"
directory="${__ush_value_74}"
__ush_at 217 'let asset = "ush-" + target + ".tar.gz"'
asset='ush-'"${target}"'.tar.gz'
__ush_at 218 'if download_url == "" {'
if {
[ "$(if [ "${download_url}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 219 'let download_url = release_url repo version asset'
  __ush_value_75="$(__ush_return_path='' __ush_capture_return='1' ush_fn_release_url "${repo}" "${version}" "${asset}" 4>&1 >&3)"
  download_url="${__ush_value_75}"
fi
__ush_at 221 'if checksum_url == "" {'
if {
[ "$(if [ "${checksum_url}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 222 'let checksum_url = release_url repo version "sha256sums.txt"'
  __ush_value_76="$(__ush_return_path='' __ush_capture_return='1' ush_fn_release_url "${repo}" "${version}" 'sha256sums.txt' 4>&1 >&3)"
  checksum_url="${__ush_value_76}"
fi
__ush_at 224 'let work = temp_dir'
__ush_value_77="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
work="${__ush_value_77}"
__ush_at 225 'let archive = work + "/ush.tar.gz"'
archive="${work}"'/ush.tar.gz'
__ush_at 226 'let sums = work + "/sha256sums.txt"'
sums="${work}"'/sha256sums.txt'
__ush_at 227 'eprint ("install.sh: downloading " + download_url)'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint 'install.sh: downloading '"${download_url}" )
__ush_at 228 'fetch download_url archive'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_fetch "${download_url}" "${archive}" )
__ush_at 229 'eprint ("install.sh: downloading " + checksum_url)'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint 'install.sh: downloading '"${checksum_url}" )
__ush_at 230 'fetch checksum_url sums'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_fetch "${checksum_url}" "${sums}" )
__ush_at 231 'let expected = expected_checksum sums asset'
__ush_value_78="$(__ush_return_path='' __ush_capture_return='1' ush_fn_expected_checksum "${sums}" "${asset}" 4>&1 >&3)"
expected="${__ush_value_78}"
__ush_at 232 'let actual = sha256 archive?'
__ush_value_79="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__sha256 "${archive}")" || exit "$?"
actual="${__ush_value_79}"
__ush_at 233 'if actual != expected {'
if {
[ "$(if [ "${actual}" != "${expected}" ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 234 'fail 1 ("install.sh: checksum verification failed for " + asset)'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'install.sh: checksum verification failed for '"${asset}"
  exit "$__ush_status"
fi
__ush_at 236 'eprint ("install.sh: verified checksum for " + asset)'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint 'install.sh: verified checksum for '"${asset}" )
__ush_at 237 'let unpack = work + "/unpack"'
unpack="${work}"'/unpack'
__ush_at 238 'mkdir_p unpack'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__mkdir_p "${unpack}" )
__ush_at 239 'tar "-xzf" archive "-C" unpack'
command tar '-xzf' "${archive}" '-C' "${unpack}"
__ush_at 240 'let package = unpack + "/ush-" + target'
package="${unpack}"'/ush-'"${target}"
__ush_at 241 'if is_file (package + "/ush") == false && is_file (unpack + "/ush") {'
if {
__ush_value_80="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${package}"'/ush')"
[ "$(if [ "${__ush_value_80}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
} && {
__ush_value_81="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${unpack}"'/ush')"
[ "${__ush_value_81}" = 'true' ]
}; then
  __ush_at 242 'let package = unpack'
  package="${unpack}"
fi
__ush_at 244 'if is_file (package + "/ush") == false {'
if {
__ush_value_82="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${package}"'/ush')"
[ "$(if [ "${__ush_value_82}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 245 'fail 1 "install.sh: release archive did not contain ush"'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'install.sh: release archive did not contain ush'
  exit "$__ush_status"
fi
__ush_at 247 'mkdir_p directory'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__mkdir_p "${directory}" )
__ush_at 248 'install "-m" "755" (package + "/ush") (directory + "/ush")'
command install '-m' '755' "${package}"'/ush' "${directory}"'/ush'
__ush_at 249 'if is_file (package + "/ush_lsp") {'
if {
__ush_value_83="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__is_file "${package}"'/ush_lsp')"
[ "${__ush_value_83}" = 'true' ]
}; then
  __ush_at 250 'install "-m" "755" (package + "/ush_lsp") (directory + "/ush_lsp")'
  command install '-m' '755' "${package}"'/ush_lsp' "${directory}"'/ush_lsp'
  __ush_at 251 'print "ush_lsp installed to " + directory + "/ush_lsp"'
  printf '%s\n' 'ush_lsp installed to '"${directory}"'/ush_lsp'
fi
__ush_at 253 'let path_status = ensure_path directory (automatic != "0")?'
__ush_value_84="$(__ush_return_path='' __ush_capture_return='1' ush_fn_ensure_path "${directory}" "$(if [ "${automatic}" != '0' ]; then printf '%s' true; else printf '%s' false; fi)" 4>&1 >&3)" || exit "$?"
path_status="${__ush_value_84}"
__ush_at 254 'print "ush installed to " + directory + "/ush"'
printf '%s\n' 'ush installed to '"${directory}"'/ush'
__ush_at 255 'if path_status == "ready" {'
if {
[ "$(if [ "${path_status}" = 'ready' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 256 'print "ush is already on PATH. Try: ush --version"'
  printf '%s\n' 'ush is already on PATH. Try: ush --version'
else
  __ush_at 258 'else if path_status == "manual" {'
  if {
  [ "$(if [ "${path_status}" = 'manual' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 259 'print "add this to your shell config, then restart your shell:"'
    printf '%s\n' 'add this to your shell config, then restart your shell:'
    __ush_at 260 'print path_line directory'
    __ush_value_85="$(__ush_return_path='' __ush_capture_return='1' ush_fn_path_line "${directory}" 4>&1 >&3)"
    printf '%s\n' "${__ush_value_85}"
  else
    __ush_at 263 'print "added " + directory + " to PATH in " + path_status'
    printf '%s\n' 'added '"${directory}"' to PATH in '"${path_status}"
    __ush_at 264 'print "restart your shell or run: . \"" + path_status + "\""'
    printf '%s\n' 'restart your shell or run: . "'"${path_status}"'"'
  fi
fi
__ush_at 266 'print "quick check: " + directory + "/ush --version"'
printf '%s\n' 'quick check: '"${directory}"'/ush --version'
