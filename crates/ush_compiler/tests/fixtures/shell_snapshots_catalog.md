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
set -eu

__ush_jobs=''
__ush_task_seq='0'
__ush_task_files=''
__ush_tmp_seq='0'

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/async_blocks.ush'

__ush_print_help() {
  cat <<'__USH_DOC__'
async_blocks.ush

Usage:
  async_blocks.ush [--help]
  async_blocks.ush [--man [ITEM]]
  async_blocks.ush [--complete [PREFIX]]
__USH_DOC__
}

__ush_print_man() {
  case "${1:-}" in
    '')
      cat <<'__USH_MAIN__'
NAME
async_blocks.ush

SYNOPSIS
async_blocks.ush [--help]
async_blocks.ush [--man [ITEM]]
async_blocks.ush [--complete [PREFIX]]
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

ush_fn_worker() {
  __ush_fn_worker_arg_0="$1"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_fn_worker_arg_0}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_fn_worker_arg_0}"
  fi
  return 0
}

printf '%s\n' 'main'
__ush_task_seq=$((__ush_task_seq + 1))
__ush_task_0__result="${TMPDIR:-/tmp}/__ush_task_0.$$.$__ush_task_seq"
: > "${__ush_task_0__result}"
__ush_task_files="${__ush_task_files}${__ush_task_files:+ }${__ush_task_0__result}"
( __ush_return_path="${__ush_task_0__result}"; ush_fn_worker 'worker' ) &
__ush_task_0__pid="$!"
__ush_task_0__awaited='0'
__ush_jobs="${__ush_jobs}${__ush_jobs:+ }$!"
ush_fn___ush_task_block_1() {
  prefix='work'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "$(printf '%s' "${prefix}" 'bench')" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "$(printf '%s' "${prefix}" 'bench')"
  fi
  return 0
}
__ush_task_seq=$((__ush_task_seq + 1))
__ush_task_2__result="${TMPDIR:-/tmp}/__ush_task_2.$$.$__ush_task_seq"
: > "${__ush_task_2__result}"
__ush_task_files="${__ush_task_files}${__ush_task_files:+ }${__ush_task_2__result}"
( __ush_return_path="${__ush_task_2__result}"; ush_fn___ush_task_block_1 ) &
__ush_task_2__pid="$!"
__ush_task_2__awaited='0'
__ush_jobs="${__ush_jobs}${__ush_jobs:+ }$!"
ush_fn___ush_task_block_3() {
  if {
  [ 'true' = 'true' ]
  }; then
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 40 > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 40
    fi
    return 0
  fi
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' 0 > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' 0
  fi
  return 0
}
__ush_task_seq=$((__ush_task_seq + 1))
__ush_task_4__result="${TMPDIR:-/tmp}/__ush_task_4.$$.$__ush_task_seq"
: > "${__ush_task_4__result}"
__ush_task_files="${__ush_task_files}${__ush_task_files:+ }${__ush_task_4__result}"
( __ush_return_path="${__ush_task_4__result}"; ush_fn___ush_task_block_3 ) &
__ush_task_4__pid="$!"
__ush_task_4__awaited='0'
__ush_jobs="${__ush_jobs}${__ush_jobs:+ }$!"
printf '%s\n' 'after'
if [ "${__ush_task_0__awaited}" = '0' ]; then
  wait "${__ush_task_0__pid}"
  __ush_task_0__awaited='1'
fi
result="$(cat "${__ush_task_0__result}")"
if [ "${__ush_task_2__awaited}" = '0' ]; then
  wait "${__ush_task_2__pid}"
  __ush_task_2__awaited='1'
fi
next="$(cat "${__ush_task_2__result}")"
if [ "${__ush_task_4__awaited}" = '0' ]; then
  wait "${__ush_task_4__pid}"
  __ush_task_4__awaited='1'
fi
value="$(cat "${__ush_task_4__result}")"
printf '%s\n' "${result}"
printf '%s\n' "${next}"
printf '%s\n' $((value + 2))

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
set -eu

__ush_jobs=''
__ush_task_seq='0'
__ush_task_files=''
__ush_tmp_seq='0'

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/control_flow.ush'

