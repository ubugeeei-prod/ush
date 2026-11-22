use crate::ast::{FunctionDef, Type};
use crate::types::HeapVec as Vec;

use super::{builtin, param};

pub(super) fn definitions() -> Vec<FunctionDef> {
    vec![
        builtin(
            "std::regex::is_match",
            vec![param("value", Type::String), param("pattern", Type::String)],
            Some(Type::Bool),
        ),
        builtin(
            "std::regex::find",
            vec![param("value", Type::String), param("pattern", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::regex::replace",
            vec![
                param("value", Type::String),
                param("pattern", Type::String),
                param("replacement", Type::String),
            ],
            Some(Type::String),
        ),
        builtin(
            "std::regex::capture",
            vec![
                param("value", Type::String),
                param("pattern", Type::String),
                param("index", Type::Int),
            ],
            Some(Type::String),
        ),
    ]
}
