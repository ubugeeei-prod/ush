use crate::{
    ast::{Statement, StatementKind},
    types::{Map as HashMap, Set as HashSet},
};
use anyhow::{Result, bail};

pub(super) fn local_functions(program: &[Statement]) -> HashSet<crate::types::AstString> {
    let mut names = HashSet::with_hasher(Default::default());
    for statement in program {
        if let StatementKind::Function(def) = &statement.kind {
            names.insert(def.name.clone());
        }
    }
    names
}

pub(super) fn collect_imports(
    program: &[Statement],
    locals: &HashSet<crate::types::AstString>,
    builtins: &crate::codegen::FunctionRegistry,
) -> Result<HashMap<crate::types::AstString, crate::types::AstString>> {
    let mut imports = HashMap::with_hasher(Default::default());
    let mut wildcards = alloc::vec::Vec::new();
    for statement in program {
        let StatementKind::Use(items) = &statement.kind else {
            continue;
        };
        for item in items {
            if item.alias == "*" {
                if item.path.starts_with("$bin") {
                    imports.insert("*".into(), "$bin".into());
                } else {
                    wildcards.push(item.path.clone());
                }
                continue;
            }
            if locals.contains(&item.alias) {
                bail!("import alias conflicts with local function: {}", item.alias);
            }
            if imports
                .insert(item.alias.clone(), item.path.clone())
                .is_some()
            {
                bail!("duplicate import alias: {}", item.alias);
            }
        }
    }
    let explicit = imports.keys().cloned().collect::<HashSet<_>>();
    for module in wildcards {
        let prefix = format!("{module}::");
        let mut found = false;
        for path in builtins.keys().filter(|path| path.starts_with(&prefix)) {
            let alias = &path[prefix.len()..];
            if alias.contains("::") {
                continue;
            }
            found = true;
            if locals.contains(alias) || explicit.contains(alias) {
                continue;
            }
            if let Some(previous) = imports.insert(alias.into(), path.clone())
                && previous != *path
            {
                bail!("ambiguous wildcard import: {alias}");
            }
        }
        if !found {
            bail!("unknown module in wildcard import: {module}");
        }
    }
    if imports.contains_key("*") {
        for name in locals {
            imports.insert(name.clone(), name.clone());
        }
    }
    Ok(imports)
}
