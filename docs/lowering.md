# How `.ush` Lowers to `sh`

This page shows how major `.ush` language features compile down to POSIX `sh`.

The examples below are representative output from `cargo run -p ush -- compile ...`.
Exact temporary variable names may change across compiler versions, but the shape is
intended to stay stable.

## What Every Script Gets

Every artifact starts with a target shebang, a source comment, a
`DO NOT EDIT MANUALLY` notice, and `set -eu`. Source locations use small
`__ush_at` calls next to the statements they describe. The compiler emits
the standard-library helpers that the program reaches. Task bookkeeping is
included when the program starts a background task.

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
| `fail Problem::X` | `printf ... >&2` then `return 1` |
| `expr?` | `|| return "$?"` or `|| exit "$?"` |
| `bin(...)` | CLI parser wrapper around `ush_fn_bin` |
| `alias ll = "ls -la"` | `alias ll='ls -la'` |
| `a \| b` | stages in `{ …; } \| { …; }` inside a status capture, then `__ush_pipe_check` |
| `a \|& b`, `\| to_file p` | `2>&1`, `> p` on the stage itself |
| `cd dir { … }` | save `$PWD`, `ush_fn_std__process__chdir`, body, `cd` back |
| `export A = v { … }` | save value and set state, `export A=…`, body, restore or `unset` |

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

The shell evaluates the arguments, creates a private result slot, and starts
the function with `&`. A handle keeps the child's PID and result slot. Await
uses POSIX `wait`, preserves the child's exit status, and reads the result.
The parent joins unfinished children before removing its private directory.

## Errors and propagation

`fail Problem::MissingConfig` reports the source location and variant to
stderr and returns status 1. `fail(code, message)` reports the supplied
message and status. A call ending in `?` checks its status before continuing.
The emitted script contains the source text necessary to locate a failure.

## Pipelines

Source:

```text
git "log" "--oneline" | head "-n1"
```

Compiled `sh`:

```sh
{ __ush_pipe=$(
  exec 7>&1 >&8 8>&-
  set +e
  { (command git 'log' '--oneline') 7>&-; printf ' 1:%s' "$?" >&7; } |
  { (command head '-n1') 7>&-; printf ' 2:%s' "$?" >&7; }
); } 8>&1
__ush_pipe_check "$__ush_pipe" 'pipeline stage 1 failed: git "log" "--oneline"' 'pipeline stage 2 failed: head "-n1"' || exit "$?"
```

POSIX `sh` has no `pipefail`, so every target, bash and zsh included, uses
this one lowering. The command substitution captures exit statuses rather
than output: descriptor 8 is the enclosing stdout, which the last stage
writes to, and descriptor 7 is the substitution's own stdout, where each
stage reports ` INDEX:STATUS` once its program exits. `set +e` keeps a
failing program from ending the stage before it reports. Every program runs
in a subshell of its own, so a builtin or function stage killed by SIGPIPE
cannot take the report with it, and it does not inherit descriptor 7, so a
daemon it starts cannot hold the pipeline open.

`__ush_pipe_check` is emitted only when a program has a pipeline. It walks
the stages left to right and reports the first failure with `__ush_report`,
returning its status. A stage holds its pipe ends until it has reported, so a
failure recorded after the next stage's report comes from writing into a pipe
that reader closed early; the check skips it, whether the writer died of
SIGPIPE or got a write error because SIGPIPE was ignored.

A captured pipeline runs the same block inside `"$(…)"`, where descriptor 8
becomes the capture. The check reports from inside the capture, so the
enclosing shell clears `__ush_line` before propagating the status, and the
failure is reported once.

Arguments, String stdin values and computed paths are evaluated into
variables before the block. `from_file`, `to_file` and `append_to` become
`<`, `>` and `>>` on their neighbouring stage, and `|&` adds `2>&1` after them,
so `cmd |& to_file log` is `cmd > log 2>&1`. A String first stage is
`(printf '%s' "$value")`.

## Scoped blocks

Source:

```text
cd "crates" {
  export RUSTFLAGS = "-D warnings" {
    cargo "build"
  }
}
```

Compiled `sh`:

```sh
__ush_cwd_0=$PWD
ush_fn_std__process__chdir 'crates' || { __ush_status=$?; __ush_report "$__ush_status"; exit "$__ush_status"; }
  __ush_had_RUSTFLAGS_1=${RUSTFLAGS+x} __ush_saved_RUSTFLAGS_1=${RUSTFLAGS-}
  export RUSTFLAGS='-D warnings'
    command cargo 'build'
  if [ -n "$__ush_had_RUSTFLAGS_1" ]; then export RUSTFLAGS="$__ush_saved_RUSTFLAGS_1"; else unset RUSTFLAGS; fi
cd "$__ush_cwd_0" || { __ush_status=$?; __ush_report "$__ush_status" 'cannot return to the directory the cd block started in'; exit "$__ush_status"; }
```

The body runs in the current shell rather than a `( … )` subshell. A
subshell would restore everything for free, but assignments to outer
bindings would silently vanish, `return` would leave only the subshell, and
`break` and `continue` could not reach the enclosing loop. Saving and
restoring keeps blocks consistent with `if` and loops. The compiler emits the
restore code at the end of the body and before each `break` or `continue`
that leaves the block. Other early exits end the process the change was made
in: `fail`, `?` and failing commands exit the script at top level, and a
function body always runs in a subshell or command substitution of its own.

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
