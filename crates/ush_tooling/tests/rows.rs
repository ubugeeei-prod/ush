//! Hovers on a function show its row, written or inferred.

use ush_tooling::hover;

#[test]
fn function_hovers_show_inferred_and_declared_rows() {
    let source = "fn loud(text: String) -> () {\n  print text\n}\nfn pure_one() -> Int / () {\n  1\n}\nloud \"x\"\nprint pure_one\n";
    let inferred = hover(source, 6, 1).expect("hover on `loud`");
    assert!(
        inferred.contents.ends_with("row: `stdout` (inferred)"),
        "{}",
        inferred.contents
    );
    let declared = hover(source, 7, 7).expect("hover on `pure_one`");
    assert!(
        declared.contents.ends_with("row: `pure` (declared)"),
        "{}",
        declared.contents
    );
}

#[test]
fn a_program_that_does_not_parse_hovers_without_a_row() {
    let hover = hover("fn f() -> String {\n  \"x\"\n}\nprint f\nlet = 1\n", 3, 6).expect("hover");
    assert!(!hover.contents.contains("row:"), "{}", hover.contents);
}
