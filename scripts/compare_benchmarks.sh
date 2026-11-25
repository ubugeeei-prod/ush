#!/bin/sh
# Generated from 'compare_benchmarks.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: compare_benchmarks.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'compare_benchmarks.ush' --checked-in -o 'compare_benchmarks.sh'
set -eu

__ush_lf='
'
__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'compare_benchmarks.ush'
__ush_origin='compare_benchmarks.ush'

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
  [ -z "$__ush_task_dir" ] || rm -rf "$__ush_task_dir"
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

ush_fn_std__env__set() {
  __ush_std_env_validate "$1" || return "$?"
  export "$1=$2"
}

ush_fn_std__fs__read_text() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  cat "$__ush_path"
}

ush_fn_std__fs__append_text() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  printf '%s' "$2" >> "$__ush_path"
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

ush_fn_std__path__cwd() {
  pwd
}

ush_fn_std__path__from_cwd() {
  __ush_std_path_make_ref cwd "$1"
}

ush_fn_std__path__resolve() {
  __ush_std_path_resolve "$1"
}

ush_fn_std__string__ends_with() {
  VALUE="$1" SUFFIX="$2" awk 'BEGIN { value = ENVIRON["VALUE"]; suffix = ENVIRON["SUFFIX"]; if (length(suffix) == 0) { printf "true"; exit } start = length(value) - length(suffix) + 1; if (start < 1) { printf "false"; exit } if (substr(value, start) == suffix) { printf "true" } else { printf "false" } }'
}

