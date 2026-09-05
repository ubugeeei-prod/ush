use std::path::Path;

use super::path::compact_path;

/// Everything a prompt template can ask about the session.
pub(crate) struct PromptContext<'a> {
    pub cwd: &'a Path,
    pub home: Option<&'a str>,
    pub user: Option<&'a str>,
    pub host: Option<&'a str>,
    pub last_status: i32,
    pub git_branch: Option<&'a str>,
}

/// Renders a user-supplied prompt template.
///
/// The escapes are the familiar `PS1` ones so a prompt copied out of
/// a `.bashrc` behaves the way it looks, plus `\g` for the git branch
/// and `\?` for the last exit status. `$VAR` is expanded by the
/// caller before this runs, so a template is free to mix both.
///
/// A template with no backslash in it renders to itself, which is why
/// the plain `shell.prompt` strings that used to be emitted verbatim
/// keep working untouched.
pub(crate) fn render_template(template: &str, ctx: &PromptContext<'_>) -> String {
    let mut out = String::with_capacity(template.len() + 16);
    let mut chars = template.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        let Some(code) = chars.next() else {
            out.push('\\');
            break;
        };
        match code {
            'w' => out.push_str(&compact_path(ctx.cwd, ctx.home)),
            'W' => out.push_str(&basename(ctx.cwd)),
            'u' => out.push_str(ctx.user.unwrap_or("")),
            'h' => out.push_str(short_host(ctx.host)),
            'H' => out.push_str(ctx.host.unwrap_or("")),
            's' => out.push_str("ush"),
            'v' => out.push_str(env!("CARGO_PKG_VERSION")),
            'g' => out.push_str(ctx.git_branch.unwrap_or("")),
            '?' => out.push_str(&ctx.last_status.to_string()),
            '$' => out.push(if is_root() { '#' } else { '$' }),
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            'a' => out.push('\u{7}'),
            'e' => out.push('\u{1b}'),
            // `\[` / `\]` fence non-printing runs in `PS1`. The line
            // editor measures the prompt itself, so the fences are
            // dropped rather than forwarded.
            '[' | ']' => {}
            '\\' => out.push('\\'),
            other => {
                out.push('\\');
                out.push(other);
            }
        }
    }

    out
}

/// True when the template asks for anything the shell has to compute
/// on demand. Used to skip the `git` subprocess for prompts that
/// never mention the branch.
pub(crate) fn wants_git_branch(template: &str) -> bool {
    let mut chars = template.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' && chars.next() == Some('g') {
            return true;
        }
    }
    false
}

fn basename(cwd: &Path) -> String {
    cwd.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| cwd.display().to_string())
}

fn short_host(host: Option<&str>) -> &str {
    let host = host.unwrap_or("");
    match host.split_once('.') {
        Some((short, _)) => short,
        None => host,
    }
}

#[cfg(unix)]
fn is_root() -> bool {
    // SAFETY: `geteuid` reads a process property and cannot fail.
    unsafe { libc::geteuid() == 0 }
}

#[cfg(not(unix))]
fn is_root() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{PromptContext, render_template, wants_git_branch};

    fn ctx<'a>(cwd: &'a Path, branch: Option<&'a str>) -> PromptContext<'a> {
        PromptContext {
            cwd,
            home: Some("/home/ubu"),
            user: Some("ubu"),
            host: Some("laptop.local"),
            last_status: 3,
            git_branch: branch,
        }
    }

    #[test]
    fn renders_the_common_ps1_escapes() {
        let cwd = Path::new("/home/ubu/src/ush");
        let rendered = render_template("\\u@\\h \\w \\? ", &ctx(cwd, None));

        assert_eq!(rendered, "ubu@laptop ~/src/ush 3 ");
    }

    #[test]
    fn renders_the_branch_and_drops_ps1_fences() {
        let cwd = Path::new("/home/ubu/src/ush");
        let rendered =
            render_template("\\[\\e[32m\\]\\W(\\g)\\[\\e[0m\\]", &ctx(cwd, Some("main")));

        assert_eq!(rendered, "\u{1b}[32mush(main)\u{1b}[0m");
    }

    #[test]
    fn leaves_a_plain_template_untouched() {
        let cwd = Path::new("/home/ubu");
        assert_eq!(render_template("ush> ", &ctx(cwd, None)), "ush> ");
    }

    #[test]
    fn only_reports_a_branch_request_for_a_real_escape() {
        assert!(wants_git_branch("\\w \\g $ "));
        assert!(!wants_git_branch("groovy \\w $ "));
    }
}
