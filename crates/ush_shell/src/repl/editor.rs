use std::path::{Path, PathBuf};

use anyhow::Result;
use rustyline::{
    CompletionType, Config, EditMode, Editor, error::ReadlineError, history::DefaultHistory,
};
use ush_config::ShellKeymap;

use super::{UshHelper, bindings};
use crate::commands::CommandNames;

pub fn create_editor(
    history_file: &Path,
    history_size: usize,
    keymap: ShellKeymap,
    commands: impl Into<CommandNames>,
    env_names: Vec<String>,
    cwd: PathBuf,
) -> Result<Editor<UshHelper, DefaultHistory>> {
    let config = Config::builder()
        .max_history_size(history_size)?
        .history_ignore_dups(true)?
        .history_ignore_space(true)
        .completion_type(CompletionType::Circular)
        .completion_show_all_if_ambiguous(false)
        .completion_prompt_limit(200)
        .keyseq_timeout(Some(300))
        .edit_mode(edit_mode(keymap))
        .auto_add_history(true)
        .build();
    let helper = UshHelper::new(commands, env_names, cwd);
    let selection = helper.selection_handle();
    let completion = helper.completion_handle();
    let mut editor = Editor::with_config(config)?;
    editor.set_helper(Some(helper));
    bindings::configure_editor(&mut editor, selection, completion);
    let _ = editor.load_history(history_file);
    Ok(editor)
}

fn edit_mode(keymap: ShellKeymap) -> EditMode {
    match keymap {
        ShellKeymap::Emacs => EditMode::Emacs,
        ShellKeymap::Vi => EditMode::Vi,
    }
}

pub fn classify_readline_error(error: ReadlineError) -> anyhow::Error {
    match error {
        ReadlineError::Interrupted => anyhow::anyhow!("interrupted"),
        ReadlineError::Eof => anyhow::anyhow!("eof"),
        other => anyhow::anyhow!(other),
    }
}
