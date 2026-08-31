//! Runtime behaviour of the loop lowerings: every program here is
//! compiled to POSIX `sh` and executed, so the assertions describe
//! what a user actually sees.

mod support;

use support::run;

#[test]
fn an_if_expression_returns_the_taken_branch() {
    let output = run(r#"
        fn classify(value: Int) -> String {
          if value < 3 {
            "small"
          }
          else {
            "large"
          }
        }
        print classify 2
        print classify 9
    "#);

    assert_eq!(output, "small\nlarge\n");
}

#[test]
fn an_else_if_chain_picks_the_first_true_branch() {
    let output = run(r#"
        fn classify(value: Int) -> String {
          if value < 0 {
            "negative"
          }
          else if value == 0 {
            "zero"
          }
          else {
            "positive"
          }
        }
        print classify 0
        print classify 5
    "#);

    assert_eq!(output, "zero\npositive\n");
}

#[test]
fn a_range_loop_walks_the_half_open_interval() {
    assert_eq!(run("for item in 0..3 {\n  print item\n}\n"), "0\n1\n2\n");
}

#[test]
fn an_empty_range_runs_no_iterations() {
    assert_eq!(
        run("for item in 0..0 {\n  print item\n}\nprint \"done\"\n"),
        "done\n"
    );
}

#[test]
fn list_and_tuple_literals_are_both_iterable() {
    let output = run(r#"
        let items = [3, 4]
        for item in items {
          print item
        }
        let pair = (5, 6)
        for item in pair {
          print item
        }
    "#);

    assert_eq!(output, "3\n4\n5\n6\n");
}

#[test]
fn a_while_loop_stops_when_its_condition_fails() {
    let output = run(r#"
        let count = 0
        while count < 3 {
          print "tick:" + count
          let count = count + 1
        }
    "#);

    assert_eq!(output, "tick:0\ntick:1\ntick:2\n");
}

#[test]
fn a_while_loop_with_a_false_condition_never_runs() {
    let output = run(r#"
        let count = 5
        while count < 3 {
          print "unreachable"
        }
        print "done"
    "#);

    assert_eq!(output, "done\n");
}

#[test]
fn break_exits_an_unbounded_loop() {
    assert_eq!(run("loop {\n  print \"once\"\n  break\n}\n"), "once\n");
}

#[test]
fn continue_skips_the_rest_of_an_iteration() {
    let output = run(r#"
        for item in 0..4 {
          if item == 1 {
            continue
          }
          print item
        }
    "#);

    assert_eq!(output, "0\n2\n3\n");
}

#[test]
fn break_leaves_a_for_loop_early() {
    let output = run(r#"
        for item in 0..5 {
          if item == 2 {
            break
          }
          print item
        }
    "#);

    assert_eq!(output, "0\n1\n");
}
