//! Linking against an in-memory loader, so these run without `std`.

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use anyhow::{Result, anyhow};

use super::{ModuleLoader, ModuleSource, link};
use crate::ast::StatementKind as S;

/// Files by path; every path is relative to one directory.
struct Memory(Vec<(&'static str, &'static str)>);

impl ModuleLoader for Memory {
    fn root(&self) -> (&str, &str) {
        ("main.ush", "main.ush")
    }

    fn load(&mut self, _: &str, path: &str) -> Result<ModuleSource> {
        let (_, text) = self
            .0
            .iter()
            .find(|(name, _)| *name == path)
            .ok_or_else(|| anyhow!("module not found: {path}"))?;
        Ok(ModuleSource {
            key: path.into(),
            display: path.into(),
            origin: path.into(),
            dir: String::new(),
            text: text.to_string(),
        })
    }
}

fn functions(root: &str, files: Vec<(&'static str, &'static str)>) -> Result<Vec<String>> {
    let linked = link(root, &mut Memory(files))?;
    Ok(linked
        .program
        .iter()
        .filter_map(|statement| match &statement.kind {
            S::Function(def) => Some(def.name.to_string()),
            _ => None,
        })
        .collect())
}

#[test]
fn module_items_move_into_their_namespace_and_unused_ones_are_dropped() {
    let names = functions(
        "mod \"a.ush\"\nmod \"b.ush\" as other\nfn main() {\n  a::run\n  other::run\n}\n",
        vec![
            (
                "a.ush",
                "pub fn run() {\n  helper\n}\nfn helper() {\n}\npub fn idle() {\n}\n",
            ),
            ("b.ush", "pub fn run() {\n}\n"),
        ],
    )
    .unwrap();
    assert_eq!(names, ["a::run", "a::helper", "other::run", "main"]);
}

#[test]
fn positions_after_the_root_name_module_lines() {
    let linked = link(
        "mod \"a.ush\"\na::run\n",
        &mut Memory(vec![("a.ush", "\npub fn run() {\n}\n")]),
    )
    .unwrap();
    let run = linked
        .program
        .iter()
        .find(|statement| matches!(&statement.kind, S::Function(def) if def.name == "a::run"))
        .unwrap();
    let (file, line) = linked.space.locate(run.line).unwrap();
    assert_eq!((file.display.as_str(), line), ("a.ush", 2));
}

#[test]
fn a_namespace_taken_by_another_module_gets_a_suffix() {
    let names = functions(
        "mod \"x/util.ush\"\nmod \"y.ush\"\nutil::a\ny::b\n",
        vec![
            ("x/util.ush", "pub fn a() {\n}\n"),
            ("y.ush", "mod \"util.ush\"\npub fn b() {\n  util::c\n}\n"),
            ("util.ush", "pub fn c() {\n}\n"),
        ],
    )
    .unwrap();
    assert_eq!(names, ["util::a", "util_2::c", "y::b"]);
}

#[test]
fn duplicate_aliases_and_extern_namespaces_are_rejected() {
    let files = || vec![("a.ush", "pub fn f() {\n}\n")];
    let error = functions("mod \"a.ush\"\nmod \"a.ush\"\n", files()).unwrap_err();
    assert!(alloc::format!("{error:#}").contains("duplicate module alias: a"));
    let error = functions(
        "mod \"a.ush\"\nextern \"a.sh\" as a {\n  fn f()\n}\n",
        files(),
    )
    .unwrap_err();
    assert!(alloc::format!("{error:#}").contains("also an extern namespace"));
}
