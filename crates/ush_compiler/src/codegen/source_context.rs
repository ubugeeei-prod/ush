use core::marker::PhantomData;

use crate::{sourcemap::OutputBuffer, util::shell_quote};

pub(crate) enum SourceContext<'a> {
    None(PhantomData<&'a ()>),
    #[cfg(feature = "std")]
    Absolute {
        directory: &'a str,
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

pub(super) fn emit(out: &mut OutputBuffer, context: SourceContext<'_>) {
    match context {
        SourceContext::None(_) => emit_absolute(out, "", ""),
        #[cfg(feature = "std")]
        SourceContext::Absolute { directory, path } => emit_absolute(out, directory, path),
        #[cfg(feature = "std")]
        SourceContext::CheckedIn {
            source_name,
            output_name,
        } => emit_checked_in(out, source_name, output_name),
    }
}

fn emit_absolute(out: &mut OutputBuffer, directory: &str, path: &str) {
    out.push_str("#!/bin/sh\nset -eu\n\n__ush_jobs=''\n__ush_task_seq='0'\n__ush_task_files=''\n__ush_tmp_seq='0'\n\n");
    out.push_str("__ush_source_dir=");
    out.push_str(&shell_quote(directory));
    out.push('\n');
    out.push_str("__ush_source_path=");
    out.push_str(&shell_quote(path));
    out.push_str("\n\n");
}

#[cfg(feature = "std")]
fn emit_checked_in(out: &mut OutputBuffer, source_name: &str, output_name: &str) {
    out.push_str("#!/bin/sh\n# Generated from ");
    out.push_str(&shell_quote(source_name));
    out.push_str(" by ush.\n# DO NOT EDIT MANUALLY.\n# Regenerate with: ush compile ");
    out.push_str(&shell_quote(source_name));
    out.push_str(" --checked-in -o ");
    out.push_str(&shell_quote(output_name));
    out.push_str("\n\nset -eu\n\n__ush_jobs=''\n__ush_task_seq='0'\n__ush_task_files=''\n__ush_tmp_seq='0'\n\n");
    out.push_str(
        "__ush_entrypoint=$0\ncase $__ush_entrypoint in\n  */*) ;;\n  *) __ush_entrypoint=$(command -v \"$__ush_entrypoint\" 2>/dev/null || printf '%s' \"$__ush_entrypoint\") ;;\nesac\n__ush_source_dir=$(\n  CDPATH= cd -P \"$(dirname \"$__ush_entrypoint\")\" >/dev/null 2>&1\n  pwd -P\n)\n__ush_source_path=$__ush_source_dir/",
    );
    out.push_str(&shell_quote(source_name));
    out.push_str("\nunset __ush_entrypoint\n\n");
}
