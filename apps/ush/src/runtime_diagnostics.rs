use std::{collections::BTreeMap, path::Path};

use ush_compiler::{CompiledScript, SourceMapLine, SourceMapSection};

pub fn instrument_compiled_script(origin: &Path, compiled: &CompiledScript) -> String {
    let mut out = String::new();
    let mut quote_state = ShellQuoteState::default();
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
    out.push_str("__ush_runtime_map_origin=");
    out.push_str(&shell_quote(&origin.display().to_string()));
    out.push('\n');
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
    out.push_str("  if [ -n \"$__ush_runtime_map_source_line\" ]; then\n");
    out.push_str(
        "    printf '\\nush runtime map: %s:%s\\n' \"$__ush_runtime_map_origin\" \"$__ush_runtime_map_source_line\" >&2\n",
    );
    out.push_str("    printf '  section: %s\\n' \"$__ush_runtime_map_section\" >&2\n");
    out.push_str(
        "    printf '  shell  : G%04d | %s\\n' \"$__ush_runtime_map_generated\" \"$__ush_runtime_map_shell\" >&2\n",
    );
    out.push_str("    printf '  source : %s\\n' \"$__ush_runtime_map_source\" >&2\n");
    out.push_str("    printf '  mapped : %s\\n' \"$__ush_runtime_map_mapped\" >&2\n");
    out.push_str("  elif [ -n \"$__ush_runtime_map_generated\" ]; then\n");
    out.push_str("    printf '\\nush runtime map: %s\\n' \"$__ush_runtime_map_origin\" >&2\n");
    out.push_str("    printf '  section: %s\\n' \"$__ush_runtime_map_section\" >&2\n");
    out.push_str(
        "    printf '  shell  : G%04d | %s\\n' \"$__ush_runtime_map_generated\" \"$__ush_runtime_map_shell\" >&2\n",
    );
    out.push_str("    printf '  source : (no direct source mapping)\\n' >&2\n");
    out.push_str("  fi\n");
    // A shell that dies on `set -u` / `set -e` rather than an
    // explicit `exit` takes its status from the last command in the
    // EXIT trap, so the handler has to hand the original status back
    // or every hard failure would be reported as success.
    out.push_str("  return \"$__ush_runtime_map_status\"\n");
    out.push_str("}\n");
    out.push('\n');
    out.push_str("trap '__ush_runtime_map_report \"$?\"' 0\n");
    out.push('\n');

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
