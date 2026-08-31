use std::{collections::BTreeMap, fs};

use tempfile::tempdir;

use super::{CommandNameCache, CommandNames, HELPER_COMMANDS};
use crate::commands::BUILTIN_COMMANDS;

fn aliases(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
        .iter()
        .map(|(name, value)| ((*name).to_string(), (*value).to_string()))
        .collect()
}

#[test]
fn every_builtin_and_helper_is_offered() {
    let mut cache = CommandNameCache::default();
    let names = cache.names(None, &BTreeMap::new());

    for builtin in BUILTIN_COMMANDS {
        assert!(names.contains(builtin), "missing builtin {builtin}");
    }
    for helper in HELPER_COMMANDS {
        assert!(names.contains(helper), "missing helper {helper}");
    }
}

#[test]
fn executables_on_the_path_are_included() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("my-tool"), "").expect("write");
    let path = dir.path().display().to_string();

    let mut cache = CommandNameCache::default();
    assert!(
        cache
            .names(Some(&path), &BTreeMap::new())
            .contains("my-tool")
    );
}

#[test]
fn aliases_are_included() {
    let mut cache = CommandNameCache::default();
    let names = cache.names(None, &aliases(&[("ll", "ls -la")]));

    assert!(names.contains("ll"));
}

#[test]
fn an_unchanged_environment_reuses_the_cached_set() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("my-tool"), "").expect("write");
    let path = dir.path().display().to_string();
    let table = aliases(&[("ll", "ls -la")]);

    let mut cache = CommandNameCache::default();
    let first = cache.names(Some(&path), &table);
    let second = cache.names(Some(&path), &table);

    assert_eq!(first.len(), second.len());
    assert!(second.contains("my-tool"));
}

#[test]
fn a_new_alias_invalidates_the_cache() {
    let mut cache = CommandNameCache::default();
    assert!(!cache.names(None, &BTreeMap::new()).contains("ll"));
    assert!(
        cache
            .names(None, &aliases(&[("ll", "ls -la")]))
            .contains("ll")
    );
}

#[test]
fn a_changed_path_invalidates_the_cache() {
    let first_dir = tempdir().expect("tempdir");
    let second_dir = tempdir().expect("tempdir");
    fs::write(first_dir.path().join("first-tool"), "").expect("write");
    fs::write(second_dir.path().join("second-tool"), "").expect("write");

    let mut cache = CommandNameCache::default();
    let names = cache.names(
        Some(&first_dir.path().display().to_string()),
        &BTreeMap::new(),
    );
    assert!(names.contains("first-tool"));

    let names = cache.names(
        Some(&second_dir.path().display().to_string()),
        &BTreeMap::new(),
    );
    assert!(names.contains("second-tool"));
    assert!(!names.contains("first-tool"));
}

#[test]
fn a_missing_path_directory_is_skipped_rather_than_failing() {
    let mut cache = CommandNameCache::default();
    let names = cache.names(Some("/definitely/not/a/real/directory"), &BTreeMap::new());

    assert!(names.contains("echo"));
}

#[test]
fn names_can_be_collected_from_an_iterator() {
    let names = CommandNames::from(vec!["b".to_string(), "a".to_string(), "a".to_string()]);
    assert_eq!(names.len(), 2);
    assert_eq!(
        names.iter().cloned().collect::<Vec<_>>(),
        vec!["a".to_string(), "b".to_string()]
    );
}
