# Language Vision

`ush` is not trying to become "bash with a few extra keywords".
The long-term target is a real small language:

- expressive enough to feel closer to MoonBit than to shell macros
- predictable enough to keep the directness of Go and Zig
- familiar enough that Rust users can read it without friction
- still portable, because the final executable form is POSIX `sh`

## Feel

The intended feel sits between Rust, Zig, and Go.

- From Rust: enums, pattern matching, traits, modules, method calls, postfix forms such as `.await` and `?`
- From Zig: explicit error-aware function signatures, simple data layouts, straightforward control flow, low hidden magic
- From Go: easy entry, fast read-time comprehension, practical stdlib, boring deployment story
- From MoonBit: a modern, cohesive small language with ADTs, pattern matching, solid modules, and a serious standard library

That means `ush` should not drift toward a hyper-magical macro language, and it should not become a Rust clone either.

## Design Rules

### 1. Shell remains the runtime contract

`.ush` can become much richer than `sh`, but execution still lowers to portable POSIX `sh`.
That keeps script distribution simple and lets `ush` stay a shell-adjacent language rather than a separate VM.

### 2. Readability beats cleverness

The default style should be understandable after a quick skim.
If a feature is powerful but makes routine scripts look cryptic, it is probably the wrong default.

### 3. Strong data and weak ceremony

The language should have real data modeling:

- `type`
- `enum`
- tuple
- list
- pattern matching
- typed errors
- modules
- traits

But the syntax should stay light enough that day-to-day scripts still feel quicker than writing Rust.

### 4. A serious stdlib is part of the language

MoonBit-level usability does not come only from syntax.
It also comes from batteries-included modules for:

- `std::path`
- `std::fs`
- `std::env`
- `std::command`
- `std::http`
- `std::json`
- `std::regex`
- `std::cli`
- `std::time`

The goal is that common automation tasks stop falling back to ad-hoc shell pipelines.

### 5. Explicit where failure matters

`ush` should feel comfortable for scripting, but not sloppy.
Errors, async boundaries, external command boundaries, and path resolution context should stay explicit enough to reason about.

That is why failure and effects both show up in a signature:
`Problem!String` says what can go wrong, and `/ { fs, net }` says what
the function needs from its caller. Both are inferred, so neither is
ceremony you have to write before your script runs — see
[`typed-errors.md`](./typed-errors.md) and [`effects.md`](./effects.md).

## Desired Syntax Direction

The syntax target is roughly:

- expression-oriented where it clearly helps
- statement-friendly where shell tasks are clearer as steps
- type-aware without requiring full annotation everywhere
- method-capable and module-capable
- simple enough that CLI tools still read like small tools, not academic programs

Examples of the desired direction:

```ush
use std::path::{from_source}
use std::http

type Config {
  owner: String
  repo: String
}

fn fetch_avatar(config: Config) -> Result!Path {
  let out = from_source("tmp").join("avatar.png")
  let bytes = http::get_bytes(
    "https://api.github.com/users/${config.owner}"
  )?
  out.write_bytes(bytes)?
  out
}
```

```ush
enum Command {
  Clone(String)
  Doctor
}

fn run(cmd: Command) {
  match cmd {
    Command::Clone(name) => print $ format("clone {}", name)
    Command::Doctor => print "doctor"
  }
}
```

This should feel more structured than shell, less ceremonious than Rust, and less sparse than Go.

## Roadmap Shape

The big missing pieces for this direction are:

1. Inherent `impl Type { ... }` methods and trait-driven abstraction
2. Real modules and imports beyond the current prototype
3. More complete type inference and generic programming
4. Structured values that flow naturally through stdlib APIs
5. Stronger async/concurrency semantics
6. A stdlib broad enough to replace many shell helper scripts directly
7. Effect polymorphism on top of the Effekt-style rows described in
   [`effects.md`](./effects.md), so a function can be generic over what
   a block parameter needs — the piece of Effekt `ush` does not have

## Non-Goals

These are intentionally not the target:

- becoming a bash-compatible parser for every native construct
- becoming a Rust clone with all of Rust's complexity
- becoming a dynamic string-only language with optional type hints
- hiding all shell/process boundaries behind opaque runtime magic

## Short Version

The target is:

- MoonBit-level seriousness as a language
- Rust-level data modeling where it pays off
- Zig-like explicitness around failure and boundaries
- Go-like approachability and deployment
- shell portability at the bottom
