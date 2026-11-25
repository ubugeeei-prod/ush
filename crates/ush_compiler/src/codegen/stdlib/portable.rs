//! APIs implemented by POSIX shell and its standard utilities.
use super::{builtin, param};
use crate::{
    ast::{FunctionDef, Type},
    types::HeapVec as Vec,
};
use alloc::boxed::Box;

pub(super) fn definitions() -> Vec<FunctionDef> {
    use Type::{Bool, Int, String, Unit};
    let strings = Type::List(Box::new(String));
    let mut defs = Vec::new();
    for name in ["is_empty", "is_ascii"] {
        defs.push(builtin(
            &format!("std::string::{name}"),
            vec![param("text", String)],
            Some(Bool),
        ));
    }
    for name in [
        "to_lower_ascii",
        "to_upper_ascii",
        "trim_start",
        "trim_end",
        "shell_quote",
    ] {
        defs.push(builtin(
            &format!("std::string::{name}"),
            vec![param("text", String)],
            Some(String),
        ));
    }
    defs.push(builtin(
        "std::string::words",
        vec![param("text", String)],
        Some(strings.clone()),
    ));
    defs.push(builtin(
        "std::string::join",
        vec![param("items", strings.clone()), param("separator", String)],
        Some(String),
    ));
    defs.push(builtin(
        "std::string::repeat",
        vec![param("text", String), param("count", Int)],
        Some(String),
    ));
    defs.push(builtin(
        "std::string::count",
        vec![param("text", String), param("part", String)],
        Some(Int),
    ));
    for name in [
        "is_symlink",
        "is_fifo",
        "is_readable",
        "is_writable",
        "is_executable",
    ] {
        defs.push(builtin(
            &format!("std::fs::{name}"),
            vec![param("path", String)],
            Some(Bool),
        ));
    }
    for name in ["mkdir", "rmdir", "touch", "remove_tree"] {
        defs.push(builtin(
            &format!("std::fs::{name}"),
            vec![param("path", String)],
            Some(Unit),
        ));
    }
    for name in ["read_dir", "read_lines"] {
        defs.push(builtin(
            &format!("std::fs::{name}"),
            vec![param("path", String)],
            Some(strings.clone()),
        ));
    }
    for name in ["symlink", "hard_link"] {
        defs.push(builtin(
            &format!("std::fs::{name}"),
            vec![param("target", String), param("link", String)],
            Some(Unit),
        ));
    }
    defs.push(builtin(
        "std::fs::chmod",
        vec![param("path", String), param("mode", String)],
        Some(Unit),
    ));
    defs.push(builtin(
        "std::fs::size",
        vec![param("path", String)],
        Some(Int),
    ));
    defs.push(builtin(
        "std::path::is_absolute",
        vec![param("path", String)],
        Some(Bool),
    ));
    for name in ["extension", "stem", "normalize"] {
        defs.push(builtin(
            &format!("std::path::{name}"),
            vec![param("path", String)],
            Some(String),
        ));
    }
    for name in ["print", "println"] {
        defs.push(builtin(
            &format!("std::io::{name}"),
            vec![param("text", String)],
            Some(Unit),
        ));
    }
    defs.push(builtin("std::io::read_lines", vec![], Some(strings)));
    defs.push(builtin("std::io::read_all", vec![], Some(String)));
    defs.push(builtin(
        "std::terminal::is_tty",
        vec![param("fd", Int)],
        Some(Bool),
    ));
    defs.push(builtin("std::process::pid", vec![], Some(Int)));
    defs.push(builtin(
        "std::process::sleep",
        vec![param("seconds", Int)],
        Some(Unit),
    ));
    defs.push(builtin(
        "std::process::chdir",
        vec![param("path", String)],
        Some(Unit),
    ));
    for name in ["name", "arch", "hostname"] {
        defs.push(builtin(&format!("std::os::{name}"), vec![], Some(String)));
    }
    for name in ["uid", "gid"] {
        defs.push(builtin(&format!("std::os::{name}"), vec![], Some(Int)));
    }
    for name in ["min", "max"] {
        defs.push(builtin(
            &format!("std::int::{name}"),
            vec![param("left", Int), param("right", Int)],
            Some(Int),
        ));
    }
    for name in ["abs", "sign"] {
        defs.push(builtin(
            &format!("std::int::{name}"),
            vec![param("value", Int)],
            Some(Int),
        ));
    }
    defs.push(builtin(
        "std::int::clamp",
        vec![param("value", Int), param("min", Int), param("max", Int)],
        Some(Int),
    ));
    defs
}
