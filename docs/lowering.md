# How `.ush` Lowers to `sh`

This page shows how major `.ush` language features compile down to POSIX `sh`.

The examples below are representative output from `cargo run -p ush -- compile ...`.
Exact temporary variable names may change across compiler versions, but the shape is
intended to stay stable.

## What Every Script Gets

Every compiled script starts with a small runtime scaffold:

```sh
#!/bin/sh
set -eu

__ush_jobs=''
__ush_task_seq='0'
__ush_task_files=''
```

That scaffold is used for:

- strict shell execution with `set -eu`
- task tracking for `async` / `await`
- cleanup of background jobs and temp files at the end of the script

For a generated artifact that will be committed, use adjacent source and
output files:

```sh
ush compile tools/check.ush --checked-in -o tools/check.sh
```

This mode adds source and regeneration comments, avoids machine-specific
absolute paths, and resolves `std::path::from_source` from the generated
script's directory at runtime.

If the source has doc comments or a `bin(...)` entrypoint, the compiler also emits:

- `__ush_print_help`
- `__ush_print_man`
- `__ush_complete`
- a small `case "${1:-}"` dispatcher for `--help`, `--man`, and `--complete`

## Quick Reference

| `.ush` feature | Generated `sh` shape |
| --- | --- |
| `let name = "x"` | `name='x'` |
| `print a + "b"` | `printf '%s\n' "$(printf '%s' ...)"` |
| `$ printf ...` | emitted as a normal shell command |
| `shell command` | `eval "${command}"` |
| `fn greet(...) -> String` | `ush_fn_greet() { ... }` |
| final expression in `-> T` function | lowered like `return`, without writing `return` |
| `expr;` in a `-> T` function | evaluated as a statement and discarded |
| function return value | printed to stdout or a temp return file |
| `$ label greet wrap` | nested `__ush_value_N="$(__ush_capture_return='1' ...)"` |
| ADT / `type` value | split shell vars like `user__tag`, `user__name` |
| `match` | temp bindings plus `if` / `case` |
| `async f()` / `task.await` | background job, temp file, `wait`, `cat` |
| `raise Problem::X` | `printf ... >&2` then `return 1` |
| `expr?` | `|| return "$?"` or `|| exit "$?"` |
| `bin(...)` | CLI parser wrapper around `ush_fn_bin` |
| `alias ll = "ls -la"` | `alias ll='ls -la'` |

## `let` and `print`

Source:

```text
let greeting = "hello"
print greeting + " world"
```

Compiled `sh`:

```sh
greeting='hello'
printf '%s\n' "$(printf '%s' "${greeting}" ' world')"
```

`let` becomes a normal shell variable assignment. String concatenation lowers to a
`printf '%s'` composition so the generated shell stays quoted.

## Inline `$` Commands

Source:

```text
$ printf '%s\n' from-ush
```

Compiled `sh`:

```sh
printf '%s\n' from-ush
```

The `$ command ...` form is an inline shell escape. It lowers directly to a shell
statement.

## `shell expr`

Source:

```text
let command = "printf '%s\n' from-shell"
shell command
print "after-shell"
```

Compiled `sh`:

```sh
command='printf '"'"'%s\n'"'"' from-shell'
eval "${command}"
printf '%s\n' 'after-shell'
```

`shell expr` is the dynamic form. The expression is first compiled as a normal value,
then executed with `eval`.

## Functions and Return Values

Source:

```text
fn greet(name: String) -> String {
  "hi " + name
}
```

Compiled `sh`:

```sh
ush_fn_greet() {
  __ush_fn_greet_arg_0="$1"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "$(printf '%s' 'hi ' "${__ush_fn_greet_arg_0}")" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "$(printf '%s' 'hi ' "${__ush_fn_greet_arg_0}")"
  fi
  return 0
}
```

User-defined functions become `ush_fn_<name>()`. Return values are carried in one of
two ways:

- stdout capture with `__ush_capture_return='1'`
- a temp file when the call runs in the background

In value-returning functions, `.ush` follows a Rust-like rule:

- the last expression returns if it has no trailing `;`
- adding `;` keeps that line as a statement and execution continues

Source:

```text
fn run() -> String {
  "ignored";
  "kept"
}
```

Compiled `sh`:

```sh
ush_fn_run() {
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' 'kept' > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' 'kept'
  fi
  return 0
}
```

## Functional `$` Application

Source:

```text
let value = $ label greet wrap
```

Compiled `sh`:

```sh
__ush_value_0="$(__ush_capture_return='1' ush_fn_label)"
__ush_value_1="$(__ush_capture_return='1' ush_fn_greet "${__ush_value_0}")"
__ush_value_2="$(__ush_capture_return='1' ush_fn_wrap "${__ush_value_1}")"
value="${__ush_value_2}"
```

When `$` is used as function application, the compiler threads intermediate values
through generated temporaries.

## ADTs, Records, and `match`

Source:

```text
let user = User::User { name: "ush", age: 7 }

match user {
  User::User { name, age } => print name + ":" + age,
  _ => print "fallback",
}
```

Compiled `sh`:

