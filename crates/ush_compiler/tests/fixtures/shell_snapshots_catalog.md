# Compiler Shell Snapshots

- `async_blocks.ush` -> `async_blocks.sh`
- `control_flow.ush` -> `control_flow.sh`
- `core_runtime.ush` -> `core_runtime.sh`
- `docs_bin.ush` -> `docs_bin.sh`
- `error_streams.ush` -> `error_streams.sh`
- `source_relative_paths.ush` -> `source_relative_paths.sh`
- `stdlib_and_shell.ush` -> `stdlib_and_shell.sh`
- `tail_expressions.ush` -> `tail_expressions.sh`
- `traits_and_methods.ush` -> `traits_and_methods.sh`
- `typed_cli_and_types.ush` -> `typed_cli_and_types.sh`

## `async_blocks.ush` -> `async_blocks.sh`

```ush
fn worker(message: String) -> String {
  message
}

print "main"
let task = async worker "worker"
let block = async {
  let prefix = "work"
  prefix + "bench"
}
let number = async {
  if true {
    return 40
  }
  0
}
print "after"
let result = task.await
let next = block.await
let value = number.await
print result
print next
print value + 2
```

```sh
#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/async_blocks.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/async_blocks.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/async_blocks.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/async_blocks.ush'

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

case "${1:-}" in
  -h|--help) printf '%s' 'async_blocks.ush

Usage:
  async_blocks.ush [--help]
  async_blocks.ush [--man [ITEM]]
  async_blocks.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
async_blocks.ush

SYNOPSIS
async_blocks.ush [--help]
async_blocks.ush [--man [ITEM]]
async_blocks.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

ush_fn_worker() {
  __ush_fn_worker_arg_0="${1}"
  # shellcheck disable=SC2034
  message="${__ush_fn_worker_arg_0}"
  __ush_at 2 'message'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_fn_worker_arg_0}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_fn_worker_arg_0}" >&4
  fi
  return 0
}

__ush_at 5 'print "main"'
printf '%s\n' 'main'
__ush_at 6 'let task = async worker "worker"'
__ush_task_init
__ush_at 6 'let task = async worker "worker"'
__ush_task_slot
__ush_task_0__result=$__ush_slot/result
: > "${__ush_task_0__result}"
: > "${__ush_task_0__result}.status"
( __ush_task_scope "${__ush_task_0__result}.status"; __ush_return_path="${__ush_task_0__result}"; ush_fn_worker 'worker' ) &
__ush_task_0__pid=$!
__ush_task_0__awaited=0
__ush_jobs="${__ush_jobs} $!"
__ush_at 7 'let block = async {'
ush_fn___ush_task_block_1() {
  __ush_at 8 'let prefix = "work"'
  prefix='work'
  __ush_at 9 'prefix + "bench"'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${prefix}"'bench' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${prefix}"'bench' >&4
  fi
  return 0
}
__ush_task_slot
__ush_task_2__result=$__ush_slot/result
: > "${__ush_task_2__result}"
: > "${__ush_task_2__result}.status"
( __ush_task_scope "${__ush_task_2__result}.status"; __ush_return_path="${__ush_task_2__result}"; ush_fn___ush_task_block_1 ) &
__ush_task_2__pid=$!
__ush_task_2__awaited=0
__ush_jobs="${__ush_jobs} $!"
__ush_at 11 'let number = async {'
ush_fn___ush_task_block_3() {
  __ush_at 12 'if true {'
  if {
  [ 'true' = 'true' ]
  }; then
    __ush_at 13 'return 40'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 40 > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 40 >&4
    fi
    return 0
  fi
  __ush_at 15 '0'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' 0 > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' 0 >&4
  fi
  return 0
}
__ush_task_slot
__ush_task_4__result=$__ush_slot/result
: > "${__ush_task_4__result}"
: > "${__ush_task_4__result}.status"
( __ush_task_scope "${__ush_task_4__result}.status"; __ush_return_path="${__ush_task_4__result}"; ush_fn___ush_task_block_3 ) &
__ush_task_4__pid=$!
__ush_task_4__awaited=0
__ush_jobs="${__ush_jobs} $!"
__ush_at 17 'print "after"'
printf '%s\n' 'after'
__ush_at 18 'let result = task.await'
if [ "${__ush_task_0__awaited}" = '0' ]; then
  __ush_wait_job "${__ush_task_0__pid}" "${__ush_task_0__result}.status" || exit "$?"
  __ush_task_0__awaited='1'
fi
result="$(cat "${__ush_task_0__result}")"
__ush_at 19 'let next = block.await'
if [ "${__ush_task_2__awaited}" = '0' ]; then
  __ush_wait_job "${__ush_task_2__pid}" "${__ush_task_2__result}.status" || exit "$?"
  __ush_task_2__awaited='1'
fi
next="$(cat "${__ush_task_2__result}")"
__ush_at 20 'let value = number.await'
if [ "${__ush_task_4__awaited}" = '0' ]; then
  __ush_wait_job "${__ush_task_4__pid}" "${__ush_task_4__result}.status" || exit "$?"
  __ush_task_4__awaited='1'
fi
value="$(cat "${__ush_task_4__result}")"
__ush_at 21 'print result'
printf '%s\n' "${result}"
__ush_at 22 'print next'
printf '%s\n' "${next}"
__ush_at 23 'print value + 2'
printf '%s\n' $((value + 2))
```

