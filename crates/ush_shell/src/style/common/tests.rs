use std::path::{Path, PathBuf};

use super::{
    BOLD, DIM, GREEN_BOLD, RESET, badge, dim, human_bytes, normalize_path, paint, pluralize,
};

#[test]
fn byte_counts_below_a_kilobyte_stay_in_bytes() {
    assert_eq!(human_bytes(0), "0 B");
    assert_eq!(human_bytes(1), "1 B");
    assert_eq!(human_bytes(1023), "1023 B");
}

#[test]
fn byte_counts_scale_through_the_unit_table() {
    assert_eq!(human_bytes(1024), "1.0 KB");
    assert_eq!(human_bytes(1536), "1.5 KB");
    assert_eq!(human_bytes(1024 * 1024), "1.0 MB");
    assert_eq!(human_bytes(1024 * 1024 * 1024), "1.0 GB");
    assert_eq!(human_bytes(1024u64.pow(4)), "1.0 TB");
}

#[test]
fn values_of_ten_or_more_drop_the_fraction() {
    assert_eq!(human_bytes(10 * 1024), "10 KB");
    assert_eq!(human_bytes(999 * 1024), "999 KB");
    assert_eq!(human_bytes(1024 * 1024 * 512), "512 MB");
}

#[test]
fn the_largest_unit_absorbs_everything_above_it() {
    assert!(human_bytes(u64::MAX).ends_with(" TB"));
    assert_eq!(human_bytes(1024u64.pow(5)), "1024 TB");
}

#[test]
fn pluralization_switches_on_exactly_one() {
    assert_eq!(pluralize(0, "match", "matches"), "0 matches");
    assert_eq!(pluralize(1, "match", "matches"), "1 match");
    assert_eq!(pluralize(2, "match", "matches"), "2 matches");
}

#[test]
fn painting_wraps_a_value_in_a_style_and_a_reset() {
    assert_eq!(paint(BOLD, "ush"), format!("{BOLD}ush{RESET}"));
    assert_eq!(paint(GREEN_BOLD, 12), format!("{GREEN_BOLD}12{RESET}"));
}

#[test]
fn dimming_uses_the_dim_escape() {
    assert_eq!(dim("note"), format!("{DIM}note{RESET}"));
}

#[test]
fn badges_bracket_the_value_inside_the_style() {
    assert_eq!(badge("ok", GREEN_BOLD), format!("{GREEN_BOLD}[ok]{RESET}"));
    assert_eq!(badge(3, BOLD), format!("{BOLD}[3]{RESET}"));
}

#[test]
fn every_style_escape_terminates_with_a_reset() {
    let painted = paint(BOLD, "x");
    assert!(painted.starts_with('\u{1b}'));
    assert!(painted.ends_with(RESET));
}

#[test]
fn relative_paths_are_resolved_against_the_working_directory() {
    let cwd = Path::new("/work/repo");
    assert_eq!(normalize_path(cwd, "src"), PathBuf::from("/work/repo/src"));
    assert_eq!(
        normalize_path(cwd, "./src/main.rs"),
        PathBuf::from("/work/repo/./src/main.rs")
    );
}

#[test]
fn absolute_paths_are_returned_untouched() {
    let cwd = Path::new("/work/repo");
    assert_eq!(
        normalize_path(cwd, "/etc/hosts"),
        PathBuf::from("/etc/hosts")
    );
}

#[test]
fn an_empty_path_resolves_to_the_working_directory() {
    let cwd = Path::new("/work/repo");
    assert_eq!(normalize_path(cwd, ""), PathBuf::from("/work/repo"));
}