__ush_print_help() {
  cat <<'__USH_DOC__'
control_flow.ush

Usage:
  control_flow.ush [--help]
  control_flow.ush [--man [ITEM]]
  control_flow.ush [--complete [PREFIX]]
__USH_DOC__
}

__ush_print_man() {
  case "${1:-}" in
    '')
      cat <<'__USH_MAIN__'
NAME
control_flow.ush

SYNOPSIS
control_flow.ush [--help]
control_flow.ush [--man [ITEM]]
control_flow.ush [--complete [PREFIX]]
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

ush_fn_pick() {
  __ush_fn_pick_arg_0="$1"
  if {
  [ "${__ush_fn_pick_arg_0}" = 'true' ]
  }; then
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'yes' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'yes'
    fi
    return 0
  else
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'no' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'no'
    fi
    return 0
  fi
}

__ush_value_0="$(__ush_capture_return='1' ush_fn_pick 'true')"
printf '%s\n' "${__ush_value_0}"
__ush_for_index_1=0
while [ "${__ush_for_index_1}" -lt 3 ]; do
  item="${__ush_for_index_1}"
  __ush_for_index_1=$((__ush_for_index_1 + 1))
  printf '%s\n' $((item))
done
items__len='2'
items__0=3
items__1=4
__ush_match_2__len="${items__len}"
__ush_copy_index_3=0
while [ "$__ush_copy_index_3" -lt "${items__len}" ]; do
  eval "__ush_copy_value_4=\"\${items__$__ush_copy_index_3}\""
  eval "__ush_match_2__$__ush_copy_index_3=\"$__ush_copy_value_4\""
  __ush_copy_index_3=$((__ush_copy_index_3 + 1))
done
__ush_for_list_index_5=0
while [ "${__ush_for_list_index_5}" -lt "${__ush_match_2__len}" ]; do
  eval "__ush_for_value_6=\"\${__ush_match_2__${__ush_for_list_index_5}}\""
  item="${__ush_for_value_6}"
  __ush_for_list_index_5=$((__ush_for_list_index_5 + 1))
  printf '%s\n' $((item))
done
pair__len='2'
pair__0=5
pair__1=6
__ush_match_7__len='2'
__ush_match_7__0="${pair__0}"
__ush_match_7__1="${pair__1}"
item=$((__ush_match_7__0))
printf '%s\n' $((item))
item=$((__ush_match_7__1))
printf '%s\n' $((item))
count=0
while {
[ "$(if [ $((count)) -lt 2 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; do
  printf '%s\n' $((count))
  count=$((count + 1))
done
while :; do
  printf '%s\n' 9
  break
done
maybe__tag='Option::Some'
maybe__0=7
if {
__ush_match_8__tag="${maybe__tag}"
case "${maybe__tag}" in
  'Option::None')
    ;;
  'Option::Some')
__ush_match_8__0="${maybe__0}"
    ;;
esac
it="${__ush_match_8__0-}"
[ "${__ush_match_8__tag}" = 'Option::Some' ]
} && {
[ "$(if [ $((it)) -eq 7 ]; then printf '%s' true; else printf '%s' false; fi)" = 'true' ]
}; then
  printf '%s\n' $((it))
fi

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
set -eu

__ush_jobs=''
__ush_task_seq='0'
__ush_task_files=''
__ush_tmp_seq='0'

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/docs_bin.ush'

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
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "$(printf '%s' 'hi ' "${__ush_fn_greet_arg_0}")" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "$(printf '%s' 'hi ' "${__ush_fn_greet_arg_0}")"
  fi
  return 0
}

ush_fn_bin() {
  __ush_fn_bin_arg_0="$1"
  __ush_fn_bin_arg_1="$2"
  __ush_fn_bin_arg_2="$3"
  __ush_value_0="$(__ush_capture_return='1' ush_fn_greet "${__ush_fn_bin_arg_0}")"
  printf '%s\n' "${__ush_value_0}"
  printf '%s\n' $((__ush_fn_bin_arg_1))
  printf '%s\n' "${__ush_fn_bin_arg_2}"
}


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
```

## `error_streams.ush` -> `error_streams.sh`

```ush
enum Problem {
  Nope,
}

fn fail() -> Problem!String {
  raise Problem::Nope
}

fn wrap(message: String) -> String {
  "<" + message + ">"
}

