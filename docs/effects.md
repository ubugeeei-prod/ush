# Effects

`ush` tracks two things about a function: which errors it can raise
(see [`typed-errors.md`](./typed-errors.md)), and what it needs from
its caller in order to run. This page is about the second one.

The design follows [Effekt][effekt]: a function's **effect row** is
what its body still needs, effects propagate outward through calls
until something handles them, and a **handler** discharges an effect
for the block it wraps.

```ush
effect log(message: String) -> ()

fn greet(name: String) -> String / log {
  do log("greeting " + name)
  "hello " + name
}

try {
  print greet("ubu")
} with log(message) {
  print "[log] " + message
}
```

```text
[log] greeting ubu
hello ubu
```

## Why

A shell script's interesting behaviour is almost entirely effects,
and a signature like

```ush
fn prepare(target: String) -> String
```

says nothing about whether calling it writes files, reads `$HOME`,
shells out, or fetches something — exactly what you want to know
before running it unattended, in CI, or from inside another tool.

## The row

The row is written after the return type, separated by `/`:

```ush
fn deploy(target: String) -> String / exec, net, audit
```

`/ ()` is the empty row: this function may need nothing at all.
Omitting the row entirely means "infer it, don't check it" — for
effects and for errors alike. Callers see the inferred row as if it
had been written, so nothing has to be annotated for the analysis to
run; see [inferred rows](./typed-errors.md#inferred-rows) for
recursion and function values.

A row is an **upper bound**, not an exact set. Declaring more than the
body currently needs is allowed on purpose, so a published signature
can stay stable while an implementation stops needing something.

## Two kinds of effect

| | Built-in | User |
| --- | --- | --- |
| Written | inferred, never declared with `effect` | `effect log(msg: String) -> ()` |
| Performed | by calling the stdlib | `do log("…")` |
| Discharged | never | `try … with log(msg) { … }` |

**Built-in** effects describe what the generated shell actually does,
so no handler can take them back — nothing un-writes a file:

| Effect | Comes from |
| --- | --- |
| `stdin` | Reading process input, including inline shell |
| `stdout` | `print` and commands streaming text |
| `stderr` | `fail(code, message)` and external command diagnostics |
| `io` | Shorthand in a declaration for all three streams |
| `fs` | `std::fs::*`, the `std::path` helpers that look at disk, `path.read_text()` and friends |
| `env` | `std::env::*`, `std::path::cwd` / `home` / `from_source` / `prepend_env` |
| `net` | `std::http::*` |
| `exec` | `$ command`, `shell`, `std::command::*` |
| `task` | `async`, `spawn`, `.await`, `async` blocks |

A pipeline needs what its stages need. Only the stage that writes to the
script's stdout adds `stdout`: earlier stages write into the pipe, a file sink
takes the last stage's output, and a captured pipeline's output becomes a
value. `from_file`, `to_file`, `append_to`, `stderr_to` and
`stderr_append_to` add `fs`, and `to_stderr` adds `stderr`; `export` and
scoped blocks add `env`, and a `cd` block also adds `fs`.

A command used for its status, as a condition, with `status_of` or with
`allow_fail`, streams like a statement and needs what the statement would.
Its own failure is handled, so it adds no `unknown` to the error row; the
evaluation of its arguments still can. `spawn` needs `task` and what its
command needs, as an `async` block does; `job.kill()` needs `task`. A deferred block needs what its body needs, in the row of the
function or program that registers it.

Inline shell is opaque to the checker. It conservatively requires `exec`, `fs`, `env`, `net`, `stdin` and `stderr`. A shell statement also streams `stdout`. A typed `$bin::` call can provide a smaller row. User-method calls with the same name share a conservative upper bound in effect inference.

`std::string` and `std::regex` are pure: they are text
transformations and add nothing. `std::path` is deliberately split,
because half of it is path algebra (`join`, `basename`, `dirname`)
and the other half asks the filesystem what is there (`exists`,
`mkdir_p`, `resolve`).

**User** effects are the Effekt ones. `effect` declares an operation,
`do` performs it, and the effect stays in the row of everything up
the call chain until a handler answers it.

## Inference

Every function gets a row whether or not it is annotated:

```bash
ush effects examples/effects.ush
```

```text
default_target  env  (declared)
deploy          stdout, env, ask, audit  (declared: stdin, stdout, stderr, env, ask, audit)
slug            pure  (declared)
(top level)     stdout, env
```

Each row lists the error types a function can raise, then its
effects. `(top level)` is the row of the statements outside any
function — the program itself.

Rows propagate through calls, so `deploy` above picks up `env` from
`default_target` without mentioning it in its own body. Propagation is
a fixpoint rather than a single pass, so mutually recursive functions
converge instead of one being reported as needing nothing.

`ush effects --undeclared` lists only the functions without a row,
which is the useful view when annotating an existing script.

## Handlers

`try { … } with effect(args) { … }` answers an effect for the
block it wraps. One `try` can carry several arms:

```ush
try {
  deploy()
} with audit(entry) {
  print "[audit] " + entry
} with ask(question) {
  "staging"
}
```

The handler's value is what `do` evaluates to, so the body continues
where it left off — `let name = do ask("…")` binds `"staging"` above.
Handlers nest, and an inner handler shadows an outer one for the
duration of its block.

An effect that reaches the top of the program was never answered, and
that is an error. Effekt makes the same demand of `main`: an entry
point has no caller left to ask.

```text
effect `log` is performed but never handled; wrap the call in
`try { … } with log { … }`
```

### How it lowers

An `effect` declaration becomes a shell function that forwards to
whichever handler is installed, and `try … with` points the
indirection at its own handler for the duration of the body:

```sh
__ush_handler_audit=''
ush_fn_audit() {
  if [ -z "${__ush_handler_audit:-}" ]; then
    printf '%s\n' 'ush: unhandled effect: audit' >&2
    exit 1
  fi
  "$__ush_handler_audit" "$@"
}

ush_fn___ush_handle_audit_5() { … }
__ush_saved__ush_handler_audit_6="${__ush_handler_audit:-}"
__ush_handler_audit='ush_fn___ush_handle_audit_5'
…body…
__ush_handler_audit="$__ush_saved__ush_handler_audit_6"
```

Save and restore is what makes nesting and shadowing work.

## Where it runs

The effect pass runs during compilation, alongside the typed-error
pass, so a bad row is reported by everything that compiles:

- `ush check script.ush`
- `ush script.ush`
- `ush compile script.ush`
- the LSP, as a diagnostic on the file

## Limits

These handlers are **tail-resumptive**. The handler runs and returns,
and its value is the result of the `do` — which covers dynamic
binding, capability passing, logging, prompting, and configuration.
It does not cover a handler that captures the continuation and runs
it twice, or not at all: there is no `resume` to pass around, because
POSIX `sh` has no delimited continuations to build one from.

Also not there yet, in rough order of how much they are missed:

- effect polymorphism: a function cannot be generic over what a
  block parameter needs, which is Effekt's headline feature
- multi-operation effect interfaces (`effect Exc { def raise…; def
  finally… }`); today an `effect` declares exactly one operation
- effect aliases, and rows on trait methods as a bound on
  implementations
- per-call refinement of the built-ins (`std::fs::exists` and
  `std::fs::write_text` are both `fs`)

[effekt]: https://effekt-lang.org
