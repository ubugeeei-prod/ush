//! Typed operating-system boundaries used by native ush scripts.
use super::{builtin, param};
use crate::{
    ast::{FunctionDef, Type},
    types::HeapVec as Vec,
};
use alloc::boxed::Box;

pub(super) fn definitions() -> Vec<FunctionDef> {
    let strings = Type::List(Box::new(Type::String));
    let mut defs = vec![
        builtin("std::process::args", vec![], Some(strings.clone())),
        builtin(
            "std::process::arg",
            vec![param("index", Type::Int)],
            Some(Type::String),
        ),
        builtin(
            "std::process::exit",
            vec![param("status", Type::Int)],
            Some(Type::Unit),
        ),
        builtin(
            "std::io::eprint",
            vec![param("text", Type::String)],
            Some(Type::Unit),
        ),
        builtin("std::fs::temp_dir", vec![], Some(Type::String)),
        builtin(
            "std::string::split",
            vec![
                param("text", Type::String),
                param("separator", Type::String),
            ],
            Some(strings.clone()),
        ),
        builtin(
            "std::string::lines",
            vec![param("text", Type::String)],
            Some(strings.clone()),
        ),
        builtin(
            "std::string::contains",
            vec![param("text", Type::String), param("part", Type::String)],
            Some(Type::Bool),
        ),
        builtin(
            "std::string::trim",
            vec![param("text", Type::String)],
            Some(Type::String),
        ),
        builtin(
            "std::string::to_int",
            vec![param("text", Type::String)],
            Some(Type::Int),
        ),
    ];
    for (name, ty) in [
        ("exec", Type::Unit),
        ("output", Type::String),
        ("status", Type::Int),
    ] {
        defs.push(builtin(
            &format!("std::process::{name}"),
            vec![
                param("program", Type::String),
                param("args", strings.clone()),
            ],
            Some(ty),
        ));
    }
    defs.push(builtin(
        "std::process::redirect",
        vec![
            param("program", Type::String),
            param("args", strings),
            param("stdout", Type::String),
            param("stderr", Type::String),
        ],
        Some(Type::Int),
    ));
    defs
}
