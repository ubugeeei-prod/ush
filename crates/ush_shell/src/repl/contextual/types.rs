use compact_str::CompactString;
use smallvec::SmallVec;

pub(crate) type Names = SmallVec<[CompactString; 16]>;
pub(crate) type Tokens = SmallVec<[CompactString; 8]>;

#[derive(Clone)]
pub(crate) enum ContextualCompletion {
    Pairs(Vec<rustyline::completion::Pair>),
    Path,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum TaskSource {
    Make,
    Just,
    Mise,
    Npm,
    Vp,
}

impl TaskSource {
    pub(crate) const ALL: [Self; 5] = [Self::Make, Self::Just, Self::Mise, Self::Npm, Self::Vp];

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Make => "make",
            Self::Just => "just",
            Self::Mise => "mise",
            Self::Npm => "npm",
            Self::Vp => "vp",
        }
    }

    pub(crate) fn command_prefix(self) -> &'static str {
        match self {
            Self::Make => "make",
            Self::Just => "just",
            Self::Mise => "mise run",
            Self::Npm => "npm run",
            Self::Vp => "vp",
        }
    }

    /// Paired with [`Self::plural_label`] so a count of one reads
    /// `1 npm task` rather than `1 npm`.
    pub(crate) fn singular_label(self) -> &'static str {
        match self {
            Self::Make => "make task",
            Self::Just => "just task",
            Self::Mise => "mise task",
            Self::Npm => "npm task",
            Self::Vp => "vp task",
        }
    }

    pub(crate) fn plural_label(self) -> &'static str {
        match self {
            Self::Make => "make tasks",
            Self::Just => "just tasks",
            Self::Mise => "mise tasks",
            Self::Npm => "npm tasks",
            Self::Vp => "vp tasks",
        }
    }

    pub(crate) fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TaskEntry {
    pub(crate) source: TaskSource,
    pub(crate) name: CompactString,
}

impl TaskEntry {
    pub(crate) fn new(source: TaskSource, name: impl Into<CompactString>) -> Self {
        Self {
            source,
            name: name.into(),
        }
    }

    pub(crate) fn command(&self) -> CompactString {
        let prefix = self.source.command_prefix();
        let mut command = CompactString::with_capacity(prefix.len() + 1 + self.name.len());
        command.push_str(prefix);
        command.push(' ');
        command.push_str(&self.name);
        command
    }

    pub(crate) fn write_command(&self, out: &mut String) {
        out.push_str(self.source.command_prefix());
        out.push(' ');
        out.push_str(&self.name);
    }
}
