# Standard library

Import helpers explicitly, including test assertions:

```ush
use std::fs::{read_dir, read_lines}
use std::string::{join, to_upper_ascii}
use std::test::assert_eq

assert_eq (join ["a", "b"] ",") "a,b"
for entry in read_dir "." {
  print entry
}
for line in read_lines "README.md" {
  print to_upper_ascii line
}
```

The following APIs compile to POSIX sh and standard command-line utilities.
The generated program runs without an installed ush binary. Only helpers
referenced by the program and their dependencies are included in the output.
These APIs supplement the existing [process, text and temporary-file APIs](native-scripting.md).

## Text

| API in `std::string` | Result and contract |
| --- | --- |
| `is_empty text` | `Bool`; true only for the empty string |
| `is_ascii text` | `Bool`; every byte is ASCII |
| `to_lower_ascii text`, `to_upper_ascii text` | Change ASCII letters; preserve non-ASCII bytes |
| `trim_start text`, `trim_end text` | Remove whitespace at one end, using the shell's locale |
| `words text` | `List<String>`; split on space, tab and LF; quotes, backslashes and globs are literal |
| `join items separator` | Join a `List<String>`; an empty list produces an empty string |
| `repeat text count` | Repeat text; count must be nonnegative |
| `count text part` | Number of non-overlapping literal matches; part must be nonempty |
| `shell_quote text` | One POSIX shell argument, including quotes for an empty string |

`shell_quote` is for generating shell source. Prefer typed argv through
`std::process` when invoking a command directly.

Strings cannot contain NUL. Scalar return values use shell command capture and
therefore lose trailing LF bytes. List elements preserve them. For exact file
contents use streaming APIs or `write_text` with a literal/concatenated string;
do not use a captured String as a binary container.

## Files and paths

| API in `std::fs` | Result and contract |
| --- | --- |
| `is_symlink path` | `Bool`; includes dangling symlinks |
| `is_fifo path` | `Bool`; named-pipe predicate |
| `is_readable path`, `is_writable path`, `is_executable path` | Shell access predicates for the current user |
| `mkdir path`, `rmdir path` | Create one directory / remove an empty directory |
| `touch path` | Create a file if absent; update timestamps if present |
| `remove_tree path` | Recursively remove a directory tree; absence is success |
| `read_dir path` | `List<String>` of immediate child paths, including hidden entries and dangling symlinks; order is unspecified |
| `read_lines path` | `List<String>`; split LF records, preserve CR, blank lines and a final unterminated record |
| `symlink target link` | Create a symbolic link; a relative target is interpreted relative to the link's directory |
| `hard_link target link` | Create a hard link |
| `chmod path mode` | Set a three- or four-digit octal mode supplied as a String |
| `size path` | Byte count as `Int`; follows symlinks and reads the file |

These functions accept the existing source-relative path references, reject
empty paths, and protect relative names beginning with `-` from option parsing.
Paths with spaces and newlines remain a single argument.

The following `std::path` operations are lexical: they take ordinary path strings
and do not access the filesystem or resolve source-relative references.

| API | Result and contract |
| --- | --- |
| `is_absolute path` | `Bool`; starts with `/` |
| `extension path` | Last suffix without a dot; `.profile` has no extension |
| `stem path` | Last component without its extension; `.profile` stays unchanged |
| `normalize path` | Remove repeated separators and `.`, reduce `..`; preserve leading relative `..` and exactly two leading slashes |

Normalization is not filesystem canonicalization: reducing `..` can change
the meaning of paths containing symlinks. An empty normalized path is `.`.
Trailing separators are removed; a trailing separator gives an empty stem and
extension.

## Streams, processes and host information

| API | Result and contract |
| --- | --- |
| `std::io::print text`, `println text` | Write stdout without / with a final LF |
| `std::io::read_lines` | Read stdin with the same record semantics as `fs::read_lines` |
| `std::io::read_all` | Read stdin as a String; trailing LF bytes follow capture semantics |
| `std::terminal::is_tty fd` | `Bool`; descriptor must be nonnegative |
| `std::process::pid` | Shell process ID as `Int`; retains POSIX `$$` semantics in subshells |
| `std::process::sleep seconds` | Sleep for a nonnegative whole number of seconds |
| `std::process::chdir path` | Change the current shell scope's directory; ignore `CDPATH` |
| `std::os::name`, `arch`, `hostname` | Values from `uname -s`, `-m`, `-n` |
| `std::os::uid`, `gid` | Effective user/group IDs from `id -u` and `id -g` |

Directory changes inside captured functions or async tasks stay in that process
scope. I/O and process operations infer their corresponding effects; APIs that
validate arguments or invoke fallible OS operations also infer `unknown`.
Failures stop execution and retain the ush source location.

## Integers

| API in `std::int` | Contract |
| --- | --- |
| `min left right`, `max left right` | Smaller / larger integer |
| `abs value` | Absolute value; the minimum signed 64-bit integer reports overflow |
| `sign value` | `-1`, `0` or `1` |
| `clamp value min max` | Inclusive bounds; min must not exceed max |

## Coverage and further work

This is the first portable std expansion, not complete POSIX/libc coverage.
The API inventory follows [POSIX utilities and functions](https://pubs.opengroup.org/onlinepubs/9799919799/),
with the everyday shell use cases surveyed in
[pure-sh-bible](https://github.com/dylanaraps/pure-sh-bible) and
[modernish](https://github.com/modernish/modernish). Implementations are native
to this project.

| Area | Available baseline | Remaining work |
| --- | --- | --- |
| Text | Literal search/split/trim, ASCII case, quoting, joining | Unicode transforms, binary-safe codecs |
| Filesystem | Text I/O, directories, links, permissions, access predicates | stat metadata, canonicalization, atomic writes, binary I/O |
| Process | Typed argv, status/streams, environment, cwd, sleep | Rich process control, signal and descriptor APIs |
| System | Host name/architecture, user/group IDs | libc system queries, clocks, randomness |
| Network | Existing HTTP helpers | DNS and sockets |

APIs requiring capabilities unavailable in POSIX sh will use an optional ush
runtime. Portable programs remain standalone; the runtime-backed API surface
will be introduced in separate changes.
