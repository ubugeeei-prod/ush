use std::{
    collections::BTreeSet,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, bail};

pub fn run(
    targets: &[String],
    config: Option<&Path>,
    target: ush_compiler::ShellTarget,
) -> Result<i32> {
    let cwd = env::current_dir().context("failed to read current directory")?;
    let tests = discover_tests(targets, &cwd)?;
    if tests.is_empty() {
        eprintln!("ush test: no test scripts found");
        return Ok(1);
    }

    let exe = env::current_exe().context("failed to locate ush executable")?;
    let mut passed = 0usize;
    let mut failed = 0usize;

    let compiler = ush_compiler::UshCompiler::for_target(target);
    for script in tests {
        let label = display_path(&script, &cwd);
        let cases = fs::read_to_string(&script)
            .map_err(anyhow::Error::from)
            .and_then(|source| compiler.test_cases_source(&source));
        let cases = match cases {
            Ok(cases) => cases,
            Err(error) => {
                println!("fail {label}\n  {error:#}");
                failed += 1;
                continue;
            }
        };
        if cases.is_empty() {
            let output = command_for_script(&exe, &script, config)
                .output()
                .with_context(|| format!("failed to run {}", script.display()))?;
            report(&label, &output, &mut passed, &mut failed);
            continue;
        }
        for case in cases {
            let label = format!("{label}:{} ({})", case.line, case.name);
            let shell = match compiler.compile_test_file(&script, case.line) {
                Ok(shell) => shell,
                Err(error) => {
                    println!("fail {label}\n  {error:#}");
                    failed += 1;
                    continue;
                }
            };
            let dir = tempfile::tempdir()?;
            let generated = dir.path().join("test.sh");
            fs::write(&generated, shell)?;
            let output = Command::new(target.to_string()).arg(&generated).output()?;
            report(&label, &output, &mut passed, &mut failed);
        }
    }

    println!("\n{passed} passed; {failed} failed");
    Ok(if failed == 0 { 0 } else { 1 })
}

fn discover_tests(targets: &[String], cwd: &Path) -> Result<Vec<PathBuf>> {
    let mut files = BTreeSet::new();
    if targets.is_empty() {
        let tests_dir = cwd.join("tests");
        if tests_dir.is_dir() {
            collect_ush_files(&tests_dir, &mut files)?;
        }
        return Ok(files.into_iter().collect());
    }

    for target in targets {
        let path = cwd.join(target);
        if path.exists() {
            collect_target(&path, &mut files)?;
            continue;
        }
        for entry in glob::glob(target).with_context(|| format!("invalid glob: {target}"))? {
            let path = entry.with_context(|| format!("invalid path from glob: {target}"))?;
            collect_target(&path, &mut files)?;
        }
    }
    Ok(files.into_iter().collect())
}

fn collect_target(path: &Path, files: &mut BTreeSet<PathBuf>) -> Result<()> {
    if path.is_dir() {
        collect_ush_files(path, files)?;
        return Ok(());
    }
    if is_ush(path) {
        files.insert(fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf()));
        return Ok(());
    }
    bail!(
        "ush test: expected a .ush file or directory: {}",
        path.display()
    )
}

fn collect_ush_files(dir: &Path, files: &mut BTreeSet<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_ush_files(&path, files)?;
        } else if is_ush(&path) {
            files.insert(fs::canonicalize(&path).unwrap_or(path));
        }
    }
    Ok(())
}

fn command_for_script(exe: &Path, script: &Path, config: Option<&Path>) -> Command {
    let mut command = Command::new(exe);
    command.arg("--plain").arg("--no-interaction");
    if let Some(config) = config {
        command.arg("--config").arg(config);
    }
    command.arg(script);
    command
}

fn display_path<'a>(path: &'a Path, cwd: &'a Path) -> String {
    path.strip_prefix(cwd).unwrap_or(path).display().to_string()
}

fn print_stream(label: &str, bytes: &[u8]) {
    if bytes.is_empty() {
        return;
    }
    let text = String::from_utf8_lossy(bytes);
    for line in text.lines() {
        println!("  {label}: {line}");
    }
}

fn is_ush(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("ush")
}

fn report(label: &str, output: &std::process::Output, passed: &mut usize, failed: &mut usize) {
    if output.status.success() {
        println!("ok   {label}");
        *passed += 1;
    } else {
        println!("fail {label}");
        print_stream("stdout", &output.stdout);
        print_stream("stderr", &output.stderr);
        *failed += 1;
    }
}