fn outer() -> Problem!String {
  let value = fail()?
  value
}

fn mixed() -> (Problem | unknown)!String {
  $ false
  $ wrap $ outer ()
}

fn awaited() -> Problem!String {
  let task = async fail ()
  let value = task.await
  value
}

print $ wrap $ awaited ()
```

```sh
#!/bin/sh
set -eu

__ush_jobs=''
__ush_task_seq='0'
__ush_task_files=''
__ush_tmp_seq='0'

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/error_streams.ush'

__ush_print_help() {
  cat <<'__USH_DOC__'
error_streams.ush

Usage:
  error_streams.ush [--help]
  error_streams.ush [--man [ITEM]]
  error_streams.ush [--complete [PREFIX]]
__USH_DOC__
}

__ush_print_man() {
  case "${1:-}" in
    '')
      cat <<'__USH_MAIN__'
NAME
error_streams.ush

SYNOPSIS
error_streams.ush [--help]
error_streams.ush [--man [ITEM]]
error_streams.ush [--complete [PREFIX]]
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

# raises: Problem
ush_fn_fail() {
  printf '%s\n' 'ush raise: Problem' >&2
  return 1
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

# raises: Problem
ush_fn_outer() {
  __ush_value_0="$(__ush_capture_return='1' ush_fn_fail)" || return "$?"
  value="${__ush_value_0}"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${value}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${value}"
  fi
  return 0
}

# raises: Problem | unknown
ush_fn_mixed() {
  false
  wrap $ outer ()
}

# raises: Problem
ush_fn_awaited() {
  __ush_task_seq=$((__ush_task_seq + 1))
  __ush_task_1__result="${TMPDIR:-/tmp}/__ush_task_1.$$.$__ush_task_seq"
  : > "${__ush_task_1__result}"
  __ush_task_files="${__ush_task_files}${__ush_task_files:+ }${__ush_task_1__result}"
  ( __ush_return_path="${__ush_task_1__result}"; ush_fn_fail ) &
  __ush_task_1__pid="$!"
  __ush_task_1__awaited='0'
  __ush_jobs="${__ush_jobs}${__ush_jobs:+ }$!"
  if [ "${__ush_task_1__awaited}" = '0' ]; then
    wait "${__ush_task_1__pid}"
    __ush_task_1__awaited='1'
  fi
  value="$(cat "${__ush_task_1__result}")"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${value}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${value}"
  fi
  return 0
}

__ush_value_2="$(__ush_capture_return='1' ush_fn_awaited)"
__ush_value_3="$(__ush_capture_return='1' ush_fn_wrap "${__ush_value_2}")"
printf '%s\n' "${__ush_value_3}"

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
set -eu

__ush_jobs=''
__ush_task_seq='0'
__ush_task_files=''
__ush_tmp_seq='0'

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/source_relative_paths.ush'

__ush_print_help() {
  cat <<'__USH_DOC__'
source_relative_paths.ush

Usage:
  source_relative_paths.ush [--help]
  source_relative_paths.ush [--man [ITEM]]
  source_relative_paths.ush [--complete [PREFIX]]
__USH_DOC__
}

__ush_print_man() {
  case "${1:-}" in
    '')
      cat <<'__USH_MAIN__'
NAME
source_relative_paths.ush

SYNOPSIS
source_relative_paths.ush [--help]
source_relative_paths.ush [--man [ITEM]]
source_relative_paths.ush [--complete [PREFIX]]
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

__ush_value_0="$(__ush_capture_return='1' ush_fn_std__path__from_source '.')"
source_root="${__ush_value_0}"
__ush_value_1=$(__ush_capture_return='1' ush_fn_std__path__join "${source_root}" 'payload.txt')
payload="${__ush_value_1}"
__ush_value_2="$(__ush_capture_return='1' ush_fn_std__path__from_cwd 'notes.txt')"
cwd_file="${__ush_value_2}"
__ush_value_3=$(__ush_capture_return='1' ush_fn_std__path__resolve "${payload}")
url="$(printf '%s' 'file://' "${__ush_value_3}")"
__ush_value_4="$(__ush_capture_return='1' ush_fn_std__path__tmpfile)"
copy="${__ush_value_4}"
printf '%s\n' "${payload}"
__ush_value_5=$(__ush_capture_return='1' ush_fn_std__path__resolve "${cwd_file}")
printf '%s\n' "${__ush_value_5}"
__ush_value_6="$(__ush_capture_return='1' ush_fn_std__http__get "${url}")"
printf '%s\n' "${__ush_value_6}"
( __ush_capture_return='0' __ush_return_path=''; ush_fn_std__http__download "${url}" "${copy}" )
__ush_value_7=$(__ush_capture_return='1' ush_fn_std__fs__remove "${copy}")

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
set -eu

__ush_jobs=''
__ush_task_seq='0'
__ush_task_files=''
__ush_tmp_seq='0'

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/stdlib_and_shell.ush'

__ush_print_help() {
  cat <<'__USH_DOC__'
stdlib_and_shell.ush

Usage:
  stdlib_and_shell.ush [--help]
  stdlib_and_shell.ush [--man [ITEM]]
  stdlib_and_shell.ush [--complete [PREFIX]]
__USH_DOC__
}

__ush_print_man() {
  case "${1:-}" in
    '')
      cat <<'__USH_MAIN__'
NAME
stdlib_and_shell.ush

SYNOPSIS
stdlib_and_shell.ush [--help]
stdlib_and_shell.ush [--man [ITEM]]
stdlib_and_shell.ush [--complete [PREFIX]]
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

page='<div>
  hello
</div>'
ush_fn_std__env__set 'USH_STD_IMPORT' 'ready'
__ush_value_0="$(__ush_capture_return='1' ush_fn_std__path__tmpfile)"
path="${__ush_value_0}"
__ush_value_1=$(__ush_capture_return='1' ush_fn_std__fs__write_text "${path}" 'alpha')
__ush_value_2=$(__ush_capture_return='1' ush_fn_std__fs__append_text "${path}" ':beta')
greeting='hello'
printf '%s\n' "$greeting"
__ush_match_3="${greeting}"
if [ "${__ush_match_3}" = 'hello' ]; then
  printf '%s\n' matched
elif :; then
  printf '%s\n' 'fallback'
fi
printf '%s\n' "${page}"
__ush_value_4=$(__ush_capture_return='1' ush_fn_std__string__starts_with "${page}" '<div>')
printf '%s\n' "${__ush_value_4}"
__ush_value_5=$(__ush_capture_return='1' ush_fn_std__string__ends_with "${page}" '</div>')
printf '%s\n' "${__ush_value_5}"
__ush_value_6="$(__ush_capture_return='1' ush_fn_std__env__get_or 'USH_STD_IMPORT' 'fallback')"
printf '%s\n' "${__ush_value_6}"
__ush_value_7="$(__ush_capture_return='1' ush_fn_std__path__join '/tmp/' '/ush')"
printf '%s\n' "${__ush_value_7}"
__ush_value_8="$(__ush_capture_return='1' ush_fn_std__string__replace 'hello world' 'world' 'ush')"
printf '%s\n' "${__ush_value_8}"
__ush_value_9="$(__ush_capture_return='1' ush_fn_std__string__starts_with 'ush' 'u')"
printf '%s\n' "${__ush_value_9}"
__ush_value_10="$(__ush_capture_return='1' ush_fn_std__command__capture 'printf '"'"'%s\n'"'"' hello')"
printf '%s\n' "${__ush_value_10}"
__ush_value_11="$(__ush_capture_return='1' ush_fn_std__command__status 'exit 7')"
printf '%s\n' $((__ush_value_11))
__ush_value_12="$(__ush_capture_return='1' ush_fn_std__regex__capture 'release-v0.3.4' 'v([0-9.]+)' 1)"
printf '%s\n' "${__ush_value_12}"
__ush_value_13=$(__ush_capture_return='1' ush_fn_std__fs__read_text "${path}")
printf '%s\n' "${__ush_value_13}"
printf '%s\n' from-shell
__ush_value_14=$(__ush_capture_return='1' ush_fn_std__fs__remove "${path}")

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
set -eu

__ush_jobs=''
__ush_task_seq='0'
__ush_task_files=''
__ush_tmp_seq='0'

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/tail_expressions.ush'

__ush_print_help() {
  cat <<'__USH_DOC__'
tail_expressions.ush

Usage:
  tail_expressions.ush [--help]
  tail_expressions.ush [--man [ITEM]]
  tail_expressions.ush [--complete [PREFIX]]
__USH_DOC__
}

__ush_print_man() {
  case "${1:-}" in
    '')
      cat <<'__USH_MAIN__'
NAME
tail_expressions.ush

SYNOPSIS
tail_expressions.ush [--help]
tail_expressions.ush [--man [ITEM]]
tail_expressions.ush [--complete [PREFIX]]
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

ush_fn_trace() {
  printf '%s\n' 'trace'
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' 'value' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' 'value'
  fi
  return 0
}

ush_fn_choose() {
  __ush_fn_choose_arg_0="$1"
  __ush_match_0="${__ush_fn_choose_arg_0}"
  if [ "${__ush_match_0}" = "true" ]; then
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'yes' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'yes'
    fi
    return 0
  elif :; then
    if [ -n "${__ush_return_path:-}" ]; then
      printf '%s' 'no' > "$__ush_return_path"
    elif [ "${__ush_capture_return:-0}" = '1' ]; then
      printf '%s' 'no'
    fi
    return 0
  fi
}

ush_fn_run() {
  ( __ush_capture_return='0' __ush_return_path=''; ush_fn_trace )
  __ush_value_1="$(__ush_capture_return='1' ush_fn_choose 'true')"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_value_1}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_value_1}"
  fi
  return 0
}

