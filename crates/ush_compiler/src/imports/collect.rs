use crate::{
    ast::{Statement, StatementKind},
    modules::Scope,
    types::{AstString as Name, Map as HashMap, Set as HashSet},
};
use anyhow::{Result, bail};

pub(super) fn local_functions(program: &[Statement]) -> HashSet<Name> {
    let mut names = HashSet::with_hasher(Default::default());
    for statement in program {
        if let StatementKind::Function(def) = &statement.kind {
            names.insert(def.name.clone());
        }
    }
    names
}

/// Maps each bare name a file may call to its target. Local functions
/// win over explicit imports, which win over wildcard imports.
pub(super) fn collect_imports(
    program: &[Statement],
    locals: &HashSet<Name>,
    paths: &HashSet<Name>,
    scope: &Scope<'_>,
) -> Result<HashMap<Name, Name>> {
    let mut imports = HashMap::with_hasher(Default::default());
    let mut wildcards = alloc::vec::Vec::new();
    let explicit_modules = scope
        .explicit
        .iter()
        .filter(|(_, item)| item.kind.is_value())
        .map(|(alias, item)| (alias, &item.internal));
    let explicit_uses = program
        .iter()
        .filter_map(|statement| match &statement.kind {
            StatementKind::Use(items) => Some(items),
            _ => None,
        })
        .flatten()
        .filter_map(|item| {
            if item.alias != "*" {
                return Some((&item.alias, &item.path));
            }
            if item.path.starts_with("$bin") {
                imports.insert("*".into(), "$bin".into());
            } else {
                wildcards.push(item.path.clone());
            }
            None
        })
        .collect::<alloc::vec::Vec<_>>();
    for (alias, path) in explicit_uses.into_iter().chain(explicit_modules) {
        if locals.contains(alias) {
            bail!("import alias conflicts with local function: {alias}");
        }
        if imports.insert(alias.clone(), path.clone()).is_some() {
            bail!("duplicate import alias: {alias}");
        }
    }
    let explicit = imports.keys().cloned().collect::<HashSet<_>>();
    let mut wildcard = |alias: &str, path: &Name| -> Result<()> {
        if locals.contains(alias) || explicit.contains(alias) {
            return Ok(());
        }
        if let Some(previous) = imports.insert(alias.into(), path.clone())
            && previous != *path
        {
            bail!("ambiguous wildcard import: {alias}");
        }
        Ok(())
    };
    for module in wildcards {
        let prefix = format!("{module}::");
        let mut found = false;
        for path in paths.iter().filter(|path| path.starts_with(&prefix)) {
            let alias = &path[prefix.len()..];
            if alias.contains("::") {
                continue;
            }
            found = true;
            wildcard(alias, path)?;
        }
        if !found {
            bail!("unknown module in wildcard import: {module}");
        }
    }
    for (alias, item) in &scope.wildcards {
        if item.kind.is_value() {
            wildcard(alias, &item.internal)?;
        }
    }
    if imports.contains_key("*") {
        for name in locals {
            imports.insert(name.clone(), name.clone());
        }
    }
    if let Some(namespace) = scope.namespace {
        for name in locals.iter().chain(&own_effects(program)) {
            imports.insert(name.clone(), format!("{namespace}::{name}").into());
        }
    }
    Ok(imports)
}

/// Effect operations are call targets too: `do log "x"` calls `log`.
fn own_effects(program: &[Statement]) -> HashSet<Name> {
    program
        .iter()
        .filter_map(|statement| match &statement.kind {
            StatementKind::Effect(def) => Some(def.name.clone()),
            _ => None,
        })
        .collect()
}
