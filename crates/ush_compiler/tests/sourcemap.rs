use std::fmt::Write as _;

use ush_compiler::{CompiledScript, SourceMapSection, UshCompiler};

fn generated_line(output: &str, needle: &str) -> usize {
    output
        .lines()
        .position(|line| line.contains(needle))
        .map(|index| index + 1)
        .unwrap_or_else(|| panic!("missing generated line: {needle}"))
}

#[test]
fn sourcemap_tracks_top_level_statements() {
    let compiled = UshCompiler::default()
        .compile_source_with_sourcemap("let greeting = \"hello\"\nprint greeting\n")
        .expect("compile");

    let assign_line = generated_line(&compiled.shell, "greeting='hello'");
    let print_line = generated_line(&compiled.shell, "printf '%s\\n' \"${greeting}\"");

    assert_eq!(compiled.sourcemap.source_line(1), None);
    assert_eq!(compiled.sourcemap.source_line(assign_line), Some(1));
    assert_eq!(compiled.sourcemap.source_line(print_line), Some(2));
    assert_eq!(
        compiled
            .sourcemap
            .line(assign_line)
            .and_then(|line| line.source_text.as_deref()),
        Some("let greeting = \"hello\"")
    );
    assert_eq!(
        compiled
            .sourcemap
            .line(print_line)
            .map(|line| line.generated_text.as_str()),
        Some("printf '%s\\n' \"${greeting}\"")
    );
}

#[test]
fn sourcemap_tracks_nested_function_body_lines() {
    let compiled = UshCompiler::default()
        .compile_source_with_sourcemap(concat!(
            "fn greet(name: String) -> String {\n",
            "  print name;\n",
            "  \"hi \" + name\n",
            "}\n",
        ))
        .expect("compile");

    let header_line = generated_line(&compiled.shell, "ush_fn_greet()");
    let body_line = generated_line(
        &compiled.shell,
        "printf '%s\\n' \"${__ush_fn_greet_arg_0}\"",
    );
    let return_line = generated_line(
        &compiled.shell,
        "printf '%s' \"$(printf '%s' 'hi ' \"${__ush_fn_greet_arg_0}\")\"",
    );

    assert_eq!(compiled.sourcemap.source_line(header_line), Some(1));
    assert_eq!(compiled.sourcemap.source_line(body_line), Some(2));
    assert_eq!(compiled.sourcemap.source_line(return_line), Some(3));
}

#[test]
fn sourcemap_render_listing_matches_snapshot() {
    let compiled = UshCompiler::default()
        .compile_source_with_sourcemap("let greeting = \"hello\"\nprint greeting\n")
        .expect("compile");

    let listing = compact_sourcemap_snapshot(&compiled);

    assert_snapshot("sourcemap_listing.txt", &listing);
}

#[test]
fn sourcemap_summary_and_source_index_group_related_lines() {
    let compiled = UshCompiler::default()
        .compile_source_with_sourcemap("if true {\n  print \"hi\"\n}\n")
        .expect("compile");

    let condition_lines = compiled.sourcemap.generated_lines_for_source(1);
    let print_lines = compiled.sourcemap.generated_lines_for_source(2);
    let summary = compiled.sourcemap.summary();
    let source_index = compiled.sourcemap.source_index();
    let user_code = summary
        .sections
        .iter()
        .find(|section| section.section == SourceMapSection::UserCode)
        .expect("user-code section");

    assert!(condition_lines.len() >= 3);
    assert_eq!(print_lines.len(), 2);
    assert_eq!(summary.source_line_count, 2);
    assert_eq!(
        source_index
            .iter()
            .find(|line| line.source_line == 1)
            .and_then(|line| line.source_text.as_deref()),
        Some("if true {")
    );
    assert_eq!(
        source_index
            .iter()
            .find(|line| line.source_line == 1)
            .map(|line| line.generated_lines.as_slice()),
        Some(condition_lines.as_slice())
    );
    assert_eq!(
        user_code.mapped_line_count,
        condition_lines.len() + print_lines.len()
    );
    assert!(user_code.generated_line_count >= user_code.mapped_line_count);
}

#[test]
fn sourcemap_render_mapped_listing_matches_snapshot() {
    let compiled = UshCompiler::default()
        .compile_source_with_sourcemap("print \"hello\"\n")
        .expect("compile");

    let listing = compiled.sourcemap.render_mapped_listing();

    assert_snapshot("sourcemap_mapped_listing.txt", &listing);
}

fn compact_sourcemap_snapshot(compiled: &CompiledScript) -> String {
    let summary = compiled.sourcemap.summary();
    let sources = compiled.sourcemap.source_index();
    let mut out = String::new();

    let _ = writeln!(out, "summary:");
    let _ = writeln!(
        out,
        "  generated_line_count: {}",
        summary.generated_line_count
    );
    let _ = writeln!(out, "  mapped_line_count: {}", summary.mapped_line_count);
    let _ = writeln!(
        out,
        "  unmapped_line_count: {}",
        summary.unmapped_line_count
    );
    let _ = writeln!(out, "  source_line_count: {}", summary.source_line_count);
    let _ = writeln!(
        out,
        "  first_mapped_generated_line: {}",
        summary.first_mapped_generated_line.unwrap_or_default()
    );
    let _ = writeln!(
        out,
        "  last_mapped_generated_line: {}",
        summary.last_mapped_generated_line.unwrap_or_default()
    );
    let _ = writeln!(out, "sections:");
    for section in summary.sections {
        let _ = writeln!(
            out,
            "  {}: generated={} mapped={}",
            section.section.label(),
            section.generated_line_count,
            section.mapped_line_count
        );
    }
    let _ = writeln!(out, "sources:");
    for source in sources {
        let rendered_lines = source
            .generated_lines
            .iter()
            .map(|line| format!("G{line:04}"))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(
            out,
            "  S{:04} -> {} | {}",
            source.source_line,
            rendered_lines,
            source.source_text.as_deref().unwrap_or("")
        );
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "mapped listing:");
    out.push_str(&compiled.sourcemap.render_mapped_listing());
    out
}

fn assert_snapshot(name: &str, listing: &str) {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    if std::env::var_os("UPDATE_SNAPSHOTS").is_some() {
        std::fs::write(&path, listing).unwrap();
    }
    assert_eq!(listing, std::fs::read_to_string(path).unwrap());
}

#[test]
fn optional_line_index_preserves_identical_artifact_bytes() {
    use std::fs;
    use ush_compiler::ShellTarget;

    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("demo.ush");
    let output = dir.path().join("demo.sh");
    let source = "fn text() -> String / unknown, fs, env, net, stdin, stderr, exec {\n  let value = $(\ncat <<'END'\n  body\nEND\n  )?\n  value\n}\nprint text()?\n";
    fs::write(&input, source).unwrap();
    for target in [
        ShellTarget::Sh,
        ShellTarget::Bash,
        ShellTarget::Zsh,
        ShellTarget::Ash,
    ] {
        let compiler = UshCompiler::for_target(target);
        let compiled = compiler
            .compile_checked_in_file_with_sourcemap(&input, &output)
            .unwrap();
        assert_eq!(
            compiled.shell,
            compiler.compile_checked_in_file(&input, &output).unwrap()
        );
        assert_eq!(
            compiled.sourcemap.lines.len(),
            compiled.shell.lines().count()
        );
        assert_eq!(
            compiler.compile_source(source).unwrap(),
            compiler
                .compile_source_with_sourcemap(source)
                .unwrap()
                .shell
        );
    }
}
