# Shell-style language

## Bindings and reassignment

`let` declares an immutable binding. Opt into reassignment with `let mut`;
assignment uses the existing name without `let` and must preserve its type.

```ush
use std::test::assert_eq

let greeting = "hello"
let mut count = 0
while count < 3 {
  count = count + 1
}
assert_eq count 3
```

`greeting = "goodbye"` is a compile error that suggests `let mut greeting`.
Parameters, loop variables, and pattern bindings are immutable too. To change a
parameter locally, introduce a mutable binding: `let mut value = value`.
A new `let` shadows the previous binding; a declaration in an inner block does
not overwrite an outer value. Bare assignments in branches and loops update
the selected outer mutable binding. Lists, records, tuples, task handles, and
awaited results use the same rules. Structured assignment evaluates the complete
right-hand side before replacing the destination.

Migration from earlier releases: change the original declaration to `let mut`
and remove `let` from each intended reassignment. Leave new declarations as
`let`. Imports are separated from the following body by one blank line;
`ush format` and editor formatting insert that separator.

## Imports and calls

```ush
use std::string::*
use $bin::*

fn greet(first: String, last: String) -> String {
  first + " " + last
}
fn label() -> String {
  "ush"
}

print greet "hello" label
let branch = git "branch" "--show-current"?
print trim_suffix branch "\n"
print greet (trim_prefix "@Ada" "@") "Lovelace"
```

Arguments are separated by whitespace and each argument is one typed value.
Quote string literals; identifiers name variables or zero-argument functions.
Use parentheses to group a nested call or an operator expression into one
argument. Calls bind more tightly than `+` and comparisons: `f x + 1` adds one
to `f x`; write `f (x + 1)` to add inside its argument. Named arguments work as
`greet last: "Lovelace" first: "Ada"`.

A variable shadows a same-named zero-argument function in value position.
Function declarations still use `fn name(parameters)`. Existing parenthesized
calls and `$` application remain accepted for older scripts; new code does
not need them. `$ command` and `$(command)` retain their shell meanings.

`use module::*` imports a module's functions. Explicit imports and local
functions take precedence over a wildcard. Conflicting names from two
wildcards are diagnosed; import those functions explicitly with `as` aliases.
`use $bin::*` imports the command catalog, with local functions and explicit
imports taking precedence. A trailing `::` is not a wildcard.

## Pipelines and redirection

```ush
use $bin::*
use std::io::{append_to, from_file, to_file}

git "log" "--oneline" | head "-n5"
let branch = git "branch" "--show-current" | tr "a-z" "A-Z"
cargo "build" |& tee "build.log"
cargo "test" |& to_file "test.log"
from_file "notes.txt" |
  sort |
  uniq |
  to_file "sorted.txt"
"## Summary\n" | append_to (std::env::get "GITHUB_STEP_SUMMARY")
let body = """
  Release notes
  - pipelines
"""
body | gh "pr" "comment" "1" "--body-file" "-"
```

A pipeline connects each stage's stdout to the next stage's stdin. A stage
is a command (`git "log"`, a `$bin::` call, or `std::process::exec` and
`output`) or a call to a function returning `()`. A function stage runs in a
subshell of its own; it reads stdin with `std::io::read_all` or `read_lines`
and prints its output. `|&` after a stage also sends that stage's stderr into
the pipe. The pipe binds more loosely than anything else in an expression, so
`(a | b) == "x"` needs its parentheses. A line that ends with `|` or `|&`
continues on the next line; blank lines and comments between stages are
skipped.

As a statement, a pipeline streams its last stage to the script's stdout. As
a value, as in `let`, `print (…)` or a condition, it captures the last
stage's stdout and removes trailing newlines like any command capture. As
the last line of a value-returning function, it is the function's value;
end that line with `;` to stream it. A declared `-> ()` function has no tail
value, so its last line always runs as a statement.

Every argument of every stage is evaluated, left to right, before the first
stage starts.

