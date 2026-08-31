use std::{
    env,
    ffi::OsStr,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

use ush_compiler::UshCompiler;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/shell_snapshots")
}

fn workspace_roots() -> Vec<String> {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_dir
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf();

    let mut roots = vec![workspace_root.to_string_lossy().into_owned()];
    if let Ok(canonical) = workspace_root.canonicalize() {
        roots.push(canonical.to_string_lossy().into_owned());
    }
    roots.sort();
    roots.dedup();
    roots
}

fn normalize_shell(shell: &str, workspace_roots: &[String]) -> String {
    let mut normalized = shell.to_owned();
    for root in workspace_roots {
        normalized = normalized.replace(root, "$WORKSPACE");
    }
    normalized
}

fn snapshot_sources() -> Vec<PathBuf> {
    let mut sources = fs::read_dir(fixtures_dir())
        .expect("read shell snapshot fixtures")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            (path.extension() == Some(OsStr::new("ush"))).then_some(path)
        })
        .collect::<Vec<_>>();
    sources.sort();
    sources
}

fn render_catalog(source: &Path, source_text: &str, shell_text: &str, out: &mut String) {
    let stem = source
        .file_stem()
        .and_then(OsStr::to_str)
        .expect("fixture stem");
    let _ = writeln!(out, "## `{stem}.ush` -> `{stem}.sh`");
    let _ = writeln!(out);
    let _ = writeln!(out, "```ush");
    out.push_str(source_text);
    if !source_text.ends_with('\n') {
        out.push('\n');
    }
    let _ = writeln!(out, "```");
    let _ = writeln!(out);
    let _ = writeln!(out, "```sh");
    out.push_str(shell_text);
    if !shell_text.ends_with('\n') {
        out.push('\n');
    }
    let _ = writeln!(out, "```");
    let _ = writeln!(out);
}

/// Matches the `UPDATE_SNAPSHOTS=1` workflow the `apps/ush` fixture
/// tests already use, so a deliberate codegen change is one command
/// away instead of a hand-edited `.sh` file.
fn updating_snapshots() -> bool {
    env::var_os("UPDATE_SNAPSHOTS").is_some()
}

#[test]
fn compiled_shell_snapshots_match_expected_output() {
    let compiler = UshCompiler::default();
    let workspace_roots = workspace_roots();
    let sources = snapshot_sources();

    assert!(!sources.is_empty(), "missing shell snapshot fixtures");

    for source in sources {
        let expected_path = source.with_extension("sh");
        let compiled = compiler
            .compile_file(&source)
            .unwrap_or_else(|error| panic!("failed to compile {}: {error}", source.display()));
        let actual = normalize_shell(&compiled.to_string(), &workspace_roots);
        if updating_snapshots() {
            fs::write(&expected_path, &actual).expect("write shell snapshot");
        }
        let expected = fs::read_to_string(&expected_path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", expected_path.display()));

        assert_eq!(
            actual,
            expected,
            "shell snapshot mismatch for {}",
            source.file_name().unwrap_or_default().to_string_lossy()
        );
    }
}

#[test]
fn compiled_shell_snapshots_are_listed_in_catalog() {
    let compiler = UshCompiler::default();
    let workspace_roots = workspace_roots();
    let sources = snapshot_sources();

    assert!(!sources.is_empty(), "missing shell snapshot fixtures");

    let mut catalog = String::new();
    catalog.push_str("# Compiler Shell Snapshots\n\n");
    for source in &sources {
        let file_name = source
            .file_name()
            .and_then(OsStr::to_str)
            .expect("fixture file name");
        let stem = source
            .file_stem()
            .and_then(OsStr::to_str)
            .expect("fixture stem");
        let _ = writeln!(catalog, "- `{file_name}` -> `{stem}.sh`");
    }
    catalog.push('\n');

    for source in sources {
        let source_text = fs::read_to_string(&source)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", source.display()));
        let compiled = compiler
            .compile_file(&source)
            .unwrap_or_else(|error| panic!("failed to compile {}: {error}", source.display()));
        let shell_text = normalize_shell(&compiled.to_string(), &workspace_roots);
        render_catalog(&source, &source_text, &shell_text, &mut catalog);
    }

    if updating_snapshots() {
        let catalog_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/shell_snapshots_catalog.md");
        fs::write(catalog_path, &catalog).expect("write snapshot catalog");
    }
    assert_eq!(catalog, include_str!("fixtures/shell_snapshots_catalog.md"));
}
