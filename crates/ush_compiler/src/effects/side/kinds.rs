//! The effect kinds and the bitset that holds them.

use core::fmt;

/// What a function reaches for beyond its own arguments.
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
    /// Concurrency: `spawn`, `.await`, and `async` blocks.
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

/// A set of [`Effect`]s, held as a bitset.
///
/// The inference walk unions these once per expression node, so the
/// representation is one byte and every operation is a mask.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EffectSet(u8);

impl EffectSet {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn of(effect: Effect) -> Self {
        Self(effect.bit())
    }

    pub fn insert(&mut self, effect: Effect) {
        self.0 |= effect.bit();
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn merge(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// The effects in `self` that `other` does not cover.
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    pub const fn contains(self, effect: Effect) -> bool {
        self.0 & effect.bit() != 0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn iter(self) -> impl Iterator<Item = Effect> {
        Effect::ALL.into_iter().filter(move |e| self.contains(*e))
    }
}

impl fmt::Display for EffectSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return f.write_str("pure");
        }
        let mut first = true;
        for effect in self.iter() {
            if !first {
                f.write_str(", ")?;
            }
            f.write_str(effect.name())?;
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
