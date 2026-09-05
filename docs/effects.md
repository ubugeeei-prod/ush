# Effects

`ush` tracks two things about a function: which errors it can raise
(see [`typed-errors.md`](./typed-errors.md)), and what it touches
beyond its own arguments. This page is about the second one.

## Why

A shell script's interesting behaviour is almost entirely side
effects, and a signature like

```ush
fn prepare(target: String) -> String
```

tells you nothing about whether calling it writes files, reads
`$HOME`, shells out, or fetches something over the network — exactly
what you want to know before running it unattended, in CI, or inside
another tool.

## The effects

| Effect | Means | Comes from |
| --- | --- | --- |
| `io` | terminal output and input | `print` |
| `fs` | the filesystem | `std::fs::*`, the `std::path` helpers that look at disk, `path.read_text()` and friends |
| `env` | the process environment | `std::env::*`, `std::path::cwd` / `home` / `from_source` / `prepend_env` |
| `net` | the network | `std::http::*` |
| `exec` | other processes | `$ command`, `shell`, `std::command::*` |
| `task` | concurrency | `async`, `spawn`, `.await`, `async` blocks |

`std::string` and `std::regex` are pure: they are text
transformations and add nothing. `std::path` is deliberately split,
because half of it is path algebra (`join`, `basename`, `dirname`)
and the other half asks the filesystem what is actually there
(`exists`, `mkdir_p`, `resolve`).

## Inference

Every function gets an effect row whether or not it is annotated.
Nothing has to be written down for the information to exist:

```bash
ush effects examples/effects.ush
```

```text
greeting_target  env  (declared)
report           io, env, exec
slug             pure  (declared)
stamp            exec  (declared)
(top level)      io, env, exec
```

`(top level)` is the row for the statements outside any function —
the program itself.

Rows propagate through calls, so `report` above picks up `env` from
`greeting_target` and `exec` from `stamp` without mentioning either
effect itself. Propagation is a fixpoint rather than a single pass,
so mutually recursive functions converge instead of one of them being
reported as pure.

`ush effects --undeclared` lists only the functions that do not yet
have a row, which is the useful view when adding annotations to an
existing script.

## Declaring

An `#[effects(...)]` row turns the inference into a check:

```ush
#[effects(net)]
fn fetch(url: String) -> String {
  std::http::get(url)
}
```

If the body later grows a `std::fs::write_text`, compilation fails:

```text
function `fetch` performs `fs` but its effect row declares `net`;
write `#[effects(fs, net)]` or drop the offending call
```

`#[pure]` is the empty row — the function may touch nothing at all:

```ush
#[pure]
fn slug(name: String) -> String {
  std::string::replace(name, " ", "-")
}
```

A row is an **upper bound**, not an exact set. Declaring more than the
body currently uses is allowed on purpose: it lets a published
signature stay stable while an implementation stops needing something.

## Where it runs

The effects pass runs during compilation, alongside the typed-error
pass, so a bad row is reported by everything that compiles:

- `ush check script.ush`
- `ush script.ush`
- `ush compile script.ush`
- the LSP, as a diagnostic on the file

## Limits

The current pass is a whole-program analysis over one file with a
fixed effect table. It does not yet cover:

- effect polymorphism (a function whose row depends on a callback's)
- per-call refinement (`std::fs::exists` and `std::fs::write_text`
  are both `fs`)
- user-declared effect kinds
- effect rows on trait methods as a bound on implementations
