# Shell-style language

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
library at runtime. Relative paths are resolved beside the `.ush` source,
or beside the generated script for `--checked-in` output. Distribute the
`.sh` file alongside relocatable artifacts. In-memory compilation resolves
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