## `control_flow.ush` -> `control_flow.sh`

```ush
fn pick(flag: Bool) -> String {
  if flag {
    "yes"
  }
  else {
    "no"
  }
}

print $ pick true

for item in 0..3 {
  print item
}

let items = [3, 4]
for item in items {
  print item
}

let pair = (5, 6)
for item in pair {
  print item
}

let count = 0
while count < 2 {
  print count
  let count = count + 1
}

loop {
  print 9
  break
}

enum Option {
  None,
  Some(Int),
}

let maybe = Option::Some(7)
if let Option::Some(it) = maybe && it == 7 {
  print it
}
```

```sh
#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/control_flow.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/control_flow.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/control_flow.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/control_flow.ush'

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
trap '__ush_report "$?"' 0

case "${1:-}" in
  -h|--help) printf '%s' 'control_flow.ush

Usage:
  control_flow.ush [--help]
  control_flow.ush [--man [ITEM]]
  control_flow.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
control_flow.ush

SYNOPSIS
control_flow.ush [--help]
control_flow.ush [--man [ITEM]]
control_flow.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

ush_fn_pick() {
  __ush_fn_pick_arg_0="${1}"
  # shellcheck disable=SC2034
  flag="${__ush_fn_pick_arg_0}"
  __ush_at 2 'if flag {'
  if {
  [ "${__ush_fn_pick_arg_0}" = 'true' ]
  }; then
    __ush_at 3 '"yes"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'yes' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'yes' >&4
    fi
    return 0
  else
    __ush_at 6 '"no"'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'no' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'no' >&4
    fi
    return 0
  fi
}

__ush_at 10 'print $ pick true'
__ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_pick 'true' 4>&1 >&3)" || exit "$?"
printf '%s\n' "${__ush_value_0}"
__ush_at 12 'for item in 0..3 {'
__ush_for_index_1=0
while [ "${__ush_for_index_1}" -lt 3 ]; do
  item="${__ush_for_index_1}"
  __ush_for_index_1=$((__ush_for_index_1 + 1))
  __ush_at 13 'print item'
  printf '%s\n' $((item))
done
__ush_at 16 'let items = [3, 4]'
items__len='2'
# shellcheck disable=SC2034
items__0=3
# shellcheck disable=SC2034
items__1=4
__ush_at 17 'for item in items {'
__ush_match_2__len="${items__len}"
__ush_copy_index_3=0
while [ "$__ush_copy_index_3" -lt "${items__len}" ]; do
  eval "__ush_copy_value_4=\"\${items__$__ush_copy_index_3}\""
  eval "__ush_match_2__${__ush_copy_index_3}=\$__ush_copy_value_4"
  __ush_copy_index_3=$((__ush_copy_index_3 + 1))
done
__ush_for_list_index_5=0
while [ "${__ush_for_list_index_5}" -lt "${__ush_match_2__len}" ]; do
  __ush_for_value_6=''
  eval "__ush_for_value_6=\"\${__ush_match_2__${__ush_for_list_index_5}}\""
  item="${__ush_for_value_6}"
  __ush_for_list_index_5=$((__ush_for_list_index_5 + 1))
  __ush_at 18 'print item'
  printf '%s\n' $((item))
done
__ush_at 21 'let pair = (5, 6)'
pair__len='2'
# shellcheck disable=SC2034
pair__0=5
# shellcheck disable=SC2034
pair__1=6
__ush_at 22 'for item in pair {'
__ush_match_7__len='2'
__ush_match_7__0="${pair__0}"
__ush_match_7__1="${pair__1}"
item=$((__ush_match_7__0))
__ush_at 23 'print item'
printf '%s\n' $((item))
item=$((__ush_match_7__1))
__ush_at 23 'print item'
printf '%s\n' $((item))
__ush_at 26 'let count = 0'
count=0
__ush_at 27 'while count < 2 {'
while {
[ "$(if [ $((count)) -lt 2 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; do
  __ush_at 28 'print count'
  printf '%s\n' $((count))
  __ush_at 29 'let count = count + 1'
  count=$((count + 1))
done
__ush_at 32 'loop {'
while :; do
  __ush_at 33 'print 9'
  printf '%s\n' 9
  __ush_at 34 'break'
  break
done
__ush_at 42 'let maybe = Option::Some(7)'
maybe__tag='Option::Some'
# shellcheck disable=SC2034
maybe__0=7
__ush_at 43 'if let Option::Some(it) = maybe && it == 7 {'
if {
{
it="${maybe__0-}"
[ "${maybe__tag}" = 'Option::Some' ]
} && {
[ "$(if [ $((it)) -eq 7 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}
}; then
  __ush_at 44 'print it'
  printf '%s\n' $((it))
fi
```