**Failure.** The pipeline fails if any stage exits unsuccessfully, like bash
`set -o pipefail`, and it does so on every target, including POSIX `sh` and
dash, which have no `pipefail`. The diagnostic names the leftmost failing stage
and the source line, and the script stops with that stage's status:

```text
deploy.ush:4: pipeline stage 2 failed: grep "ready" (exit 1)
  kubectl "get" "pods" | grep "ready" | wc "-l"
```

bash's `pipefail` reports the rightmost failure instead; the leftmost one is
usually the cause. A trailing `?` (`a | b?`) covers the whole pipeline and
behaves as it does for a single command. `?` on an earlier stage is rejected,
because stages run concurrently and cannot fail on their own.

**Readers that stop early.** A stage may stop reading before its input ends,
as `head` does. The stage writing to it then fails on its next write: it dies
of SIGPIPE with status 141, or, where SIGPIPE is ignored as on some CI runners,
it sees a write error and exits with a status of its own choosing. Neither is a
failure. A stage that fails after the stage reading from it has finished was
cut off by that reader, and the reader's own status decides. So
`seq "1" "1000000" | head "-n1"` succeeds. The order is exact, not a timing
guess: each stage keeps its pipe ends open until its status is recorded, so a
reader that stopped early is always recorded first, and a reader that read to
the end is always recorded after its writer. A failing last stage always fails
the pipeline.

**Sources and sinks.** `std::io` provides three stages that lower to plain
shell redirections rather than extra processes:

| Stage | Position | Lowers to |
| --- | --- | --- |
| `from_file path` | first | `< path` on the next stage |
| `to_file path` | last | `> path` on the previous stage (truncate) |
| `append_to path` | last | `>> path` on the previous stage |

`cmd |& to_file "log"` is `cmd > log 2>&1`. stderr alone already has
`std::process::redirect`. A path is always one shell word, including spaces
and newlines. A computed path follows the `std::fs` rules: source-relative
path references resolve, and an empty path is an error. A failure to open
the file fails the stage that owns the redirection, and the diagnostic shows
both: `pipeline stage 1 failed: from_file "notes.txt" | sort`. Calling
`to_file` outside a pipeline is a compile error.

**String stdin.** A String expression as the first stage feeds its exact bytes
to the next stage's stdin, which covers here-strings and here-documents. No
newline is added: `"a\nb" | wc "-l"` prints `1`, so write `"a\nb\n"` for a
line-oriented consumer. A `"""…"""` string drops the line breaks after the
opening and before the closing quotes, so its last line has no terminator
either. A call to a command is always a command stage and never a String,
so `git "log" | head` runs git; a call to a function that returns a String
feeds that String, which has lost its trailing newlines like every returned
value.

## Local modules

Split a program into files with `mod`. A module is a `.ush` file of
declarations; its `pub` items are what other files may use.

```ush
# lib/github.ush
use std::string::{trim}

pub enum Level {
  Notice,
  Warning,
}

pub fn annotate(level: Level, message: String) {
  match level {
    Level::Notice => print "::notice::" + clean message,
    Level::Warning => print "::warning::" + clean message,
  }
}

fn clean(message: String) -> String {
  trim message
}
```

```ush
# ci/check.ush
mod "../lib/github.ush" as gh
use gh::{Level, annotate}

annotate Level::Notice "starting"
gh::annotate Level::Warning "  disk is nearly full  "
```

- `mod "path.ush" as name` imports the file at `path`, relative to the
  importing file. Without `as`, the module is named after the file stem,
  which must then be an identifier (`mod "lib/github.ush"` is `github`).
  Paths must be relative and end in `.ush`; `std`, `self` and `super` are
  reserved names.
- `name::item` calls or references a function (`gh::annotate`,
  `&gh::annotate`). Types, enums, traits and effects are brought in with
  `use`: `use gh::{Level}`, `use gh::{annotate as note}` or `use gh::*`.
  A record type can also be built as `gh::Entry { … }`.
