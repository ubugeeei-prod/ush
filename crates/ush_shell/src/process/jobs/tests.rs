use std::{
    thread,
    time::{Duration, Instant},
};

use ush_config::UshConfig;

use crate::{Shell, ShellOptions, ValueStream};

fn shell() -> Shell {
    Shell::new(
        UshConfig::default(),
        ShellOptions {
            stylish: false,
            interaction: false,
            print_ast: false,
        },
    )
    .expect("shell")
}

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn text(stream: ValueStream) -> String {
    stream.to_text().expect("text")
}

#[test]
fn listing_jobs_takes_no_arguments() {
    let error = shell()
        .handle_jobs(&args(&["%1"]))
        .expect_err("jobs with arguments");

    assert!(error.to_string().contains("does not accept arguments"));
}

#[test]
fn a_shell_without_jobs_lists_nothing() {
    let (stream, status) = shell().handle_jobs(&[]).expect("jobs");

    assert_eq!(text(stream), "");
    assert_eq!(status, 0);
}

#[test]
fn a_background_job_is_announced_and_then_listed() {
    let mut shell = shell();
    let announcement = shell
        .spawn_background_job("sleep 5")
        .expect("spawn background job");

    assert!(announcement.starts_with("[1] "));
    assert!(announcement.ends_with('\n'));

    let (stream, _) = shell.handle_jobs(&[]).expect("jobs");
    let listing = text(stream);
    assert!(listing.starts_with("[1] Running"), "{listing}");
    assert!(listing.contains("sleep 5"), "{listing}");

    shell.handle_disown(&[]).expect("disown");
}

#[test]
fn job_ids_keep_counting_up() {
    let mut shell = shell();
    assert!(
        shell
            .spawn_background_job("sleep 5")
            .expect("first")
            .starts_with("[1] ")
    );
    assert!(
        shell
            .spawn_background_job("sleep 5")
            .expect("second")
            .starts_with("[2] ")
    );

    shell.handle_disown(&args(&["%1", "%2"])).expect("disown");
}

#[test]
fn waiting_returns_the_exit_status_of_the_job() {
    let mut shell = shell();
    shell.spawn_background_job("exit 7").expect("spawn");

    let (_, status) = shell.handle_wait(&[]).expect("wait");
    assert_eq!(status, 7);
}

#[test]
fn waiting_on_a_named_job_returns_its_status() {
    let mut shell = shell();
    shell.spawn_background_job("exit 3").expect("spawn");

    let (_, status) = shell.handle_wait(&args(&["%1"])).expect("wait");
    assert_eq!(status, 3);
}

/// Polls the job table until `job` reaches a terminal label, so the
/// assertion does not race the child process.
fn settled_listing(shell: &mut Shell) -> String {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let (stream, _) = shell.handle_jobs(&[]).expect("jobs");
        let listing = text(stream);
        if !listing.contains("Running") || Instant::now() >= deadline {
            return listing;
        }
        thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn a_finished_job_is_labelled_by_how_it_ended() {
    let mut shell = shell();
    shell.spawn_background_job("exit 0").expect("spawn");
    shell.spawn_background_job("exit 4").expect("spawn");

    let listing = settled_listing(&mut shell);
    assert!(listing.contains("[1] Done"), "{listing}");
    assert!(listing.contains("[2] Exit"), "{listing}");

    shell.handle_disown(&args(&["%1", "%2"])).expect("disown");
}

#[test]
fn foreground_on_a_job_that_already_finished_reports_its_status() {
    let mut shell = shell();
    shell.spawn_background_job("exit 6").expect("spawn");
    let _ = settled_listing(&mut shell);

    let (_, status) = shell.handle_fg(&args(&["%1"])).expect("fg");
    assert_eq!(status, 6);
}

#[test]
fn background_on_a_job_that_already_finished_says_so() {
    let mut shell = shell();
    shell.spawn_background_job("exit 0").expect("spawn");
    let _ = settled_listing(&mut shell);

    let error = shell.handle_bg(&args(&["%1"])).expect_err("finished job");
    assert!(error.to_string().contains("no longer running"));
}

#[test]
fn bringing_a_job_to_the_foreground_returns_its_status() {
    let mut shell = shell();
    shell.spawn_background_job("exit 5").expect("spawn");

    let (_, status) = shell.handle_fg(&args(&["%1"])).expect("fg");
    assert_eq!(status, 5);
}

#[test]
fn disowning_removes_a_job_from_the_table() {
    let mut shell = shell();
    shell.spawn_background_job("sleep 5").expect("spawn");
    shell.handle_disown(&args(&["%1"])).expect("disown");

    let (stream, _) = shell.handle_jobs(&[]).expect("jobs");
    assert_eq!(text(stream), "");
}

#[test]
fn disowning_without_a_spec_drops_the_most_recent_job() {
    let mut shell = shell();
    shell.spawn_background_job("sleep 5").expect("first");
    shell.spawn_background_job("sleep 5").expect("second");
    shell.handle_disown(&[]).expect("disown");

    let (stream, _) = shell.handle_jobs(&[]).expect("jobs");
    assert!(text(stream).starts_with("[1] "));
    shell.handle_disown(&[]).expect("disown");
}

#[test]
fn an_unknown_job_spec_is_reported_with_its_id() {
    let mut shell = shell();
    let error = shell.handle_fg(&args(&["%9"])).expect_err("unknown job");

    assert!(error.to_string().contains("unknown job: %9"));
}

#[test]
fn a_non_numeric_job_spec_is_rejected() {
    let mut shell = shell();
    let error = shell
        .handle_wait(&args(&["%oops"]))
        .expect_err("invalid job spec");

    assert!(error.to_string().contains("invalid job spec"));
}

#[test]
fn foreground_and_background_take_at_most_one_spec() {
    let mut shell = shell();
    assert!(
        shell
            .handle_fg(&args(&["%1", "%2"]))
            .expect_err("too many specs")
            .to_string()
            .contains("at most one job spec")
    );
    assert!(
        shell
            .handle_bg(&args(&["%1", "%2"]))
            .expect_err("too many specs")
            .to_string()
            .contains("at most one job spec")
    );
}

#[test]
fn foreground_without_any_jobs_reports_that_there_are_none() {
    let mut shell = shell();
    assert!(
        shell
            .handle_fg(&[])
            .expect_err("no jobs")
            .to_string()
            .contains("no jobs")
    );
}

#[test]
fn repl_job_candidates_describe_each_job() {
    let mut shell = shell();
    shell.spawn_background_job("sleep 5").expect("spawn");

    let candidates = shell.repl_job_candidates();
    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].spec, "%1");
    assert!(candidates[0].summary.contains("sleep 5"));

    shell.handle_disown(&[]).expect("disown");
}
