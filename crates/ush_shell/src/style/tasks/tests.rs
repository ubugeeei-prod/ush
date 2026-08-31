use super::{render_tasks, task_source_color};
use crate::{
    repl::contextual::{TaskEntry, TaskSource},
    style::{
        common::{BLUE_BOLD, CYAN_BOLD, GREEN_BOLD, MAGENTA_BOLD, YELLOW_BOLD},
        strip_ansi,
    },
};

#[test]
fn an_empty_list_says_so() {
    let rendered = strip_ansi(&render_tasks(&[]));
    assert_eq!(rendered, "tasks 0 tasks\n(empty)\n");
}

#[test]
fn a_single_task_uses_singular_wording_and_shows_its_command() {
    let rendered = strip_ansi(&render_tasks(&[TaskEntry::new(TaskSource::Npm, "build")]));

    assert_eq!(
        rendered,
        "tasks 1 task\n1 npm task\nbuild [npm] npm run build\n"
    );
}

#[test]
fn tasks_are_grouped_into_per_source_counts() {
    let rendered = strip_ansi(&render_tasks(&[
        TaskEntry::new(TaskSource::Make, "all"),
        TaskEntry::new(TaskSource::Npm, "build"),
        TaskEntry::new(TaskSource::Npm, "test"),
    ]));

    assert!(rendered.starts_with("tasks 3 tasks\n"));
    assert!(rendered.contains("1 make task, 2 npm tasks\n"));
}

#[test]
fn source_counts_follow_the_declared_source_order() {
    let rendered = strip_ansi(&render_tasks(&[
        TaskEntry::new(TaskSource::Vp, "dev"),
        TaskEntry::new(TaskSource::Make, "all"),
    ]));

    let make = rendered.find("1 make task").expect("make");
    let vp = rendered.find("1 vp task").expect("vp");
    assert!(make < vp);
}

#[test]
fn every_source_renders_its_own_command_prefix() {
    let rendered = strip_ansi(&render_tasks(&[
        TaskEntry::new(TaskSource::Make, "all"),
        TaskEntry::new(TaskSource::Just, "fmt"),
        TaskEntry::new(TaskSource::Mise, "lint"),
        TaskEntry::new(TaskSource::Npm, "build"),
        TaskEntry::new(TaskSource::Vp, "dev"),
    ]));

    assert!(rendered.contains("all [make] make all\n"));
    assert!(rendered.contains("fmt [just] just fmt\n"));
    assert!(rendered.contains("lint [mise] mise run lint\n"));
    assert!(rendered.contains("build [npm] npm run build\n"));
    assert!(rendered.contains("dev [vp] vp dev\n"));
}

#[test]
fn every_source_has_its_own_colour() {
    let colors = TaskSource::ALL.map(task_source_color);
    assert_eq!(
        colors,
        [YELLOW_BOLD, BLUE_BOLD, MAGENTA_BOLD, GREEN_BOLD, CYAN_BOLD]
    );
}
