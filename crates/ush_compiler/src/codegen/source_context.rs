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
    #[cfg(feature = "std")]
    CheckedIn {
        source_name: &'a str,
        output_name: &'a str,
    },
}

impl SourceContext<'_> {
    pub(crate) const fn none() -> Self {
        Self::None(PhantomData)
    }
}

pub(super) fn emit(
    out: &mut OutputBuffer,
    context: SourceContext<'_>,
    target: crate::ShellTarget,
    tasks: bool,
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
            out.push_str("__ush_entrypoint=$0\ncase $__ush_entrypoint in\n  */*) ;;\n  *) __ush_entrypoint=$(command -v \"$__ush_entrypoint\") ;;\nesac\n__ush_source_dir=$(CDPATH= cd -P \"$(dirname \"$__ush_entrypoint\")\" && pwd -P)\n");
            out.push_str(&format!(
                "__ush_source_path=$__ush_source_dir/{}\n",
                shell_quote(source_name)
            ));
        }
    }
    out.push_str(&format!("__ush_origin={}\n", shell_quote(name)));
    out.push_str(r#"
__ush_at() { __ush_line=$1; __ush_source=$2; }
__ush_report() {
  if [ "$1" -ne 0 ] && [ -n "${__ush_line:-}" ]; then
    printf '%s:%s: %s (exit %s)\n  %s\n' "$__ush_origin" "$__ush_line" "${2:-command failed}" "$1" "$__ush_source" >&2
    __ush_line=''
  fi
}
"#);
    if tasks {
        out.push_str(include_str!("task_runtime.sh"));
    } else {
        out.push_str("trap '__ush_report \"$?\"' 0\n\n");
    }
}
