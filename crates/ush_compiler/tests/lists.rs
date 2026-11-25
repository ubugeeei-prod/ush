//! The List API under every installed shell: length, membership, ends,
//! joining, indexing, concatenation and indexed iteration, including empty
//! lists, out-of-range indexes and values that look like shell syntax.

mod support;

use support::each_program;
use ush_compiler::UshCompiler;

const PRELUDE: &str = "use std::string::split\n";

fn succeeds(body: &str, expected: &str) {
    each_program(
        &format!("{PRELUDE}{body}"),
        |shell, _, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(stdout, expected, "{shell}");
        },
    );
}

fn fails(body: &str, line: usize, message: &str) {
    each_program(
        &format!("{PRELUDE}{body}"),
        |shell, _, (status, _, stderr)| {
            assert_eq!(status, 1, "{shell}: {stderr}");
            let expected = format!("<memory>:{line}: {message} (exit 1)");
            assert!(stderr.contains(&expected), "{shell}: {stderr}");
        },
    );
}

fn rejects(body: &str, message: &str) {
    let error = UshCompiler
        .compile_source(&format!("{PRELUDE}{body}"))
        .expect_err("the program must not compile");
    assert!(format!("{error:#}").contains(message), "{error:#}");
}

#[test]
fn queries_on_a_list_of_strings() {
    succeeds(
        r#"let files = ["a b.ts", "$HOME", "*", "x\ny", ""]
print files.len()
print files.is_empty()
print files.contains("*")
print files.contains("a")
print files.contains("")
print files.first()
print files.last() == ""
print files[1]
print files[files.len() + -2]
print files.join("|")
print [1, 2, 3].join(", ")
"#,
        "5\nfalse\ntrue\nfalse\ntrue\na b.ts\ntrue\n$HOME\nx\ny\na b.ts|$HOME|*|x\ny|\n1, 2, 3\n",
    );
}

#[test]
fn empty_lists_have_no_ends() {
    succeeds(
        r#"let none = split "" ","
let blank = [x for x in ["a"] if x == "b"]
print none.len()
print blank.is_empty()
print blank.contains("a")
print $"[${blank.join(",")}]"
for x in blank {
  print "unreachable"
}
"#,
        "1\ntrue\nfalse\n[]\n",
    );
    fails(
        "let blank = [x for x in [\"a\"] if x == \"b\"]\nprint blank.first()\n",
        3,
        "first() needs a nonempty list",
    );
    fails(
        "let blank = [x for x in [\"a\"] if x == \"b\"]\nlet last = blank.last()\n",
        3,
        "last() needs a nonempty list",
    );
}

#[test]
fn indexes_outside_the_list_stop_the_program() {
    fails(
        "let xs = split \"a,b,c\" \",\"\nprint xs[3]\n",
        3,
        "index 3 is out of range for a list of length 3",
    );
    fails(
        "let xs = split \"a,b,c\" \",\"\nlet i = -1\nprint xs[i]\n",
        4,
        "index -1 is out of range for a list of length 3",
    );
    fails(
        "fn pick(xs: List<String>, i: Int) -> String / unknown {\n  xs[i]\n}\nprint pick [\"a\"] 1\n",
        3,
        "index 1 is out of range for a list of length 1",
    );
}

#[test]
fn concatenation_builds_a_new_list() {
    succeeds(
        r#"let base = ["a", "b"]
let more = base + ["c"] + split "d,e" ","
print more.join(",")
print base.join(",")
let mut grown = ["z"] + base
grown = grown + ["y"] + grown
print grown.join(",")
let nested = [["a", "b"], ["c"]] + [["d"]]
print nested.len()
print nested[0].join("+")
print nested.last().join("+")
"#,
        "a,b,c,d,e\na,b\nz,a,b,y,z,a,b\n3\na+b\nd\n",
    );
}

#[test]
fn enumerate_and_tuple_destructuring_iterate_with_an_index() {
    succeeds(
        r#"let files = ["a.ts", "b c.md"]
for (i, file) in files.enumerate() {
  print $"$i: $file"
}
for (_, file) in files.enumerate() {
  print file
}
let pairs = files.enumerate()
print pairs.len()
let pair = pairs[1]
print pair.0
print pair.1
for (n, ws) in [["x", "y"], ["z"]].enumerate() {
  print $"$n=${ws.join("")}"
}
"#,
        "0: a.ts\n1: b c.md\na.ts\nb c.md\n2\n1\nb c.md\n0=xy\n1=z\n",
    );
}

#[test]
fn list_methods_are_typed() {
    rejects(
        "let xs = [\"a\"]\nprint xs.contains(1)\n",
        "`contains` on a [String] needs a String argument",
    );
    rejects(
        "let xs = [[\"a\"]]\nprint xs.join(\",\")\n",
        "`join` needs a list of String, Int or Bool",
    );
    rejects(
        "let xs = [\"a\"]\nprint xs[\"0\"]\n",
        "a list index must be an Int",
    );
    rejects(
        "let xs = [\"a\"]\nlet ys = xs + [1]\n",
        "`+` joins lists of one type only",
    );
    rejects(
        "let x = \"abc\"\nprint x[0]\n",
        "only a List can be indexed",
    );
    rejects(
        "let pair = (\"a\", 1)\nprint pair.2\n",
        "out of range for a tuple of 2 elements",
    );
    rejects(
        "fn first() -> String / () {\n  let xs = [\"a\"]\n  xs[0]\n}\nprint first\n",
        "can raise `unknown`",
    );
}

#[test]
fn the_checker_rejects_indexes_it_can_prove_out_of_range() {
    rejects(
        "fn pick<T>(xs: List<T>) -> T {\n  xs[0]\n}\nlet xs = [\"a\", \"b\"]\nprint xs[2]\nprint pick $xs\n",
        "index 2 is out of range for a list of length 2",
    );
    rejects(
        "fn pick<T>(xs: List<T>) -> T {\n  xs[-1]\n}\nprint pick [\"a\"]\n",
        "index -1 is negative",
    );
}
