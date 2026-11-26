#!/bin/sh
# Generated from 'test_task_await.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: test_task_await.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'test_task_await.ush' --checked-in -o 'test_task_await.sh'
set -eu

__ush_lf='
'
__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'test_task_await.ush'
__ush_origin='test_task_await.ush'

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

ush_fn_std__fs__read_text() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  cat "$__ush_path"
}

ush_fn_std__fs__write_text() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  printf '%s' "$2" > "$__ush_path"
}

ush_fn_std__fs__remove() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  rm -f "$__ush_path"
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

__ush_argv="$(__ush_std_encode_args "$@")"
# raises: unknown
ush_fn_compile() {
  __ush_fn_compile_arg_0="${1}"
  # shellcheck disable=SC2034
  compiler="${__ush_fn_compile_arg_0}"
  __ush_fn_compile_arg_1="${2}"
  # shellcheck disable=SC2034
  target="${__ush_fn_compile_arg_1}"
  __ush_fn_compile_arg_2="${3}"
  # shellcheck disable=SC2034
  work="${__ush_fn_compile_arg_2}"
  __ush_at 8 'exec $compiler ["compile", work + "/job.ush", "--target", target, "--checked-in", "-o", work + "/job.sh"]'
  __ush_match_0__len='7'
  # shellcheck disable=SC2034
  __ush_match_0__0='compile'
  # shellcheck disable=SC2034
  __ush_match_0__1="${__ush_fn_compile_arg_2}"'/job.ush'
  # shellcheck disable=SC2034
  __ush_match_0__2='--target'
  # shellcheck disable=SC2034
  __ush_match_0__3="${__ush_fn_compile_arg_1}"
  # shellcheck disable=SC2034
  __ush_match_0__4='--checked-in'
  # shellcheck disable=SC2034
  __ush_match_0__5='-o'
  # shellcheck disable=SC2034
  __ush_match_0__6="${__ush_fn_compile_arg_2}"'/job.sh'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__process__exec "${__ush_fn_compile_arg_0}" "$(ush_fn_std__pack __ush_match_0)" ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 9 'chmod +x (work + "/job.sh")'
  command chmod '+x' "${__ush_fn_compile_arg_2}"'/job.sh' || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 10 'remove (work + "/job.ush")'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__remove "${__ush_fn_compile_arg_2}"'/job.ush' ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
}

