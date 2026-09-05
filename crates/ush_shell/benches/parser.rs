use std::collections::BTreeMap;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use ush_shell::parse_line;

/// Lines a REPL session actually sees: the native fast path, the
/// POSIX fallback path, alias expansion, and helper pipelines.
const NATIVE_LINES: &[(&str, &str)] = &[
    ("bare command", "ls"),
    ("command with flags", "ls -la src/repl"),
    (
        "long argument list",
        "git commit -m 'tidy up the parser hot path' --no-verify",
    ),
    (
        "assignments",
        "EDITOR=vim RUST_LOG=debug cargo test --workspace",
    ),
    (
        "quoted arguments",
        "rg --hidden 'fn needs_posix_fallback' crates apps",
    ),
];

const FALLBACK_LINES: &[(&str, &str)] = &[
    ("redirect", "cargo test --workspace > out.log"),
    ("command substitution", "echo $(git rev-parse HEAD)"),
    (
        "shell keyword",
        "for file in src/*.rs; do wc -l $file; done",
    ),
    ("subshell", "(cd build && cmake --build .)"),
];

/// And-or lists, which `ush` parses itself.
///
/// These used to live in `FALLBACK_LINES`, where the measurement was
/// the cost of copying the line into `ParsedLine::Fallback` and
/// handing the whole thing to `/bin/sh` — which is exactly the bug
/// that made `mkdir x && cd x` unable to reach the `cd` builtin.
/// They now measure what they say: parsing every command in the
/// list.
const LIST_LINES: &[(&str, &str)] = &[
    (
        "boolean chain",
        "cargo fmt --all && cargo clippy --workspace",
    ),
    ("sequence", "cd crates/ush_shell; cargo test --lib"),
    ("or fallback", "cargo build || echo build failed"),
];

fn alias_table() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("ll".to_string(), "ls -la".to_string()),
        ("gs".to_string(), "git status".to_string()),
        ("gc".to_string(), "git commit".to_string()),
        ("ls".to_string(), "ls --color=auto".to_string()),
    ])
}

fn bench_parser(criterion: &mut Criterion) {
    let aliases = BTreeMap::from([("ll".to_string(), "ls -la".to_string())]);
    criterion.bench_function("parse pipeline with helper", |bench| {
        bench.iter(|| {
            let _ = parse_line(
                "ll src | filter(it -> contains(it, \"rs\")) | len",
                &aliases,
            );
        });
    });

    let empty = BTreeMap::new();
    let mut native = criterion.benchmark_group("parse native line");
    for (name, line) in NATIVE_LINES {
        native.bench_with_input(BenchmarkId::from_parameter(name), line, |bench, line| {
            bench.iter(|| parse_line(line, &empty));
        });
    }
    native.finish();

    let mut fallback = criterion.benchmark_group("parse fallback line");
    for (name, line) in FALLBACK_LINES {
        fallback.bench_with_input(BenchmarkId::from_parameter(name), line, |bench, line| {
            bench.iter(|| parse_line(line, &empty));
        });
    }
    fallback.finish();

    let mut lists = criterion.benchmark_group("parse and-or list");
    for (name, line) in LIST_LINES {
        lists.bench_with_input(BenchmarkId::from_parameter(name), line, |bench, line| {
            bench.iter(|| parse_line(line, &empty));
        });
    }
    lists.finish();

    let table = alias_table();
    criterion.bench_function("parse with alias table", |bench| {
        bench.iter(|| parse_line("ls -la src", &table));
    });

    criterion.bench_function("parse long pipeline", |bench| {
        bench.iter(|| {
            parse_line(
                "cat notes.txt | lines | filter(it -> contains(it, \"todo\")) \
                 | map(it -> upper(it)) | fsort | funiq | len",
                &empty,
            )
        });
    });

    criterion.bench_function("parse comment only", |bench| {
        bench.iter(|| parse_line("# just a note about the parser", &empty));
    });
}

criterion_group!(benches, bench_parser);
criterion_main!(benches);
