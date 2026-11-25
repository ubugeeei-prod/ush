use core::marker::PhantomData;

use crate::{sourcemap::OutputBuffer, util::shell_quote};

pub(crate) enum SourceContext<'a> {
    None(PhantomData<&'a ()>),
    #[cfg(feature = "std")]
    Absolute {
        directory: &'a str,
        origin: &'a str,
        path: &'a str,
    },
    /// A committed artifact. Both paths are relative to the output's
    /// directory, so the script finds its source from wherever it sits.
    #[cfg(feature = "std")]
    CheckedIn {
        source_name: &'a str,
        output_name: &'a str,
    },
}

/// Which runtime support the prelude carries.
pub(crate) struct Prelude {
    pub tasks: bool,
    pub literal_newlines: bool,
    /// Statements from module files name their file in failure reports.
    pub modules: bool,
}

impl SourceContext<'_> {
    pub(crate) const fn none() -> Self {
        Self::None(PhantomData)
    }
}

/// Statement tracking and the failure report every script carries.
const REPORT: &str = r#"
__ush_at() { __ush_line=$1; __ush_source=$2; }
__ush_report() {
  if [ "$1" -ne 0 ] && [ -n "${__ush_line:-}" ]; then
    __ush_message=${2:-command failed}
    if [ "$#" -lt 2 ]; then
      case "$1" in
        127) __ush_message='command not found; check the command name and PATH' ;;
        126) __ush_message='command cannot be executed; check permissions and interpreter' ;;
        130) __ush_message='interrupted' ;;
        137) __ush_message='process killed (SIGKILL)' ;;
        143) __ush_message='terminated (SIGTERM)' ;;
      esac
    fi
    printf '%s:%s: %s (exit %s)\n  %s\n' "$__ush_origin" "$__ush_line" "$__ush_message" "$1" "$__ush_source" >&2
    __ush_line=''
  fi
}
"#;

pub(super) fn emit(
    out: &mut OutputBuffer,
    context: SourceContext<'_>,
    target: crate::ShellTarget,
    prelude: Prelude,
) {
    use crate::ShellTarget;
    if target == ShellTarget::Sh {
        out.push_str("#!/bin/sh\n");
    } else {
        out.push_str(&format!("#!/usr/bin/env {target}\n"));
    }
    let name = match &context {
        SourceContext::None(_) => "<memory>",
        #[cfg(feature = "std")]
        SourceContext::Absolute { origin, .. } => origin,
        #[cfg(feature = "std")]
        SourceContext::CheckedIn { source_name, .. } => source_name,
    };
    out.push_str("# Generated from ");
    out.push_str(&shell_quote(
        &name.replace('\r', "\\r").replace('\n', "\\n"),
    ));
    out.push_str(" by ush.\n# DO NOT EDIT MANUALLY.\n");
    out.push_str("# Source: ");
    let source = match &context {
        #[cfg(feature = "std")]
        SourceContext::Absolute { path, .. } => {
            out.push_str("file://");
            *path
        }
        _ => name,
    };
    for byte in source.bytes() {
        if byte.is_ascii_alphanumeric() || b"/-_.~".contains(&byte) {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out.push_str("\n# Source map: embedded __ush_at markers; ush explain SOURCE LINE\n");
    #[cfg(feature = "std")]
    if let SourceContext::CheckedIn {
        source_name,
        output_name,
    } = &context
    {
        out.push_str(&format!(
            "# Regenerate with: ush compile {} --checked-in",
            shell_quote(source_name)
        ));
        if target != ShellTarget::Sh {
            out.push_str(&format!(" --target {target}"));
        }
        out.push_str(&format!(" -o {}\n", shell_quote(output_name)));
    }
    if target == ShellTarget::Zsh {
        out.push_str("if [ -n \"${ZSH_VERSION:-}\" ] && [ \"$(emulate)\" != sh ]; then exec zsh --emulate sh \"$0\" \"$@\"; fi\n");
    }
    out.push_str("set -eu\n\n");
    if prelude.literal_newlines {
        out.push_str("__ush_lf='\n'\n");
    }
    match context {
        SourceContext::None(_) => {
            out.push_str("__ush_source_dir=''\n__ush_source_path='<memory>'\n")
        }
        #[cfg(feature = "std")]
        SourceContext::Absolute {
            directory, path, ..
        } => {
            out.push_str(&format!(
                "__ush_source_dir={}\n__ush_source_path={}\n",
                shell_quote(directory),
                shell_quote(path)
            ));
        }
        #[cfg(feature = "std")]
        SourceContext::CheckedIn { source_name, .. } => {
            out.push_str("__ush_entrypoint=$0\ncase $__ush_entrypoint in\n  */*) ;;\n  *) if [ -f \"$__ush_entrypoint\" ]; then __ush_entrypoint=./$__ush_entrypoint; else __ush_entrypoint=$(command -v \"$__ush_entrypoint\"); fi ;;\nesac\n");
            // Source-relative lookups resolve where the source lives,
            // which need not be beside the generated script.
            let (directory, file) = match source_name.rsplit_once('/') {
                Some((directory, file)) => (format!("/{}", shell_quote(directory)), file),
                None => (alloc::string::String::new(), source_name),
            };
            out.push_str(&format!(
                "__ush_source_dir=$(CDPATH='' cd -P \"$(dirname \"$__ush_entrypoint\")\"{directory} && pwd -P)\n__ush_source_path=$__ush_source_dir/{}\n",
                shell_quote(file)
            ));
        }
    }
    out.push_str(&format!("__ush_origin={}\n", shell_quote(name)));
    if prelude.modules {
        // Module statements pass their file as a third argument.
        out.push_str(
            &REPORT
                .replace(
                    "__ush_source=$2; }",
                    "__ush_source=$2; __ush_file=${3:-$__ush_origin}; }",
                )
                .replace(
                    "\"$__ush_origin\" \"$__ush_line\"",
                    "\"${__ush_file:-$__ush_origin}\" \"$__ush_line\"",
                ),
        );
    } else {
        out.push_str(REPORT);
    }
    if prelude.tasks {
        out.push_str(include_str!("task_runtime.sh"));
        out.push('\n');
    } else {
        out.push_str("trap '__ush_report \"$?\"' 0\n\n");
    }
}
