# Optional std runtime

The [portable standard library](stdlib.md) still produces standalone POSIX sh.
The APIs below require ush 0.15 or newer on the destination machine. Generated
code invokes a versioned, argv-only interface in the ush executable; it never
evaluates arguments as shell source.

The executable is `ush` on PATH, or the single executable path in
`USH_RUNTIME`. Paths containing spaces work. An unavailable executable reports
the API name, the ush source location and how to configure the runtime, then
exits with status 127. `std::runtime::available` returns a Bool after checking
protocol support. It does not fail when the runtime is absent.

```ush
use std::fs::{read_bytes, write_bytes, write_atomic}
use std::bytes::{encode_base64, decode_base64}
use std::time::{unix_seconds, format_utc}

let contents = read_bytes "input.bin"
write_bytes "copy.bin" contents
write_atomic "timestamp.txt" (format_utc unix_seconds "%Y-%m-%dT%H:%M:%SZ")
echo (encode_base64 contents)
```

All runtime-backed operations except `available` can raise `unknown`, including
when the executable is missing. Filesystem, environment, clock, randomness and
network APIs retain their corresponding effect capabilities. Integer and text
operations remain free of observable side effects.

## Unicode and binary data

| API | Contract |
| --- | --- |
| `std::string::length text` | Unicode scalar count, not grapheme/display width |
| `byte_length text` | UTF-8 byte count |
| `chars text` | `List<String>`, one Unicode scalar per element |
| `reverse text` | Reverse Unicode scalars |
| `to_lower text`, `to_upper text` | Unicode case conversion; may expand a character |
| `slice text start end` | Scalar indices, end exclusive; bounds must be valid |
| `find text part` | First scalar index, or -1; empty part matches at 0 |
| `std::bytes::from_text text` | UTF-8 as `List<Int>` bytes |
| `to_text bytes` | Validate UTF-8 and reject NUL |
| `from_hex text`, `to_hex bytes` | Decode even-length hex / encode lowercase hex |
| `decode_base64 text`, `encode_base64 bytes` | Strict standard alphabet and padding; reject noncanonical trailing bits |

Each byte must be in 0..255. Binary values preserve NUL and trailing newlines;
String returns retain shell capture semantics and lose trailing LF bytes.
Unicode transformations use Rust's Unicode tables, not the shell locale.

Byte lists cross the runtime boundary through argv as encoded values. They
are intended for small buffers; operating-system argv limits apply. Streaming
large binary files requires process redirection until streaming byte APIs land.

## Filesystem

All path arguments support ush source-relative references. Metadata queries
follow symlinks; `read_link` reads the link itself. Path String results must be
UTF-8 and follow String capture semantics.

| API in `std::fs` | Contract |
| --- | --- |
| `read_bytes path`, `write_bytes path bytes` | Binary file I/O with `List<Int>` |
| `read_link path` | Link target as stored |
| `canonicalize path` | Absolute existing path with symlinks resolved |
| `mode path` | Permission/special bits as an Int (mask 07777) |
| `owner path`, `group path` | Numeric user/group IDs |
| `modified path`, `accessed path` | Signed Unix timestamp in seconds |
| `inode path`, `links path` | Inode / hard-link count; fail if not representable as Int |
| `is_socket path` | Bool; a missing path returns false |
| `write_atomic path text` | Write a sibling temporary file, sync its contents, then rename over the destination |

`write_atomic` provides atomic visibility on the destination filesystem. It
preserves an existing regular file's mode; new files use the private temporary
file mode (0600). It replaces a destination symlink instead of following it.
It does not preserve ownership, extended attributes or hard-link identity,
and does not fsync the parent directory for crash durability.

## Clocks, randomness and host information

| API | Contract |
| --- | --- |
| `std::time::unix_seconds`, `unix_millis` | Current Unix time as Int |
| `sleep_millis milliseconds` | Sleep for a nonnegative duration |
| `format_utc seconds format` | UTC calendar formatting; independent of TZ |
| `std::random::bytes count`, `hex count` | OS random bytes / their hexadecimal encoding; count is bytes |
| `std::random::int min max` | Uniform inclusive range using rejection sampling; supports the full signed 64-bit range |
| `std::net::resolve host` | Sorted unique IP addresses from the system resolver; no connection is opened |
| `std::os::euid`, `egid` | Effective IDs |
| `page_size`, `cpu_count` | System page size / available parallelism |
| `error_message code` | Platform message for a signed 32-bit OS error code |

`format_utc` accepts the single-letter POSIX directives
`aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%`. Modifiers, widths and extensions such
as `%s` are rejected. `%z` is always `+0000` and `%Z` is `UTC`.
The empty format produces an empty string; formatted output is limited to
65535 bytes. The runtime uses the C locale.

## Checked integer operations

`std::int::{add, sub, mul, div, rem, pow}` each take two Int arguments.
They reject signed 64-bit overflow and division by zero. Division truncates
toward zero; remainder has the dividend's sign. The exponent for `pow` must
fit a nonnegative 32-bit integer.

The runtime is supported on the same Linux/macOS targets as ush releases.
This remains an incremental POSIX/libc implementation: sockets, richer process
control, signals, resource limits and streaming descriptors are future work.
