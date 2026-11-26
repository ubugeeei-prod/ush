use crate::ast::{FunctionDef, Type};
use crate::types::HeapVec as Vec;

use super::{builtin, param};

pub(super) fn definitions() -> Vec<FunctionDef> {
    vec![
        builtin("std::path::cwd", Vec::new(), Some(Type::String)),
        builtin("std::path::home", Vec::new(), Some(Type::String)),
        builtin("std::path::source_root", Vec::new(), Some(Type::String)),
        builtin("std::path::script", Vec::new(), Some(Type::String)),
        builtin("std::path::tmpfile", Vec::new(), Some(Type::String)),
        builtin(
            "std::path::from_cwd",
            vec![param("path", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::path::from_source",
            vec![param("path", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::path::resolve",
            vec![param("path", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::path::join",
            vec![param("left", Type::String), param("right", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::path::dirname",
            vec![param("path", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::path::basename",
            vec![param("path", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::path::exists",
            vec![param("path", Type::String)],
            Some(Type::Bool),
        ),
        builtin(
            "std::path::is_file",
            vec![param("path", Type::String)],
            Some(Type::Bool),
        ),
        builtin(
            "std::path::is_dir",
            vec![param("path", Type::String)],
            Some(Type::Bool),
        ),
        builtin(
            "std::path::mkdir_p",
            vec![param("path", Type::String)],
            Some(Type::Unit),
        ),
        builtin(
            "std::path::prepend_env",
            vec![param("name", Type::String), param("segment", Type::String)],
            Some(Type::Unit),
        ),
    ]
}

pub(super) fn runs_in_parent(name: &str) -> bool {
    matches!(name, "std::path::prepend_env")
}
