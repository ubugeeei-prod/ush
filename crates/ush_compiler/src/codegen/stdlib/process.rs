use crate::{
    ast::{FunctionDef, Type},
    types::HeapVec as Vec,
};

use super::{builtin, param};

pub(super) fn definitions() -> Vec<FunctionDef> {
    vec![
        builtin(
            "std::command::exists",
            vec![param("name", Type::String)],
            Some(Type::Bool),
        ),
        builtin(
            "std::command::capture",
            vec![param("command", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::command::capture_stderr",
            vec![param("command", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::command::status",
            vec![param("command", Type::String)],
            Some(Type::Int),
        ),
        builtin(
            "std::command::run",
            vec![param("command", Type::String)],
            Some(Type::Unit),
        ),
    ]
}
