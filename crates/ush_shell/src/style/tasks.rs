#[cfg(test)]
mod tests;

use std::fmt::Write as _;

use crate::repl::contextual::{TaskEntry, TaskSource};

use super::common::{
    BLUE_BOLD, CYAN_BOLD, GREEN_BOLD, MAGENTA_BOLD, YELLOW_BOLD, badge, dim, paint, pluralize,
};

pub fn render_tasks(entries: &[TaskEntry]) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{} {}",
        paint(BLUE_BOLD, "tasks"),
        dim(pluralize(entries.len(), "task", "tasks"))
    );
    if entries.is_empty() {
        let _ = writeln!(out, "{}", dim("(empty)"));
        return out;
    }

    let mut counts = [0usize; 5];
    for entry in entries {
        counts[entry.source.index()] += 1;
    }
    write_task_meta(&mut out, &counts);

    for entry in entries {
        let command = entry.command();
        let _ = writeln!(
            out,
            "{} {} {}",
            paint(CYAN_BOLD, &entry.name),
            badge(entry.source.as_str(), task_source_color(entry.source)),
            dim(command)
        );
    }
    out
}

fn write_task_meta(out: &mut String, counts: &[usize; 5]) {
    let mut meta = String::new();
    for source in TaskSource::ALL {
        let count = counts[source.index()];
        if count == 0 {
            continue;
        }
        if meta.is_empty() {
            meta.push_str(&pluralize(
                count,
                source.singular_label(),
                source.plural_label(),
            ));
        } else {
            meta.push_str(", ");
            meta.push_str(&pluralize(
                count,
                source.singular_label(),
                source.plural_label(),
            ));
        }
    }
    if !meta.is_empty() {
        let _ = writeln!(out, "{}", dim(meta));
    }
}

fn task_source_color(source: TaskSource) -> &'static str {
    match source {
        TaskSource::Make => YELLOW_BOLD,
        TaskSource::Just => BLUE_BOLD,
        TaskSource::Mise => MAGENTA_BOLD,
        TaskSource::Npm => GREEN_BOLD,
        TaskSource::Vp => CYAN_BOLD,
    }
}