__ush_value_2="$(__ush_capture_return='1' ush_fn_run)"
printf '%s\n' "${__ush_value_2}"

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
set -eu

__ush_jobs=''
__ush_task_seq='0'
__ush_task_files=''
__ush_tmp_seq='0'

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/traits_and_methods.ush'

__ush_print_help() {
  cat <<'__USH_DOC__'
traits_and_methods.ush

Usage:
  traits_and_methods.ush [--help]
  traits_and_methods.ush [--man [ITEM]]
  traits_and_methods.ush [--complete [PREFIX]]
__USH_DOC__
}

__ush_print_man() {
  case "${1:-}" in
    '')
      cat <<'__USH_MAIN__'
NAME
traits_and_methods.ush

SYNOPSIS
traits_and_methods.ush [--help]
traits_and_methods.ush [--man [ITEM]]
traits_and_methods.ush [--complete [PREFIX]]
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

user__tag='User::User'
user__name='ush'
user__age=7
__ush_match_0__tag="${user__tag}"
case "${user__tag}" in
  'User::User')
__ush_match_0__name="${user__name}"
__ush_match_0__age="${user__age}"
    ;;
esac
__ush_method_1() {
__ush_match_2__tag="${__ush_match_0__tag}"
case "${__ush_match_0__tag}" in
  'User::User')