## `core_runtime.ush` -> `core_runtime.sh`

```ush
enum Response {
  Ok(String),
  Err(String),
}

enum Envelope {
  Wrap(Response),
  Empty,
}

fn greet(message: String, count: Int) -> String {
  message + ":" + count
}

fn wrap(message: String) -> String {
  "<" + message + ">"
}

let wrapped = Envelope::Wrap(Response::Ok("done"))
match wrapped {
  Envelope::Wrap(Response::Ok(message)) => print $ wrap message
  _ => print "fallback"
}

let value = greet "hello" 2
print value
print $ wrap (greet "team" 3)
```

```sh
#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/core_runtime.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/core_runtime.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/core_runtime.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/core_runtime.ush'

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
trap '__ush_report "$?"' 0

case "${1:-}" in
  -h|--help) printf '%s' 'core_runtime.ush

Usage:
  core_runtime.ush [--help]
  core_runtime.ush [--man [ITEM]]
  core_runtime.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
core_runtime.ush

SYNOPSIS
core_runtime.ush [--help]
core_runtime.ush [--man [ITEM]]
core_runtime.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

ush_fn_greet() {
  __ush_fn_greet_arg_0="${1}"
  # shellcheck disable=SC2034
  message="${__ush_fn_greet_arg_0}"
  __ush_fn_greet_arg_1="${2}"
  # shellcheck disable=SC2034
  count="${__ush_fn_greet_arg_1}"
  __ush_at 12 'message + ":" + count'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_fn_greet_arg_0}"':'"${__ush_fn_greet_arg_1}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_fn_greet_arg_0}"':'"${__ush_fn_greet_arg_1}" >&4
  fi
  return 0
}

ush_fn_wrap() {
  __ush_fn_wrap_arg_0="${1}"
  # shellcheck disable=SC2034
  message="${__ush_fn_wrap_arg_0}"
  __ush_at 16 '"<" + message + ">"'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' '<'"${__ush_fn_wrap_arg_0}"'>' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' '<'"${__ush_fn_wrap_arg_0}"'>' >&4
  fi
  return 0
}

__ush_at 19 'let wrapped = Envelope::Wrap(Response::Ok("done"))'
wrapped__tag='Envelope::Wrap'
wrapped__0__tag='Response::Ok'
# shellcheck disable=SC2034
wrapped__0__0='done'
__ush_at 20 'match wrapped {'
if [ "${wrapped__tag}" = 'Envelope::Wrap' ] && [ "${wrapped__0__tag}" = 'Response::Ok' ]; then
  message="${wrapped__0__0-}"
  __ush_at 21 'Envelope::Wrap(Response::Ok(message)) => print $ wrap message'
  __ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_wrap "${message}" 4>&1 >&3)" || exit "$?"
  printf '%s\n' "${__ush_value_0}"
elif :; then
  __ush_at 22 '_ => print "fallback"'
  printf '%s\n' 'fallback'
fi
__ush_at 25 'let value = greet "hello" 2'
__ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_greet 'hello' 2 4>&1 >&3)" || exit "$?"
value="${__ush_value_1}"
__ush_at 26 'print value'
printf '%s\n' "${value}"
__ush_at 27 'print $ wrap (greet "team" 3)'
__ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_greet 'team' 3 4>&1 >&3)" || exit "$?"
__ush_value_3="$(__ush_return_path='' __ush_capture_return='1' ush_fn_wrap "${__ush_value_2}" 4>&1 >&3)" || exit "$?"
printf '%s\n' "${__ush_value_3}"
```

## `docs_bin.ush` -> `docs_bin.sh`

```ush
#| Demo CLI.
#|
#| A snapshot fixture for generated docs and wrapper code.
#| @usage docs_bin.ush --man greet
#| @note Generated docs stay available through --help and --man.
#| @see docs_bin.ush --help
#| @example docs_bin.ush --help

#| Greet a user.
#| @param name target user
#| @return greeting text
fn greet(name: String) -> String {
  "hi " + name
}

#| Run the CLI.
#| @param name target user
#| @param count repeat count
fn bin(#[alias("n")] name: String, #[default(2)] count: Int, verbose: Bool) {
  print $ greet name
  print count
  print verbose
}
```

