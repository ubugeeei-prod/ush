//! Pipeline file stages and String stdin under every installed shell:
//! paths with spaces and newlines, open failures, and byte-exact input.

mod support;

use std::fs;

use support::each_program;

const PRELUDE: &str = "use $bin::*\nuse std::io::{append_to, from_file, to_file}\n";

fn each_run(body: &str, check: impl FnMut(&str, &std::path::Path, (i32, String, String))) {
    each_program(&format!("{PRELUDE}{body}"), check);
}

#[test]
fn file_sources_and_sinks_take_any_path_as_one_word() {
    each_run(
        r#"std::fs::mkdir "with space"
let computed = "with space/" + "line\nbreak"
"one\n" | to_file "with space/odd\nname"
"two\n" | append_to "with space/odd\nname"
from_file "with space/odd\nname" | to_file computed
from_file computed | tr "a-z" "A-Z"
"#,
        |shell, dir, (status, stdout, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            assert_eq!(stdout, "ONE\nTWO\n", "{shell}");
            let copied = fs::read(dir.join("with space/line\nbreak")).unwrap();
            assert_eq!(copied, b"one\ntwo\n", "{shell}");
        },
    );
}

#[test]
fn a_missing_source_or_unwritable_sink_names_its_stage() {
    each_run(
        "from_file \"missing\" | cat\n",
        |shell, _, (status, _, stderr)| {
            assert_ne!(status, 0, "{shell}");
            assert!(
                stderr.contains("pipeline stage 1 failed: from_file \"missing\" | cat"),
                "{shell}: {stderr}"
            );
        },
    );
    each_run(
        "\"x\" | to_file \"missing/dir/file\"\n",
        |shell, _, (status, _, stderr)| {
            assert_ne!(status, 0, "{shell}");
            assert!(
                stderr.contains("stage 1 failed: \"x\" | to_file \"missing/dir/file\""),
                "{shell}: {stderr}"
            );
        },
    );
}

#[test]
fn string_stdin_is_byte_exact_and_adds_no_newline() {
    each_run(
        r#""a\nb" | to_file "unterminated"
"a\nb\n" | to_file "terminated"
let body = """
  first
    second
"""
body | to_file "heredoc"
"""
  inline
""" | to_file "inline"
"#,
        |shell, dir, (status, _, stderr)| {
            assert_eq!(status, 0, "{shell}: {stderr}");
            let read = |name: &str| fs::read(dir.join(name)).unwrap();
            assert_eq!(read("unterminated"), b"a\nb", "{shell}");
            assert_eq!(read("terminated"), b"a\nb\n", "{shell}");
            assert_eq!(read("heredoc"), b"first\n  second", "{shell}");
            assert_eq!(read("inline"), b"inline", "{shell}");
        },
    );
}