__ush_match_2__name="${__ush_match_0__name}"
__ush_match_2__age="${__ush_match_0__age}"
    ;;
esac
__ush_match_4__tag="${__ush_match_0__tag}"
case "${__ush_match_0__tag}" in
  'User::User')
__ush_match_4__name="${__ush_match_0__name}"
__ush_match_4__age="${__ush_match_0__age}"
    ;;
esac
if [ -n "${__ush_return_path:-}" ]; then
  printf '%s' "$(printf '%s' "${__ush_match_2__name}" ':' "${__ush_match_4__age}")" > "$__ush_return_path"
elif [ "${__ush_capture_return:-0}" = '1' ]; then
  printf '%s' "$(printf '%s' "${__ush_match_2__name}" ':' "${__ush_match_4__age}")"
fi
return 0
}
__ush_value_6="$(__ush_capture_return='1' __ush_return_path='' __ush_method_1)"
printf '%s\n' "${__ush_value_6}"
__ush_match_7__tag="${user__tag}"
case "${user__tag}" in
  'User::User')
__ush_match_7__name="${user__name}"
__ush_match_7__age="${user__age}"
    ;;
esac
__ush_method_8() {
__ush_match_9__tag="${__ush_match_7__tag}"
case "${__ush_match_7__tag}" in
  'User::User')
__ush_match_9__name="${__ush_match_7__name}"
__ush_match_9__age="${__ush_match_7__age}"
    ;;
