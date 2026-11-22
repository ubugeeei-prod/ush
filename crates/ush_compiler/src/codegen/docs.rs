use alloc::collections::BTreeSet;

use crate::{
    ScriptDocs, sourcemap::OutputBuffer, types::OutputString as String, util::shell_quote,
};

pub(crate) fn push_doc_support(
    out: &mut OutputBuffer,
    docs: &ScriptDocs,
    script_name: &str,
    extra_completion: &[String],
) {
    if docs.is_empty() && extra_completion.is_empty() {
        out.push_str("case \"${1:-}\" in\n  -h|--help) printf '%s' ");
        out.push_str(&shell_quote(&docs.render_help(script_name)));
        out.push_str("; exit 0 ;;\n  man|--man)\n    if [ -n \"${2:-}\" ]; then printf 'No documented item named: %s\\n' \"$2\" >&2; exit 1; fi\n    printf '%s' ");
        out.push_str(&shell_quote(&docs.render_man(script_name, None)));
        out.push_str("; exit 0 ;;\n  complete|--complete)\n    for item in --complete --help --man -h complete man; do\n      case $item in \"${2:-}\"*) printf '%s\\n' \"$item\" ;; esac\n    done\n    exit 0 ;;\nesac\n\n");
        return;
    }
    push_simple_doc_function(out, "__ush_print_help", &docs.render_help(script_name));
    push_man_function(out, docs, script_name);
    push_complete_function(out, docs, extra_completion);
    out.push_str("case \"${1:-}\" in\n");
    out.push_str("  -h|--help)\n");
    out.push_str("    __ush_print_help\n");
    out.push_str("    exit 0\n");
    out.push_str("    ;;\n");
    out.push_str("  man|--man)\n");
    out.push_str("    shift\n");
    out.push_str("    __ush_print_man \"${1:-}\"\n");
    out.push_str("    exit $?\n");
    out.push_str("    ;;\n");
    out.push_str("  complete|--complete)\n");
    out.push_str("    shift\n");
    out.push_str("    __ush_complete \"${1:-}\"\n");
    out.push_str("    exit 0\n");
    out.push_str("    ;;\n");
    out.push_str("esac\n\n");
}

fn push_simple_doc_function(out: &mut OutputBuffer, name: &str, body: &str) {
    out.push_str(name);
    out.push_str("() {\n");
    out.push_str("  cat <<'__USH_DOC__'\n");
    out.push_str(body);
    if !body.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("__USH_DOC__\n");
    out.push_str("}\n\n");
}

fn push_man_function(out: &mut OutputBuffer, docs: &ScriptDocs, script_name: &str) {
    out.push_str("__ush_print_man() {\n");
    out.push_str("  case \"${1:-}\" in\n");
    push_here_doc_case(out, "", &docs.render_man(script_name, None), "MAIN");
    for item in docs.items() {
        push_here_doc_case(
            out,
            item.name(),
            &docs.render_man(script_name, Some(item.name())),
            item.name(),
        );
    }
    out.push_str("    *)\n");
    out.push_str("      printf '%s\\n' \"No documented item named: ${1}\" >&2\n");
    out.push_str("      return 1\n");
    out.push_str("      ;;\n");
    out.push_str("  esac\n");
    out.push_str("}\n\n");
}

fn push_here_doc_case(out: &mut OutputBuffer, key: &str, body: &str, marker: &str) {
    let label = sanitize_marker(marker);
    out.push_str("    ");
    out.push_str(&shell_quote(key));
    out.push_str(")\n");
    out.push_str("      cat <<'__USH_");
    out.push_str(&label);
    out.push_str("__'\n");
    out.push_str(body);
    if !body.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("__USH_");
    out.push_str(&label);
    out.push_str("__\n");
    out.push_str("      ;;\n");
}

fn push_complete_function(out: &mut OutputBuffer, docs: &ScriptDocs, extra_completion: &[String]) {
    let mut values = BTreeSet::new();
    for item in docs.completion_candidates() {
        values.insert(item);
    }
    for item in extra_completion {
        values.insert(item.clone());
    }
    out.push_str("__ush_complete() {\n");
    out.push_str("  __ush_prefix=\"${1:-}\"\n");
    out.push_str("  while IFS= read -r __ush_item; do\n");
    out.push_str("    case \"$__ush_item\" in\n");
    out.push_str("      \"$__ush_prefix\"*) printf '%s\\n' \"$__ush_item\" ;;\n");
    out.push_str("    esac\n");
    out.push_str("  done <<'__USH_COMPLETE__'\n");
    for item in values {
        out.push_str(&item);
        out.push('\n');
    }
    out.push_str("__USH_COMPLETE__\n");
    out.push_str("}\n\n");
}

fn sanitize_marker(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch == '_' || ch.is_ascii_alphanumeric() {
                ch.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}
