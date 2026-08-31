use crate::parser::fallback::needs_posix_fallback;

#[test]
fn plain_commands_stay_on_the_native_path() {
    for line in [
        "ls",
        "ls -la src",
        "echo hello world",
        "git commit -m 'wip'",
        "ls | len",
        "cat a.txt | lines | len",
        "printf '%s' ok",
        "cargo build --release",
    ] {
        assert!(!needs_posix_fallback(line), "{line} should stay native");
    }
}

#[test]
fn leading_shell_only_prefixes_fall_back() {
    assert!(needs_posix_fallback("! true"));
    assert!(needs_posix_fallback("(cd /tmp && ls)"));
    assert!(needs_posix_fallback("{ echo hi; }"));
    assert!(needs_posix_fallback("   ! true"));
}

#[test]
fn control_operators_fall_back() {
    for line in [
        "true; false",
        "echo `date`",
        "sleep 1 & wait",
        "cat < input.txt",
        "echo hi > out.txt",
        "echo hi >> out.txt",
        "echo $(date)",
        "true && false",
        "true || false",
    ] {
        assert!(needs_posix_fallback(line), "{line} should fall back");
    }
}

#[test]
fn lambda_arrows_are_not_redirections() {
    assert!(!needs_posix_fallback("filter(it -> contains(it, \"rs\"))"));
    assert!(!needs_posix_fallback("ls | map(it -> upper(it))"));
    assert!(!needs_posix_fallback("ls | ffilter(a b -> gt(a, b))"));
}

#[test]
fn quoted_operators_do_not_trigger_a_fallback() {
    assert!(!needs_posix_fallback("echo ';'"));
    assert!(!needs_posix_fallback("echo \"a;b\""));
    assert!(!needs_posix_fallback("echo '`'"));
    assert!(!needs_posix_fallback("echo \"<\""));
    assert!(!needs_posix_fallback("grep 'if' file"));
    assert!(!needs_posix_fallback("echo \"then\""));
}

#[test]
fn shell_keywords_as_bare_words_fall_back() {
    for keyword in [
        "if", "elif", "else", "for", "while", "until", "case", "do", "done", "then", "fi", "esac",
    ] {
        let line = format!("{keyword} something");
        assert!(needs_posix_fallback(&line), "{line} should fall back");
    }
}

#[test]
fn keyword_substrings_inside_identifiers_stay_native() {
    for line in [
        "iffy --flag",
        "cat casefile",
        "elsewhere",
        "format",
        "ls fifo",
        "docker ps",
        "whiles",
        "undone",
    ] {
        assert!(!needs_posix_fallback(line), "{line} should stay native");
    }
}

#[test]
fn keywords_are_detected_at_the_end_of_a_line() {
    assert!(needs_posix_fallback("echo hi then"));
    assert!(needs_posix_fallback("something fi"));
}

#[test]
fn keywords_delimited_by_punctuation_are_detected() {
    assert!(needs_posix_fallback("x=1 if.y"));
    assert!(needs_posix_fallback("run if/then"));
}

#[test]
fn empty_and_whitespace_lines_stay_native() {
    assert!(!needs_posix_fallback(""));
    assert!(!needs_posix_fallback("   "));
    assert!(!needs_posix_fallback("\t"));
}

#[test]
fn multibyte_lines_are_scanned_without_panicking() {
    assert!(!needs_posix_fallback("echo こんにちは"));
    assert!(needs_posix_fallback("echo こんにちは; echo またね"));
    assert!(!needs_posix_fallback("echo '日本語 ; つき'"));
}

#[test]
fn an_escaped_keyword_is_still_a_keyword() {
    // `i\f` is the word `if` once the shell strips the escape, so it
    // has to take the POSIX path like the unescaped spelling.
    assert!(needs_posix_fallback("i\\f x"));
    assert!(needs_posix_fallback("th\\en"));
}

#[test]
fn a_closed_escape_pair_ends_the_current_word() {
    assert!(!needs_posix_fallback("a\\\\b"));
    assert!(!needs_posix_fallback("i\\\\f"));
}

#[test]
fn words_longer_than_any_keyword_never_match() {
    for line in [
        "elifelif",
        "whilewhile",
        "untilx",
        "cases",
        "esacs",
        "doneness",
    ] {
        assert!(!needs_posix_fallback(line), "{line} should stay native");
    }
}

#[test]
fn a_keyword_directly_after_a_quoted_word_is_detected() {
    assert!(needs_posix_fallback("'quoted'then"));
    assert!(needs_posix_fallback("\"quoted\"fi"));
}

#[test]
fn a_word_ending_at_a_quote_is_checked_before_the_quote_opens() {
    assert!(needs_posix_fallback("then'quoted'"));
    assert!(!needs_posix_fallback("cmd'quoted'"));
}

#[test]
fn keywords_are_matched_case_sensitively() {
    assert!(!needs_posix_fallback("IF x"));
    assert!(!needs_posix_fallback("Then"));
}
