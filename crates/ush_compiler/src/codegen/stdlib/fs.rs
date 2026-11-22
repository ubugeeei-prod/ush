use crate::{
    ast::{FunctionDef, Type},
    types::HeapVec as Vec,
};

use super::{builtin, param};

pub(super) fn definitions() -> Vec<FunctionDef> {
    vec![
        builtin(
            "std::fs::read_text",
            vec![param("path", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::fs::write_text",
            vec![param("path", Type::String), param("content", Type::String)],
            Some(Type::Unit),
        ),
        builtin(
            "std::fs::append_text",
            vec![param("path", Type::String), param("content", Type::String)],
            Some(Type::Unit),
        ),
        builtin(
            "std::fs::remove",
            vec![param("path", Type::String)],
            Some(Type::Unit),
        ),
        builtin(
            "std::fs::move",
            vec![param("from", Type::String), param("to", Type::String)],
            Some(Type::Unit),
        ),
        builtin(
            "std::fs::copy",
            vec![param("from", Type::String), param("to", Type::String)],
            Some(Type::Unit),
        ),
        builtin(
            "std::fs::exists",
            vec![param("path", Type::String)],
            Some(Type::Bool),
        ),
        builtin(
            "std::fs::is_file",
            vec![param("path", Type::String)],
            Some(Type::Bool),
        ),
        builtin(
            "std::fs::is_dir",
            vec![param("path", Type::String)],
            Some(Type::Bool),
        ),
        builtin(
            "std::fs::mkdir_p",
            vec![param("path", Type::String)],
            Some(Type::Unit),
        ),
        builtin("std::fs::tmpfile", Vec::new(), Some(Type::String)),
        builtin(
            "std::fs::sha256",
            vec![param("path", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::fs::mime_type",
            vec![param("path", Type::String)],
            Some(Type::String),
        ),
    ]
}

pub(super) fn runs_in_parent(_name: &str) -> bool {
    false
}
