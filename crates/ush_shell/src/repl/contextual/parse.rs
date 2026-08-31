#[cfg(test)]
mod tests;

use bumpalo::{Bump, collections::String as BumpString};
use compact_str::CompactString;
use memchr::{memchr, memmem};
use rustc_hash::FxHashSet;
use serde_json::Value as JsonValue;
use toml::Value as TomlValue;

use super::types::Names;

pub(crate) fn parse_make_targets(source: &str) -> Names {
    let arena = Bump::new();
    let mut logical = BumpString::new_in(&arena);
    let mut seen = FxHashSet::default();
    let mut names = Names::new();

    let mut in_define = false;

    for raw in source.lines() {
        let line = raw.trim_end();
        if let Some(prefix) = line.strip_suffix('\\') {
            logical.push_str(prefix);
            logical.push(' ');
            continue;
        }
        logical.push_str(line);
        let current = logical.trim();
        // A `define ... endef` block holds recipe text, not rules, so
        // its `name: value` lines must not become completions.
        if in_define {
            in_define = !current.starts_with("endef");
        } else if current.starts_with("define ") {
            in_define = true;
        } else if keep_make_rule(raw, current)
            && let Some(colon) = memchr(b':', current.as_bytes())
        {
            push_make_targets(&current[..colon], &mut seen, &mut names);
        }
        logical.clear();
    }

    names.sort_unstable();
    names
}

pub(crate) fn parse_just_recipes(source: &str) -> Names {
    let mut seen = FxHashSet::default();
    let mut names = Names::new();

    for raw in source.lines() {
        if raw.starts_with(char::is_whitespace) {
            continue;
        }
        let line = raw.trim();
        if skip_just_line(line) {
            continue;
        }
        if let Some(colon) = memchr(b':', line.as_bytes()) {
            let head = &line[..colon];
            if let Some(name) = head.split_ascii_whitespace().next()
                && valid_just_name(name)
            {
                push_unique(name, &mut seen, &mut names);
            }
        }
    }

    names.sort_unstable();
    names
}

pub(crate) fn parse_mise_toml_tasks(source: &str) -> Names {
    let Ok(toml) = toml::from_str::<TomlValue>(source) else {
        return Names::new();
    };
    let Some(table) = toml.get("tasks").and_then(TomlValue::as_table) else {
        return Names::new();
    };

    let mut names = Names::new();
    for (name, value) in table {
        if value.is_table() || value.is_str() || value.is_array() {
            names.push(CompactString::from(name.as_str()));
        }
    }
    names.sort_unstable();
    names.dedup();
    names
}

pub(crate) fn package_json_scripts(json: &JsonValue) -> Names {
    let Some(scripts) = json.get("scripts").and_then(JsonValue::as_object) else {
        return Names::new();
    };
    let mut names = Names::new();
    for name in scripts.keys() {
        names.push(CompactString::from(name.as_str()));
    }
    names.sort_unstable();
    names
}

pub(crate) fn package_json_has_name(json: &JsonValue, name: &str) -> bool {
    [
        "dependencies",
        "devDependencies",
        "optionalDependencies",
        "peerDependencies",
    ]
    .into_iter()
    .filter_map(|field| json.get(field).and_then(JsonValue::as_object))
    .any(|table| table.contains_key(name))
}

pub(crate) fn package_json_uses_vp(json: &JsonValue) -> bool {
    json.get("scripts")
        .and_then(JsonValue::as_object)
        .into_iter()
        .flat_map(|scripts| scripts.values())
        .filter_map(JsonValue::as_str)
        .any(|value| matches!(value.split_ascii_whitespace().next(), Some("vite" | "vp")))
}

fn keep_make_rule(raw: &str, current: &str) -> bool {
    !current.is_empty()
        && !current.starts_with('#')
        && !raw.starts_with(char::is_whitespace)
        && !current.starts_with("define ")
        && !contains_make_assignment(current)
}

fn contains_make_assignment(source: &str) -> bool {
    [b":=", b"?=", b"+=", b"!="]
        .into_iter()
        .any(|needle| memmem::find(source.as_bytes(), needle).is_some())
}

fn push_make_targets(head: &str, seen: &mut FxHashSet<CompactString>, names: &mut Names) {
    if memchr(b'=', head.as_bytes()).is_some() {
        return;
    }
    for name in head.split_ascii_whitespace() {
        if !name.is_empty() && !name.starts_with('.') && memchr(b'%', name.as_bytes()).is_none() {
            push_unique(name, seen, names);
        }
    }
}

fn skip_just_line(line: &str) -> bool {
    line.is_empty()
        || line.starts_with('#')
        || line.starts_with('[')
        || line.starts_with("alias ")
        || line.starts_with("set ")
        || line.starts_with("export ")
        || line.starts_with("import ")
        || line.starts_with("mod ")
        || memmem::find(line.as_bytes(), b":=").is_some()
}

fn valid_just_name(name: &str) -> bool {
    name.bytes().all(
        |byte| matches!(byte, b'_' | b'-' | b':' | b'/' | b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z'),
    )
}

pub(crate) fn push_unique(value: &str, seen: &mut FxHashSet<CompactString>, names: &mut Names) {
    let value = CompactString::from(value);
    if seen.insert(value.clone()) {
        names.push(value);
    }
}
