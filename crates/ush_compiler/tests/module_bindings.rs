mod support;

use support::modules::Project;

#[test]
fn module_functions_preserve_mutable_bindings() {
    let project = Project::new(&[
        (
            "counter.ush",
            "pub fn count() -> Int {\n  let mut value = 0\n  while value < 3 {\n    value = value + 1\n  }\n  value\n}\n",
        ),
        (
            "main.ush",
            "mod \"counter.ush\"\nuse counter::*\n\nlet value = 10\nprint count\nprint value\n",
        ),
    ]);
    assert_eq!(project.run_everywhere("main.ush"), "3\n10\n");
}

#[test]
fn immutable_module_errors_name_the_original_file_and_line() {
    let project = Project::new(&[
        (
            "counter.ush",
            "pub fn count() -> Int {\n  let value = 0\n  value = 1\n  value\n}\n",
        ),
        ("main.ush", "mod \"counter.ush\"\nprint counter::count\n"),
    ]);
    let error = project.error("main.ush");
    assert!(error.contains("counter.ush: line 3"), "{error}");
    assert!(error.contains("immutable binding `value`"), "{error}");
}