```sh
#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/docs_bin.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/docs_bin.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/docs_bin.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/docs_bin.ush'

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
trap '__ush_report "$?"' 0

__ush_print_help() {
  cat <<'__USH_DOC__'
docs_bin.ush - Demo CLI.

Usage:
  docs_bin.ush [--help]
  docs_bin.ush [--man [ITEM]]
  docs_bin.ush [--complete [PREFIX]]
  docs_bin.ush --man greet

Description:

  A snapshot fixture for generated docs and wrapper code.

Notes:
  - Generated docs stay available through --help and --man.

Examples:
  docs_bin.ush --help

See also:
  - docs_bin.ush --help

Documented items:
  fn greet(name: String) -> String
      Greet a user.
  fn bin(#[alias("n")] name: String, #[default(2)] count: Int, verbose: Bool)
      Run the CLI.
__USH_DOC__
}

__ush_print_man() {
  case "${1:-}" in
    '')
      cat <<'__USH_MAIN__'
NAME
docs_bin.ush - Demo CLI.

SYNOPSIS
docs_bin.ush [--help]
docs_bin.ush [--man [ITEM]]
docs_bin.ush [--complete [PREFIX]]
docs_bin.ush --man greet

DESCRIPTION
Demo CLI.

A snapshot fixture for generated docs and wrapper code.

NOTES
Generated docs stay available through --help and --man.

EXAMPLES
docs_bin.ush --help

FUNCTIONS
greet
  fn greet(name: String) -> String
  Greet a user.
bin
  fn bin(#[alias("n")] name: String, #[default(2)] count: Int, verbose: Bool)
  Run the CLI.

SEE ALSO
docs_bin.ush --help
__USH_MAIN__
      ;;
    'greet')
      cat <<'__USH_GREET__'
NAME
docs_bin.ush greet

SIGNATURE
fn greet(name: String) -> String

DESCRIPTION
Greet a user.

PARAMETERS
name - target user

RETURNS
greeting text
__USH_GREET__
      ;;
    'bin')
      cat <<'__USH_BIN__'
NAME
docs_bin.ush bin

SIGNATURE
fn bin(#[alias("n")] name: String, #[default(2)] count: Int, verbose: Bool)

DESCRIPTION
Run the CLI.

PARAMETERS
name - target user
count - repeat count
__USH_BIN__
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
bin
complete
greet
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

exec 3>&1

ush_fn_greet() {
  __ush_fn_greet_arg_0="${1}"
  # shellcheck disable=SC2034
  name="${__ush_fn_greet_arg_0}"
  __ush_at 13 '"hi " + name'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' 'hi '"${__ush_fn_greet_arg_0}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' 'hi '"${__ush_fn_greet_arg_0}" >&4
  fi
  return 0
}

ush_fn_bin() {
  __ush_fn_bin_arg_0="${1}"
  # shellcheck disable=SC2034
  name="${__ush_fn_bin_arg_0}"
  __ush_fn_bin_arg_1="${2}"
  # shellcheck disable=SC2034
  count="${__ush_fn_bin_arg_1}"
  __ush_fn_bin_arg_2="${3}"
  # shellcheck disable=SC2034
  verbose="${__ush_fn_bin_arg_2}"
  __ush_at 20 'print $ greet name'
  __ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_greet "${__ush_fn_bin_arg_0}" 4>&1 >&3)" || return "$?"
  printf '%s\n' "${__ush_value_0}"
  __ush_at 21 'print count'
  printf '%s\n' $((__ush_fn_bin_arg_1))
  __ush_at 22 'print verbose'
  printf '%s\n' "${__ush_fn_bin_arg_2}"
}
```

## `error_streams.ush` -> `error_streams.sh`

```ush
enum Problem {
  Nope,
}

fn fail() -> String / Problem, stderr {
  fail Problem::Nope
}

fn wrap(message: String) -> String {
  "<" + message + ">"
}

fn outer() -> String / Problem, stderr {
  let value = fail()?
  value
}

fn mixed() -> String / Problem, unknown, exec, fs, env, net, io {
  $ false
  wrap(outer()?)
}

fn awaited() -> String / Problem, stderr, task {
  let task = async fail ()
  let value = task.await
  value
}

print $ wrap $ awaited ()
```