- A module may contain only declarations: `fn`, `type`, `enum`, `trait`,
  `impl`, `effect`, `alias`, `use`, `mod`, `extern` and `test` blocks.
  Any other top-level statement is an error naming the module line.
- In a module, items are private unless marked `pub` (`pub fn`, `pub type`,
  `pub enum`, `pub trait`, `pub effect`, `pub extern … { … }`). Using a
  private item from another file is an error. Root scripts may use `pub`
  but need not.
- A file's own declarations win over its imports, and explicit imports win
  over `use m::*`. Two wildcards that offer different items under one name
  are an error unless the file declares or explicitly imports that name, as
  with std wildcards. A local declaration that is also imported by name is
  an error.
- Each file is resolved against its own imports: a module's
  `use std::string::*` does not leak into the script that imports it.
- `extern` libraries and `std::path::from_source` inside a module resolve
  beside the module file. `std::path::source_root` and
  `__ush_source_dir` stay the root script's directory.

The compiler inlines modules into one standalone script; nothing is
sourced at run time. Each module's items move into a namespace named after
the alias that first imported it (`gh::annotate` becomes
`ush_fn_gh__annotate`), so two modules may both define `annotate`. A
module imported from several places is linked once; a later module that
wants a taken namespace gets a numbered one (`util_2`). Only functions the
program can reach are emitted, like standard-library helpers. Import
cycles are rejected with the chain of files.

Compile errors inside a module name the importing line of the root script,
then the module file and its line:

```text
ush: failed to compile ci/check.ush: line 1: lib/github.ush: line 17: unknown variable: mesage
```

At run time, a failing statement inside a module reports the module file
(`lib/github.ush:12: command failed (exit 1)`), and `ush explain` and
sourcemaps map generated lines back to it. `ush check`, `ush test`,
`ush effects` and the language server read a script's modules from disk;
source compiled from memory cannot import modules. Run `ush check` on a
module file to check all of its items, including ones no script uses yet.

## Generics, kinds, and function values

```ush
fn identity<T>(value: T) -> T {
  value
}
fn keep<F<_>, T>(value: F<T>) -> F<T> {
  value
}
fn map<A, B>(items: List<A>, f: Fn<A, B>) -> List<B> {
  [f x for x in items]
}
fn greet(name: String) -> String {
  "hello " + name
}

let greetings = keep (map ["Ada", "Lin"] &greet)
let numbers = map [1, 2, 3] &identity
```

`T` has kind Type. `F<_>` has kind Type → Type, and `F<_, _>` takes two
types. Applied types are checked for arity and consistency before erasure.
For example, `List<T>` fits `F<T>`, while `Tuple<A, B>` fits a binary
constructor. Type arguments are inferred from values at a call site.

`Fn<A, B>` is a function taking `A` and returning `B`; `Fn<A, B, C>` takes
two arguments and returns `C`, and `Fn<R>` takes no arguments. Write `&greet`
to pass a function itself rather than invoke it. References can be bound
with `let f = &greet` and called as `f "Ada"`. Referenced functions need
an annotated result type. Generic references are instantiated from the
callback's expected parameter types.

The compiler specializes named callbacks and checks their actual errors and
effects. A callback that prints cannot be smuggled through a `/ ()` function.
Function values currently resolve to named functions at compile time;
anonymous capturing closures and runtime selection between different
functions are not supported.

## List comprehensions

```ush
let words = ["alpha", "beta", "gamma"]
let selected = [word + "!" for word in words if word != "beta"]
let sums = [x + y for x in 0..3 if x > 0 for y in 1..3 if y > x]
```

Comprehensions evaluate `for` and `if` clauses from left to right. Later
clauses can use earlier bindings. The result expression runs only after all
filters pass. Every generator takes a list or integer range; filters must
be Boolean. Results can contain nested lists, tuples, or records. Empty
ranges or filters that reject everything produce a typed empty list.
Bindings are local to the comprehension, and the source list is evaluated
once per enclosing iteration. Strings keep their whitespace and shell
metacharacters as data.

