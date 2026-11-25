# Returning a failure

A function lists its success type after `->`. The slash introduces a comma-separated row of error types, I/O capabilities and user effects. Declare an error type as an enum. A failure keeps its variant name at runtime.

## Inferred rows

The row is optional. Without one, the compiler infers which errors a
function can raise and which effects it needs, and callers see that row
exactly as if it had been written:

```ush
use $bin::*

enum ConfigError {
  Missing,
}

fn head_sha() -> String {
  git "rev-parse" "HEAD"
}

fn load(path: String) -> String {
  if path == "" {
    fail ConfigError::Missing
  }
  path
}

let config = load "app.toml"?
print head_sha
```

`ush effects` prints every function's row, written or inferred, error
types first:

```text
head_sha     unknown, stdin, stderr, fs, env, net, exec
load         ConfigError
```

- A written row is a checked contract and an upper bound: the body may
  raise and need less, never more. `/ ()` is written purity — no errors and
  no effects.
- `?`, propagation through callers and typed failures such as
  `fail ConfigError::Missing` work the same with an inferred row.
- Recursive and mutually recursive functions need no annotation. Rows are
  computed as a fixpoint over the whole program: each round re-reads every
  body with the rows found so far and keeps what it found, so a cycle settles
  on the smallest row that covers every function in it.
- A function referenced with `&f` still needs a written result type (`-> T`),
  because the type of a function value is checked before calls are resolved.
  Its row does not: each call through a function value is specialized to the
  named function it carries, so the caller's row includes the callback's
  actual errors and effects, and a callback that prints cannot slip through a
  `/ ()` function.
- Publish rows where callers rely on them — library entry points, `bin`
  functions — and leave them off helpers whose row is whatever their body
  does.

```ush
enum ConfigError {
  Missing,
}

type Config {
  name: String,
}

fn load(found: Bool) -> Config / ConfigError, fs {
  if found {
    return Config { name: "production" }
  }
  fail ConfigError::Missing
}

let config = load(true)?
print config.name
```

Use `?` to pass a nonzero status to the calling function or, at the top level, to the process that launched the script. Awaiting a failed task also preserves its status. A typed `fail` returns status 1. Its diagnostic is emitted by the runtime and the signature declares the error type. A shell-facing failure can supply a status between 1 and 255 and a message.

```ush
fn require_name(name: String) -> () / unknown, stderr {
  if name == "" {
    fail(2, "missing name")
  }
}

require_name("")?
```

External programs choose their own exit codes. The error row calls that possibility `unknown`. A capture returns standard output as a string, strips trailing newlines like shell command substitution, and leaves standard error connected to the caller.

```ush
let branch = $bin::git("branch", "--show-current")?
print branch
```

The generated script writes the failure location, exit status and original source line to standard error. The same diagnostic works on a host without ush or the original source. Native command diagnostics pass through. Use `ush explain` to inspect the lowering for a shell line.

To migrate `ConfigError!Config`, put the success type first, followed by `/ ConfigError` and the effects used by the function, or drop the row and let it be inferred. Replace a braced row with comma-separated names. Use `/ ()` to require purity.
