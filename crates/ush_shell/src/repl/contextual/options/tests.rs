use std::path::{Path, PathBuf};

use compact_str::CompactString;

use super::{
    explicit_makefile_path, match_option, option_spec, pending_value_kind, positional_args,
};

const SPECS: &[super::OptionSpec] = &[
    option_spec(&["--jobs", "-j"], 1, false, true),
    option_spec(&["--file", "-f"], 1, true, true),
    option_spec(&["--verbose", "-v"], 0, false, false),
];

fn args(values: &[&str]) -> Vec<CompactString> {
    values
        .iter()
        .map(|value| CompactString::from(*value))
        .collect()
}

fn positionals(values: &[&str]) -> Vec<String> {
    positional_args(&args(values), SPECS)
        .into_iter()
        .map(|value| value.to_string())
        .collect()
}

#[test]
fn a_flag_without_a_value_leaves_nothing_pending() {
    assert_eq!(pending_value_kind(&args(&["--verbose"]), SPECS), None);
    assert_eq!(pending_value_kind(&args(&[]), SPECS), None);
}

#[test]
fn a_trailing_value_flag_leaves_its_value_pending() {
    assert_eq!(pending_value_kind(&args(&["--jobs"]), SPECS), Some(false));
    assert_eq!(pending_value_kind(&args(&["--file"]), SPECS), Some(true));
    assert_eq!(pending_value_kind(&args(&["-f"]), SPECS), Some(true));
}

#[test]
fn a_satisfied_value_flag_is_no_longer_pending() {
    assert_eq!(pending_value_kind(&args(&["--jobs", "4"]), SPECS), None);
    assert_eq!(
        pending_value_kind(&args(&["--file", "Makefile"]), SPECS),
        None
    );
}

#[test]
fn an_inline_value_does_not_leave_anything_pending() {
    assert_eq!(pending_value_kind(&args(&["--jobs=4"]), SPECS), None);
    assert_eq!(pending_value_kind(&args(&["-j4"]), SPECS), None);
}

#[test]
fn positional_arguments_skip_flags_and_their_values() {
    assert_eq!(positionals(&["build"]), vec!["build"]);
    assert_eq!(positionals(&["--jobs", "4", "build"]), vec!["build"]);
    assert_eq!(
        positionals(&["--verbose", "build", "test"]),
        vec!["build", "test"]
    );
}

#[test]
fn unknown_flags_are_not_treated_as_positionals() {
    assert!(positionals(&["--unknown"]).is_empty());
    assert_eq!(positionals(&["--unknown", "build"]), vec!["build"]);
}

#[test]
fn inline_values_do_not_swallow_the_next_argument() {
    assert_eq!(positionals(&["--jobs=4", "build"]), vec!["build"]);
    assert_eq!(positionals(&["-j4", "build"]), vec!["build"]);
}

#[test]
fn options_match_by_exact_name_and_by_inline_value() {
    assert!(match_option("--jobs", SPECS).is_some_and(|(_, inline)| !inline));
    assert!(match_option("--jobs=4", SPECS).is_some_and(|(_, inline)| inline));
    assert!(match_option("-j4", SPECS).is_some_and(|(_, inline)| inline));
    assert!(match_option("--nope", SPECS).is_none());
    assert!(match_option("build", SPECS).is_none());
}

#[test]
fn short_inline_values_are_only_matched_when_the_spec_allows_them() {
    const NO_INLINE: &[super::OptionSpec] = &[option_spec(&["-x"], 1, false, false)];
    assert!(match_option("-x", NO_INLINE).is_some());
    assert!(match_option("-xvalue", NO_INLINE).is_none());
}

#[test]
fn an_explicit_makefile_is_resolved_against_the_working_directory() {
    let cwd = Path::new("/work");
    assert_eq!(
        explicit_makefile_path(cwd, &args(&["-f", "build.mk"])),
        Some(PathBuf::from("/work/build.mk"))
    );
    assert_eq!(
        explicit_makefile_path(cwd, &args(&["--file", "build.mk"])),
        Some(PathBuf::from("/work/build.mk"))
    );
    assert_eq!(
        explicit_makefile_path(cwd, &args(&["--makefile=build.mk"])),
        Some(PathBuf::from("/work/build.mk"))
    );
    assert_eq!(
        explicit_makefile_path(cwd, &args(&["-fbuild.mk"])),
        Some(PathBuf::from("/work/build.mk"))
    );
}

#[test]
fn an_absolute_makefile_path_is_used_as_is() {
    assert_eq!(
        explicit_makefile_path(Path::new("/work"), &args(&["-f", "/etc/build.mk"])),
        Some(PathBuf::from("/etc/build.mk"))
    );
}

#[test]
fn the_last_makefile_flag_wins() {
    assert_eq!(
        explicit_makefile_path(Path::new("/work"), &args(&["-f", "a.mk", "-f", "b.mk"])),
        Some(PathBuf::from("/work/b.mk"))
    );
}

#[test]
fn no_makefile_flag_means_no_explicit_path() {
    assert_eq!(
        explicit_makefile_path(Path::new("/work"), &args(&["build"])),
        None
    );
    assert_eq!(
        explicit_makefile_path(Path::new("/work"), &args(&["-f"])),
        None
    );
}