```sh
#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/error_streams.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/error_streams.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/error_streams.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/error_streams.ush'

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

case "${1:-}" in
  -h|--help) printf '%s' 'error_streams.ush

Usage:
  error_streams.ush [--help]
  error_streams.ush [--man [ITEM]]
  error_streams.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
error_streams.ush

SYNOPSIS
error_streams.ush [--help]
error_streams.ush [--man [ITEM]]
error_streams.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

# raises: Problem
ush_fn_fail() {
  __ush_at 6 'fail Problem::Nope'
  __ush_report 1 'Problem::Nope'
  return 1
}

ush_fn_wrap() {
  __ush_fn_wrap_arg_0="${1}"
  # shellcheck disable=SC2034
  message="${__ush_fn_wrap_arg_0}"
  __ush_at 10 '"<" + message + ">"'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' '<'"${__ush_fn_wrap_arg_0}"'>' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' '<'"${__ush_fn_wrap_arg_0}"'>' >&4
  fi
  return 0
}

# raises: Problem
ush_fn_outer() {
  __ush_at 14 'let value = fail()?'
  __ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_fail 4>&1 >&3)" || return "$?"
  value="${__ush_value_0}"
  __ush_at 15 'value'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${value}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${value}" >&4
  fi
  return 0
}

# raises: Problem | unknown
ush_fn_mixed() {
  __ush_at 19 '$ false'
  {
false
  } || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 20 'wrap(outer()?)'
  __ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_outer 4>&1 >&3)" || return "$?"
  __ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_wrap "${__ush_value_1}" 4>&1 >&3)" || return "$?"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_value_2}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_value_2}" >&4
  fi
  return 0
}

# raises: Problem
ush_fn_awaited() {
  (
    __ush_task_scope
    __ush_at 24 'let task = async fail ()'
    __ush_task_slot
    __ush_task_3__result=$__ush_slot/result
    : > "${__ush_task_3__result}"
    : > "${__ush_task_3__result}.status"
    ( __ush_task_scope "${__ush_task_3__result}.status"; __ush_return_path="${__ush_task_3__result}"; ush_fn_fail ) &
    __ush_task_3__pid=$!
    __ush_task_3__awaited=0
    __ush_jobs="${__ush_jobs} $!"
    __ush_at 25 'let value = task.await'
    if [ "${__ush_task_3__awaited}" = '0' ]; then
      __ush_wait_job "${__ush_task_3__pid}" "${__ush_task_3__result}.status" || exit "$?"
      __ush_task_3__awaited='1'
    fi
    value="$(cat "${__ush_task_3__result}")"
    __ush_at 26 'value'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' "${value}" > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' "${value}" >&4
    fi
    return 0
  )
}

__ush_at 29 'print $ wrap $ awaited ()'
__ush_task_init
__ush_at 29 'print $ wrap $ awaited ()'
__ush_value_4="$(__ush_return_path='' __ush_capture_return='1' ush_fn_awaited 4>&1 >&3)" || exit "$?"
__ush_value_5="$(__ush_return_path='' __ush_capture_return='1' ush_fn_wrap "${__ush_value_4}" 4>&1 >&3)" || exit "$?"
printf '%s\n' "${__ush_value_5}"
```

## `source_relative_paths.ush` -> `source_relative_paths.sh`

```ush
use std::http::{download, get}
use std::path::{from_cwd, from_source, tmpfile}

let source_root = from_source "."
let payload = source_root.join("payload.txt")
let cwd_file = from_cwd "notes.txt"
let url = "file://" + payload.resolve()
let copy = tmpfile()

print payload
print $ cwd_file.resolve()
print $ get url
download url copy
copy.remove()
```

```sh
#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/source_relative_paths.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/source_relative_paths.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/source_relative_paths.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/source_relative_paths.ush'

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
__ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__from_source '.')" || exit "$?"
source_root="${__ush_value_0}"
__ush_at 5 'let payload = source_root.join("payload.txt")'
__ush_value_1=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__join "${source_root}" 'payload.txt')
payload="${__ush_value_1}"
__ush_at 6 'let cwd_file = from_cwd "notes.txt"'
__ush_value_2="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__from_cwd 'notes.txt')" || exit "$?"
cwd_file="${__ush_value_2}"
__ush_at 7 'let url = "file://" + payload.resolve()'
__ush_value_3=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__resolve "${payload}")
url='file://'"${__ush_value_3}"
__ush_at 8 'let copy = tmpfile()'
__ush_value_4="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__tmpfile)" || exit "$?"
copy="${__ush_value_4}"
__ush_at 10 'print payload'
printf '%s\n' "${payload}"
__ush_at 11 'print $ cwd_file.resolve()'
__ush_value_5=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__resolve "${cwd_file}")
printf '%s\n' "${__ush_value_5}"
__ush_at 12 'print $ get url'
__ush_value_6="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__http__get "${url}")" || exit "$?"
printf '%s\n' "${__ush_value_6}"
__ush_at 13 'download url copy'
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__http__download "${url}" "${copy}" )
__ush_at 14 'copy.remove()'
__ush_value_7=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__remove "${copy}")
```

## `stdlib_and_shell.ush` -> `stdlib_and_shell.sh`

