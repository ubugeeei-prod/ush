//! Multi-file projects for module tests.
#![allow(dead_code)]

use std::{fs, path::PathBuf, process::Command};

use tempfile::TempDir;
use ush_compiler::UshCompiler;

pub struct Project {
    _dir: TempDir,
    pub root: PathBuf,
}

impl Project {
    pub fn new(files: &[(&str, &str)]) -> Self {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = fs::canonicalize(dir.path()).expect("canonical tempdir");
        for (path, text) in files {
            let path = root.join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, text).unwrap();
        }
        Self { _dir: dir, root }
    }

    pub fn compile(&self, script: &str) -> anyhow::Result<String> {
        UshCompiler.compile_file(&self.root.join(script))
    }

    /// The compile error, with the project directory left out of paths.
    pub fn error(&self, script: &str) -> String {
        let error = self.compile(script).expect_err("compilation should fail");
        format!("{error:#}").replace(&format!("{}/", self.root.display()), "")
    }

    /// Runs the compiled script under every available shell from
    /// another directory and returns the stdout they agree on.
    pub fn run_everywhere(&self, script: &str) -> String {
        let shell_text = self
            .compile(script)
            .unwrap_or_else(|error| panic!("{script}: {error:#}"));
        let path = self.root.join(script).with_extension("sh");
        fs::write(&path, shell_text).unwrap();
        let mut agreed = None;
        for shell in shells() {
            let output = Command::new(shell)
                .arg(&path)
                .current_dir("/")
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{shell}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            assert_eq!(
                agreed.get_or_insert_with(|| stdout.clone()),
                &stdout,
                "{shell}"
            );
        }
        agreed.unwrap()
    }
}

/// `sh`, plus `dash` and `bash` where installed.
pub fn shells() -> Vec<&'static str> {
    ["sh", "dash", "bash"]
        .into_iter()
        .filter(|shell| {
            Command::new(shell)
                .args(["-c", ":"])
                .status()
                .is_ok_and(|status| status.success())
        })
        .collect()
}
