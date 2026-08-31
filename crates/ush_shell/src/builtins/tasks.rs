#[cfg(test)]
mod tests;

use compact_str::CompactString;
use smallvec::SmallVec;

use anyhow::Result;

use crate::{
    Shell, ValueStream,
    repl::contextual::{TaskEntry, discover_tasks},
    style,
};

impl Shell {
    pub(super) fn handle_tasks(&self, args: &[String]) -> Result<(ValueStream, i32)> {
        let entries = filter_tasks(discover_tasks(&self.cwd), args);
        let output = if self.options.stylish {
            style::render_tasks(&entries)
        } else {
            render_tasks_plain(&entries)
        };
        Ok((ValueStream::Text(output), 0))
    }
}

fn render_tasks_plain(entries: &[TaskEntry]) -> String {
    let mut out = String::new();
    for entry in entries {
        entry.write_command(&mut out);
        out.push('\n');
    }
    out
}

fn filter_tasks(mut entries: Vec<TaskEntry>, filters: &[String]) -> Vec<TaskEntry> {
    if filters.is_empty() {
        return entries;
    }

    let needles = filters
        .iter()
        .map(|filter| CompactString::from(filter.to_ascii_lowercase()))
        .collect::<SmallVec<[CompactString; 4]>>();
    entries.retain(|entry| needles.iter().all(|needle| matches_task(entry, needle)));
    entries
}

fn matches_task(entry: &TaskEntry, needle: &str) -> bool {
    contains_ignore_ascii_case(entry.source.as_str(), needle)
        || contains_ignore_ascii_case(entry.name.as_str(), needle)
        || contains_ignore_ascii_case(entry.source.command_prefix(), needle)
        || contains_ignore_ascii_case(entry.command().as_str(), needle)
}

fn contains_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    let needle = needle.as_bytes();
    needle.is_empty()
        || haystack
            .as_bytes()
            .windows(needle.len())
            .any(|window| window.eq_ignore_ascii_case(needle))
}
