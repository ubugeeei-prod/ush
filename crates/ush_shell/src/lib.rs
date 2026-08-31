//! Interactive shell runtime for `ush`.
//!
//! Owns the interactive REPL, the parser for shell-level input
//! (one-liners, pipelines, aliases, helpers), the dispatcher into
//! either a builtin or `Command::spawn`, the structured helper
//! pipelines (`json`, `xml`, `len`, `lines`, …), the stylish
//! renderers (`ls`, `ps`, …), and the integration with the vendored
//! `rustyline` line editor.
//!
//! See [docs/architecture.md][arch] for the workspace-level map.
//!
//! [arch]: https://github.com/ubugeeei/ush/blob/main/docs/architecture.md

mod builtins;
mod commands;
mod execute;
mod expand;
mod helpers;
mod options;
mod parser;
mod process;
mod prompt;
mod repl;
mod signal;
mod startup;
mod style;

use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    path::PathBuf,
};

use ush_config::{RuntimePaths, UshConfig};

pub use self::{
    helpers::{HelperInvocation, ValueStream},
    options::{ShellOptions, run_posix_script},
    parser::{ParsedLine, parse_line},
    startup::SessionStartup,
};

pub struct Shell {
    config: UshConfig,
    options: ShellOptions,
    env: HashMap<String, String>,
    aliases: BTreeMap<String, String>,
    cwd: PathBuf,
    jobs: Vec<process::Job>,
    next_job_id: usize,
    last_status: i32,
    paths: RuntimePaths,
    command_names: RefCell<commands::CommandNameCache>,
}
