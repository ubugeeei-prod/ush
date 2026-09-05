use std::{collections::BTreeMap, path::Path};

use ush_compiler::{CompiledScript, SourceMapLine, SourceMapSection};

/// A compiled script plus the diagnostics scaffolding wrapped around
/// it.
///
/// `header_lines` is the whole reason this is a struct rather than a
/// `String`: `/bin/sh` numbers its error messages against the text it
/// is actually running, which is the instrumentation header followed
/// by the generated shell. Knowing how tall that header is turns a
/// bare `sh: line 495:` back into a sourcemap entry, and from there
/// into a `.ush` line.
pub struct InstrumentedScript {
    pub text: String,
    pub header_lines: usize,
}

impl InstrumentedScript {
    /// The line `/bin/sh` will report for a generated shell line.
    pub fn shell_line(&self, generated_line: usize) -> usize {
        generated_line + self.header_lines
    }

    /// The inverse: the generated shell line behind a line number
    /// `/bin/sh` printed. `None` when the number lands inside the
    /// instrumentation header itself.
    pub fn generated_line(&self, shell_line: usize) -> Option<usize> {
        shell_line
            .checked_sub(self.header_lines)
            .filter(|line| *line > 0)
    }
}

pub fn instrument_compiled_script(origin: &Path, compiled: &CompiledScript) -> InstrumentedScript {
    // The header prints the offset it is itself responsible for, so
    // render it once to measure it and once for real. Only the digits
    // of a number change between the two, never the line count.
    let header_lines = render_header(origin, 0).matches('\n').count();
    let mut out = render_header(origin, header_lines);
    debug_assert_eq!(out.matches('\n').count(), header_lines);

    let generated_groups = compiled
        .sourcemap
        .source_index()
        .into_iter()
        .map(|line| {
            (
                line.source_line,
                format_generated_lines(&line.generated_lines),
            )
        })
        .collect::<BTreeMap<_, _>>();

    let mut quote_state = ShellQuoteState::default();
    for line in &compiled.sourcemap.lines {
        let started_inside_multiline_literal = quote_state.is_open();
        quote_state.observe(&line.generated_text);
        let touches_multiline_literal = started_inside_multiline_literal || quote_state.is_open();

        if line.section == SourceMapSection::UserCode
            && !touches_multiline_literal
            && should_inline_track(&line.generated_text)
        {
            let mapped = line
                .source_line
                .and_then(|source_line| generated_groups.get(&source_line))
                .map(String::as_str)
                .unwrap_or("");
            append_tracking_prefix(&mut out, line, mapped);
            out.push_str("; ");
        }
        out.push_str(&line.generated_text);
        out.push('\n');
    }

    InstrumentedScript {
        text: out,
        header_lines,
    }
}

/// The tracking state, the EXIT trap, and the report it prints.
///
/// `offset` is how many lines this header occupies, so the report can
/// translate a sourcemap `G` id into the line number `/bin/sh` uses
/// in its own messages — the number the user is actually looking at
/// when a generated script fails.
fn render_header(origin: &Path, offset: usize) -> String {
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

fn append_tracking_prefix(out: &mut String, line: &SourceMapLine, mapped: &str) {
    out.push_str("__ush_runtime_map_track ");
    out.push_str(&shell_quote(&line.generated_line.to_string()));
    out.push(' ');
    out.push_str(&shell_quote(line.section.label()));
    out.push(' ');
    out.push_str(&shell_quote(
        &line
            .source_line
            .map(|value| value.to_string())
            .unwrap_or_default(),
    ));
    out.push(' ');
    out.push_str(&shell_quote(line.source_text.as_deref().unwrap_or("")));
    out.push(' ');
    out.push_str(&shell_quote(&line.generated_text));
    out.push(' ');
    out.push_str(&shell_quote(mapped));
}

fn should_inline_track(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return false;
    }
    if matches!(
        trimmed,
        "then" | "do" | "else" | "fi" | "done" | "esac" | "}" | ";;"
    ) {
        return false;
    }
    if trimmed.starts_with("elif ") || trimmed == "elif" {
        return false;
    }
    if trimmed.starts_with('}') {
        return false;
    }
    if trimmed.contains(')') && trimmed.ends_with(";;") {
        return false;
    }
    if trimmed.ends_with(')') && !trimmed.contains('(') {
        return false;
    }
    true
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

fn format_generated_lines(lines: &[usize]) -> String {
    lines
        .iter()
        .map(|line| format!("G{line:04}"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[derive(Clone, Copy, Debug, Default)]
struct ShellQuoteState {
    in_single: bool,
    in_double: bool,
    escaped: bool,
}

impl ShellQuoteState {
    fn is_open(self) -> bool {
        self.in_single || self.in_double
    }

    fn observe(&mut self, line: &str) {
        for ch in line.chars() {
            if self.in_single {
                if ch == '\'' {
                    self.in_single = false;
                }
                continue;
            }

            if self.escaped {
                self.escaped = false;
                continue;
            }

            match ch {
                '\\' if self.in_double => self.escaped = true,
                '"' if self.in_double => self.in_double = false,
                '"' => self.in_double = true,
                '\'' => self.in_single = true,
                _ => {}
            }
        }
        self.escaped = false;
    }
}

#[cfg(test)]
mod tests;
