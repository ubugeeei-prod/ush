use crate::ast::{FunctionDef, Type};
use crate::types::HeapVec as Vec;

use super::{builtin, param};

pub(super) fn definitions() -> Vec<FunctionDef> {
    vec![
        builtin(
            "std::http::get",
            vec![param("url", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::http::download",
            vec![param("url", Type::String), param("path", Type::String)],
            Some(Type::Unit),
        ),
    ]
}
