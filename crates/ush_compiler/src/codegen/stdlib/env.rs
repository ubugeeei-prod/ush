use crate::ast::{FunctionDef, Type};
use crate::types::HeapVec as Vec;

use super::{builtin, param};

pub(super) fn definitions() -> Vec<FunctionDef> {
    vec![
        builtin(
            "std::env::contains",
            vec![param("name", Type::String)],
            Some(Type::Bool),
        ),
        builtin(
            "std::env::get",
            vec![param("name", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::env::get_or",
            vec![param("name", Type::String), param("default", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::env::set",
            vec![param("name", Type::String), param("value", Type::String)],
            Some(Type::Unit),
        ),
        builtin(
            "std::env::unset",
            vec![param("name", Type::String)],
            Some(Type::Unit),
        ),
    ]
}

pub(super) fn runs_in_parent(name: &str) -> bool {
    matches!(name, "std::env::set" | "std::env::unset")
}