## Scoped cwd and environment

```ush
use $bin::*

export RUST_BACKTRACE = "1"
cd "crates/ush_compiler" {
  export CARGO_TERM_COLOR = "never", RUSTFLAGS = "-D warnings" {
    cargo "build"
  }
}
```

`cd path { … }` runs the block in `path` and returns to the previous
directory afterwards. `cd path` without a block keeps its meaning: it changes
directory for the rest of the current process scope. The path must be a
String; `CDPATH` is ignored.

`export NAME = value` sets and exports an environment variable for the rest
of the current process scope, as `std::env::set` does. `export NAME = value,
OTHER = value { … }` scopes the variables to the block: afterwards each name
gets back its previous value, or becomes unset again if it was unset. Every
value is a String and is evaluated before any variable changes, so
`export A = "2", B = std::env::get "A"` gives `B` the old `A`. A name cannot be
a ush binding in scope, `PWD`, `OLDPWD`, `IFS`, or start with `__ush`.

Blocks nest and compose with every other statement. They run in the current
shell: the block saves what it changes, runs the body and restores the saved
state. Assignments to outer bindings inside a block therefore stay visible
afterwards, as they do for `if` and loops, and bindings first made inside a
block stay inside it. `break` and `continue` restore every block they leave
before leaving it. `fail`, `?` and failing commands keep their status and
source line: at top level they end the script, and a function body runs in
a process of its own, so a failing function cannot leak its directory or
variables to its caller either. A variable that was set before the block is
exported again when restored, because a POSIX shell cannot tell whether it was
exported.

## Test blocks

```ush
use std::test::{assert, assert_eq}

test "addition" {
  assert_eq (1 + 2) 3
}
test {
  assert true
  assert_eq [x + 1 for x in 0..2] [1, 2]
}
```

Run `ush test file.ush`, `ush test tests/`, or `ush test` for the `tests/`
directory. Both named and anonymous blocks are supported. Each block runs in
a separate shell process with the file's top-level setup. Failure stops that
block, reports its label and diagnostic, and does not prevent later blocks
from running. The process exits unsuccessfully if any case fails. Files
without blocks retain the original script-as-test behavior.

Ordinary execution compiles and checks test bodies but does not run them.
There is no assertion prelude: import `std::test::assert`, `assert_eq`, or
`std::test::*`. Fully qualified calls also work. `assert_eq` requires two
values of the same type and compares scalars or serialized structured values;
it does not compare task handles. Assertion failures carry `unknown` and
`stderr` when used in ordinary typed functions. Test blocks infer their errors.

## Shell libraries through extern

Given `helpers.sh` next to the source file:

```sh
greet() { printf 'hello %s' "$1"; }
answer() { printf 42; }
```

```ush
extern "helpers.sh" as helpers {
  fn greet(name: String) -> String
  fn answer() -> Int
}
use helpers::*

print greet "ush"
print answer
```

The optional module name provides qualified calls such as `helpers::greet`;
without `as`, declarations use their direct names. `extern` sources the
library at runtime. Relative paths are resolved beside the `.ush` source
that declares the `extern`, including from a `--checked-in` script compiled
to another directory. Distribute the `.sh` file alongside relocatable
artifacts. In-memory compilation resolves
relative libraries from the working directory.

Signatures declare scalar arguments and `String`, `Int`, `Bool`, or `()`
results. Every argument is quoted as one shell word. String results capture
stdout with the shell's usual trailing-newline removal. Integers must fit
signed 64-bit decimal notation; Booleans must be `true` or `false`. Unit
functions stream their output. Nonzero shell status and malformed return
values fail the call. Without an explicit row, extern declarations expose
`unknown` errors and infer conservative shell effects. Explicit rows are
checked. The shell library itself is trusted shell code and must match the
selected target dialect; the compiler checks its declared interface and
runtime scalar results, not its implementation.
