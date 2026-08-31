use std::path::Path;

use super::super::{default_prompt, path::compact_path_with};

fn compact(cwd: &str, home: Option<&str>, length: usize) -> String {
    compact_path_with(Path::new(cwd), home, length, ".../", "~")
}

#[test]
fn the_filesystem_root_renders_as_itself() {
    assert_eq!(compact("/", None, 2), "/");
    assert_eq!(compact("/", Some("/Users/user"), 2), "/");
}

#[test]
fn a_path_shorter_than_the_limit_is_shown_in_full() {
    assert_eq!(compact("/usr", None, 2), "/usr");
    assert_eq!(compact("/usr/local", None, 2), "/usr/local");
}

#[test]
fn a_path_longer_than_the_limit_keeps_its_tail() {
    assert_eq!(compact("/usr/local/bin", None, 2), "/.../local/bin");
    assert_eq!(compact("/a/b/c/d/e", None, 3), "/.../c/d/e");
}

#[test]
fn a_truncation_length_of_one_keeps_only_the_last_segment() {
    assert_eq!(compact("/usr/local/bin", None, 1), "/.../bin");
}

#[test]
fn home_relative_paths_are_truncated_under_the_home_symbol() {
    let home = Some("/Users/user");
    assert_eq!(compact("/Users/user", home, 2), "~");
    assert_eq!(compact("/Users/user/src", home, 2), "~/src");
    assert_eq!(compact("/Users/user/a/b/c", home, 2), "~/.../b/c");
}

#[test]
fn a_path_outside_home_is_rendered_from_the_root() {
    assert_eq!(compact("/etc/nginx", Some("/Users/user"), 2), "/etc/nginx");
}

#[test]
fn a_home_prefix_only_matches_on_component_boundaries() {
    assert_eq!(
        compact("/Users/username/src", Some("/Users/user"), 2),
        "/.../username/src"
    );
}

#[test]
fn custom_symbols_are_used_verbatim() {
    assert_eq!(
        compact_path_with(
            Path::new("/Users/user/a/b/c"),
            Some("/Users/user"),
            2,
            "»/",
            "H"
        ),
        "H/»/b/c"
    );
    assert_eq!(
        compact_path_with(Path::new("/a/b/c"), None, 1, "", ""),
        "/c"
    );
}

#[test]
fn relative_paths_render_without_a_leading_slash_component() {
    assert_eq!(
        compact_path_with(Path::new("a/b"), None, 2, ".../", "~"),
        "/a/b"
    );
}

#[test]
fn multibyte_directory_names_survive_truncation() {
    assert_eq!(compact("/一/二/三", None, 2), "/.../二/三");
}

#[test]
fn the_prompt_mark_reflects_the_last_exit_status() {
    let cwd = Path::new("/Users/user");
    assert_eq!(default_prompt(cwd, Some("/Users/user"), 0), "~ $ ");
    assert_eq!(default_prompt(cwd, Some("/Users/user"), 1), "~ ! ");
    assert_eq!(default_prompt(cwd, Some("/Users/user"), 130), "~ ! ");
}

#[test]
fn the_prompt_falls_back_to_absolute_paths_without_a_home() {
    assert_eq!(
        default_prompt(Path::new("/usr/local/bin"), None, 0),
        "/.../local/bin $ "
    );
}
