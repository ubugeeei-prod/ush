# Types at the shell boundary

`ush check script.ush` runs the compiler’s type and effect checks. Local bindings infer their types and are immutable by default. Use `let mut name = value` for mutable bindings and `name = value` for reassignment; assignments preserve the binding type. Functions can infer their argument and return types.

```ush
fn identity(x) {
  x
}

fn keep<F<_>, T>(x: F<T>) -> F<T> / () {
  x
}

echo (identity(42))
echo (identity("ready"))
let names = keep(["test", "release"])
```

Generic functions are checked at their definitions and specialized for the concrete types used by callers. `T` stands for a type. `F<_>` stands for a constructor taking one type, such as a list. `F<_, _>` takes two, such as a pair. The checker validates constructor arity and repeated uses of the same type variable. Recursive specializations need an annotated return type.

## Quantities and value-dependent types

```ush
unit ms

fn delay(n: Int<ms> where n > 0 && n <= 1000) -> Int<ms> / () {
  n
}

fn length<const N: Int>(items: [String; N]) -> Int / () {
  N
}

echo (delay (5ms + 10ms))
echo (length ["a", "b"])
```

Units are nominal. Declare each suffix once. Addition and comparison require compatible units. The runtime representation of a quantity is an integer in its declared unit.

A length can be a nonnegative integer, a const parameter, or the name of an integer argument. Reusing an index requires the same length. Integer refinements support `<`, `<=`, `>`, `>=` and `==` against integer literals joined by `&&`. The checker uses constants, addition, and comparisons in true branches to prove bounds. An argument whose bounds cannot be proved produces a diagnostic. The supported dependent-type fragment covers indexed sequences and integer refinements.

## Calling installed commands

```ush
use $bin::*

cd "tools"?
let status_text = git "status" "--short"?
echo status_text
```

Fully qualified calls work without an import, for example `$bin::git "status" "--short"`. Local functions and explicit imports take precedence over the wildcard. The catalog covers the 155 named entries in the POSIX.1-2024 utilities index and common tools including git, curl, jq, and package managers. Availability depends on the target host. [POSIX utilities index](https://pubs.opengroup.org/onlinepubs/9799919799/idx/utilities.html)

The checker validates positional arguments, accepted arity, return type and effect row. Each argument becomes one quoted argv entry. Arguments generally accept `String` or `Int`. `cd` takes an optional string path and changes the current shell directory when called as a statement. Text commands can stream output as statements or return it in an expression, actions return `()`, and `$bin::test(...)` returns a Boolean. General-purpose tools such as git, curl, and jq accept variadic argv. Their option grammars, subcommands and output schemas remain the responsibility of the tool.

## Inline shell and tasks

```ush
let text = $(
  printf '%s\n' "a shell pipeline" | tr 'a-z' 'A-Z'
)?
echo text

fn build(path: String) -> String {
  "built " + path
}
let job = async build "release"
let result = job.await
echo result
```

Inside a capture the selected shell handles quotes, pipes, redirections, here-documents and variable expansion. For multiline captures, put the final `)?` at the indentation of the opening source line. `ush script.ush` inherits standard input. Inline shell code follows the dialect of the chosen interpreter.

Launching a task evaluates the call arguments in the parent and starts a background shell process. Handles can be awaited more than once, including from sibling tasks that capture the handle. Each waiter observes the same completed value or exit status. The enclosing task scope waits for unfinished children before cleaning up its private result directory. Task processes inherit a snapshot of shell state and can share files and other external resources.

See [shell-style language](shell-language.md) for `Fn<A, B>` callbacks, list comprehensions, test blocks, and `.sh` extern modules.
