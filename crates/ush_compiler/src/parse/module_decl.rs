//! `mod "lib/gh.ush" as gh` — a local module import.

use anyhow::{Result, anyhow, bail, ensure};

use super::path::parse_identifier;
use crate::{
    ast::{ModuleDecl, StatementKind},
    types::AstString as String,
    util::{is_identifier, parse_string_literal, split_top_level_whitespace},
};

/// Names a module cannot take: they would shadow std paths.
const RESERVED: &[&str] = &["std", "self", "super"];

pub(super) fn parse(source: &str) -> Result<StatementKind> {
    let words = split_top_level_whitespace(source.trim());
    ensure!(
        words.len() == 1 || (words.len() == 3 && words[1] == "as"),
        "expected `mod \"path.ush\"` or `mod \"path.ush\" as name`"
    );
    let path = parse_string_literal(words[0])
        .ok_or_else(|| anyhow!("module path must be a string literal"))?;
    ensure!(
        path.ends_with(".ush") && !path.contains(['\n', '\r']),
        "module path must name a .ush file"
    );
    ensure!(
        !path.starts_with('/'),
        "module path must be relative to the importing file: {path}"
    );
    let alias = match words.get(2) {
        Some(alias) => parse_identifier(alias)?,
        None => default_alias(&path)?,
    };
    if RESERVED.contains(&alias.as_str()) {
        bail!("`{alias}` is reserved; import the module with `as` another name");
    }
    Ok(StatementKind::Module(ModuleDecl { path, alias }))
}

/// The file stem, which must already be an identifier.
fn default_alias(path: &str) -> Result<String> {
    let file = path.rsplit('/').next().unwrap_or(path);
    let stem = file.strip_suffix(".ush").unwrap_or(file);
    ensure!(
        is_identifier(stem),
        "module file name `{file}` is not an identifier; name the module with `as`"
    );
    Ok(stem.into())
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::StatementKind as S,
        parse::{FileRole, parse_file},
    };

    fn module(source: &str) -> anyhow::Result<alloc::vec::Vec<crate::ast::Statement>> {
        parse_file(source, FileRole::Module { dir: "lib" })
    }

    fn error(
        result: anyhow::Result<alloc::vec::Vec<crate::ast::Statement>>,
    ) -> alloc::string::String {
        format!("{:#}", result.expect_err("parse should fail"))
    }

    #[test]
    fn the_alias_defaults_to_the_file_stem() {
        let program = parse_file(
            "mod \"lib/gh_actions.ush\"\nmod \"x.ush\" as y\n",
            FileRole::Script,
        )
        .unwrap();
        let aliases = program
            .iter()
            .map(|s| match &s.kind {
                S::Module(decl) => (decl.path.as_str(), decl.alias.as_str()),
                _ => unreachable!(),
            })
            .collect::<alloc::vec::Vec<_>>();
        assert_eq!(
            aliases,
            [("lib/gh_actions.ush", "gh_actions"), ("x.ush", "y")]
        );
    }

    #[test]
    fn module_paths_are_relative_ush_files_with_usable_names() {
        for (source, message) in [
            ("mod \"/abs/x.ush\"", "must be relative"),
            ("mod \"x.sh\"", "must name a .ush file"),
            ("mod x", "must be a string literal"),
            (
                "mod \"gh-actions.ush\"",
                "is not an identifier; name the module with `as`",
            ),
            ("mod \"std.ush\"", "`std` is reserved"),
            ("mod \"x.ush\" gh", "expected `mod"),
        ] {
            let message_found = error(parse_file(source, FileRole::Script));
            assert!(message_found.contains(message), "{source}: {message_found}");
        }
    }

    #[test]
    fn pub_marks_exportable_declarations() {
        let program = module(
            "pub fn f() {\n}\nfn g() {\n}\npub enum E {\n  A,\n}\npub extern \"x.sh\" as x {\n  fn h()\n}\n",
        )
        .unwrap();
        let public = program
            .iter()
            .map(|s| s.public)
            .collect::<alloc::vec::Vec<_>>();
        // The extern's sourcing statement stays private; its function is exported.
        assert_eq!(public, [true, false, true, false, true]);
        assert!(error(module("pub impl E {}\n")).contains("`pub` applies to fn"));
        assert!(error(module("pub test \"t\" {\n}\n")).contains("cannot be `pub`"));
    }

    #[test]
    fn modules_hold_only_declarations() {
        assert!(
            error(module("fn f() {\n}\nprint \"x\"\n"))
                .contains("line 3: a module contains only declarations")
        );
        assert!(error(module("unit ms\n")).contains("belong in the root script"));
        assert!(
            module("use std::string::{trim}\nmod \"y.ush\"\nalias ll = \"ls\"\ntest {\n}\n")
                .is_ok()
        );
    }

    #[test]
    fn module_externs_are_sourced_from_beside_the_module() {
        let program = module("extern \"h.sh\" {\n  fn h()\n}\n").unwrap();
        let S::Shell(crate::ast::Expr::String(source)) = &program[0].kind else {
            panic!("expected the sourcing statement");
        };
        assert_eq!(source.as_str(), ". \"${__ush_source_dir:-.}\"/'lib/h.sh'");
    }
}
