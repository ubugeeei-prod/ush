#!/bin/sh
# Generated from 'test_rustc_wrapper.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: test_rustc_wrapper.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile 'test_rustc_wrapper.ush' --checked-in -o 'test_rustc_wrapper.sh'
set -eu

__ush_lf='
'
__ush_entrypoint=$0
case $__ush_entrypoint in
  */*) ;;
  *) if [ -f "$__ush_entrypoint" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v "$__ush_entrypoint"); fi ;;
esac
__ush_source_dir=$(CDPATH='' cd -P "$(dirname "$__ush_entrypoint")" && pwd -P)
__ush_source_path=$__ush_source_dir/'test_rustc_wrapper.ush'
__ush_origin='test_rustc_wrapper.ush'

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
  -h|--help) printf '%s' 'test_rustc_wrapper.ush

Usage:
  test_rustc_wrapper.ush [--help]
  test_rustc_wrapper.ush [--man [ITEM]]
  test_rustc_wrapper.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
test_rustc_wrapper.ush

SYNOPSIS
test_rustc_wrapper.ush [--help]
test_rustc_wrapper.ush [--man [ITEM]]
test_rustc_wrapper.ush [--complete [PREFIX]]
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

ush_fn_std__env__set() {
  __ush_std_env_validate "$1" || return "$?"
  export "$1=$2"
}

ush_fn_std__env__unset() {
  __ush_std_env_validate "$1" || return "$?"
  unset "$1"
}

ush_fn_std__fs__write_text() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  printf '%s' "$2" > "$__ush_path"
}

ush_fn_std__fs__mkdir_p() {
  __ush_path=$(ush_fn_std__path__resolve "$1")
  mkdir -p "$__ush_path"
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

ush_fn_std__test__assert_eq() {
  if [ "$1" = "$2" ]; then return 0; fi
  printf 'assert_eq failed\n  actual: %s\n  expected: %s\n' "$1" "$2" >&2
  return 1
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

ush_fn_std__string__join() {
  __ush_join_separator=$2
  __ush_join_items__len=0
  __ush_join_item=''
  ush_fn_std__unpack __ush_join_items "$1" || return "$?"
  __ush_join_index=0
  while [ "$__ush_join_index" -lt "$__ush_join_items__len" ]; do
    [ "$__ush_join_index" -eq 0 ] || printf '%s' "$__ush_join_separator"
    eval "__ush_join_item=\${__ush_join_items__${__ush_join_index}}"
    printf '%s' "$__ush_join_item"
    __ush_join_index=$((__ush_join_index + 1))
  done
}

__ush_std_fs_path() {
  __ush_path=$(ush_fn_std__path__resolve "$1" && printf .) || return "$?"
  __ush_path=${__ush_path%.}
  [ -n "$__ush_path" ] || { printf '%s\n' 'std::fs: path must not be empty' >&2; return 1; }
  case $__ush_path in .|..|/*|./*|../*) ;; *) __ush_path=./$__ush_path ;; esac
}

ush_fn_std__fs__chmod() {
  case $2 in [0-7][0-7][0-7]|[0-7][0-7][0-7][0-7]) ;; *)
    printf '%s\n' 'std::fs::chmod: mode must contain three or four octal digits' >&2; return 1 ;;
  esac
  __ush_std_fs_path "$1" || return "$?"
  chmod "$2" "$__ush_path"
}

__ush_at 10 'let work = temp_dir'
__ush_task_init
__ush_at 10 'let work = temp_dir'
__ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__temp_dir)" || exit "$?"
__ush_binding_10_work="${__ush_value_0}"
# shellcheck disable=SC2034
work="${__ush_binding_10_work}"
__ush_at 13 'let wrapper = work + "/scripts/launchers/rustc.sh"'
__ush_binding_13_wrapper="${__ush_binding_10_work}"'/scripts/launchers/rustc.sh'
# shellcheck disable=SC2034
wrapper="${__ush_binding_13_wrapper}"
__ush_at 14 'mkdir_p (work + "/scripts/launchers")'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__mkdir_p "${__ush_binding_10_work}"'/scripts/launchers' )
__ush_at 15 'cp (resolve (source_root + "/launchers/rustc.sh")) $wrapper'
__ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__source_root)" || exit "$?"
__ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__resolve "${__ush_value_1}"'/launchers/rustc.sh')" || exit "$?"
command cp "${__ush_value_2}" "${__ush_binding_13_wrapper}"
__ush_at 16 'chmod $wrapper "755"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__chmod "${__ush_binding_13_wrapper}" '755' )
__ush_at 17 'let rustc = work + "/fake-rustc"'
__ush_binding_17_rustc="${__ush_binding_10_work}"'/fake-rustc'
# shellcheck disable=SC2034
rustc="${__ush_binding_17_rustc}"
__ush_at 18 'write_text $rustc "#!/bin/sh\nprintf '"'"'bootstrap=%s\\n'"'"' \"${RUSTC_BOOTSTRAP:-}\"\nprintf '"'"'%s\\n'"'"' \"$@\"\n"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__write_text "${__ush_binding_17_rustc}" '#!/bin/sh'"${__ush_lf}"'printf '"'"'bootstrap=%s\n'"'"' "${RUSTC_BOOTSTRAP:-}"'"${__ush_lf}"'printf '"'"'%s\n'"'"' "$@"'"${__ush_lf}"'' )
__ush_at 19 'chmod $rustc "755"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__chmod "${__ush_binding_17_rustc}" '755' )
__ush_at 20 'let kache = work + "/fake-kache"'
__ush_binding_20_kache="${__ush_binding_10_work}"'/fake-kache'
# shellcheck disable=SC2034
kache="${__ush_binding_20_kache}"
__ush_at 21 'write_text $kache "#!/bin/sh\necho via-kache\nexec \"$@\"\n"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__write_text "${__ush_binding_20_kache}" '#!/bin/sh'"${__ush_lf}"'echo via-kache'"${__ush_lf}"'exec "$@"'"${__ush_lf}"'' )
__ush_at 22 'chmod $kache "755"'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__fs__chmod "${__ush_binding_20_kache}" '755' )
__ush_at 24 'for name in ["RUSTC_BOOTSTRAP", "USH_KACHE", "USH_RUSTC_FRONTEND_JOBS", "CARGO_MANIFEST_DIR", "CARGO_CRATE_NAME"] {'
__ush_match_3__len='5'
# shellcheck disable=SC2034
__ush_match_3__0='RUSTC_BOOTSTRAP'
# shellcheck disable=SC2034
__ush_match_3__1='USH_KACHE'
# shellcheck disable=SC2034
__ush_match_3__2='USH_RUSTC_FRONTEND_JOBS'
# shellcheck disable=SC2034
__ush_match_3__3='CARGO_MANIFEST_DIR'
# shellcheck disable=SC2034
__ush_match_3__4='CARGO_CRATE_NAME'
__ush_for_list_index_4=0
while [ "${__ush_for_list_index_4}" -lt "${__ush_match_3__len}" ]; do
  __ush_for_value_5=''
  eval "__ush_for_value_5=\"\${__ush_match_3__${__ush_for_list_index_4}}\""
  name="${__ush_for_value_5}"
  __ush_for_list_index_4=$((__ush_for_list_index_4 + 1))
  __ush_at 25 'unset $name'
  ush_fn_std__env__unset "${name}"
done
__ush_at 27 'let member = work + "/crates/ush_shell"'
__ush_binding_27_member="${__ush_binding_10_work}"'/crates/ush_shell'
# shellcheck disable=SC2034
member="${__ush_binding_27_member}"
__ush_at 30 'set USH_RUSTC_FRONTEND_JOBS "8"'
ush_fn_std__env__set 'USH_RUSTC_FRONTEND_JOBS' '8'
__ush_at 31 'set CARGO_MANIFEST_DIR $member'
ush_fn_std__env__set 'CARGO_MANIFEST_DIR' "${__ush_binding_27_member}"
__ush_at 32 'set CARGO_CRATE_NAME ush_shell'
ush_fn_std__env__set 'CARGO_CRATE_NAME' 'ush_shell'
__ush_at 33 'assert_eq (output $wrapper [rustc, "--crate-name", "ush_shell", "--edition=2024", "src/lib.rs"]) (join ["bootstrap=ush_shell", "--crate-name", "ush_shell", "--edition=2024", "src/lib.rs", "-Zunstable-options", "--jobs-frontend=8"] "\n")'
__ush_match_6__len='5'
# shellcheck disable=SC2034
__ush_match_6__0="${__ush_binding_17_rustc}"
# shellcheck disable=SC2034
__ush_match_6__1='--crate-name'
# shellcheck disable=SC2034
__ush_match_6__2='ush_shell'
# shellcheck disable=SC2034
__ush_match_6__3='--edition=2024'
# shellcheck disable=SC2034
__ush_match_6__4='src/lib.rs'
__ush_value_8="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${__ush_binding_13_wrapper}" "$(ush_fn_std__pack __ush_match_6)")" || exit "$?"
__ush_match_9__len='7'
# shellcheck disable=SC2034
__ush_match_9__0='bootstrap=ush_shell'
# shellcheck disable=SC2034
__ush_match_9__1='--crate-name'
# shellcheck disable=SC2034
__ush_match_9__2='ush_shell'
# shellcheck disable=SC2034
__ush_match_9__3='--edition=2024'
# shellcheck disable=SC2034
__ush_match_9__4='src/lib.rs'
# shellcheck disable=SC2034
__ush_match_9__5='-Zunstable-options'
# shellcheck disable=SC2034
__ush_match_9__6='--jobs-frontend=8'
__ush_value_11="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__join "$(ush_fn_std__pack __ush_match_9)" ''"${__ush_lf}"'')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_8}" "${__ush_value_11}" )
__ush_at 35 'assert_eq (output $wrapper [rustc, "rustc", "--crate-name", "ush_shell", "src/lib.rs"]) (join ["bootstrap=ush_shell", "rustc", "--crate-name", "ush_shell", "src/lib.rs", "-Zunstable-options", "--jobs-frontend=8"] "\n")'
__ush_match_12__len='5'
# shellcheck disable=SC2034
__ush_match_12__0="${__ush_binding_17_rustc}"
# shellcheck disable=SC2034
__ush_match_12__1='rustc'
# shellcheck disable=SC2034
__ush_match_12__2='--crate-name'
# shellcheck disable=SC2034
__ush_match_12__3='ush_shell'
# shellcheck disable=SC2034
__ush_match_12__4='src/lib.rs'
__ush_value_14="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${__ush_binding_13_wrapper}" "$(ush_fn_std__pack __ush_match_12)")" || exit "$?"
__ush_match_15__len='7'
# shellcheck disable=SC2034
__ush_match_15__0='bootstrap=ush_shell'
# shellcheck disable=SC2034
__ush_match_15__1='rustc'
# shellcheck disable=SC2034
__ush_match_15__2='--crate-name'
# shellcheck disable=SC2034
__ush_match_15__3='ush_shell'
# shellcheck disable=SC2034
__ush_match_15__4='src/lib.rs'
# shellcheck disable=SC2034
__ush_match_15__5='-Zunstable-options'
# shellcheck disable=SC2034
__ush_match_15__6='--jobs-frontend=8'
__ush_value_17="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__join "$(ush_fn_std__pack __ush_match_15)" ''"${__ush_lf}"'')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_14}" "${__ush_value_17}" )
__ush_at 36 'echo "rustc wrapper: checkout crate passed"'
printf '%s\n' 'rustc wrapper: checkout crate passed'
__ush_at 39 'assert_eq (output $wrapper [rustc, "-vV"]) (join ["bootstrap=", "-vV"] "\n")'
__ush_match_18__len='2'
# shellcheck disable=SC2034
__ush_match_18__0="${__ush_binding_17_rustc}"
# shellcheck disable=SC2034
__ush_match_18__1='-vV'
__ush_value_20="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${__ush_binding_13_wrapper}" "$(ush_fn_std__pack __ush_match_18)")" || exit "$?"
__ush_match_21__len='2'
# shellcheck disable=SC2034
__ush_match_21__0='bootstrap='
# shellcheck disable=SC2034
__ush_match_21__1='-vV'
__ush_value_23="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__join "$(ush_fn_std__pack __ush_match_21)" ''"${__ush_lf}"'')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_20}" "${__ush_value_23}" )
__ush_at 42 'set CARGO_CRATE_NAME build_script_build'
ush_fn_std__env__set 'CARGO_CRATE_NAME' 'build_script_build'
__ush_at 43 'assert_eq (output $wrapper [rustc, "--crate-name", "probe", "--crate-type=lib", "-"]) (join ["bootstrap=", "--crate-name", "probe", "--crate-type=lib", "-"] "\n")'
__ush_match_24__len='5'
# shellcheck disable=SC2034
__ush_match_24__0="${__ush_binding_17_rustc}"
# shellcheck disable=SC2034
__ush_match_24__1='--crate-name'
# shellcheck disable=SC2034
__ush_match_24__2='probe'
# shellcheck disable=SC2034
__ush_match_24__3='--crate-type=lib'
# shellcheck disable=SC2034
__ush_match_24__4='-'
__ush_value_26="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${__ush_binding_13_wrapper}" "$(ush_fn_std__pack __ush_match_24)")" || exit "$?"
__ush_match_27__len='5'
# shellcheck disable=SC2034
__ush_match_27__0='bootstrap='
# shellcheck disable=SC2034
__ush_match_27__1='--crate-name'
# shellcheck disable=SC2034
__ush_match_27__2='probe'
# shellcheck disable=SC2034
__ush_match_27__3='--crate-type=lib'
# shellcheck disable=SC2034
__ush_match_27__4='-'
__ush_value_29="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__join "$(ush_fn_std__pack __ush_match_27)" ''"${__ush_lf}"'')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_26}" "${__ush_value_29}" )
__ush_at 44 'echo "rustc wrapper: build-script probe passed"'
printf '%s\n' 'rustc wrapper: build-script probe passed'
__ush_at 47 'set CARGO_MANIFEST_DIR /home/runner/.cargo/registry/src/index.crates.io-0000/memchr-2.7.6'
ush_fn_std__env__set 'CARGO_MANIFEST_DIR' '/home/runner/.cargo/registry/src/index.crates.io-0000/memchr-2.7.6'
__ush_at 48 'set CARGO_CRATE_NAME memchr'
ush_fn_std__env__set 'CARGO_CRATE_NAME' 'memchr'
__ush_at 49 'assert_eq (output $wrapper [rustc, "--crate-name", "memchr", "src/lib.rs"]) (join ["bootstrap=", "--crate-name", "memchr", "src/lib.rs"] "\n")'
__ush_match_30__len='4'
# shellcheck disable=SC2034
__ush_match_30__0="${__ush_binding_17_rustc}"
# shellcheck disable=SC2034
__ush_match_30__1='--crate-name'
# shellcheck disable=SC2034
__ush_match_30__2='memchr'
# shellcheck disable=SC2034
__ush_match_30__3='src/lib.rs'
__ush_value_32="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${__ush_binding_13_wrapper}" "$(ush_fn_std__pack __ush_match_30)")" || exit "$?"
__ush_match_33__len='4'
# shellcheck disable=SC2034
__ush_match_33__0='bootstrap='
# shellcheck disable=SC2034
__ush_match_33__1='--crate-name'
# shellcheck disable=SC2034
__ush_match_33__2='memchr'
# shellcheck disable=SC2034
__ush_match_33__3='src/lib.rs'
__ush_value_35="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__join "$(ush_fn_std__pack __ush_match_33)" ''"${__ush_lf}"'')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_32}" "${__ush_value_35}" )
__ush_at 51 'set CARGO_MANIFEST_DIR (work + "-base/crates/ush_shell")'
ush_fn_std__env__set 'CARGO_MANIFEST_DIR' "${__ush_binding_10_work}"'-base/crates/ush_shell'
__ush_at 52 'set CARGO_CRATE_NAME ush_shell'
ush_fn_std__env__set 'CARGO_CRATE_NAME' 'ush_shell'
__ush_at 53 'assert_eq (output $wrapper [rustc, "--crate-name", "ush_shell", "src/lib.rs"]) (join ["bootstrap=", "--crate-name", "ush_shell", "src/lib.rs"] "\n")'
__ush_match_36__len='4'
# shellcheck disable=SC2034
__ush_match_36__0="${__ush_binding_17_rustc}"
# shellcheck disable=SC2034
__ush_match_36__1='--crate-name'
# shellcheck disable=SC2034
__ush_match_36__2='ush_shell'
# shellcheck disable=SC2034
__ush_match_36__3='src/lib.rs'
__ush_value_38="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${__ush_binding_13_wrapper}" "$(ush_fn_std__pack __ush_match_36)")" || exit "$?"
__ush_match_39__len='4'
# shellcheck disable=SC2034
__ush_match_39__0='bootstrap='
# shellcheck disable=SC2034
__ush_match_39__1='--crate-name'
# shellcheck disable=SC2034
__ush_match_39__2='ush_shell'
# shellcheck disable=SC2034
__ush_match_39__3='src/lib.rs'
__ush_value_41="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__join "$(ush_fn_std__pack __ush_match_39)" ''"${__ush_lf}"'')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_38}" "${__ush_value_41}" )
__ush_at 54 'echo "rustc wrapper: dependencies passed"'
printf '%s\n' 'rustc wrapper: dependencies passed'
__ush_at 57 'set CARGO_MANIFEST_DIR $member'
ush_fn_std__env__set 'CARGO_MANIFEST_DIR' "${__ush_binding_27_member}"
__ush_at 58 'for jobs in ["1", "0", ""] {'
__ush_match_42__len='3'
# shellcheck disable=SC2034
__ush_match_42__0='1'
# shellcheck disable=SC2034
__ush_match_42__1='0'
# shellcheck disable=SC2034
__ush_match_42__2=''
__ush_for_list_index_43=0
while [ "${__ush_for_list_index_43}" -lt "${__ush_match_42__len}" ]; do
  __ush_for_value_44=''
  eval "__ush_for_value_44=\"\${__ush_match_42__${__ush_for_list_index_43}}\""
  jobs="${__ush_for_value_44}"
  __ush_for_list_index_43=$((__ush_for_list_index_43 + 1))
  __ush_at 59 'set USH_RUSTC_FRONTEND_JOBS $jobs'
  ush_fn_std__env__set 'USH_RUSTC_FRONTEND_JOBS' "${jobs}"
  __ush_at 60 'assert_eq (output $wrapper [rustc, "--crate-name", "ush_shell", "src/lib.rs"]) (join ["bootstrap=", "--crate-name", "ush_shell", "src/lib.rs"] "\n")'
  __ush_match_45__len='4'
  # shellcheck disable=SC2034
  __ush_match_45__0="${__ush_binding_17_rustc}"
  # shellcheck disable=SC2034
  __ush_match_45__1='--crate-name'
  # shellcheck disable=SC2034
  __ush_match_45__2='ush_shell'
  # shellcheck disable=SC2034
  __ush_match_45__3='src/lib.rs'
  __ush_value_47="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${__ush_binding_13_wrapper}" "$(ush_fn_std__pack __ush_match_45)")" || exit "$?"
  __ush_match_48__len='4'
  # shellcheck disable=SC2034
  __ush_match_48__0='bootstrap='
  # shellcheck disable=SC2034
  __ush_match_48__1='--crate-name'
  # shellcheck disable=SC2034
  __ush_match_48__2='ush_shell'
  # shellcheck disable=SC2034
  __ush_match_48__3='src/lib.rs'
  __ush_value_50="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__join "$(ush_fn_std__pack __ush_match_48)" ''"${__ush_lf}"'')" || exit "$?"
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_47}" "${__ush_value_50}" )
done
__ush_at 62 'unset USH_RUSTC_FRONTEND_JOBS'
ush_fn_std__env__unset 'USH_RUSTC_FRONTEND_JOBS'
__ush_at 63 'assert_eq (output $wrapper [rustc, "--crate-name", "ush_shell", "src/lib.rs"]) (join ["bootstrap=", "--crate-name", "ush_shell", "src/lib.rs"] "\n")'
__ush_match_51__len='4'
# shellcheck disable=SC2034
__ush_match_51__0="${__ush_binding_17_rustc}"
# shellcheck disable=SC2034
__ush_match_51__1='--crate-name'
# shellcheck disable=SC2034
__ush_match_51__2='ush_shell'
# shellcheck disable=SC2034
__ush_match_51__3='src/lib.rs'
__ush_value_53="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${__ush_binding_13_wrapper}" "$(ush_fn_std__pack __ush_match_51)")" || exit "$?"
__ush_match_54__len='4'
# shellcheck disable=SC2034
__ush_match_54__0='bootstrap='
# shellcheck disable=SC2034
__ush_match_54__1='--crate-name'
# shellcheck disable=SC2034
__ush_match_54__2='ush_shell'
# shellcheck disable=SC2034
__ush_match_54__3='src/lib.rs'
__ush_value_56="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__join "$(ush_fn_std__pack __ush_match_54)" ''"${__ush_lf}"'')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_53}" "${__ush_value_56}" )
__ush_at 64 'echo "rustc wrapper: frontend off passed"'
printf '%s\n' 'rustc wrapper: frontend off passed'
__ush_at 67 'set USH_KACHE $kache'
ush_fn_std__env__set 'USH_KACHE' "${__ush_binding_20_kache}"
__ush_at 68 'set USH_RUSTC_FRONTEND_JOBS "4"'
ush_fn_std__env__set 'USH_RUSTC_FRONTEND_JOBS' '4'
__ush_at 69 'assert_eq (output $wrapper [rustc, "--crate-name", "ush_shell", "src/lib.rs"]) (join ["via-kache", "bootstrap=ush_shell", "--crate-name", "ush_shell", "src/lib.rs", "-Zunstable-options", "--jobs-frontend=4"] "\n")'
__ush_match_57__len='4'
# shellcheck disable=SC2034
__ush_match_57__0="${__ush_binding_17_rustc}"
# shellcheck disable=SC2034
__ush_match_57__1='--crate-name'
# shellcheck disable=SC2034
__ush_match_57__2='ush_shell'
# shellcheck disable=SC2034
__ush_match_57__3='src/lib.rs'
__ush_value_59="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${__ush_binding_13_wrapper}" "$(ush_fn_std__pack __ush_match_57)")" || exit "$?"
__ush_match_60__len='7'
# shellcheck disable=SC2034
__ush_match_60__0='via-kache'
# shellcheck disable=SC2034
__ush_match_60__1='bootstrap=ush_shell'
# shellcheck disable=SC2034
__ush_match_60__2='--crate-name'
# shellcheck disable=SC2034
__ush_match_60__3='ush_shell'
# shellcheck disable=SC2034
__ush_match_60__4='src/lib.rs'
# shellcheck disable=SC2034
__ush_match_60__5='-Zunstable-options'
# shellcheck disable=SC2034
__ush_match_60__6='--jobs-frontend=4'
__ush_value_62="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__join "$(ush_fn_std__pack __ush_match_60)" ''"${__ush_lf}"'')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_59}" "${__ush_value_62}" )
__ush_at 70 'assert_eq (output $wrapper [rustc, "-vV"]) (join ["via-kache", "bootstrap=", "-vV"] "\n")'
__ush_match_63__len='2'
# shellcheck disable=SC2034
__ush_match_63__0="${__ush_binding_17_rustc}"
# shellcheck disable=SC2034
__ush_match_63__1='-vV'
__ush_value_65="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${__ush_binding_13_wrapper}" "$(ush_fn_std__pack __ush_match_63)")" || exit "$?"
__ush_match_66__len='3'
# shellcheck disable=SC2034
__ush_match_66__0='via-kache'
# shellcheck disable=SC2034
__ush_match_66__1='bootstrap='
# shellcheck disable=SC2034
__ush_match_66__2='-vV'
__ush_value_68="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__join "$(ush_fn_std__pack __ush_match_66)" ''"${__ush_lf}"'')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_65}" "${__ush_value_68}" )
__ush_at 71 'set USH_KACHE ""'
ush_fn_std__env__set 'USH_KACHE' ''
__ush_at 72 'assert_eq (output $wrapper [rustc, "-vV"]) (join ["bootstrap=", "-vV"] "\n")'
__ush_match_69__len='2'
# shellcheck disable=SC2034
__ush_match_69__0="${__ush_binding_17_rustc}"
# shellcheck disable=SC2034
__ush_match_69__1='-vV'
__ush_value_71="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__process__output "${__ush_binding_13_wrapper}" "$(ush_fn_std__pack __ush_match_69)")" || exit "$?"
__ush_match_72__len='2'
# shellcheck disable=SC2034
__ush_match_72__0='bootstrap='
# shellcheck disable=SC2034
__ush_match_72__1='-vV'
__ush_value_74="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__join "$(ush_fn_std__pack __ush_match_72)" ''"${__ush_lf}"'')" || exit "$?"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__test__assert_eq "${__ush_value_71}" "${__ush_value_74}" )
__ush_at 73 'echo "rustc wrapper: kache passed"'
printf '%s\n' 'rustc wrapper: kache passed'
