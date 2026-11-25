use ush_compiler::UshCompiler;
mod support;
use support::run;

#[test]
fn shell_calls_and_wildcards_work_in_value_positions() {
    assert_eq!(
        run(r#"
use std::string::*
fn answer() -> Int {
  42
}
fn greet(a: String, b: String) -> String {
  a + b
}
print greet "hello " (trim_prefix " ush" " ")
print replace "a(b)" "a" "c"
print answer
let result = answer
print result
let answer = "shadow"
print answer
"#),
        "hello ush\nc(b)\n42\n42\nshadow\n"
    );
}

#[test]
fn comprehensions_preserve_scope_and_nested_values() {
    assert_eq!(
        run(r#"
let x = "outside"
let words = [x + "!" for x in ["a b", "c'$;"] if x != "skip"]
for word in words {
  print word
}
print x
for n in [x + y for x in 0..3 if x > 0 for y in 1..3 if y > x] {
  print n
}
for row in [[x, x + 1] for x in 1..3] {
  for n in row {
    print n
  }
}
for n in [x for x in 3..1] {
  print "unreachable"
}
"#),
        "a b!\nc'$;!\noutside\n3\n1\n2\n2\n3\n"
    );
}

#[test]
fn generic_higher_order_functions_specialize_callbacks() {
    assert_eq!(
        run(r#"
fn inc(x: Int) -> Int {
  x + 1
}
fn twice(x: Int) -> Int {
  x + x
}
fn map<A, B>(items: List<A>, f: Fn<A, B>) -> List<B> {
  [f $x for x in items]
}
fn keep<F<_>, T>(x: F<T>) -> F<T> {
  x
}
for n in keep (map [1, 2] &inc) {
  print n
}
for n in map [1, 2] &twice {
  print n
}
let f = &inc
print f 40
"#),
        "2\n3\n2\n4\n41\n"
    );
}

#[test]
fn invalid_filters_callbacks_and_wildcards_are_rejected() {
    for (source, expected) in [
        ("use std::missing::*", "unknown module"),
        ("use std::string::", "use module::*"),
        ("use std::path::*\nuse std::fs::*", "ambiguous wildcard"),
        ("let xs = [x for x in 0..2 if 1]", "filter must be Bool"),
        ("let xs = [x for x in 1]", "list or range"),
        (
            "fn apply(f: Fn<Int, Int>) -> Int {\n f \"bad\"\n}",
            "expected Int",
        ),
    ] {
        let error = UshCompiler.compile_source(source).unwrap_err();
        assert!(
            format!("{error:#}").contains(expected),
            "{source}: {error:#}"
        );
    }
}

#[test]
fn generic_function_references_are_instantiated_from_callback_types() {
    assert_eq!(
        run(r#"
fn id<T>(x: T) -> T {
  x
}
fn map<A, B>(xs: List<A>, f: Fn<A, B>) -> List<B> {
  [f $x for x in xs]
}
for word in map ["one", "two"] &id {
  print word
}
"#),
        "one\ntwo\n"
    );
}

#[test]
fn callback_effects_and_errors_are_checked_after_specialization() {
    let source = r#"
fn noisy(x: Int) -> Int {
  print "side effect"
  x
}
fn apply(f: Fn<Int, Int>, x: Int) -> Int / () {
  f $x
}
print apply &noisy 1
"#;
    let error = UshCompiler.compile_source(source).unwrap_err();
    assert!(format!("{error:#}").contains("stdout"), "{error:#}");
}

#[test]
fn function_collections_cannot_silently_select_the_wrong_callback() {
    let source =
        "fn a(x: Int) -> Int {\n x + 1\n}\nfn b(x: Int) -> Int {\n x + 2\n}\nlet fs = [&a, &b]\n";
    let error = UshCompiler.compile_source(source).unwrap_err();
    assert!(
        format!("{error:#}").contains("different function values"),
        "{error:#}"
    );
}

#[test]
fn grouped_arguments_keep_statement_call_effects_in_the_parent_shell() {
    assert_eq!(
        run(r#"
use std::env::{set, get_or}
set ("USH_GROUPED_CALL_TEST") "kept"
print get_or "USH_GROUPED_CALL_TEST" "lost"
"#),
        "kept\n"
    );
}

#[test]
fn aliases_of_function_values_preserve_identity() {
    assert_eq!(
        run(r#"
use std::test::assert_eq
fn inc(x: Int) -> Int {
  x + 1
}
let alias = &inc
assert_eq &alias &inc
print alias 2
"#),
        "3\n"
    );
}

#[test]
fn assertions_compare_source_types_before_erasure() {
    let source =
        "use std::test::assert_eq\nfn inc(x: Int) -> Int {\n x + 1\n}\nassert_eq &inc \"inc\"\n";
    let error = UshCompiler.compile_source(source).unwrap_err();
    assert!(format!("{error:#}").contains("same type"), "{error:#}");
}
