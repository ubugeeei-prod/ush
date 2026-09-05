# Sourcemaps

`ush compile --sourcemap` writes a JSON sidecar that explains how generated
`/bin/sh` lines relate back to `.ush` source.

This is useful for:

- debugging lowering output
- understanding runtime failures in generated shell
- building editor and tooling integrations on top of `ush`

## Generate a sourcemap

```bash
cargo run -p ush -- compile examples/hello.ush -o /tmp/hello.sh --sourcemap /tmp/hello.sh.map.json
```

The generated shell can still run on its own. The sourcemap file is extra
metadata for humans and tools.

## JSON format

Current sourcemaps use `version: 2`.

Top-level fields:

- `source`: original `.ush` file path
- `generated`: output shell path when `-o` is used
- `summary`: counts for generated, mapped, unmapped, and per-section lines
- `sources`: reverse index from one source line to every generated shell line
- `lines`: per-generated-line mapping entries

Each line entry includes:

- `generated_line`
- `section`
- `source_line`
- `generated_text`
- `source_text`

Current sections are:

- `runtime-support`: emitted helpers and runtime scaffolding
- `doc-support`: generated help/man/completion support
- `user-code`: lowered code that came from the user program

Example:

```json
{
  "version": 2,
  "summary": {
    "mapped_line_count": 2,
    "source_line_count": 2
  },
  "sources": [
    {
      "source_line": 1,
      "source_text": "let greeting = \"hello\"",
      "generated_lines": [449]
    }
  ],
  "lines": [
    {
      "generated_line": 449,
      "section": "user-code",
      "source_line": 1,
      "generated_text": "greeting='hello'",
      "source_text": "let greeting = \"hello\""
    }
  ]
}
```

## Runtime failure mapping

When `.ush` scripts run through `ush`, the generated shell is instrumented so
runtime failures print a sourcemap report to `stderr`.

Example:

```text
/bin/sh: line 490: definitely_not_a_real_command: command not found

ush runtime map: script.ush:3 (exit 127)
  source : $ definitely_not_a_real_command
  section: user-code
  shell  : line 490 | G0451 | definitely_not_a_real_command
  mapped : G0451
  explain: ush explain script.ush 490
```

The `line 490` in the report is the same number `/bin/sh` printed: the
instrumentation knows how tall its own header is, so a shell diagnostic can be
matched to a sourcemap entry without counting lines by hand. `G0451` is the
sourcemap id for the same line, the one that appears in the JSON above and in
the listings below.

For source lines that lower into multiple shell lines, `mapped` shows the whole
generated group, not just the failing line. That makes control-flow lowering
much easier to inspect.

## `ush explain`

`ush explain` is the other direction: hand it a line number out of a `/bin/sh`
diagnostic (or a `G####` id) and it prints the generated line, the `.ush` line
behind it, and the surrounding source.

```bash
ush explain script.ush 490
```

```text
script.ush: shell line 490 | G0451 | user-code
  shell  : definitely_not_a_real_command
  source : script.ush:3
        1 | let greeting = "hello"
        2 | print greeting
  ->    3 | $ definitely_not_a_real_command
        4 | print "after"
```

When one `.ush` line lowered into several shell lines, the whole group is
listed with the failing one marked. A number that lands inside the runtime
scaffolding `ush` adds ahead of the program says so rather than pointing at
something misleading.

The JSON sourcemap and mapped listings still include `runtime-support` and
`doc-support` sections, so tooling can inspect generated support code even when
runtime diagnostics focus on user-originated lines.