```ush
use std::command::{capture, status}
use std::env::{get_or, set as env_set}
use std::path::{join, tmpfile}
use std::regex::capture as regex_capture
use std::string::{replace, starts_with}

let page = """
  <div>
    hello
  </div>
"""

env_set "USH_STD_IMPORT" "ready"
let path = tmpfile()
path.write_text("alpha")
path.append_text(":beta")

let greeting = "hello"
$ printf '%s\n' "$greeting"

match greeting {
  "hello" => $ printf '%s\n' matched
  _ => print "fallback"
}

print page
print $ page.starts_with("<div>")
print $ page.ends_with("</div>")
print $ get_or "USH_STD_IMPORT" "fallback"
print $ join "/tmp/" "/ush"
print $ replace "hello world" "world" "ush"
print $ starts_with "ush" "u"
print $ capture "printf '%s\n' hello"
print $ status "exit 7"
print $ regex_capture("release-v0.3.4", "v([0-9.]+)", 1)
print $ path.read_text()
shell "printf '%s\n' from-shell"
path.remove()
```

```sh
#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/stdlib_and_shell.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/stdlib_and_shell.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_lf='
'
__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/stdlib_and_shell.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/stdlib_and_shell.ush'

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
  __ush_std_env_validate "$1" || return "$?"
  __ush_default=$2
  eval "printf '%s' \"\${$1-\${__ush_default}}\""
}

ush_fn_std__env__set() {
  __ush_std_env_validate "$1" || return "$?"
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
page='<div>'"${__ush_lf}"'  hello'"${__ush_lf}"'</div>'
__ush_at 13 'env_set "USH_STD_IMPORT" "ready"'
ush_fn_std__env__set 'USH_STD_IMPORT' 'ready'
__ush_at 14 'let path = tmpfile()'
__ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__tmpfile)" || exit "$?"
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
__ush_value_5="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__env__get_or 'USH_STD_IMPORT' 'fallback')" || exit "$?"
printf '%s\n' "${__ush_value_5}"
__ush_at 30 'print $ join "/tmp/" "/ush"'
__ush_value_6="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__path__join '/tmp/' '/ush')" || exit "$?"
printf '%s\n' "${__ush_value_6}"
__ush_at 31 'print $ replace "hello world" "world" "ush"'
__ush_value_7="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__replace 'hello world' 'world' 'ush')" || exit "$?"
printf '%s\n' "${__ush_value_7}"
__ush_at 32 'print $ starts_with "ush" "u"'
__ush_value_8="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__string__starts_with 'ush' 'u')" || exit "$?"
printf '%s\n' "${__ush_value_8}"
__ush_at 33 'print $ capture "printf '"'"'%s\n'"'"' hello"'
__ush_value_9="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__capture 'printf '"'"'%s'"${__ush_lf}"''"'"' hello')" || exit "$?"
printf '%s\n' "${__ush_value_9}"
__ush_at 34 'print $ status "exit 7"'
__ush_value_10="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__command__status 'exit 7')" || exit "$?"
printf '%s\n' $((__ush_value_10))
__ush_at 35 'print $ regex_capture("release-v0.3.4", "v([0-9.]+)", 1)'
__ush_value_11="$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__regex__capture 'release-v0.3.4' 'v([0-9.]+)' 1)" || exit "$?"
printf '%s\n' "${__ush_value_11}"
__ush_at 36 'print $ path.read_text()'
__ush_value_12=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__read_text "${path}")
printf '%s\n' "${__ush_value_12}"
__ush_at 37 'shell "printf '"'"'%s\n'"'"' from-shell"'
printf '%s
' from-shell
__ush_at 38 'path.remove()'
__ush_value_13=$(__ush_return_path='' __ush_capture_return='1' ush_fn_std__fs__remove "${path}")
```

## `tail_expressions.ush` -> `tail_expressions.sh`

```ush
fn trace() -> String {
  print "trace"
  "value"
}

fn choose(flag: Bool) -> String {
  match flag {
    true => "yes",
    _ => "no",
  }
}

fn run() -> String {
  trace();
  choose true
}

print $ run ()
```

```sh
#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/tail_expressions.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/tail_expressions.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/tail_expressions.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/tail_expressions.ush'

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
trap '__ush_report "$?"' 0

case "${1:-}" in
  -h|--help) printf '%s' 'tail_expressions.ush

Usage:
  tail_expressions.ush [--help]
  tail_expressions.ush [--man [ITEM]]
  tail_expressions.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
tail_expressions.ush

SYNOPSIS
tail_expressions.ush [--help]
tail_expressions.ush [--man [ITEM]]
tail_expressions.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

ush_fn_trace() {
  __ush_at 2 'print "trace"'
  printf '%s\n' 'trace'
  __ush_at 3 '"value"'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' 'value' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' 'value' >&4
  fi
  return 0
}

ush_fn_choose() {
  __ush_fn_choose_arg_0="${1}"
  # shellcheck disable=SC2034
  flag="${__ush_fn_choose_arg_0}"
  __ush_at 7 'match flag {'
  if [ "${__ush_fn_choose_arg_0}" = "true" ]; then
    __ush_at 8 'true => "yes",'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'yes' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'yes' >&4
    fi
    return 0
  elif :; then
    __ush_at 9 '_ => "no",'
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'no' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'no' >&4
    fi
    return 0
  fi
}

ush_fn_run() {
  __ush_at 14 'trace();'
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_trace ) || { __ush_status=$?; __ush_report "$__ush_status"; return "$__ush_status"; }
  __ush_at 15 'choose true'
  __ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_choose 'true' 4>&1 >&3)" || return "$?"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_value_0}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_value_0}" >&4
  fi
  return 0
}

__ush_at 18 'print $ run ()'
__ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_run 4>&1 >&3)" || exit "$?"
printf '%s\n' "${__ush_value_1}"
```