__ush_at 13 'let compiler = arg 0'
__ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__arg 0)" || exit "$?"
__ush_binding_13_compiler="${__ush_value_2}"
# shellcheck disable=SC2034
compiler="${__ush_binding_13_compiler}"
__ush_at 14 'if compiler == "" {'
if {
[ "$(if [ "${__ush_binding_13_compiler}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 15 'fail 1 "compiler path required"'
  __ush_status=1
  if [ "$__ush_status" -lt 1 ] || [ "$__ush_status" -gt 255 ]; then __ush_status=1; fi
  __ush_report "$__ush_status" 'compiler path required'
  exit "$__ush_status"
fi
__ush_at 17 'let mut target = arg 1'
__ush_value_3="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__arg 1)" || exit "$?"
__ush_binding_17_target="${__ush_value_3}"
# shellcheck disable=SC2034
target="${__ush_binding_17_target}"
__ush_at 18 'if target == "" {'
if {
[ "$(if [ "${__ush_binding_17_target}" = '' ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  __ush_at 19 'target = "sh"'
  __ush_binding_17_target='sh'
  # shellcheck disable=SC2034
  target="${__ush_binding_17_target}"
fi
__ush_at 21 'let work = temp_dir'
__ush_task_init
__ush_at 21 'let work = temp_dir'
__ush_value_4="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
__ush_binding_21_work="${__ush_value_4}"
# shellcheck disable=SC2034
work="${__ush_binding_21_work}"
__ush_at 22 'write_text (work + "/job.ush") """'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__write_text "${__ush_binding_21_work}"'/job.ush' 'fn build() -> Int {'"${__ush_lf}"'  fail 23 "missing input"'"${__ush_lf}"'}'"${__ush_lf}"'let job = async build'"${__ush_lf}"'print job.await' )
__ush_at 29 'compile $compiler $target $work'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_compile "${__ush_binding_13_compiler}" "${__ush_binding_17_target}" "${__ush_binding_21_work}" )
__ush_at 30 'assert_eq (redirect $target [work + "/job.sh"] (work + "/out") (work + "/err")) 23'
__ush_match_5__len='1'
# shellcheck disable=SC2034
__ush_match_5__0="${__ush_binding_21_work}"'/job.sh'
__ush_value_7="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect "${__ush_binding_17_target}" "$(ush_fn_std__pack __ush_match_5)" "${__ush_binding_21_work}"'/out' "${__ush_binding_21_work}"'/err')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_7}" 23 )
__ush_at 31 'assert_eq (test -s (work + "/out")) false'
__ush_value_8="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-s' "${__ush_binding_21_work}"'/out')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_8}" 'false' )
__ush_at 32 'assert_eq (read_text (work + "/err")) """'
__ush_value_9="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_21_work}"'/err')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_9}" 'job.ush:2: missing input (exit 23)'"${__ush_lf}"'  fail 23 "missing input"'"${__ush_lf}"'job.ush:5: command failed (exit 23)'"${__ush_lf}"'  print job.await' )
__ush_at 39 'write_text (work + "/job.ush") """'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__write_text "${__ush_binding_21_work}"'/job.ush' 'use $bin::*'"${__ush_lf}"'fn build() -> Int {'"${__ush_lf}"'  sleep "0.05"'"${__ush_lf}"'  41'"${__ush_lf}"'}'"${__ush_lf}"'let job = async build'"${__ush_lf}"'let first = async {'"${__ush_lf}"'  let value = job.await'"${__ush_lf}"'  value + 1'"${__ush_lf}"'}'"${__ush_lf}"'let second = async {'"${__ush_lf}"'  let value = job.await'"${__ush_lf}"'  value + 2'"${__ush_lf}"'}'"${__ush_lf}"'print first.await'"${__ush_lf}"'print second.await'"${__ush_lf}"'print job.await'"${__ush_lf}"'print job.await' )
__ush_at 59 'compile $compiler $target $work'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_compile "${__ush_binding_13_compiler}" "${__ush_binding_17_target}" "${__ush_binding_21_work}" )
__ush_at 60 'assert_eq (redirect $target [work + "/job.sh"] (work + "/out") (work + "/err")) 0'
__ush_match_10__len='1'
# shellcheck disable=SC2034
__ush_match_10__0="${__ush_binding_21_work}"'/job.sh'
__ush_value_12="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect "${__ush_binding_17_target}" "$(ush_fn_std__pack __ush_match_10)" "${__ush_binding_21_work}"'/out' "${__ush_binding_21_work}"'/err')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_12}" 0 )
__ush_at 61 'assert_eq (read_text (work + "/out")) "42\n43\n41\n41"'
__ush_value_13="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_21_work}"'/out')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_13}" '42'"${__ush_lf}"'43'"${__ush_lf}"'41'"${__ush_lf}"'41' )
__ush_at 62 'assert_eq (test -s (work + "/err")) false'
__ush_value_14="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-s' "${__ush_binding_21_work}"'/err')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_14}" 'false' )
__ush_at 64 'write_text (work + "/job.ush") """'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__write_text "${__ush_binding_21_work}"'/job.ush' 'fn build() -> Int {'"${__ush_lf}"'  fail 23 "missing input"'"${__ush_lf}"'}'"${__ush_lf}"'let job = async build'"${__ush_lf}"'let sibling = async {'"${__ush_lf}"'  let value = job.await'"${__ush_lf}"'  value'"${__ush_lf}"'}'"${__ush_lf}"'print sibling.await' )
__ush_at 75 'compile $compiler $target $work'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_compile "${__ush_binding_13_compiler}" "${__ush_binding_17_target}" "${__ush_binding_21_work}" )
__ush_at 76 'assert_eq (redirect $target [work + "/job.sh"] (work + "/out") (work + "/err")) 23'
__ush_match_15__len='1'
# shellcheck disable=SC2034
__ush_match_15__0="${__ush_binding_21_work}"'/job.sh'
__ush_value_17="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__redirect "${__ush_binding_17_target}" "$(ush_fn_std__pack __ush_match_15)" "${__ush_binding_21_work}"'/out' "${__ush_binding_21_work}"'/err')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_17}" 23 )
__ush_at 77 'assert_eq (test -s (work + "/out")) false'
__ush_value_18="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__test '-s' "${__ush_binding_21_work}"'/out')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_18}" 'false' )
__ush_at 78 'assert_eq (read_text (work + "/err")) """'
__ush_value_19="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${__ush_binding_21_work}"'/err')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_19}" 'job.ush:2: missing input (exit 23)'"${__ush_lf}"'  fail 23 "missing input"'"${__ush_lf}"'job.ush:6: command failed (exit 23)'"${__ush_lf}"'  let value = job.await'"${__ush_lf}"'job.ush:9: command failed (exit 23)'"${__ush_lf}"'  print sibling.await' )
