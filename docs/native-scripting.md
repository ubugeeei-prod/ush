# Native scripts and operating-system boundaries

The installer and every source in `scripts/` use ush functions, bindings, loops,
conditions and explicit imports. Their adjacent `.sh` files are generated with
`ush compile --checked-in`, so an initial install does not require ush.

```ush
use std::process::{args, output, status}
use std::fs::{temp_dir, write_text}
use std::string::lines
use $bin::*

let work = temp_dir
for argument in args {
  print argument
}
let files = output "git" ["ls-files", "--", "*.ush"]
for file in lines files {
  print file
}
write_text (work + "/message") "hello\n"
if status "test" ["-s", work + "/message"] != 0 {
  fail 1 "missing output"
}
```

`$bin::*` calls accept consecutive arguments. `std::process` accepts a program
name and a `List<String>` when the command or argument list is computed at run
time. Each value remains one argv entry, including empty strings, spaces, quotes,
newlines and shell metacharacters. These APIs do not evaluate shell source.

| API | Result and behavior |
| --- | --- |
| `std::process::args` | Original script arguments as `List<String>` |
| `std::process::arg index` | Zero-based argument, or an empty string if absent |
| `std::process::exec program arguments` | Stream stdout/stderr; propagate failure |
| `std::process::output program arguments` | Capture stdout; propagate failure; trailing newlines follow shell capture semantics |
| `std::process::status program arguments` | Suppress both streams and return the exit status |
| `std::process::redirect program arguments stdout_path stderr_path` | Write separate streams to files and return status, including failure to open a file |
| `std::process::exit status` | Exit the current process scope with a status from 0 through 255 |
| `std::io::eprint text` | Write a line to stderr |
| `std::fs::temp_dir` | Create a private directory, removed when the enclosing script exits |
| `std::string::split text separator` | Split on a literal nonempty separator; preserve empty fields |
| `std::string::lines text` | Split LF/CRLF lines; omit the final terminator |
| `std::string::contains text part` | Literal substring predicate |
| `std::string::trim text` | Remove leading/trailing whitespace |
| `std::string::to_int text` | Validate a signed 64-bit decimal integer |

Programs that read argv handle their own CLI, including `--help` and `--man`.
Temporary directories are shared with the script's task lifetime and cleaned
after outstanding tasks finish, including on failure and handled signals.
As with ordinary process cleanup, SIGKILL cannot run cleanup handlers.

Double-quoted strings decode `\n`, `\r`, `\t`, `\"` and `\\` starting in 0.13.
Single-quoted and triple-quoted strings remain raw. Use raw strings for regular
expressions or command formats that need literal backslashes. Concatenation
preserves trailing newlines; command captures strip them.

The benchmark and dependency checks require `jq` for JSON and floating-point
calculations. Their orchestration, iteration and pass/fail policy stay in ush.
The installer requires ordinary POSIX tools and a SHA-256 tool, with curl or wget
for HTTPS downloads. The tynix development shell supplies repository check tools.

## Checked-in output

`--checked-in` emits a deterministic script meant to be committed, so
contributors without ush can run it. The output may live in any directory:

```sh
ush compile ci/check.ush --checked-in -o ci/compiled/check.sh
```

The header names the source relative to the output and the command that
regenerates it; both paths are relative to the output's directory:

```sh
#!/bin/sh
# Generated from '../check.ush' by ush.
# DO NOT EDIT MANUALLY.
# Source: ../check.ush
# Source map: embedded __ush_at markers; ush explain SOURCE LINE
# Regenerate with: ush compile '../check.ush' --checked-in -o 'check.sh'
```

At run time the script finds its own directory, follows the recorded
relative path, and from there resolves `std::path::from_source`,
`std::path::source_root`, `__ush_source_dir` and relative `extern`
libraries, so `ci/compiled/check.sh` behaves as if it sat next to
`ci/check.ush`, from any working directory. [Local modules](shell-language.md#local-modules)
are inlined, so the output needs no other `.ush` file; `extern` `.sh`
libraries must still be shipped at their relative paths. The generated
text contains no absolute paths, and compiling the same source to the same
relative location gives the same bytes on every machine.

### Many scripts and freshness checks

One process can compile any number of scripts. `--out-dir DIR` writes each
input as `DIR/<stem>.sh`; a relative `DIR` is taken from each input's own
directory, so scripts spread across many directories each get a `compiled/`
beside them:

```sh
ush compile --checked-in --out-dir compiled ci/*/scripts/*.ush
```

Each output is byte-identical to what a single `-o` command writes, and an
unchanged file is not rewritten. `--check` compiles in memory and compares
instead of writing; use it as a CI gate:

```sh
ush compile --check --checked-in --out-dir compiled ci/*/scripts/*.ush
```

It exits 0 when every output matches. Otherwise it exits 1, reports each
missing or stale file with its first differing lines, and asks for the same
command without `--check`:

```text
ush: ci/lint/scripts/compiled/run.sh is stale
--- ci/lint/scripts/compiled/run.sh (committed)
+++ ci/lint/scripts/compiled/run.sh (generated)
@@ -88,1 +88,1 @@
-  __ush_at 4 'print "lint"'
+  __ush_at 4 'print "lint: ok"'
ush: 1 output(s) out of date; run the same command without --check
```

Every input is processed even when one fails to compile, so a single run
lists all broken or stale scripts. Several inputs require `--out-dir`;
`-o` and `--sourcemap` take one input.

