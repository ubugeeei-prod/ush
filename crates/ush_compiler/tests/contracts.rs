use ush_compiler::UshCompiler;
mod support;
use support::run;

#[test]
fn inferred_arguments_polymorphism_and_applied_constructor_kinds() {
    let source = r#"
fn identity(x) {
  x
}
fn keep<F<_>, T>(x: F<T>) -> F<T> {
  x
}
fn pair<F<_, _>, A, B>(x: F<A, B>) -> F<A, B> {
  x
}
let two = keep(["a", "b"])
for word in two {
  print word
}
print identity(42)
print identity("done")
let values = pair((1, 2))
for number in values {
  print number
}
"#;
    assert_eq!(run(source), "a\nb\n42\ndone\n1\n2\n");
}

#[test]
fn dependent_indices_and_predicates_are_checked_before_erasure() {
    let source = r#"
unit ms
fn count<const N: Int>(xs: [Int; N]) -> Int {
  N
}
fn same<const N: Int>(xs: [Int; N], ys: [Int; N]) -> [Int; N] {
  xs
}
fn duration(n: Int<ms> where n >= 1 && n <= 100) -> Int<ms> {
  n
}
print count(same([1, 2], [3, 4]))
print duration(5ms + 10ms)
"#;
    assert_eq!(run(source), "2\n15\n");
    for (bad, msg) in [
        (source.replace("[3, 4]", "[3]"), "array length"),
        (source.replace("5ms + 10ms", "0ms"), "refinement"),
        (source.replace("5ms + 10ms", "101ms"), "refinement"),
        (source.replace("5ms + 10ms", "15"), "type constructor arity"),
        (
            source.replace("5ms + 10ms", "10kg"),
            "unknown measurement unit",
        ),
    ] {
        let error = UshCompiler.compile_source(&bad).unwrap_err();
        assert!(format!("{error:#}").contains(msg), "{error:#}");
    }
}

#[test]
fn unused_generic_bodies_and_kinds_are_checked() {
    for (source, expected) in [
        ("fn bad<T>(x: T) -> T {\n  123\n}\n", "expected T"),
        (
            "fn bad<F<_>, T>(x: F<T,T>) -> T {\n  x\n}\n",
            "kind mismatch",
        ),
        (
            "fn bad<const N: Int>(x: [Int; M]) -> Int {\n  1\n}\n",
            "undeclared length",
        ),
        ("fn bad<T>(x: T) -> () / () {\n  print x\n}\n", "stdout"),
        (
            "fn bad() -> Problem!String {\n  \"x\"\n}\n",
            "success type first",
        ),
    ] {
        let error = UshCompiler.compile_source(source).unwrap_err();
        assert!(format!("{error:#}").contains(expected), "{error:#}");
    }
}

#[test]
fn inferred_returns_and_dependent_arguments_preserve_named_binding_order() {
    let source = r#"
fn answer() {
  2
}
fn exact(n: Int, values: [String; n]) -> [String; n] {
  values
}
let words = exact(values: ["ok", "done"], n: 2)
print answer()
for word in words {
  print word
}
"#;
    assert_eq!(run(source), "2\nok\ndone\n");
    let error = UshCompiler
        .compile_source(&source.replace("n: 2)", "n: 3)"))
        .unwrap_err();
    assert!(format!("{error:#}").contains("length"), "{error:#}");
}

#[test]
fn records_and_nested_values_cross_call_and_task_boundaries() {
    let src = r#"
type Config {
  name: String,
}
enum ConfigError {
  Missing,
}
fn load(found: Bool) -> Config / ConfigError, stderr, stdout {
  if found {
    print "loading"
    return Config { name: "a b'$;*" }
  }
  fail ConfigError::Missing
}
fn keep<T>(x: T) -> T / () {
  x
}
let job = async load(true)
let config = job.await
print config.name
let next = keep(load(true)?)
print next.name
let items = keep([config, next])
for value in items {
  print value.name
}
"#;
    assert_eq!(
        run(src),
        "loading\na b'$;*\nloading\na b'$;*\na b'$;*\na b'$;*\n"
    );
}

#[test]
fn checked_effects_follow_source_functions_and_command_capture() {
    let src = r#"
using $bin::
fn show(n) {
  print n
}
fn plus(n) {
  n + 1
}
fn get() -> String / unknown, exec, stderr {
  printf("%s", "ok")?
}
show(plus(4))
print get()?
"#;
    assert_eq!(run(src), "5\nok\n");
    let report = UshCompiler.effects_source(src).unwrap();
    let show = report.functions.iter().find(|f| f.name == "show").unwrap();
    assert_eq!(show.inferred.to_string(), "stdout");
    assert!(
        report
            .functions
            .iter()
            .all(|f| !f.name.starts_with("__ush_inst"))
    );
    let error = UshCompiler
        .compile_source(&src.replace("unknown, exec, stderr", "unknown, exec, stdout"))
        .unwrap_err();
    assert!(format!("{error:#}").contains("stderr"), "{error:#}");
}
