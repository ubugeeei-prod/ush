mod bindings;
mod builtin_completion;
mod complete;
mod completion_state;
pub(crate) mod contextual;
mod descriptions;
mod display;
mod editor;
mod git_completion;
mod highlight;
mod selection;
mod syntax;
#[cfg(test)]
mod tests;
mod validate;

use std::{
    borrow::Cow,
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use crate::commands::CommandNames;
use crate::parser::is_builtin;
use rustyline::{
    CompletionType, Context, Helper,
    completion::{Completer, FilenameCompleter, Pair},
    highlight::{CmdKind, Highlighter},
    hint::{Hinter, HistoryHinter},
    validate::{ValidationContext, ValidationResult, Validator},
};

pub use self::editor::{classify_readline_error, create_editor};
use self::selection::SelectionHandle;

pub(crate) use self::validate::validate_input;

pub struct UshHelper {
    commands: CommandNames,
    env_names: BTreeSet<String>,
    alias_names: BTreeSet<String>,
    jobs: Vec<ReplJobCandidate>,
    cwd: PathBuf,
    files: FilenameCompleter,
    hinter: HistoryHinter,
    completion: completion_state::CompletionState,
    selection: SelectionHandle,
}

#[derive(Clone)]
pub(crate) struct ReplJobCandidate {
    pub(crate) spec: String,
    pub(crate) summary: String,
}

impl UshHelper {
    pub fn new(commands: impl Into<CommandNames>, env_names: Vec<String>, cwd: PathBuf) -> Self {
        Self {
            commands: commands.into(),
            env_names: env_names.into_iter().collect(),
            alias_names: BTreeSet::new(),
            jobs: Vec::new(),
            cwd,
            files: FilenameCompleter::new(),
            hinter: HistoryHinter::new(),
            completion: completion_state::CompletionState::default(),
            selection: SelectionHandle::default(),
        }
    }

    pub fn refresh(
        &mut self,
        commands: impl Into<CommandNames>,
        env_names: Vec<String>,
        cwd: PathBuf,
        alias_names: Vec<String>,
        jobs: Vec<ReplJobCandidate>,
    ) {
        self.commands = commands.into();
        self.env_names = env_names.into_iter().collect();
        self.alias_names = alias_names.into_iter().collect();
        self.jobs = jobs;
        self.cwd = cwd;
        self.completion.clear();
        self.selection.clear();
    }

    pub fn selection_handle(&self) -> SelectionHandle {
        self.selection.clone()
    }

    pub fn completion_handle(&self) -> completion_state::CompletionState {
        self.completion.clone()
    }

    pub fn selection_range(&self) -> Option<(usize, usize)> {
        self.selection.range()
    }

    pub fn has_selection(&self) -> bool {
        self.selection.has_selection()
    }

    pub(crate) fn update_completion(&self, line: &str, pos: usize, start: usize, pairs: &[Pair]) {
        self.completion.update(line, pos, start, pairs);
    }

    pub(crate) fn cwd(&self) -> &Path {
        &self.cwd
    }

    fn command_pairs(&self, needle: &str) -> Vec<Pair> {
        let keywords = syntax::keywords().iter().copied().map(str::to_string);
        self.commands
            .iter()
            .cloned()
            .chain(keywords)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .filter(|item| item.starts_with(needle))
            .map(|item| {
                let detail = builtin_completion::command_summary(&item).unwrap_or_else(|| {
                    descriptions::command(&item, is_builtin(&item), syntax::is_keyword(&item))
                });
                display::same_pair(item, Some(detail))
            })
            .collect()
    }

    fn env_pairs(&self, needle: &str, brace: bool, suffix: &str) -> Vec<Pair> {
        self.env_names
            .iter()
            .filter(|item| item.starts_with(needle))
            .map(|item| {
                let replacement = if brace {
                    format!("${{{item}}}{suffix}")
                } else {
                    format!("${item}{suffix}")
                };
                display::pair(item, replacement, Some(descriptions::ENV_VAR))
            })
            .collect()
    }

    fn env_name_pairs(&self, needle: &str) -> Vec<Pair> {
        self.env_names
            .iter()
            .filter(|item| item.starts_with(needle))
            .map(|item| display::pair(item, format!("{item}="), Some(descriptions::ENV_BINDING)))
            .collect()
    }
}

impl Helper for UshHelper {}

impl Completer for UshHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        complete::complete(self, line, pos, ctx)
    }
}

impl Hinter for UshHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        if let Some(hint) = self.completion.hint(line, pos) {
            return Some(hint);
        }
        if let Some(hint) = self.hinter.hint(line, pos, ctx) {
            return Some(hint);
        }
        if let Some(hint) = builtin_completion::hint(self, line, pos) {
            return Some(hint);
        }
        if let Some(hint) = git_completion::hint(self, line, pos) {
            return Some(hint);
        }
        None
    }
}

impl Highlighter for UshHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Owned(highlight::highlight_line(self, line))
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        Cow::Owned(highlight::highlight_prompt(prompt))
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned(highlight::highlight_hint(hint))
    }

    fn highlight_candidate<'c>(
        &self,
        candidate: &'c str,
        completion: CompletionType,
    ) -> Cow<'c, str> {
        Cow::Owned(highlight::highlight_candidate(
            self, candidate, completion, false,
        ))
    }

    fn highlight_candidate_with_state<'c>(
        &self,
        candidate: &'c str,
        completion: CompletionType,
        active: bool,
    ) -> Cow<'c, str> {
        Cow::Owned(highlight::highlight_candidate(
            self, candidate, completion, active,
        ))
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
        kind == CmdKind::ForcedRefresh || self.has_selection() || syntax::needs_refresh(line, pos)
    }
}

impl Validator for UshHelper {
    fn validate(&self, ctx: &mut ValidationContext<'_>) -> rustyline::Result<ValidationResult> {
        Ok(validate::validate_input(ctx.input()))
    }
}