```sh
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
if [ "${__ush_match_0__tag}" = 'User::User' ]; then
  name="${__ush_match_0__name}"
  age="${__ush_match_0__age}"
  printf '%s\n' "$(printf '%s' "${name}" ':' "${age}")"
elif :; then
  printf '%s\n' 'fallback'
fi
```

ADT values are flattened into related shell variables:

- `<value>__tag`
- `<value>__field`

Pattern matching lowers to a combination of tag extraction, `case`, and `if`.

## `async` and `.await`

Source:

```text
let task = async double(21)
let result = task.await
print result
```

Compiled `sh`:

```sh
__ush_task_seq=$((__ush_task_seq + 1))
__ush_task_0__result="${TMPDIR:-/tmp}/__ush_task_0.$$.$__ush_task_seq"
: > "${__ush_task_0__result}"
__ush_task_files="${__ush_task_files}${__ush_task_files:+ }${__ush_task_0__result}"
( __ush_return_path="${__ush_task_0__result}"; ush_fn_double 21 ) &
__ush_task_0__pid="$!"
__ush_task_0__awaited='0'
__ush_jobs="${__ush_jobs}${__ush_jobs:+ }$!"
if [ "${__ush_task_0__awaited}" = '0' ]; then
  wait "${__ush_task_0__pid}"
  __ush_task_0__awaited='1'
fi
result="$(cat "${__ush_task_0__result}")"
```

`async` launches a background subshell and gives it a temp result file. `.await`
becomes `wait` plus `cat` from that file.

## Typed Errors, `raise`, and `?`

Source:

```text
enum Problem {
  MissingConfig,
}

fn load_config() -> Problem!String {
  raise Problem::MissingConfig
}

fn run() -> Problem!String {
  let value = load_config()?
  wrap(value)
}

run()?
```

Compiled `sh`:

```sh
# raises: Problem
ush_fn_load_config() {
  printf '%s\n' 'ush raise: Problem' >&2
  return 1
}

# raises: Problem
ush_fn_run() {
  __ush_value_0="$(__ush_capture_return='1' ush_fn_load_config)" || return "$?"
  value="${__ush_value_0}"
  __ush_value_1="$(__ush_capture_return='1' ush_fn_wrap "${value}")"
  if [ -n "${__ush_return_path:-}" ]; then
    printf '%s' "${__ush_value_1}" > "$__ush_return_path"
  elif [ "${__ush_capture_return:-0}" = '1' ]; then
    printf '%s' "${__ush_value_1}"
  fi
  return 0
}

ush_fn_run || exit "$?"
```

The important pieces are:

- inferred error comments are emitted as `# raises: ...`
- `raise` writes an error message to stderr and returns non-zero
- `?` inside a function lowers to `|| return "$?"`
- top-level `?` lowers to `|| exit "$?"`

For external commands, the compiler cannot know the exact error type, so they are
tracked as `unknown` in the signature and inferred error stream.

## `alias`

Source:

```text
alias ll = "ls -la"
alias gs = "git status -sb"

alias ll
alias gs
```

Compiled `sh`:

```sh
alias ll='ls -la'
alias gs='git status -sb'
alias ll
alias gs
```

Alias declarations lower directly to shell `alias` statements.

## `bin(...)` Entrypoints

Source:

```text
fn bin(#[alias("n")] name: String, #[default(2)] count: Int, verbose: Bool) {
  print name + ":" + count
  print verbose
}
```

Compiled `sh`:

```sh
ush_fn_bin() {
  __ush_fn_bin_arg_0="$1"
  __ush_fn_bin_arg_1="$2"
  __ush_fn_bin_arg_2="$3"
  printf '%s\n' "$(printf '%s' "${__ush_fn_bin_arg_0}" ':' "${__ush_fn_bin_arg_1}")"
  printf '%s\n' "${__ush_fn_bin_arg_2}"
}

__ush_run_bin() {
  __ush_bin_pos='0'
  __ush_bin_name_seen='0'
  __ush_bin_name=''
  __ush_bin_count_seen='0'
  __ush_bin_count=2
  __ush_bin_verbose_seen='0'
  __ush_bin_verbose='false'
  while [ "$#" -gt 0 ]; do
    case "$1" in
      '--name'|'-n')
        shift
        [ "$#" -gt 0 ] || { printf '%s\n' "missing value for name" >&2; return 2; }
        __ush_bin_name="$1"; __ush_bin_name_seen='1'
        ;;
      '--verbose')
        __ush_bin_verbose='true'; __ush_bin_verbose_seen='1'
        ;;
      *)
        ...
        ;;
    esac
    shift
  done
  [ "$__ush_bin_name_seen" = '1' ] || {
    printf '%s\n' "missing argument: name" >&2
    return 2
  }
  ush_fn_bin "${__ush_bin_name}" "${__ush_bin_count}" "${__ush_bin_verbose}"
}

__ush_run_bin "$@"
```

A `bin(...)` entrypoint generates:

- the normal compiled function body as `ush_fn_bin`
- a CLI parser wrapper
- default handling
- alias handling like `--name` and `-n`
- usage / man / completion output

## See Also

- [`typed-errors.md`](./typed-errors.md)
- [`../examples/README.md`](../examples/README.md)
