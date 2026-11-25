//! Boundaries that need the optional ush executable on the target machine.
use super::{builtin, param};
use crate::{
    ast::{FunctionDef, Type},
    types::HeapVec as Vec,
};
use alloc::boxed::Box;

pub(super) fn definitions() -> Vec<FunctionDef> {
    use Type::{Bool, Int, String, Unit};
    let bytes = Type::List(Box::new(Int));
    let strings = Type::List(Box::new(String));
    let mut defs = vec![builtin("std::runtime::available", vec![], Some(Bool))];
    for name in ["length", "byte_length"] {
        defs.push(builtin(
            &format!("std::string::{name}"),
            vec![param("text", String)],
            Some(Int),
        ));
    }
    for name in ["reverse", "to_lower", "to_upper"] {
        defs.push(builtin(
            &format!("std::string::{name}"),
            vec![param("text", String)],
            Some(String),
        ));
    }
    defs.push(builtin(
        "std::string::chars",
        vec![param("text", String)],
        Some(strings.clone()),
    ));
    defs.push(builtin(
        "std::string::slice",
        vec![
            param("text", String),
            param("start", Int),
            param("end", Int),
        ],
        Some(String),
    ));
    defs.push(builtin(
        "std::string::find",
        vec![param("text", String), param("part", String)],
        Some(Int),
    ));
    for name in ["read_link", "canonicalize"] {
        defs.push(builtin(
            &format!("std::fs::{name}"),
            vec![param("path", String)],
            Some(String),
        ));
    }
    for name in [
        "mode", "owner", "group", "modified", "accessed", "inode", "links",
    ] {
        defs.push(builtin(
            &format!("std::fs::{name}"),
            vec![param("path", String)],
            Some(Int),
        ));
    }
    defs.push(builtin(
        "std::fs::is_socket",
        vec![param("path", String)],
        Some(Bool),
    ));
    defs.push(builtin(
        "std::fs::read_bytes",
        vec![param("path", String)],
        Some(bytes.clone()),
    ));
    defs.push(builtin(
        "std::fs::write_bytes",
        vec![param("path", String), param("bytes", bytes.clone())],
        Some(Unit),
    ));
    defs.push(builtin(
        "std::fs::write_atomic",
        vec![param("path", String), param("text", String)],
        Some(Unit),
    ));
    for name in ["from_hex", "from_text", "decode_base64"] {
        defs.push(builtin(
            &format!("std::bytes::{name}"),
            vec![param("text", String)],
            Some(bytes.clone()),
        ));
    }
    for name in ["to_hex", "to_text", "encode_base64"] {
        defs.push(builtin(
            &format!("std::bytes::{name}"),
            vec![param("bytes", bytes.clone())],
            Some(String),
        ));
    }
    for name in ["unix_seconds", "unix_millis"] {
        defs.push(builtin(&format!("std::time::{name}"), vec![], Some(Int)));
    }
    defs.push(builtin(
        "std::time::sleep_millis",
        vec![param("milliseconds", Int)],
        Some(Unit),
    ));
    defs.push(builtin(
        "std::time::format_utc",
        vec![param("seconds", Int), param("format", String)],
        Some(String),
    ));
    defs.push(builtin(
        "std::random::bytes",
        vec![param("count", Int)],
        Some(bytes),
    ));
    defs.push(builtin(
        "std::random::hex",
        vec![param("count", Int)],
        Some(String),
    ));
    defs.push(builtin(
        "std::random::int",
        vec![param("min", Int), param("max", Int)],
        Some(Int),
    ));
    defs.push(builtin(
        "std::net::resolve",
        vec![param("host", String)],
        Some(strings),
    ));
    for name in ["euid", "egid", "page_size", "cpu_count"] {
        defs.push(builtin(&format!("std::os::{name}"), vec![], Some(Int)));
    }
    defs.push(builtin(
        "std::os::error_message",
        vec![param("code", Int)],
        Some(String),
    ));
    for name in ["add", "sub", "mul", "div", "rem", "pow"] {
        defs.push(builtin(
            &format!("std::int::{name}"),
            vec![param("left", Int), param("right", Int)],
            Some(Int),
        ));
    }
    defs
}

pub(crate) fn required(name: &str) -> bool {
    matches!(
        name,
        "std::string::length"
            | "std::string::byte_length"
            | "std::string::reverse"
            | "std::string::to_lower"
            | "std::string::to_upper"
            | "std::string::chars"
            | "std::string::slice"
            | "std::string::find"
            | "std::fs::read_link"
            | "std::fs::canonicalize"
            | "std::fs::mode"
            | "std::fs::owner"
            | "std::fs::group"
            | "std::fs::modified"
            | "std::fs::accessed"
            | "std::fs::inode"
            | "std::fs::links"
            | "std::fs::is_socket"
            | "std::fs::read_bytes"
            | "std::fs::write_bytes"
            | "std::fs::write_atomic"
            | "std::bytes::from_hex"
            | "std::bytes::from_text"
            | "std::bytes::decode_base64"
            | "std::bytes::to_hex"
            | "std::bytes::to_text"
            | "std::bytes::encode_base64"
            | "std::time::unix_seconds"
            | "std::time::unix_millis"
            | "std::time::sleep_millis"
            | "std::time::format_utc"
            | "std::random::bytes"
            | "std::random::hex"
            | "std::random::int"
            | "std::net::resolve"
            | "std::os::euid"
            | "std::os::egid"
            | "std::os::page_size"
            | "std::os::cpu_count"
            | "std::os::error_message"
            | "std::int::add"
            | "std::int::sub"
            | "std::int::mul"
            | "std::int::div"
            | "std::int::rem"
            | "std::int::pow"
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn static_wrappers_match_the_registered_runtime_apis() {
        let wrappers = include_str!("host_runtime.sh");
        for def in super::definitions() {
            let name = super::super::super::shared::shell_function_name(&def.name);
            assert!(wrappers.contains(&format!("{name}() {{")), "{}", def.name);
            assert_eq!(
                super::required(&def.name),
                def.name != "std::runtime::available"
            );
        }
    }
}
