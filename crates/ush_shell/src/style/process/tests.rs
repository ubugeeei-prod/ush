use super::{PsRow, render_kill, render_ps, render_ps_row};
use crate::style::strip_ansi;

fn row(command: &str) -> PsRow {
    PsRow {
        pid: "42".to_string(),
        ppid: "1".to_string(),
        stat: "S".to_string(),
        cpu: "0.5".to_string(),
        mem: "1.2".to_string(),
        command: command.to_string(),
    }
}

#[test]
fn a_process_row_shows_the_basename_and_its_metadata() {
    let mut out = String::new();
    render_ps_row(&mut out, &row("ush"));

    assert_eq!(
        strip_ansi(&out),
        "ush [pid 42] [S] ppid 1, cpu 0.5%, mem 1.2%\n"
    );
}

#[test]
fn a_full_path_keeps_the_original_on_a_second_line() {
    let mut out = String::new();
    render_ps_row(&mut out, &row("/usr/local/bin/ush"));
    let text = strip_ansi(&out);

    assert!(text.starts_with("ush [pid 42]"));
    assert!(text.ends_with("  /usr/local/bin/ush\n"));
}

#[test]
fn ps_only_takes_over_when_it_is_called_without_arguments() {
    assert!(render_ps(&["-ef".to_string()]).expect("render").is_none());
}

#[test]
fn kill_without_a_pid_falls_back_to_the_plain_command() {
    assert!(render_kill(&[]).expect("render").is_none());
    assert!(render_kill(&["-9".to_string()]).expect("render").is_none());
}

#[test]
fn a_real_process_listing_is_summarized() {
    let Some(rendered) = render_ps(&[]).expect("render") else {
        return;
    };
    let text = strip_ansi(&rendered.to_text().expect("text"));

    assert!(text.starts_with("ps "));
    assert!(text.contains("process"));
}
