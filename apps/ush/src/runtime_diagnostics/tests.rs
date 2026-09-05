use std::fmt::Write as _;
use std::{env, fs, path::PathBuf};

use ush_compiler::UshCompiler;

use super::{format_generated_lines, instrument_compiled_script, should_inline_track};

#[test]
fn instrumented_script_reports_mapped_source_lines() {
    let compiled = UshCompiler
        .compile_source_with_sourcemap("print \"hello\"\n")
        .expect("compile");

    let script = compact_runtime_snapshot(
        &instrument_compiled_script("example.ush".as_ref(), &compiled).text,
    );

    assert_fixture("runtime_diagnostics_simple.sh", &script);
}

#[test]
fn control_join_lines_are_not_instrumented() {
    assert!(!should_inline_track("}; do"));
    assert!(!should_inline_track("}; then"));
    assert!(!should_inline_track("} && {"));
    assert!(!should_inline_track("done"));
    assert!(!should_inline_track("'0') value='ok'; ;;"));
    assert!(!should_inline_track("'--name') shift; value=\"$1\"; ;;"));
    assert!(should_inline_track("[ \"$(printf '%s' true)\" = 'true' ]"));
    assert!(should_inline_track("count=$((count + 1))"));
}

#[test]
fn multiline_shell_literals_are_left_uninstrumented() {
    let compiled = UshCompiler
        .compile_source_with_sourcemap(
            "let article = \"\"\"\n  <article>\n    hello\n  </article>\n\"\"\"\nprint article\n",
        )
        .expect("compile");

    let script = compact_runtime_snapshot(
        &instrument_compiled_script("example.ush".as_ref(), &compiled).text,
    );
    assert_fixture("runtime_diagnostics_multiline.sh", &script);
}

#[test]
fn generated_line_groups_render_as_shell_line_spans() {
    assert_eq!(format_generated_lines(&[7]), "G0007");
    assert_eq!(format_generated_lines(&[7, 8, 11]), "G0007, G0008, G0011");
}

/// Same `UPDATE_SNAPSHOTS=1` convention the integration tests use.
fn assert_fixture(name: &str, actual: &str) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/fixtures")
        .join(name);
    if env::var_os("UPDATE_SNAPSHOTS").is_some() {
        fs::write(&path, actual).expect("write fixture");
    }
    let expected = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    assert_eq!(actual, expected, "fixture mismatch: {}", path.display());
}

fn compact_runtime_snapshot(script: &str) -> String {
    let lines = script.lines().collect::<Vec<_>>();
    let shell_start = lines
        .iter()
        .position(|line| *line == "#!/bin/sh")
        .expect("shell start");
    let tail_start = lines.len().saturating_sub(12);
    let mut out = String::new();
    let _ = writeln!(out, "[runtime prelude]");
    out.push_str(&lines[..shell_start].join("\n"));
    out.push('\n');
    let _ = writeln!(out);
    let _ = writeln!(out, "[tail]");
    out.push_str(&lines[tail_start..].join("\n"));
    out.push('\n');
    out
}

#[test]
fn the_header_offset_maps_shell_line_numbers_onto_generated_lines() {
    let compiled = UshCompiler
        .compile_source_with_sourcemap("print \"hello\"\n")
        .expect("compile");
    let script = instrument_compiled_script("example.ush".as_ref(), &compiled);

    // Line 1 of the generated shell sits directly after the header,
    // and the round trip has to survive both directions.
    assert_eq!(script.shell_line(1), script.header_lines + 1);
    assert_eq!(script.generated_line(script.shell_line(42)), Some(42));
    assert_eq!(script.generated_line(script.header_lines), None);

    // The offset the header prints has to be the offset it occupies,
    // or the report points at the wrong line.
    assert!(script.text.contains(&format!(
        "__ush_runtime_map_offset='{}'",
        script.header_lines
    )));
}
