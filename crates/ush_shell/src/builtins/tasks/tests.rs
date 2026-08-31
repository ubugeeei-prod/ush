use super::{contains_ignore_ascii_case, filter_tasks, render_tasks_plain};
use crate::repl::contextual::{TaskEntry, TaskSource};

fn entries() -> Vec<TaskEntry> {
    vec![
        TaskEntry::new(TaskSource::Make, "build"),
        TaskEntry::new(TaskSource::Npm, "build"),
        TaskEntry::new(TaskSource::Npm, "test"),
        TaskEntry::new(TaskSource::Just, "fmt"),
    ]
}

fn filters(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn names(entries: &[TaskEntry]) -> Vec<String> {
    entries
        .iter()
        .map(|entry| entry.command().to_string())
        .collect()
}

#[test]
fn no_filter_keeps_every_task() {
    assert_eq!(filter_tasks(entries(), &[]).len(), 4);
}

#[test]
fn a_filter_matches_the_task_name() {
    assert_eq!(
        names(&filter_tasks(entries(), &filters(&["test"]))),
        vec!["npm run test"]
    );
}

#[test]
fn a_filter_matches_the_task_source() {
    assert_eq!(
        names(&filter_tasks(entries(), &filters(&["npm"]))),
        vec!["npm run build", "npm run test"]
    );
}

#[test]
fn several_filters_must_all_match() {
    assert_eq!(
        names(&filter_tasks(entries(), &filters(&["npm", "build"]))),
        vec!["npm run build"]
    );
    assert!(filter_tasks(entries(), &filters(&["npm", "fmt"])).is_empty());
}

#[test]
fn filters_ignore_case() {
    assert_eq!(
        names(&filter_tasks(entries(), &filters(&["NPM", "Build"]))),
        vec!["npm run build"]
    );
}

#[test]
fn a_filter_can_match_the_rendered_command() {
    assert_eq!(
        names(&filter_tasks(entries(), &filters(&["run test"]))),
        vec!["npm run test"]
    );
}

#[test]
fn a_filter_that_matches_nothing_yields_nothing() {
    assert!(filter_tasks(entries(), &filters(&["deploy"])).is_empty());
}

#[test]
fn an_empty_needle_matches_everything() {
    assert!(contains_ignore_ascii_case("anything", ""));
    assert_eq!(filter_tasks(entries(), &filters(&[""])).len(), 4);
}

#[test]
fn a_needle_longer_than_the_haystack_never_matches() {
    assert!(!contains_ignore_ascii_case("np", "npm"));
}

#[test]
fn case_insensitive_search_finds_interior_matches() {
    assert!(contains_ignore_ascii_case("npm run BUILD", "build"));
    assert!(contains_ignore_ascii_case("NPM", "npm"));
    assert!(!contains_ignore_ascii_case("npm", "yarn"));
}

#[test]
fn the_plain_listing_is_one_runnable_command_per_line() {
    assert_eq!(
        render_tasks_plain(&entries()),
        "make build\nnpm run build\nnpm run test\njust fmt\n"
    );
}

#[test]
fn the_plain_listing_of_no_tasks_is_empty() {
    assert_eq!(render_tasks_plain(&[]), "");
}
