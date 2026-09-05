//! Leading `NAME=value` assignments, and what counts as a name.

pub(super) fn split_assignments(tokens: Vec<String>) -> (Vec<(String, String)>, Vec<String>) {
    let mut assignments = Vec::new();
    let mut rest = Vec::new();
    let mut assigning = true;

    for token in tokens {
        if assigning && is_assignment(&token) {
            if let Some((name, value)) = token.split_once('=') {
                assignments.push((name.to_string(), value.to_string()));
            }
            continue;
        }

        assigning = false;
        rest.push(token);
    }

    (assignments, rest)
}

pub(super) fn is_assignment(token: &str) -> bool {
    let Some((name, _)) = token.split_once('=') else {
        return false;
    };
    is_identifier(name)
}

pub(super) fn is_identifier(source: &str) -> bool {
    let mut chars = source.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}
