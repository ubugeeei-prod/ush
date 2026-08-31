use criterion::{Criterion, criterion_group, criterion_main};
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
}

criterion_group!(benches, bench_compile);
criterion_main!(benches);
