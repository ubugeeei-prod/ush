//! The shell-shaped forms: pipelines and blocks that scope the working
//! directory or environment variables.

use super::Expr;
use crate::types::{AstString as String, HeapVec as Vec};

/// `stage | stage |& stage`, written left to right.
#[derive(Debug, Clone)]
pub(crate) struct Pipeline {
    pub stages: Vec<Stage>,
}

#[derive(Debug, Clone)]
pub(crate) struct Stage {
    pub expr: Expr,
    /// The stage is followed by `|&`: its stderr joins its stdout.
    pub merge_stderr: bool,
    /// The stage as written, for the diagnostic that names it.
    pub text: String,
}

/// What a scoped block changes for the duration of its body.
#[derive(Debug, Clone)]
pub(crate) enum Scope {
    /// `cd path { … }`
    Cd(Expr),
    /// `export NAME = value, … { … }`
    Env(Vec<EnvBinding>),
}

/// One `NAME = value` of an `export` statement.
#[derive(Debug, Clone)]
pub(crate) struct EnvBinding {
    pub name: String,
    pub value: Expr,
}

impl Pipeline {
    pub(crate) fn exprs(&self) -> impl Iterator<Item = &Expr> {
        self.stages.iter().map(|stage| &stage.expr)
    }

    pub(crate) fn exprs_mut(&mut self) -> impl Iterator<Item = &mut Expr> {
        self.stages.iter_mut().map(|stage| &mut stage.expr)
    }
}

impl Scope {
    pub(crate) fn exprs(&self) -> impl Iterator<Item = &Expr> {
        let (path, bindings) = match self {
            Self::Cd(path) => (Some(path), &[][..]),
            Self::Env(bindings) => (None, bindings.as_slice()),
        };
        path.into_iter()
            .chain(bindings.iter().map(|binding| &binding.value))
    }

    pub(crate) fn exprs_mut(&mut self) -> impl Iterator<Item = &mut Expr> {
        let (path, bindings) = match self {
            Self::Cd(path) => (Some(path), &mut [][..]),
            Self::Env(bindings) => (None, bindings.as_mut_slice()),
        };
        path.into_iter()
            .chain(bindings.iter_mut().map(|binding| &mut binding.value))
    }
}
