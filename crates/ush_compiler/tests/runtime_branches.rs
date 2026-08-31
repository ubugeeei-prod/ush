//! Runtime behaviour of `if let` and `match` lowerings.

mod support;

use support::run;

#[test]
fn if_let_binds_and_guards_at_once() {
    let output = run(r#"
        enum Option {
          None,
          Some(Int),
        }
        let maybe = Option::Some(7)
        if let Option::Some(it) = maybe && it == 7 {
          print "bound:" + it
        }
        if let Option::Some(it) = maybe && it == 8 {
          print "unreachable"
        }
        print "done"
    "#);

    assert_eq!(output, "bound:7\ndone\n");
}

#[test]
fn if_let_skips_a_non_matching_variant() {
    let output = run(r#"
        enum Option {
          None,
          Some(Int),
        }
        let maybe = Option::None
        if let Option::Some(it) = maybe {
          print "unreachable"
        }
        print "done"
    "#);

    assert_eq!(output, "done\n");
}

#[test]
fn nested_loops_keep_their_own_counters() {
    let output = run(r#"
        for outer in 0..2 {
          for inner in 0..2 {
            print outer + ":" + inner
          }
        }
    "#);

    assert_eq!(output, "0:0\n0:1\n1:0\n1:1\n");
}

#[test]
fn a_match_on_literals_falls_through_to_the_wildcard() {
    let output = run(r#"
        fn describe(value: Int) -> String {
          match value {
            0 => "zero"
            1 => "one"
            _ => "many"
          }
        }
        print describe 0
        print describe 1
        print describe 9
    "#);

    assert_eq!(output, "zero\none\nmany\n");
}

#[test]
fn a_match_on_strings_compares_by_value() {
    let output = run(r#"
        fn describe(value: String) -> String {
          match value {
            "a" => "first"
            "b" => "second"
            _ => "other"
          }
        }
        print describe "a"
        print describe "z"
    "#);

    assert_eq!(output, "first\nother\n");
}

#[test]
fn continue_also_advances_a_list_loop() {
    let output = run(r#"
        let items = [1, 2, 3, 4]
        for item in items {
          if item == 2 {
            continue
          }
          print item
        }
    "#);

    assert_eq!(output, "1\n3\n4\n");
}

#[test]
fn break_leaves_a_list_loop_early() {
    let output = run(r#"
        let items = [1, 2, 3]
        for item in items {
          if item == 3 {
            break
          }
          print item
        }
    "#);

    assert_eq!(output, "1\n2\n");
}
