mod paths;

use std::path::Path;

use ush_config::StarshipPromptConfig;

use super::{default_prompt, path::compact_path, render_prompt};

#[test]
fn abbreviates_home_and_deep_paths() {
    let home = Some("/Users/user");

    assert_eq!(compact_path(Path::new("/Users/user"), home), "~");
    assert_eq!(compact_path(Path::new("/Users/user/src"), home), "~/src");
    assert_eq!(
        compact_path(Path::new("/Users/user/Code/github.com/ubugeeei/ush"), home),
        "~/.../ubugeeei/ush"
    );
    assert_eq!(
        compact_path(Path::new("/usr/local/bin"), home),
        "/.../local/bin"
    );
}

#[test]
fn formats_default_prompt_with_short_path() {
    let prompt = default_prompt(
        Path::new("/Users/user/Code/github.com/ubugeeei/ush"),
        Some("/Users/user"),
        0,
    );

    assert_eq!(prompt, "~/.../ubugeeei/ush $ ");
}

#[test]
fn renders_starship_style_prompt_when_config_is_present() {
    let mut starship = StarshipPromptConfig {
        add_newline: true,
        ..StarshipPromptConfig::default()
    };
    starship.directory.truncation_length = 2;
    starship.character.success_symbol = "❯".into();

    let prompt = render_prompt(
        Path::new("/Users/user/Code/github.com/ubugeeei/ush"),
        Some("/Users/user"),
        0,
        Some(&starship),
    );

    assert_eq!(prompt, "~/.../ubugeeei/ush\n❯ ");
}

#[test]
fn honors_starship_home_symbol_and_error_character() {
    let mut starship = StarshipPromptConfig::default();
    starship.directory.home_symbol = "~home".into();
    starship.character.error_symbol = "✗".into();

    let prompt = render_prompt(
        Path::new("/Users/user/project"),
        Some("/Users/user"),
        1,
        Some(&starship),
    );

    assert_eq!(prompt, "~home/project ✗ ");
}
