# Examples

`ush` now ships with a larger example catalog.
Most `.ush` files can be run directly:

```bash
cargo run -p ush -- examples/hello.ush
```

Compile any example to portable `sh` with:

```bash
cargo run -p ush -- compile examples/hello.ush
```

Value-returning functions now follow a Rust-like style: the last expression returns,
and adding `;` keeps that expression as a statement instead.

## Language Basics

- `hello.ush`: minimal `let`, `print`, inline `$ command`, and `match`
- `functional.ush`: `$` application and grouped calls
- `zero_arg.ush`: zero-arg functions with `()`
- `unit.ush`: omitted `-> ()`, tail `()`, and `== ()`
- `named_args.ush`: labeled arguments and `#[default(...)]`
- `literal_match.ush`: literal `match` arms
- `multiline_string.ush`: `""" ... """` strings with common-indent dedent
- `primitives.ush`: integer addition plus `Eq` and `Ord` comparisons
- `std_modules.ush`: `std::env`, `from_cwd` / `from_source` path refs, plus method-style `path.resolve()`, `path.exists()`, `path.read_text()`, `std::fs`, `std::command`, and `std::string`
- `http_regex.ush`: `std::http::{get, download}` plus `std::regex::{is_match, find, capture, replace}` and String regex methods
- `control_flow.ush`: `if` tail values, `for in`, ranges, `while`, `loop`, and `if let ... && ...`
- `effects.ush`: Effekt-style effect rows (`-> T / { … }`), `effect` declarations, `do`, and `try … with` handlers

## ADT and Pattern Matching

- `type.ush`: struct-like `type { ... }` declarations
- `adt.ush`: nested tuple-style enums
- `option.ush`: wildcard matching on a small enum
- `response.ush`: struct-style enum payloads with shorthand bindings
- `error_streams.ush`: typed `Problem!T` signatures, `raise`, tail expressions, inferred `# raises:`, and Rust-like `?` propagation

## Async and Tasks

- `async.ush`: basic `async` plus postfixed `.await`
- `async_block.ush`: block-based `async { ... }` tasks
- `task_math.ush`: async `Int` return values
- `task_fanout.ush`: multiple tasks awaited independently

## CLI and Docs

- `bin.ush`: minimal generated CLI entrypoint
- `bin_defaults.ush`: defaults, aliases, and bool flags
- `docs.ush`: `#|` comments with std-like generated `--help`, `--man`, and `--complete`
- `smoke_test.ush`: a tiny passing script you can run with `ush test`

## Shell Integration

- `alias.ush`: `alias name = ...` lowering to shell aliases
- `hello.ush`: inline `$ command ...` shell escape
- `shell_string.ush`: dynamic shell command strings
- `traits.ush`: marker traits and builtin trait impl examples

## Data Files for Helper Demos

- `config.pkl`: sample runtime config
- `sample.json`: input for `| json`
- `sample.xml`: input for `| xml`
- `sample.html`: input for `| html`
- `sample_lines.txt`: input for `| car`, `| cdr`, and `| flat(...)`

Try the helper side like this:

```bash
cargo run -p ush -- -c 'cat examples/sample.json | json'
cargo run -p ush -- -c 'cat examples/sample.xml | xml'
cargo run -p ush -- -c 'cat examples/sample.html | html'
cargo run -p ush -- -c 'cat examples/sample_lines.txt | car'
cargo run -p ush -- -c 'cat examples/sample_lines.txt | cdr'
cargo run -p ush -- -c 'cat examples/sample_lines.txt | take(2)'
cargo run -p ush -- -c 'cat examples/sample_lines.txt | drop(1)'
cargo run -p ush -- -c 'cat examples/sample_lines.txt | nth(1)'
cargo run -p ush -- -c 'cat examples/sample_lines.txt | enumerate(1)'
cargo run -p ush -- -c 'printf "b\na\nb\n" | fsort | funiq | fjoin(",")'
cargo run -p ush -- -c 'cat examples/sample_lines.txt | flat(\head, rest -> [head, "tail", rest])'
cargo run -p ush -- -c 'cat examples/sample_lines.txt | fmap(\it -> upper(it))'
cargo run -p ush -- -c 'printf "a\nb\n" | fzip(["1", "2"])'
cargo run -p ush -- -c 'printf "a\nb\n" | fzip(["1", "2"]) | swap'
cargo run -p ush -- -c 'printf "a\nb\n" | fzip(["1", "2"]) | fst'
cargo run -p ush -- -c 'printf "a\nb\n" | fzip(["1", "2"]) | snd'
cargo run -p ush -- --config examples/config.pkl
cargo run -p ush -- test examples/smoke_test.ush
```
