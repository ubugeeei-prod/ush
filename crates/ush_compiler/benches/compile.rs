use criterion::{Criterion, criterion_group, criterion_main};
use std::fmt::Write;
use ush_compiler::UshCompiler;

const SMALL_SOURCE: &str = r#"
let greeting = "hello"
print greeting + " world"
$ printf '%s\n' hi
"#;

const ADT_SOURCE: &str = r#"
enum Result {
  Ok(String),
  Err(String),
}

enum Envelope {
  Wrap(Result),
  Missing,
}

let payload = Envelope::Wrap(Result::Ok("done"))

match payload {
  Envelope::Wrap(Result::Ok(message)) => print message
  Envelope::Wrap(Result::Err(message)) => print "error: " + message
  _ => print "missing"
}
"#;

fn bench_compile(criterion: &mut Criterion) {
    let compiler = UshCompiler;

    criterion.bench_function("compile empty ush program", |bench| {
        bench.iter(|| {
            let _ = compiler.compile_source("").expect("compile empty");
        });
    });

    criterion.bench_function("compile small ush program", |bench| {
        bench.iter(|| {
            let _ = compiler
                .compile_source(SMALL_SOURCE)
                .expect("compile small");
        });
    });

    criterion.bench_function("compile adt ush program", |bench| {
        bench.iter(|| {
            let _ = compiler.compile_source(ADT_SOURCE).expect("compile adt");
        });
    });
    let mut large = String::new();
    for i in 0..1000 {
        writeln!(large, "fn value_{i}(n: Int) -> Int {{\n  n + {i}\n}}").unwrap();
    }
    for i in 0..1000 {
        writeln!(large, "print value_{i}(1)").unwrap();
    }
    criterion.bench_function("compile 1000 functions", |bench| {
        bench.iter(|| compiler.compile_source(&large).unwrap());
    });
    criterion.bench_function("compile typed shell interop", |bench| {
        bench.iter(|| {
            compiler
                .compile_source(include_str!("../../../examples/contracts.ush"))
                .unwrap()
        });
    });
}

criterion_group!(benches, bench_compile);
criterion_main!(benches);
