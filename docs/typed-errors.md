# Returning a failure

A function lists its success type after `->`. The slash introduces a comma-separated row of error types, I/O capabilities and user effects. Declare an error type as an enum. A failure keeps its variant name at runtime.

```ush
enum ConfigError {
  Missing,
}

type Config {
  name: String,
}

fn load(found: Bool) -> Config / ConfigError, stderr {
  if found {
    return Config { name: "production" }
  }
  fail ConfigError::Missing
}

let config = load(true)?
print config.name
```

Use `?` to pass a nonzero status to the calling function or, at the top level, to the process that launched the script. Awaiting a failed task also preserves its status. A typed `fail` returns status 1. A shell-facing failure can supply a status between 1 and 255 and a message.

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

To migrate `ConfigError!Config`, put the success type first, followed by `/ ConfigError` and the effects used by the function. Replace a braced row with comma-separated names. Use `/ ()` to require purity.