ush_fn_std__string__trim_suffix() {
  VALUE="$1" SUFFIX="$2" awk 'BEGIN { value = ENVIRON["VALUE"]; suffix = ENVIRON["SUFFIX"]; if (length(suffix) > 0 && length(value) >= length(suffix) && substr(value, length(value) - length(suffix) + 1) == suffix) { value = substr(value, 1, length(value) - length(suffix)) } printf "%s", value }'
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

ush_fn_std__process__args() {
  # Keep the encoder linked even when args is the only process helper used.
  if [ "${__ush_argv+x}" != x ]; then __ush_std_encode_args; else printf '%s' "$__ush_argv"; fi
}

ush_fn_std__process__arg() {
  __ush_requested=$1
  __ush_args__len=0
  ush_fn_std__unpack __ush_args "$(ush_fn_std__process__args)"
  case $__ush_requested in ''|*[!0-9]*) return 1 ;; esac
  if [ "$__ush_requested" -lt "$__ush_args__len" ]; then
    eval "printf '%s' \"\${__ush_args__${__ush_requested}}\""
  fi
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

ush_fn_std__process__output() {
  ush_fn_std__process__exec "$@"
}

ush_fn_std__process__status() {
  if (ush_fn_std__process__exec "$@") >/dev/null 2>&1; then printf 0; else printf '%s' "$?"; fi
}

ush_fn_std__process__redirect() {
  if (ush_fn_std__process__exec "$1" "$2") > "$3" 2> "$4"; then printf 0; else printf '%s' "$?"; fi
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
ush_fn_report() {
  __ush_fn_report_arg_0="${1}"
  # shellcheck disable=SC2034
  text="${__ush_fn_report_arg_0}"
  __ush_fn_report_arg_1="${2}"
  # shellcheck disable=SC2034
  summary="${__ush_fn_report_arg_1}"
  __ush_at 12 'print text'
  printf '%s\n' "${__ush_fn_report_arg_0}"
  __ush_at 13 'if summary != "" {'
  if {
  [ "$(if [ "${__ush_fn_report_arg_1}" != '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 14 'append_text summary (text + "\n")'
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__append_text "${__ush_fn_report_arg_1}" "${__ush_fn_report_arg_0}"''"${__ush_lf}"'' ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  fi
}

# raises: unknown
ush_fn_build() {
  __ush_fn_build_arg_0="${1}"
  # shellcheck disable=SC2034
  checkout="${__ush_fn_build_arg_0}"
  __ush_fn_build_arg_1="${2}"
  # shellcheck disable=SC2034
  package="${__ush_fn_build_arg_1}"
  __ush_fn_build_arg_2="${3}"
  # shellcheck disable=SC2034
  suite="${__ush_fn_build_arg_2}"
  __ush_fn_build_arg_3="${4}"
  # shellcheck disable=SC2034
  destination="${__ush_fn_build_arg_3}"
  __ush_fn_build_arg_4="${5}"
  # shellcheck disable=SC2034
  target="${__ush_fn_build_arg_4}"
  __ush_fn_build_arg_5="${6}"
  # shellcheck disable=SC2034
  work="${__ush_fn_build_arg_5}"
  __ush_at 19 'cd checkout'
  command cd "${__ush_fn_build_arg_0}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 20 'set "CARGO_TARGET_DIR" target'
  ush_fn_std__env__set 'CARGO_TARGET_DIR' "${__ush_fn_build_arg_4}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 21 'let log = work + "/cargo.jsonl"'
  log="${__ush_fn_build_arg_5}"'/cargo.jsonl'
  __ush_at 22 'let error_log = work + "/cargo.err"'
  error_log="${__ush_fn_build_arg_5}"'/cargo.err'
  __ush_at 23 'let code = redirect "cargo" ["bench", "--locked", "--no-run", "-p", package, "--bench", suite, "--message-format=json"] log error_log'
  __ush_match_0__len='8'
  # shellcheck disable=SC2034
  __ush_match_0__0='bench'
  # shellcheck disable=SC2034
  __ush_match_0__1='--locked'
  # shellcheck disable=SC2034
  __ush_match_0__2='--no-run'
  # shellcheck disable=SC2034
  __ush_match_0__3='-p'
  # shellcheck disable=SC2034
  __ush_match_0__4="${__ush_fn_build_arg_1}"
  # shellcheck disable=SC2034
  __ush_match_0__5='--bench'
  # shellcheck disable=SC2034
  __ush_match_0__6="${__ush_fn_build_arg_2}"
  # shellcheck disable=SC2034
  __ush_match_0__7='--message-format=json'
  __ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect 'cargo' "$(ush_fn_std__pack __ush_match_0)" "${log}" "${error_log}")" || return "$?"
  code="${__ush_value_2}"
  __ush_at 24 'eprint (read_text error_log)'
  __ush_value_3="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${error_log}")" || return "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint "${__ush_value_3}" ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 25 'if code != 0 {'
  if {
  [ "$(if [ "${code}" -ne 0 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 26 'fail code ("benchmark build failed in " + checkout)'
    __ush_status="${code}"
    if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
    __ush_report "$__ush_status" 'benchmark build failed in '"${__ush_fn_build_arg_0}"
    return "$__ush_status"
  fi
  __ush_at 28 'let binary = output "jq" ["-ser", "--arg", "suite", suite, "[.[] | select(.reason == \"compiler-artifact\" and .executable != null and .target.name == $suite) | .executable] | if length == 1 then .[0] else error(\"expected one benchmark executable\") end", log]?'
  __ush_match_4__len='6'
  # shellcheck disable=SC2034
  __ush_match_4__0='-ser'
  # shellcheck disable=SC2034
  __ush_match_4__1='--arg'
  # shellcheck disable=SC2034
  __ush_match_4__2='suite'
  # shellcheck disable=SC2034
  __ush_match_4__3="${__ush_fn_build_arg_2}"
  # shellcheck disable=SC2034
  __ush_match_4__4='[.[] | select(.reason == "compiler-artifact" and .executable != null and .target.name == $suite) | .executable] | if length == 1 then .[0] else error("expected one benchmark executable") end'
  # shellcheck disable=SC2034
  __ush_match_4__5="${log}"
  __ush_value_6="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output 'jq' "$(ush_fn_std__pack __ush_match_4)")" || return "$?"
  binary="${__ush_value_6}"
  __ush_at 29 'cp binary destination'
  command cp "${binary}" "${__ush_fn_build_arg_3}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 30 'destination'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_fn_build_arg_3}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_fn_build_arg_3}" >&4
  fi
  return 0
}

# raises: unknown
ush_fn_cases() {
  __ush_fn_cases_arg_0="${1}"
  # shellcheck disable=SC2034
  binary="${__ush_fn_cases_arg_0}"
  __ush_at 34 'let names = ""'
  names=''
  __ush_at 35 'for line in lines (output binary ["--list"]) {'
  __ush_match_8__len='1'
  # shellcheck disable=SC2034
  __ush_match_8__0='--list'
  __ush_value_10="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${__ush_fn_cases_arg_0}" "$(ush_fn_std__pack __ush_match_8)")" || return "$?"
  __ush_packed_11="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__lines "${__ush_value_10}")" || return "$?"
  __ush_match_7__len=0
  ush_fn_std__unpack __ush_match_7 "${__ush_packed_11}"
  __ush_for_list_index_12=0
  while [ "${__ush_for_list_index_12}" -lt "${__ush_match_7__len}" ]; do
    __ush_for_value_13=''
    eval "__ush_for_value_13=\"\${__ush_match_7__${__ush_for_list_index_12}}\""
    line="${__ush_for_value_13}"
    __ush_for_list_index_12=$((__ush_for_list_index_12 + 1))
    __ush_at 36 'if ends_with line ": benchmark" {'
    if {
    __ush_value_14="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__ends_with "${line}" ': benchmark')" || return "$?"
    [ "${__ush_value_14}" = 'true' ]
    }; then
      __ush_at 37 'let names = names + trim_suffix line ": benchmark" + "\n"'
      __ush_value_15="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__trim_suffix "${line}" ': benchmark')" || return "$?"
      names="${names}""${__ush_value_15}"''"${__ush_lf}"''
    fi
  done
  __ush_at 40 'if names == "" {'
  if {
  [ "$(if [ "${names}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 41 'fail 1 ("No benchmarks in " + binary)'
    __ush_status=1
    if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
    __ush_report "$__ush_status" 'No benchmarks in '"${__ush_fn_cases_arg_0}"
    return "$__ush_status"
  fi
  __ush_at 43 'names'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${names}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${names}" >&4
  fi
  return 0
}

# raises: unknown
ush_fn_measure() {
  __ush_fn_measure_arg_0="${1}"
  # shellcheck disable=SC2034
  binary="${__ush_fn_measure_arg_0}"
  __ush_fn_measure_arg_1="${2}"
  # shellcheck disable=SC2034
  name="${__ush_fn_measure_arg_1}"
  __ush_at 47 'let run = temp_dir'
  __ush_value_16="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || return "$?"
  run="${__ush_value_16}"
  __ush_at 48 'set "CRITERION_HOME" run'
  ush_fn_std__env__set 'CRITERION_HOME' "${run}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 49 'let code = redirect binary ["--bench", "--exact", name, "--noplot", "--sample-size", "50", "--warm-up-time", "1.5", "--measurement-time", "2.5", "--nresamples", "10000"] (run + "/out") (run + "/err")'
  __ush_match_17__len='12'
  # shellcheck disable=SC2034
  __ush_match_17__0='--bench'
  # shellcheck disable=SC2034
  __ush_match_17__1='--exact'
  # shellcheck disable=SC2034
  __ush_match_17__2="${__ush_fn_measure_arg_1}"
  # shellcheck disable=SC2034
  __ush_match_17__3='--noplot'
  # shellcheck disable=SC2034
  __ush_match_17__4='--sample-size'
  # shellcheck disable=SC2034
  __ush_match_17__5='50'
  # shellcheck disable=SC2034
  __ush_match_17__6='--warm-up-time'
  # shellcheck disable=SC2034
  __ush_match_17__7='1.5'
  # shellcheck disable=SC2034
  __ush_match_17__8='--measurement-time'
  # shellcheck disable=SC2034
  __ush_match_17__9='2.5'
  # shellcheck disable=SC2034
  __ush_match_17__10='--nresamples'
  # shellcheck disable=SC2034
  __ush_match_17__11='10000'
  __ush_value_19="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect "${__ush_fn_measure_arg_0}" "$(ush_fn_std__pack __ush_match_17)" "${run}"'/out' "${run}"'/err')" || return "$?"
  code="${__ush_value_19}"
  __ush_at 50 'if code != 0 {'
  if {
  [ "$(if [ "${code}" -ne 0 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 51 'eprint (read_text (run + "/out"))'
    __ush_value_20="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${run}"'/out')" || return "$?"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint "${__ush_value_20}" ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
    __ush_at 52 'eprint (read_text (run + "/err"))'
    __ush_value_21="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${run}"'/err')" || return "$?"
    ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__io__eprint "${__ush_value_21}" ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
    __ush_at 53 'fail code ("benchmark failed: " + name)'
    __ush_status="${code}"
    if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
    __ush_report "$__ush_status" 'benchmark failed: '"${__ush_fn_measure_arg_1}"
    return "$__ush_status"
  fi
  __ush_at 55 'let estimate = ""'
  estimate=''
  __ush_at 56 'let count = 0'
  count=0
  __ush_at 57 'for file in lines (find run "-path" "*/new/estimates.json") {'
  __ush_value_23="$(__ush_return_path='' __ush_capture_return='1' command find "${run}" '-path' '*/new/estimates.json')" || return "$?"
  __ush_packed_24="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__lines "${__ush_value_23}")" || return "$?"
  __ush_match_22__len=0
  ush_fn_std__unpack __ush_match_22 "${__ush_packed_24}"
  __ush_for_list_index_25=0
  while [ "${__ush_for_list_index_25}" -lt "${__ush_match_22__len}" ]; do
    __ush_for_value_26=''
    eval "__ush_for_value_26=\"\${__ush_match_22__${__ush_for_list_index_25}}\""
    file="${__ush_for_value_26}"
    __ush_for_list_index_25=$((__ush_for_list_index_25 + 1))
    __ush_at 58 'let estimate = file'
    estimate="${file}"
    __ush_at 59 'let count = count + 1'
    count=$((count + 1))
  done
  __ush_at 61 'if count != 1 {'
  if {
  [ "$(if [ "${count}" -ne 1 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 62 'fail 1 ("expected one Criterion estimate for " + name)'
    __ush_status=1
    if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
    __ush_report "$__ush_status" 'expected one Criterion estimate for '"${__ush_fn_measure_arg_1}"
    return "$__ush_status"
  fi
  __ush_at 64 'let value = output "jq" ["-er", "(.slope // .mean).point_estimate | if type == \"number\" and . > 0 and isfinite then . else error(\"invalid timing estimate\") end", estimate]?'
  __ush_match_27__len='3'
  # shellcheck disable=SC2034
  __ush_match_27__0='-er'
  # shellcheck disable=SC2034
  __ush_match_27__1='(.slope // .mean).point_estimate | if type == "number" and . > 0 and isfinite then . else error("invalid timing estimate") end'
  # shellcheck disable=SC2034
  __ush_match_27__2="${estimate}"
  __ush_value_29="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output 'jq' "$(ush_fn_std__pack __ush_match_27)")" || return "$?"
  value="${__ush_value_29}"
  __ush_at 65 'rm "-rf" run'
  command rm '-rf' "${run}" || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 66 'value'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${value}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${value}" >&4
  fi
  return 0
}

# raises: unknown
ush_fn_geometric() {
  __ush_fn_geometric_arg_0="${1}"
  # shellcheck disable=SC2034
  first="${__ush_fn_geometric_arg_0}"
  __ush_fn_geometric_arg_1="${2}"
  # shellcheck disable=SC2034
  second="${__ush_fn_geometric_arg_1}"
  __ush_at 70 'output "jq" ["-nr", "--argjson", "a", first, "--argjson", "b", second, "($a * $b) | sqrt"]'
  __ush_match_30__len='8'
  # shellcheck disable=SC2034
  __ush_match_30__0='-nr'
  # shellcheck disable=SC2034
  __ush_match_30__1='--argjson'
  # shellcheck disable=SC2034
  __ush_match_30__2='a'
  # shellcheck disable=SC2034
  __ush_match_30__3="${__ush_fn_geometric_arg_0}"
  # shellcheck disable=SC2034
  __ush_match_30__4='--argjson'
  # shellcheck disable=SC2034
  __ush_match_30__5='b'
  # shellcheck disable=SC2034
  __ush_match_30__6="${__ush_fn_geometric_arg_1}"
  # shellcheck disable=SC2034
  __ush_match_30__7='($a * $b) | sqrt'
  __ush_value_32="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output 'jq' "$(ush_fn_std__pack __ush_match_30)")" || return "$?"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_value_32}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_value_32}" >&4
  fi
  return 0
}

# raises: unknown
ush_fn_compare() {
  __ush_fn_compare_arg_0="${1}"
  # shellcheck disable=SC2034
  base="${__ush_fn_compare_arg_0}"
  __ush_fn_compare_arg_1="${2}"
  # shellcheck disable=SC2034
  head="${__ush_fn_compare_arg_1}"
  __ush_fn_compare_arg_2="${3}"
  # shellcheck disable=SC2034
  summary="${__ush_fn_compare_arg_2}"
  __ush_at 74 'let before_cases = cases base + "\n"'
  __ush_value_33="$(__ush_return_path='' __ush_capture_return='1' ush_fn_cases "${__ush_fn_compare_arg_0}" 4>&1 >&3)" || return "$?"
  before_cases="${__ush_value_33}"''"${__ush_lf}"''
  __ush_at 75 'let after_cases = cases head + "\n"'
  __ush_value_34="$(__ush_return_path='' __ush_capture_return='1' ush_fn_cases "${__ush_fn_compare_arg_1}" 4>&1 >&3)" || return "$?"
  after_cases="${__ush_value_34}"''"${__ush_lf}"''
  __ush_at 76 'let seen = "\n"'
  seen=''"${__ush_lf}"''
  __ush_at 77 'let failures = 0'
  failures=0
  __ush_at 78 'for name in lines (before_cases + after_cases) {'
  __ush_packed_36="$(__ush_return_path='' __ush_capture_return=1 ush_fn_std__string__lines "${before_cases}""${after_cases}")" || return "$?"
  __ush_match_35__len=0
  ush_fn_std__unpack __ush_match_35 "${__ush_packed_36}"
  __ush_for_list_index_37=0
  while [ "${__ush_for_list_index_37}" -lt "${__ush_match_35__len}" ]; do
    __ush_for_value_38=''
    eval "__ush_for_value_38=\"\${__ush_match_35__${__ush_for_list_index_37}}\""
    name="${__ush_for_value_38}"
    __ush_for_list_index_37=$((__ush_for_list_index_37 + 1))
    __ush_at 79 'let key = "\n" + name + "\n"'
    key=''"${__ush_lf}"''"${name}"''"${__ush_lf}"''
    __ush_at 80 'if contains seen key == false {'
    if {
    __ush_value_39="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains "${seen}" "${key}")" || return "$?"
    [ "$(if [ "${__ush_value_39}" = 'false' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
    }; then
      __ush_at 81 'let seen = seen + name + "\n"'
      seen="${seen}""${name}"''"${__ush_lf}"''
      __ush_at 82 'let in_base = contains ("\n" + before_cases) key'
      __ush_value_40="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains ''"${__ush_lf}"''"${before_cases}" "${key}")" || return "$?"
      in_base="${__ush_value_40}"
      __ush_at 83 'let in_head = contains ("\n" + after_cases) key'
      __ush_value_41="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__contains ''"${__ush_lf}"''"${after_cases}" "${key}")" || return "$?"
      in_head="${__ush_value_41}"
      __ush_at 84 'if in_base && in_head {'
      if {
      {
      [ "${in_base}" = 'true' ]
      } && {
      [ "${in_head}" = 'true' ]
      }
      }; then
        __ush_at 86 'let b1 = measure base name'
        __ush_value_42="$(__ush_return_path='' __ush_capture_return='1' ush_fn_measure "${__ush_fn_compare_arg_0}" "${name}" 4>&1 >&3)" || return "$?"
        b1="${__ush_value_42}"
        __ush_at 87 'let h1 = measure head name'
        __ush_value_43="$(__ush_return_path='' __ush_capture_return='1' ush_fn_measure "${__ush_fn_compare_arg_1}" "${name}" 4>&1 >&3)" || return "$?"
        h1="${__ush_value_43}"
        __ush_at 88 'let h2 = measure head name'
        __ush_value_44="$(__ush_return_path='' __ush_capture_return='1' ush_fn_measure "${__ush_fn_compare_arg_1}" "${name}" 4>&1 >&3)" || return "$?"
        h2="${__ush_value_44}"
        __ush_at 89 'let b2 = measure base name'
        __ush_value_45="$(__ush_return_path='' __ush_capture_return='1' ush_fn_measure "${__ush_fn_compare_arg_0}" "${name}" 4>&1 >&3)" || return "$?"
        b2="${__ush_value_45}"
        __ush_at 90 'let before = geometric b1 b2'
        __ush_value_46="$(__ush_return_path='' __ush_capture_return='1' ush_fn_geometric "${b1}" "${b2}" 4>&1 >&3)" || return "$?"
        before="${__ush_value_46}"
        __ush_at 91 'let after = geometric h1 h2'
        __ush_value_47="$(__ush_return_path='' __ush_capture_return='1' ush_fn_geometric "${h1}" "${h2}" 4>&1 >&3)" || return "$?"
        after="${__ush_value_47}"
        __ush_at 92 'let ratio = output "jq" ["-nr", "--argjson", "a", after, "--argjson", "b", before, "$a / $b"]'
        __ush_match_48__len='8'
        # shellcheck disable=SC2034
        __ush_match_48__0='-nr'
        # shellcheck disable=SC2034
        __ush_match_48__1='--argjson'
        # shellcheck disable=SC2034
        __ush_match_48__2='a'
        # shellcheck disable=SC2034
        __ush_match_48__3="${after}"
        # shellcheck disable=SC2034
        __ush_match_48__4='--argjson'
        # shellcheck disable=SC2034
        __ush_match_48__5='b'
        # shellcheck disable=SC2034
        __ush_match_48__6="${before}"
        # shellcheck disable=SC2034
        __ush_match_48__7='$a / $b'
        __ush_value_50="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output 'jq' "$(ush_fn_std__pack __ush_match_48)")" || return "$?"
        ratio="${__ush_value_50}"
        __ush_at 93 'let row = printf "| %s | %.2f | %.2f | %.3f× |" name before after ratio'
        __ush_value_51="$(__ush_return_path='' __ush_capture_return='1' command printf '| %s | %.2f | %.2f | %.3f× |' "${name}" "${before}" "${after}" "${ratio}")" || return "$?"
        row="${__ush_value_51}"
        __ush_at 94 'report row summary'
        ( __ush_capture_return='0' __ush_return_path=''; ush_fn_report "${row}" "${__ush_fn_compare_arg_2}" ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
        __ush_at 95 'if status "jq" ["-ne", "--argjson", "ratio", ratio, "$ratio > 1.25"] == 0 {'
        if {
        __ush_match_52__len='5'
        # shellcheck disable=SC2034
        __ush_match_52__0='-ne'
        # shellcheck disable=SC2034
        __ush_match_52__1='--argjson'
        # shellcheck disable=SC2034
        __ush_match_52__2='ratio'
        # shellcheck disable=SC2034
        __ush_match_52__3="${ratio}"
        # shellcheck disable=SC2034
        __ush_match_52__4='$ratio > 1.25'
        __ush_value_54="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__status 'jq' "$(ush_fn_std__pack __ush_match_52)")" || return "$?"
        [ "$(if [ "${__ush_value_54}" -eq 0 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
        }; then
          __ush_at 96 'let failures = failures + 1'
          failures=$((failures + 1))
        fi
      else
        __ush_at 99 'else if in_head {'
        if {
        [ "${in_head}" = 'true' ]
        }; then
          __ush_at 100 'let value = measure head name'
          __ush_value_55="$(__ush_return_path='' __ush_capture_return='1' ush_fn_measure "${__ush_fn_compare_arg_1}" "${name}" 4>&1 >&3)" || return "$?"
          value="${__ush_value_55}"
          __ush_at 101 'report (printf "| %s | — | %.2f | new |" name value) summary'
          __ush_value_56="$(__ush_return_path='' __ush_capture_return='1' command printf '| %s | — | %.2f | new |' "${name}" "${value}")" || return "$?"
          ( __ush_capture_return='0' __ush_return_path=''; ush_fn_report "${__ush_value_56}" "${__ush_fn_compare_arg_2}" ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
        else
          __ush_at 104 'let value = measure base name'
          __ush_value_57="$(__ush_return_path='' __ush_capture_return='1' ush_fn_measure "${__ush_fn_compare_arg_0}" "${name}" 4>&1 >&3)" || return "$?"
          value="${__ush_value_57}"
          __ush_at 105 'report (printf "| %s | %.2f | — | removed |" name value) summary'
          __ush_value_58="$(__ush_return_path='' __ush_capture_return='1' command printf '| %s | %.2f | — | removed |' "${name}" "${value}")" || return "$?"
          ( __ush_capture_return='0' __ush_return_path=''; ush_fn_report "${__ush_value_58}" "${__ush_fn_compare_arg_2}" ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
        fi
      fi
    fi
  done
  __ush_at 109 'failures'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${failures}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${failures}" >&4
  fi
  return 0
}

__ush_at 112 'let base_arg = arg 0'
__ush_value_59="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__arg 0)" || exit "$?"
base_arg="${__ush_value_59}"
__ush_at 113 'if base_arg == "" {'
if {
[ "$(if [ "${base_arg}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 114 'fail 1 "usage: compare_benchmarks.sh BASE_CHECKOUT"'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'usage: compare_benchmarks.sh BASE_CHECKOUT'
  exit "$__ush_status"
fi
__ush_at 116 'let head = cwd'
__ush_value_60="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__cwd)" || exit "$?"
head="${__ush_value_60}"
__ush_at 117 'let base = resolve (from_cwd base_arg)'
__ush_value_61="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__from_cwd "${base_arg}")" || exit "$?"
__ush_value_62="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__resolve "${__ush_value_61}")" || exit "$?"
base="${__ush_value_62}"
__ush_at 118 'let summary = get "GITHUB_STEP_SUMMARY"'
__ush_value_63="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get 'GITHUB_STEP_SUMMARY')" || exit "$?"
summary="${__ush_value_63}"
__ush_at 119 'report "## Paired Criterion measurements" summary'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_report '## Paired Criterion measurements' "${summary}" )
__ush_at 120 'report "" summary'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_report '' "${summary}" )
__ush_at 121 'report "Base and PR binaries run on the same runner. The regression threshold is 1.25×." summary'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_report 'Base and PR binaries run on the same runner. The regression threshold is 1.25×.' "${summary}" )
__ush_at 122 'report "" summary'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_report '' "${summary}" )
__ush_at 123 'report "| Benchmark | Base (ns) | PR (ns) | Ratio |" summary'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_report '| Benchmark | Base (ns) | PR (ns) | Ratio |' "${summary}" )
__ush_at 124 'report "| --- | ---: | ---: | ---: |" summary'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_report '| --- | ---: | ---: | ---: |' "${summary}" )
__ush_at 125 'let work = temp_dir'
__ush_task_init
__ush_at 125 'let work = temp_dir'
__ush_value_64="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
work="${__ush_value_64}"
__ush_at 126 'let failures = 0'
failures=0
__ush_at 127 'for package in ["ush_shell", "ush_compiler"] {'
__ush_match_65__len='2'
# shellcheck disable=SC2034
__ush_match_65__0='ush_shell'
# shellcheck disable=SC2034
__ush_match_65__1='ush_compiler'
__ush_for_list_index_66=0
while [ "${__ush_for_list_index_66}" -lt "${__ush_match_65__len}" ]; do
  __ush_for_value_67=''
  eval "__ush_for_value_67=\"\${__ush_match_65__${__ush_for_list_index_66}}\""
  package="${__ush_for_value_67}"
  __ush_for_list_index_66=$((__ush_for_list_index_66 + 1))
  __ush_at 128 'let suite = "parser"'
  suite='parser'
  __ush_at 129 'if package == "ush_compiler" {'
  if {
  [ "$(if [ "${package}" = 'ush_compiler' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
  }; then
    __ush_at 130 'let suite = "compile"'
    suite='compile'
  fi
  __ush_at 132 'let before = build base package suite (work + "/base-" + suite) (head + "/target/bench-base") work'
  __ush_value_68="$(__ush_return_path='' __ush_capture_return='1' ush_fn_build "${base}" "${package}" "${suite}" "${work}"'/base-'"${suite}" "${head}"'/target/bench-base' "${work}" 4>&1 >&3)" || exit "$?"
  before="${__ush_value_68}"
  __ush_at 133 'let after = build head package suite (work + "/head-" + suite) (head + "/target") work'
  __ush_value_69="$(__ush_return_path='' __ush_capture_return='1' ush_fn_build "${head}" "${package}" "${suite}" "${work}"'/head-'"${suite}" "${head}"'/target' "${work}" 4>&1 >&3)" || exit "$?"
  after="${__ush_value_69}"
  __ush_at 134 'let failures = failures + compare before after summary'
  __ush_value_70="$(__ush_return_path='' __ush_capture_return='1' ush_fn_compare "${before}" "${after}" "${summary}" 4>&1 >&3)" || exit "$?"
  failures=$((failures + __ush_value_70))
done
__ush_at 136 'if failures > 0 {'
if {
[ "$(if [ "${failures}" -gt 0 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 137 'fail 1 ("Performance regression above 25% in " + failures + " benchmark(s)")'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'Performance regression above 25% in '"${failures}"' benchmark(s)'
  exit "$__ush_status"
fi
