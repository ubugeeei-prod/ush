//! The instrumentation header: tracking state, the EXIT trap, and
//! the failure report it prints.

use std::path::Path;

use super::shell_quote;

/// The tracking state, the EXIT trap, and the report it prints.
///
/// `offset` is how many lines this header occupies, so the report can
/// translate a sourcemap `G` id into the line number `/bin/sh` uses
/// in its own messages — the number the user is actually looking at
/// when a generated script fails.
pub(super) fn render_header(origin: &Path, offset: usize) -> String {
    let mut out = String::new();
    out.push_str("__ush_runtime_map_origin=");
    out.push_str(&shell_quote(&origin.display().to_string()));
    out.push('\n');
    out.push_str(&format!("__ush_runtime_map_offset='{offset}'\n"));
    out.push_str("__ush_runtime_map_generated=''\n");
    out.push_str("__ush_runtime_map_section=''\n");
    out.push_str("__ush_runtime_map_source_line=''\n");
    out.push_str("__ush_runtime_map_source=''\n");
    out.push_str("__ush_runtime_map_shell=''\n");
    out.push_str("__ush_runtime_map_mapped=''\n");
    out.push('\n');
    out.push_str("__ush_runtime_map_track() {\n");
    out.push_str("  __ush_runtime_map_generated=\"$1\"\n");
    out.push_str("  __ush_runtime_map_section=\"$2\"\n");
    out.push_str("  __ush_runtime_map_source_line=\"$3\"\n");
    out.push_str("  __ush_runtime_map_source=\"$4\"\n");
    out.push_str("  __ush_runtime_map_shell=\"$5\"\n");
    out.push_str("  __ush_runtime_map_mapped=\"$6\"\n");
    out.push_str("}\n");
    out.push('\n');
    out.push_str("__ush_runtime_map_report() {\n");
    out.push_str("  __ush_runtime_map_status=\"$1\"\n");
    out.push_str("  [ \"$__ush_runtime_map_status\" -eq 0 ] && return 0\n");
    out.push_str(
        "  [ -z \"$__ush_runtime_map_generated\" ] && return \"$__ush_runtime_map_status\"\n",
    );
    out.push_str(
        "  __ush_runtime_map_line=$((__ush_runtime_map_generated + __ush_runtime_map_offset))\n",
    );
    out.push_str("  if [ -n \"$__ush_runtime_map_source_line\" ]; then\n");
    out.push_str(
        "    printf '\\nush runtime map: %s:%s (exit %s)\\n' \"$__ush_runtime_map_origin\" \"$__ush_runtime_map_source_line\" \"$__ush_runtime_map_status\" >&2\n",
    );
    out.push_str("    printf '  source : %s\\n' \"$__ush_runtime_map_source\" >&2\n");
    out.push_str("  else\n");
    out.push_str(
        "    printf '\\nush runtime map: %s (exit %s)\\n' \"$__ush_runtime_map_origin\" \"$__ush_runtime_map_status\" >&2\n",
    );
    out.push_str("    printf '  source : (no direct source mapping)\\n' >&2\n");
    out.push_str("  fi\n");
    out.push_str("  printf '  section: %s\\n' \"$__ush_runtime_map_section\" >&2\n");
    // `line N` is the number `/bin/sh` puts in its own diagnostics,
    // and `G####` is the sourcemap id — printing both is what lets a
    // reader connect the shell's complaint to the listing.
    out.push_str(
        "  printf '  shell  : line %s | G%04d | %s\\n' \"$__ush_runtime_map_line\" \"$__ush_runtime_map_generated\" \"$__ush_runtime_map_shell\" >&2\n",
    );
    out.push_str("  printf '  mapped : %s\\n' \"$__ush_runtime_map_mapped\" >&2\n");
    out.push_str(
        "  printf '  explain: ush explain %s %s\\n' \"$__ush_runtime_map_origin\" \"$__ush_runtime_map_line\" >&2\n",
    );
    // A shell that dies on `set -u` / `set -e` rather than an
    // explicit `exit` takes its status from the last command in the
    // EXIT trap, so the handler has to hand the original status back
    // or every hard failure would be reported as success.
    out.push_str("  return \"$__ush_runtime_map_status\"\n");
    out.push_str("}\n");
    out.push('\n');
    out.push_str("trap '__ush_runtime_map_report \"$?\"' 0\n");
    out.push('\n');
    out
}
