//! Effects and effect rows.
//!
//! An effect is a capability a computation needs from its caller.
//! Two kinds exist, and the difference is what can discharge them:
//!
//! - **Built-in** effects (`io`, `fs`, `env`, `net`, `exec`, `task`)
//!   describe what the generated shell actually does. They are
//!   inferred from the stdlib and propagate outward forever — no
//!   handler can take back the fact that a program wrote a file.
//! - **User** effects are declared with `effect`, performed with
//!   `do`, and discharged by a `try … with` handler, the way an
//!   effect works in Effekt.

use core::fmt;

use crate::types::{AstString, HeapVec as Vec};

/// A built-in effect: something the generated shell does to the
/// world, inferred rather than declared.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Effect {
    /// Terminal output and input: `print`, and the interactive
    /// builtins.
    Io,
    /// The filesystem: `std::fs`, and the `std::path` helpers that
    /// look at what is actually on disk.
    Fs,
    /// Process environment: `std::env`, and `$PATH`-shaped helpers.
    Env,
    /// The network: `std::http`.
    Net,
    /// Other processes: `$ cmd`, `shell`, and `std::command`.
    Exec,
    /// Concurrency: `async`, `spawn`, `.await`, and `async` blocks.
    Task,
}

impl Effect {
    pub const ALL: [Self; 6] = [
        Self::Io,
        Self::Fs,
        Self::Env,
        Self::Net,
        Self::Exec,
        Self::Task,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Io => "io",
            Self::Fs => "fs",
            Self::Env => "env",
            Self::Net => "net",
            Self::Exec => "exec",
            Self::Task => "task",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|effect| effect.name() == name)
    }

    const fn bit(self) -> u8 {
        1 << (self as u8)
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// An effect row: everything a computation still needs from its
/// caller.
///
/// The six built-ins live in a bitset because the inference walk
/// unions rows once per expression node; user effects are rare
/// enough to keep in a sorted list beside it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EffectSet {
    builtin: u8,
    user: Vec<AstString>,
}

impl EffectSet {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn of(effect: Effect) -> Self {
        Self {
            builtin: effect.bit(),
            user: Vec::new(),
        }
    }

    pub fn of_user(name: impl Into<AstString>) -> Self {
        Self {
            builtin: 0,
            user: alloc::vec![name.into()],
        }
    }

    pub fn insert(&mut self, effect: Effect) {
        self.builtin |= effect.bit();
    }

    pub fn insert_user(&mut self, name: impl Into<AstString>) {
        let name = name.into();
        if let Err(index) = self.user.binary_search(&name) {
            self.user.insert(index, name);
        }
    }

    pub fn union(mut self, other: &Self) -> Self {
        self.merge(other);
        self
    }

    pub fn merge(&mut self, other: &Self) {
        self.builtin |= other.builtin;
        for name in &other.user {
            self.insert_user(name.clone());
        }
    }

    /// The effects in `self` that `other` does not cover — what is
    /// still unhandled after `other` has been discharged.
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            builtin: self.builtin & !other.builtin,
            user: self
                .user
                .iter()
                .filter(|name| !other.user.contains(name))
                .cloned()
                .collect(),
        }
    }

    pub fn contains(&self, effect: Effect) -> bool {
        self.builtin & effect.bit() != 0
    }

    pub fn contains_user(&self, name: &str) -> bool {
        self.user.iter().any(|item| item == name)
    }

    pub fn is_empty(&self) -> bool {
        self.builtin == 0 && self.user.is_empty()
    }

    pub fn builtins(&self) -> impl Iterator<Item = Effect> + '_ {
        Effect::ALL
            .into_iter()
            .filter(move |effect| self.contains(*effect))
    }

    pub fn user_effects(&self) -> impl Iterator<Item = &AstString> {
        self.user.iter()
    }

    /// The row as it is written in a signature: `{ io, fs }`, or
    /// `{}` for a computation that needs nothing.
    pub fn render_row(&self) -> crate::types::OutputString {
        let mut out = crate::types::OutputString::from("{");
        let mut first = true;
        for name in self.names() {
            out.push_str(if first { " " } else { ", " });
            out.push_str(&name);
            first = false;
        }
        out.push_str(if first { "}" } else { " }" });
        out
    }

    fn names(&self) -> Vec<crate::types::OutputString> {
        let mut names = Vec::new();
        for effect in self.builtins() {
            names.push(crate::types::OutputString::from(effect.name()));
        }
        for name in &self.user {
            names.push(crate::types::OutputString::from(name.as_str()));
        }
        names
    }
}

impl fmt::Display for EffectSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return f.write_str("pure");
        }
        let mut first = true;
        for name in self.names() {
            if !first {
                f.write_str(", ")?;
            }
            f.write_str(&name)?;
            first = false;
        }
        Ok(())
    }
}

impl FromIterator<Effect> for EffectSet {
    fn from_iter<I: IntoIterator<Item = Effect>>(effects: I) -> Self {
        let mut set = Self::empty();
        for effect in effects {
            set.insert(effect);
        }
        set
    }
}
