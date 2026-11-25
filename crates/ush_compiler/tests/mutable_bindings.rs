use ush_compiler::UshCompiler;
mod support;

fn succeeds(source: &str, expected: &str) {
    support::each_program(source, |shell, _, (status, stdout, stderr)| {
        assert_eq!(status, 0, "{shell}: {stderr}");
        assert_eq!(stdout, expected, "{shell}: {stderr}");
    });
}

#[test]
fn assignments_update_outer_storage_across_control_flow() {
    succeeds(
        r#"
let mut count = 0
while count < 3 {
  count = count + 1
}
for n in [1, 2] {
  count = count + n
}
match count {
  6 => count = 7
  _ => count = 0
}
if count == 7 {
  count = count + 1
}
export USH_BINDING_TEST = "scoped" {
  count = count + 1
}
print count
"#,
        "9\n",
    );
}

#[test]
fn shadowing_does_not_mutate_outer_bindings_or_parameters() {
    succeeds(
        r#"
let mut value = 10
if true {
  let mut value = value + 1
  value = value + 1
  print value
}
print value
fn add(value: Int) -> Int {
  let mut value = value + 1
  value = value + 1
  value
}
print add 1
let value = "new type"
print value
"#,
        "12\n10\n3\nnew type\n",
    );
}

#[test]
fn aggregates_can_be_reassigned_from_their_previous_values() {
    succeeds(
        r#"
let mut values = [1, 2]
if true {
  values = [x + 1 for x in values]
}
for n in values {
  print n
}
values = [9]
for n in values {
  print n
}
let mut pair = (1, 2)
pair = (3, 4)
for n in pair {
  print n
}
"#,
        "2\n3\n9\n3\n4\n",
    );
}

#[test]
fn mutable_tasks_and_await_results_keep_stable_storage() {
    succeeds(
        r#"
fn answer(value: Int) -> Int {
  value
}
let mut task = async answer 1
let mut value = task.await
if true {
  task = async answer 2
}
value = task.await
print value
task = async {
  let mut n = 2
  n = n + 1
  n
}
value = task.await
print value
"#,
        "2\n3\n",
    );
}

#[test]
fn invalid_assignments_explain_the_required_declaration() {
    for (source, expected) in [
        (
            "let x = 1\nx = 2\n",
            "line 2: cannot assign to immutable binding `x`; declare it with `let mut x = ...`",
        ),
        ("x = 1\n", "undeclared binding `x`"),
        ("fn f(x: Int) {\n x = 1\n}\n", "immutable binding `x`"),
        ("for x in [1] {\n x = 2\n}\n", "immutable binding `x`"),
        ("let mut x = 1\nx = \"bad\"\n", "expected Int, found String"),
        (
            "let mut x = [1]\nx = [\"bad\"]\n",
            "expected [Int], found [String]",
        ),
        (
            "fn unused<T>(x: T) -> T {\n x = x\n x\n}\n",
            "immutable binding `x`",
        ),
        (
            "let mut x = 1\nif true {\n let x = 2\n x = 3\n}\n",
            "immutable binding `x`",
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
fn mutable_generic_locals_do_not_keep_initializer_refinements() {
    succeeds(
        r#"
fn replace<T>(first: T, second: T) -> T {
  let mut result = first
  result = second
  result
}

print replace 1 2
print replace "old" "new"
fn increment() -> Int {
  let mut x = 0
  if x == 0 {
    x = x + 1
  }
  x
}
print increment
"#,
        "2\nnew\n1\n",
    );
}

#[test]
fn loop_bindings_and_callees_cannot_clobber_immutable_values() {
    succeeds(
        r#"
let value = 10
for value in [1, 2] {
  print value
}
for value in 3..5 {
  print value
}
for value in (5, 6) {
  print value
}
print value
fn child() -> () {
  let value = 20
  let local = "child"
  print local
}
fn parent() -> () {
  let local = "parent"
  child
  print local
}
parent
print value
"#,
        "1\n2\n3\n4\n5\n6\n10\nchild\nparent\n10\n",
    );
}

#[test]
fn structured_assignment_finishes_reading_before_overwriting_fields() {
    succeeds(
        r#"
enum Pair {
  Pair { first: Int, second: Int },
}
let mut pair = Pair::Pair { first: 1, second: 2 }
pair = Pair::Pair { first: pair.second, second: pair.first }
print pair.first
print pair.second
let mut xs = [1, 2]
xs = xs
for x in xs {
  print x
}
"#,
        "2\n1\n1\n2\n",
    );
}

#[test]
fn pattern_and_handler_blocks_preserve_outer_bindings() {
    succeeds(
        r#"
enum Boxed {
  Value(Int),
}
let value = 10
match Boxed::Value(2) {
  Boxed::Value(value) => print value
}
if let Boxed::Value(value) = Boxed::Value(3) {
  print value
}
print value
try {
  let value = 4
  print value
} with log(message) {
  print message
}
effect log(message: String) -> ()
print value
"#,
        "2\n3\n10\n4\n10\n",
    );
}