## `traits_and_methods.ush` -> `traits_and_methods.sh`

```ush
trait Named {}

type User {
  name: String,
  age: Int,
}

impl Named for User {}

impl Display for User {
  fn fmt(self) -> String {
    self.name + ":" + self.age
  }
}

impl User {
  fn prefixed(self, prefix: String) -> String {
    prefix + ":" + self.name
  }
}

let user = User { name: "ush", age: 7 }
print user
print format user
print user.prefixed("id")
print () == ()
print "ant" < "bee"
```

```sh
#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/traits_and_methods.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/traits_and_methods.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/traits_and_methods.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/traits_and_methods.ush'

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
trap '__ush_report "$?"' 0

case "${1:-}" in
  -h|--help) printf '%s' 'traits_and_methods.ush

Usage:
  traits_and_methods.ush [--help]
  traits_and_methods.ush [--man [ITEM]]
  traits_and_methods.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
traits_and_methods.ush

SYNOPSIS
traits_and_methods.ush [--help]
traits_and_methods.ush [--man [ITEM]]
traits_and_methods.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

__ush_walk_record_4_User() {
  "$1" "$2__tag" || return "$?"
  eval "__ush_tag=\${${2}__tag}"
  case $__ush_tag in
    'User::User')
      "$1" "$2__name" || return "$?"
      "$1" "$2__age" || return "$?"
      ;;
    *) return 1 ;;
  esac
}

ush_fn_std__write_cell() {
  eval "__ush_cell=\${$1}"
  printf x
  printf '%s' "$__ush_cell" | od -A n -v -t x1 | tr -d ' \n'
  printf ' '
}

ush_fn_std__read_cell() {
  case $__ush_cells in x*' '*) ;; *) return 1 ;; esac
  __ush_hex=${__ush_cells%% *}
  __ush_cells=${__ush_cells#* }
  __ush_cell=$(LC_ALL=C awk -v s="${__ush_hex#x}" 'BEGIN {
    digits = "0123456789abcdef"
    for (i = 1; i <= length(s); i += 2)
      printf "%c", (index(digits, substr(s,i,1))-1)*16 + index(digits,substr(s,i+1,1))-1
    printf "."
  }') || return "$?"
  __ush_cell=${__ush_cell%.}
  eval "$1=\$__ush_cell"
}

__ush_at 22 'let user = User { name: "ush", age: 7 }'
user__tag='User::User'
# shellcheck disable=SC2034
user__name='ush'
# shellcheck disable=SC2034
user__age=7
__ush_at 23 'print user'
__ush_cells="$(__ush_walk_record_4_User ush_fn_std__write_cell user)"
__ush_walk_record_4_User ush_fn_std__read_cell __ush_match_0
__ush_method_1() {
__ush_at 12 'self.name + ":" + self.age'
if [ -n "${__ush_return_path:-}" ]; then
  printf '%s' "${__ush_match_0__name}"':'"${__ush_match_0__age}" > "$__ush_return_path"
elif [ "${__ush_capture_return:-0}" = '1' ]; then
  printf '%s' "${__ush_match_0__name}"':'"${__ush_match_0__age}" >&4
fi
return 0
}
__ush_value_4="$(__ush_capture_return='1' __ush_return_path='' __ush_method_1 4>&1 >&3)"
printf '%s\n' "${__ush_value_4}"
__ush_at 24 'print format user'
__ush_cells="$(__ush_walk_record_4_User ush_fn_std__write_cell user)"
__ush_walk_record_4_User ush_fn_std__read_cell __ush_match_5
__ush_method_6() {
__ush_at 12 'self.name + ":" + self.age'
if [ -n "${__ush_return_path:-}" ]; then
  printf '%s' "${__ush_match_5__name}"':'"${__ush_match_5__age}" > "$__ush_return_path"
elif [ "${__ush_capture_return:-0}" = '1' ]; then
  printf '%s' "${__ush_match_5__name}"':'"${__ush_match_5__age}" >&4
fi
return 0
}
__ush_value_9="$(__ush_capture_return='1' __ush_return_path='' __ush_method_6 4>&1 >&3)"
__ush_value_10="${__ush_value_9}"
printf '%s\n' "${__ush_value_10}"
__ush_at 25 'print user.prefixed("id")'
__ush_cells="$(__ush_walk_record_4_User ush_fn_std__write_cell user)"
__ush_walk_record_4_User ush_fn_std__read_cell __ush_match_11
__ush_prefix_12='id'
__ush_method_13() {
__ush_at 18 'prefix + ":" + self.name'
if [ -n "${__ush_return_path:-}" ]; then
  printf '%s' "${__ush_prefix_12}"':'"${__ush_match_11__name}" > "$__ush_return_path"
elif [ "${__ush_capture_return:-0}" = '1' ]; then
  printf '%s' "${__ush_prefix_12}"':'"${__ush_match_11__name}" >&4
fi
return 0
}
__ush_value_15="$(__ush_capture_return='1' __ush_return_path='' __ush_method_13 4>&1 >&3)"
__ush_value_16="${__ush_value_15}"
printf '%s\n' "${__ush_value_16}"
__ush_at 26 'print () == ()'
printf '%s\n' "$(if :; then printf '%s' true; else printf '%s' false; fi)"
__ush_at 27 'print "ant" < "bee"'
printf '%s\n' "$(if [ 'ant' != 'bee' ] && [ "$(printf '%s\n%s\n' 'ant' 'bee' | LC_ALL=C sort | head -n 1)" = 'ant' ]; then printf '%s' true; else printf '%s' false; fi)"
```

