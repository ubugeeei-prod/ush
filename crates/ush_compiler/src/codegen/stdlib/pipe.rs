//! `std::io` sources and sinks: pipeline stages that lower to redirections.
use super::{builtin, param};
use crate::{
    ast::{FunctionDef, Type},
    types::HeapVec as Vec,
};

pub(crate) const FROM_FILE: &str = "std::io::from_file";
pub(crate) const TO_FILE: &str = "std::io::to_file";
pub(crate) const APPEND_TO: &str = "std::io::append_to";
pub(crate) const TO_STDERR: &str = "std::io::to_stderr";
pub(crate) const STDERR_TO: &str = "std::io::stderr_to";
pub(crate) const STDERR_APPEND_TO: &str = "std::io::stderr_append_to";
pub(crate) const DISCARD_STDERR: &str = "std::io::discard_stderr";

pub(super) fn definitions() -> Vec<FunctionDef> {
    let mut defs: Vec<_> = [FROM_FILE, TO_FILE, APPEND_TO, STDERR_TO, STDERR_APPEND_TO]
        .into_iter()
        .map(|name| builtin(name, vec![param("path", Type::String)], Some(Type::Unit)))
        .collect();
    for name in [TO_STDERR, DISCARD_STDERR] {
        defs.push(builtin(name, vec![], Some(Type::Unit)));
    }
    defs
}

/// A stage that lowers to a redirection on its neighbour, not a process.
pub(crate) fn is_redirection(name: &str) -> bool {
    misuse(name).is_some()
}

/// Why an endpoint cannot be called on its own, or `None` for other names.
pub(crate) fn misuse(name: &str) -> Option<&'static str> {
    match name {
        FROM_FILE => Some(
            "`from_file` is a pipeline source; use it as the first stage: `from_file path | command`",
        ),
        TO_FILE | APPEND_TO | TO_STDERR => Some(
            "`to_file`, `append_to` and `to_stderr` are pipeline sinks; use one as the last stage: \
             `command | to_file path`",
        ),
        STDERR_TO | STDERR_APPEND_TO | DISCARD_STDERR => Some(
            "`stderr_to`, `stderr_append_to` and `discard_stderr` redirect the stderr of the stage \
             before them; use one after a command: `command | discard_stderr`",
        ),
        _ => None,
    }
}
