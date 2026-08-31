use super::{extract_symbol, parse_starship_prompt, unescape_starship_text};

#[test]
fn an_empty_config_keeps_every_default() {
    let config = parse_starship_prompt("").expect("parse");

    assert!(config.format.is_none());
    assert!(!config.add_newline);
    assert_eq!(config.directory.truncation_length, 2);
    assert_eq!(config.directory.truncation_symbol, ".../");
    assert_eq!(config.directory.home_symbol, "~");
    assert_eq!(config.character.success_symbol, "$ ");
    assert_eq!(config.character.error_symbol, "! ");
    assert!(config.git_branch.format.is_none());
    assert_eq!(config.git_branch.symbol, "");
    assert_eq!(config.git_branch.style, "cyan");
}

#[test]
fn a_zero_truncation_length_is_clamped_to_one() {
    let config = parse_starship_prompt("[directory]\ntruncation_length = 0\n").expect("parse");

    assert_eq!(config.directory.truncation_length, 1);
}

#[test]
fn partial_sections_only_override_what_they_set() {
    let config = parse_starship_prompt("[character]\nsuccess_symbol = \"> \"\n").expect("parse");

    assert_eq!(config.character.success_symbol, "> ");
    assert_eq!(config.character.error_symbol, "! ");
}

#[test]
fn a_git_branch_style_overrides_the_default_colour() {
    let config = parse_starship_prompt("[git_branch]\nstyle = \"bold purple\"\n").expect("parse");

    assert_eq!(config.git_branch.style, "bold purple");
}

#[test]
fn malformed_toml_is_reported_as_an_error() {
    assert!(parse_starship_prompt("[directory").is_err());
    assert!(parse_starship_prompt("[directory]\ntruncation_length = \"three\"").is_err());
}

#[test]
fn a_bracketed_symbol_drops_its_style_annotation() {
    assert_eq!(extract_symbol("[X](bold green)"), "X");
    assert_eq!(extract_symbol("  [X](bold green)  "), "X");
    assert_eq!(extract_symbol("[](red)"), "");
}

#[test]
fn an_unbracketed_symbol_is_used_verbatim() {
    assert_eq!(extract_symbol("> "), "> ");
    assert_eq!(extract_symbol(""), "");
}

#[test]
fn escapes_are_unwrapped_one_level() {
    assert_eq!(unescape_starship_text(r"a\[b\]"), "a[b]");
    assert_eq!(unescape_starship_text(r"plain"), "plain");
    assert_eq!(unescape_starship_text(r"trailing\"), r"trailing\");
}

#[test]
fn parses_directory_and_character_settings() {
    let config = parse_starship_prompt(
        r#"
add_newline = true
format = "$directory$line_break$character"

[directory]
truncation_length = 3
truncation_symbol = "»/"
home_symbol = "~home"

[character]
success_symbol = "[❯](bold green)"
error_symbol = "[✗](bold red)"

[git_branch]
format = " [$symbol$branch]($style)"
symbol = "|- "
"#,
    )
    .expect("parse");

    assert!(config.add_newline);
    assert_eq!(
        config.format.as_deref(),
        Some("$directory$line_break$character")
    );
    assert_eq!(config.directory.truncation_length, 3);
    assert_eq!(config.directory.truncation_symbol, "»/");
    assert_eq!(config.directory.home_symbol, "~home");
    assert_eq!(config.character.success_symbol, "❯");
    assert_eq!(config.character.error_symbol, "✗");
    assert_eq!(
        config.git_branch.format.as_deref(),
        Some(" [$symbol$branch]($style)")
    );
    assert_eq!(config.git_branch.symbol, "|- ");
}
