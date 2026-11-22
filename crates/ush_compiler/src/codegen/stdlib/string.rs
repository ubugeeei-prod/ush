use crate::ast::{FunctionDef, Type};
use crate::types::HeapVec as Vec;

use super::{builtin, param};

pub(super) fn definitions() -> Vec<FunctionDef> {
    vec![
        builtin(
            "std::string::starts_with",
            vec![param("value", Type::String), param("prefix", Type::String)],
            Some(Type::Bool),
        ),
        builtin(
            "std::string::ends_with",
            vec![param("value", Type::String), param("suffix", Type::String)],
            Some(Type::Bool),
        ),
        builtin(
            "std::string::replace",
            vec![
                param("value", Type::String),
                param("from", Type::String),
                param("to", Type::String),
            ],
            Some(Type::String),
        ),
        builtin(
            "std::string::trim_prefix",
            vec![param("value", Type::String), param("prefix", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::string::trim_suffix",
            vec![param("value", Type::String), param("suffix", Type::String)],
            Some(Type::String),
        ),
    ]
}