esac
__ush_match_11__tag="${__ush_match_7__tag}"
case "${__ush_match_7__tag}" in
  'User::User')
__ush_match_11__name="${__ush_match_7__name}"
__ush_match_11__age="${__ush_match_7__age}"
    ;;
esac
if [ -n "${__ush_return_path:-}" ]; then
  printf '%s' "$(printf '%s' "${__ush_match_9__name}" ':' "${__ush_match_11__age}")" > "$__ush_return_path"
elif [ "${__ush_capture_return:-0}" = '1' ]; then
  printf '%s' "$(printf '%s' "${__ush_match_9__name}" ':' "${__ush_match_11__age}")"
fi
return 0
}
__ush_value_13="$(__ush_capture_return='1' __ush_return_path='' __ush_method_8)"
__ush_value_14="${__ush_value_13}"
printf '%s\n' "${__ush_value_14}"
__ush_match_15__tag="${user__tag}"
case "${user__tag}" in
  'User::User')
__ush_match_15__name="${user__name}"
__ush_match_15__age="${user__age}"
    ;;
esac
__ush_prefix_16='id'
__ush_method_17() {
__ush_match_18__tag="${__ush_match_15__tag}"
case "${__ush_match_15__tag}" in
  'User::User')
__ush_match_18__name="${__ush_match_15__name}"
__ush_match_18__age="${__ush_match_15__age}"
    ;;
esac
if [ -n "${__ush_return_path:-}" ]; then
  printf '%s' "$(printf '%s' "${__ush_prefix_16}" ':' "${__ush_match_18__name}")" > "$__ush_return_path"
elif [ "${__ush_capture_return:-0}" = '1' ]; then
  printf '%s' "$(printf '%s' "${__ush_prefix_16}" ':' "${__ush_match_18__name}")"
fi
return 0
}
__ush_value_20="$(__ush_capture_return='1' __ush_return_path='' __ush_method_17)"
__ush_value_21="${__ush_value_20}"
printf '%s\n' "${__ush_value_21}"
printf '%s\n' "$(if :; then printf '%s' true; else printf '%s' false; fi)"
printf '%s\n' "$(if [ 'ant' != 'bee' ] && [ "$(printf '%s\n%s\n' 'ant' 'bee' | LC_ALL=C sort | head -n 1)" = 'ant' ]; then printf '%s' true; else printf '%s' false; fi)"

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
set -eu

__ush_jobs=''
__ush_task_seq='0'
__ush_task_files=''
__ush_tmp_seq='0'

__ush_source_dir='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots'
__ush_source_path='$WORKSPACE/crates/ush_compiler/tests/fixtures/shell_snapshots/typed_cli_and_types.ush'

__ush_print_help() {
  cat <<'__USH_DOC__'
typed_cli_and_types.ush

Usage:
  typed_cli_and_types.ush [--help]
  typed_cli_and_types.ush [--man [ITEM]]
  typed_cli_and_types.ush [--complete [PREFIX]]
__USH_DOC__
}

__ush_print_man() {
  case "${1:-}" in
    '')
      cat <<'__USH_MAIN__'
NAME
typed_cli_and_types.ush

SYNOPSIS
typed_cli_and_types.ush [--help]
typed_cli_and_types.ush [--man [ITEM]]
typed_cli_and_types.ush [--complete [PREFIX]]
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

alias ll='ls -la'
user__tag='User::User'
user__name='ush'
user__age=7
__ush_value_0="$(__ush_capture_return='1' ush_fn_greet 'ush' 3)"
printf '%s\n' "${__ush_value_0}"
__ush_value_1="$(__ush_capture_return='1' ush_fn_greet 'mini' 2)"
printf '%s\n' "${__ush_value_1}"
__ush_match_2__tag="${user__tag}"
case "${user__tag}" in
  'User::User')
__ush_match_2__name="${user__name}"
__ush_match_2__age="${user__age}"
    ;;
esac
if [ "${__ush_match_2__tag}" = 'User::User' ]; then
  name="${__ush_match_2__name-}"
  age="${__ush_match_2__age-}"
  printf '%s\n' "$(printf '%s' "${name}" ':' "${age}")"
elif :; then
  printf '%s\n' 'fallback'
fi

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
```

