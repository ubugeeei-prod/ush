use serde_json::json;

use super::{
    package_json_has_name, package_json_scripts, package_json_uses_vp, parse_just_recipes,
    parse_make_targets, parse_mise_toml_tasks,
};
use crate::repl::contextual::types::Names;

fn names(values: Names) -> Vec<String> {
    values.into_iter().map(|value| value.to_string()).collect()
}

#[test]
fn make_rules_become_completion_targets() {
    let targets = names(parse_make_targets(
        "build:\n\tcargo build\ntest: build\n\tcargo test\n",
    ));
    assert_eq!(targets, vec!["build", "test"]);
}

#[test]
fn several_targets_on_one_rule_are_all_offered() {
    let targets = names(parse_make_targets("fmt lint:\n\techo hi\n"));
    assert_eq!(targets, vec!["fmt", "lint"]);
}

#[test]
fn make_variable_assignments_are_not_targets() {
    let targets = names(parse_make_targets(
        "CC := gcc\nFLAGS ?= -O2\nEXTRA += -g\nSHELL != echo sh\nbuild:\n\t$(CC) main.c\n",
    ));
    assert_eq!(targets, vec!["build"]);
}

#[test]
fn special_and_pattern_targets_are_skipped() {
    let targets = names(parse_make_targets(
        ".PHONY: build\n%.o: %.c\n\tcc -c $<\nbuild:\n\ttrue\n",
    ));
    assert_eq!(targets, vec!["build"]);
}

#[test]
fn comments_recipe_lines_and_defines_are_skipped() {
    let targets = names(parse_make_targets(
        "# comment: not a target\ndefine block\nbody: here\nendef\n\tindented: not a target\nreal:\n\ttrue\n",
    ));
    assert_eq!(targets, vec!["real"]);
}

#[test]
fn several_define_blocks_are_each_closed_by_their_own_endef() {
    let targets = names(parse_make_targets(
        "define one\na: b\nendef\nfirst:\n\ttrue\ndefine two\nc: d\nendef\nsecond:\n\ttrue\n",
    ));
    assert_eq!(targets, vec!["first", "second"]);
}

#[test]
fn line_continuations_are_joined_before_parsing() {
    let targets = names(parse_make_targets("one \\\ntwo:\n\ttrue\n"));
    assert_eq!(targets, vec!["one", "two"]);
}

#[test]
fn duplicate_make_targets_are_offered_once() {
    let targets = names(parse_make_targets("build:\n\ttrue\nbuild:\n\ttrue\n"));
    assert_eq!(targets, vec!["build"]);
}

#[test]
fn an_empty_makefile_offers_nothing() {
    assert!(names(parse_make_targets("")).is_empty());
}

#[test]
fn just_recipes_become_completion_targets() {
    let recipes = names(parse_just_recipes(
        "build:\n  cargo build\ntest: build\n  cargo test\n",
    ));
    assert_eq!(recipes, vec!["build", "test"]);
}

#[test]
fn just_assignments_and_directives_are_skipped() {
    let recipes = names(parse_just_recipes(
        "set shell := [\"sh\"]\nexport A := \"1\"\nimport 'other.just'\nmod sub\nalias b := build\n[private]\nbuild:\n  true\n",
    ));
    assert_eq!(recipes, vec!["build"]);
}

#[test]
fn indented_just_lines_are_recipe_bodies() {
    let recipes = names(parse_just_recipes("build:\n  other: not-a-recipe\n"));
    assert_eq!(recipes, vec!["build"]);
}

#[test]
fn just_recipes_with_parameters_keep_only_the_name() {
    let recipes = names(parse_just_recipes("greet name:\n  echo {{name}}\n"));
    assert_eq!(recipes, vec!["greet"]);
}

#[test]
fn a_just_recipe_name_stops_at_its_first_parameter() {
    // `bad name!:` declares recipe `bad` taking a parameter, so the
    // name is what is offered, not the whole head.
    let recipes = names(parse_just_recipes("bad name!:\n  true\nok-name:\n  true\n"));
    assert_eq!(recipes, vec!["bad", "ok-name"]);
}

#[test]
fn just_names_with_unusual_characters_are_rejected() {
    let recipes = names(parse_just_recipes("bad!name:\n  true\nok-name:\n  true\n"));
    assert_eq!(recipes, vec!["ok-name"]);
}

#[test]
fn mise_tasks_are_read_from_the_tasks_table() {
    let tasks = names(parse_mise_toml_tasks(
        "[tasks]\nbuild = \"cargo build\"\n\n[tasks.test]\nrun = \"cargo test\"\n",
    ));
    assert_eq!(tasks, vec!["build", "test"]);
}

#[test]
fn a_mise_file_without_tasks_offers_nothing() {
    assert!(names(parse_mise_toml_tasks("[tools]\nnode = \"22\"\n")).is_empty());
    assert!(names(parse_mise_toml_tasks("")).is_empty());
}

#[test]
fn malformed_mise_toml_is_ignored_rather_than_failing() {
    assert!(names(parse_mise_toml_tasks("[tasks")).is_empty());
}

#[test]
fn package_scripts_are_listed_in_sorted_order() {
    let scripts = names(package_json_scripts(&json!({
        "scripts": { "test": "vitest", "build": "vite build" }
    })));
    assert_eq!(scripts, vec!["build", "test"]);
}

#[test]
fn a_package_without_scripts_offers_nothing() {
    assert!(names(package_json_scripts(&json!({}))).is_empty());
    assert!(names(package_json_scripts(&json!({ "scripts": [] }))).is_empty());
}

#[test]
fn dependencies_are_searched_across_every_dependency_field() {
    let json = json!({
        "dependencies": { "vue": "^3" },
        "devDependencies": { "vitest": "^2" },
        "optionalDependencies": { "fsevents": "^2" },
        "peerDependencies": { "react": "^19" }
    });

    for name in ["vue", "vitest", "fsevents", "react"] {
        assert!(package_json_has_name(&json, name), "{name}");
    }
    assert!(!package_json_has_name(&json, "svelte"));
    assert!(!package_json_has_name(&json!({}), "vue"));
}

#[test]
fn a_vite_or_vp_script_marks_the_package_as_a_vp_project() {
    assert!(package_json_uses_vp(
        &json!({ "scripts": { "dev": "vite" } })
    ));
    assert!(package_json_uses_vp(
        &json!({ "scripts": { "dev": "vp dev" } })
    ));
    assert!(!package_json_uses_vp(
        &json!({ "scripts": { "dev": "next dev" } })
    ));
    assert!(!package_json_uses_vp(&json!({})));
}
