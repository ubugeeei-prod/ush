//! `std::io` sources and sinks: pipeline stages that lower to redirections.
use super::{builtin, param};
use crate::{
    ast::{FunctionDef, Type},
    types::HeapVec as Vec,
};

pub(crate) const FROM_FILE: &str = "std::io::from_file";
pub(crate) const TO_FILE: &str = "std::io::to_file";
pub(crate) const APPEND_TO: &str = "std::io::append_to";

pub(super) fn definitions() -> Vec<FunctionDef> {
    [FROM_FILE, TO_FILE, APPEND_TO]
        .into_iter()
        .map(|name| builtin(name, vec![param("path", Type::String)], Some(Type::Unit)))
        .collect()
}

/// Why an endpoint cannot be called on its own, or `None` for other names.
pub(crate) fn misuse(name: &str) -> Option<&'static str> {
    match name {
        FROM_FILE => Some(
            "`from_file` is a pipeline source; use it as the first stage: `from_file path | command`",
        ),
        TO_FILE | APPEND_TO => Some(
            "`to_file` and `append_to` are pipeline sinks; use one as the last stage: \
             `command | to_file path`",
        ),
        _ => None,
    }
}
