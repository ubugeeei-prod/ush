#[derive(Default)]
pub(super) struct DiffOptions {
    pub(super) recursive: bool,
    pub(super) new_file: bool,
    pub(super) text: bool,
    pub(super) ignore_all_space: bool,
    pub(super) ignore_space_change: bool,
    pub(super) ignore_blank_lines: bool,
    pub(super) ignore_case: bool,
    pub(super) context: usize,
    pub(super) targets: Vec<String>,
}

#[derive(Default)]
pub(super) struct DiffReport {
    pub(super) sections: Vec<DiffSection>,
    pub(super) notes: Vec<String>,
}

pub(super) struct DiffSection {
    pub(super) old_label: String,
    pub(super) new_label: String,
    pub(super) hunks: Vec<DiffHunk>,
    pub(super) additions: usize,
    pub(super) deletions: usize,
    pub(super) notes: Vec<String>,
}

impl DiffSection {
    pub(super) fn new(old_label: String, new_label: String) -> Self {
        Self {
            old_label,
            new_label,
            hunks: Vec::new(),
            additions: 0,
            deletions: 0,
            notes: Vec::new(),
        }
    }
}

pub(super) struct DiffHunk {
    pub(super) header: String,
    pub(super) lines: Vec<DiffLine>,
}

pub(super) struct DiffLine {
    pub(super) kind: DiffLineKind,
    pub(super) text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DiffLineKind {
    Context,
    Added,
    Removed,
    Note,
}
