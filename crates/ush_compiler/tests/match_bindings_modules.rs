mod support;

use support::modules::Project;

#[test]
fn modules_qualify_types_and_keep_functions_used_only_in_match_arms() {
    let project = Project::new(&[
        (
            "lib.ush",
            r#"use std::string::to_upper_ascii

pub enum Choice {
  Text(String),
  Empty,
}
fn upper(text: String) -> String {
  to_upper_ascii $text
}
pub fn choose(choice: Choice) -> String {
  let result = match choice {
    Choice::Text(text) => upper $text
    Choice::Empty => "EMPTY"
  }
  result
}
"#,
        ),
        (
            "main.ush",
            r#"mod "lib.ush"
use lib::{Choice, choose}

let input = match true {
  true => Choice::Text("hello")
  false => Choice::Empty
}
echo (choose $input)
"#,
        ),
    ]);
    assert_eq!(project.run_everywhere("main.ush"), "HELLO\n");
    assert!(
        project
            .compile("main.ush")
            .unwrap()
            .contains("ush_fn_lib__upper")
    );
}