## `typed_cli_and_types.ush` -> `typed_cli_and_types.sh`

```ush
fn greet(name: String, #[default(2)] count: Int) -> String {
  name + ":" + count
}

type User {
  name: String,
  age: Int,
}

alias ll = "ls -la"

let user = User { name: "ush", age: 7 }
print $ greet count: 3 name: "ush"
print $ greet name: "mini"
match user {
  User { name, age } => print name + ":" + age
  _ => print "fallback"
}
```

```sh
#!/bin/sh
# Generated from '$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/typed_cli_and_types.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: file://$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/typed_cli_and_types.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
set -eu

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/typed_cli_and_types.ush'
__ush_origin='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/typed_cli_and_types.ush'

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
trap '__ush_report "$?"' 0

case "${1:-}" in
  -h|--help) printf '%s' 'typed_cli_and_types.ush

Usage:
  typed_cli_and_types.ush [--help]
  typed_cli_and_types.ush [--man [ITEM]]
  typed_cli_and_types.ush [--complete [PREFIX]]
'; exit 0 ;;
  man|--man)
    if [ -n "${2:-}" ]; then printf 'No documented item named: %s\n' "$2" >&2; exit 1; fi
    printf '%s' 'NAME
typed_cli_and_types.ush

SYNOPSIS
typed_cli_and_types.ush [--help]
typed_cli_and_types.ush [--man [ITEM]]
typed_cli_and_types.ush [--complete [PREFIX]]
'; exit 0 ;;
  complete|--complete)
    for item in --complete --help --man -h complete man; do
      case $item in "${2:-}"*) printf '%s\n' "$item" ;; esac
    done
    exit 0 ;;
esac

exec 3>&1

ush_fn_greet() {
  __ush_fn_greet_arg_0="${1}"
  # shellcheck disable=SC2034
  name="${__ush_fn_greet_arg_0}"
  __ush_fn_greet_arg_1="${2}"
  # shellcheck disable=SC2034
  count="${__ush_fn_greet_arg_1}"
  __ush_at 2 'name + ":" + count'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_fn_greet_arg_0}"':'"${__ush_fn_greet_arg_1}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_fn_greet_arg_0}"':'"${__ush_fn_greet_arg_1}" >&4
  fi
  return 0
}

__ush_at 10 'alias ll = "ls -la"'
alias ll='ls -la'
__ush_at 12 'let user = User { name: "ush", age: 7 }'
user__tag='User::User'
# shellcheck disable=SC2034
user__name='ush'
# shellcheck disable=SC2034
user__age=7
__ush_at 13 'print $ greet count: 3 name: "ush"'
__ush_value_0="$(__ush_return_path='' __ush_capture_return='1' ush_fn_greet 'ush' 3 4>&1 >&3)" || exit "$?"
printf '%s\n' "${__ush_value_0}"
__ush_at 14 'print $ greet name: "mini"'
__ush_value_1="$(__ush_return_path='' __ush_capture_return='1' ush_fn_greet 'mini' 2 4>&1 >&3)" || exit "$?"
printf '%s\n' "${__ush_value_1}"
__ush_at 15 'match user {'
if [ "${user__tag}" = 'User::User' ]; then
  name="${user__name-}"
  age="${user__age-}"
  __ush_at 16 'User { name, age } => print name + ":" + age'
  printf '%s\n' "${name}"':'"${age}"
elif :; then
  __ush_at 17 '_ => print "fallback"'
  printf '%s\n' 'fallback'
fi
```

