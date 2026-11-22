use super::{Effect, EffectDeclarations, EffectSet};
use crate::{
    ast::{EffectDef, FunctionDef, Statement, StatementKind},
    types::AstString as String,
};
use anyhow::{Result, bail};

pub(super) fn collect_declarations(program: &[Statement]) -> Result<EffectDeclarations> {
    let mut declarations = EffectDeclarations::with_hasher(Default::default());
    for def in effect_defs(program) {
        if Effect::parse(&def.name).is_some() {
            bail!(
                "`{}` is a built-in effect and cannot be declared; built-ins are inferred, \
                 not performed",
                def.name
            );
        }
        if !declarations.insert(def.name.clone()) {
            bail!("duplicate effect declaration: {}", def.name);
        }
    }
    Ok(declarations)
}

pub(super) fn function_key(def: &FunctionDef) -> String {
    def.receiver.as_ref().map_or_else(
        || def.name.clone(),
        |ty| format!("{}::{}", ty.render(), def.name).into(),
    )
}

pub(super) fn function_defs(program: &[Statement]) -> impl Iterator<Item = &FunctionDef> {
    program.iter().flat_map(|statement| match &statement.kind {
        StatementKind::Function(def) => core::slice::from_ref(def),
        StatementKind::Impl(item) => item.methods.as_slice(),
        _ => &[] as &[FunctionDef],
    })
}

fn effect_defs(program: &[Statement]) -> impl Iterator<Item = &EffectDef> {
    program
        .iter()
        .filter_map(|statement| match &statement.kind {
            StatementKind::Effect(def) => Some(def),
            _ => None,
        })
}

pub(super) fn validate_declaration(
    def: &FunctionDef,
    inferred: &EffectSet,
    declarations: &EffectDeclarations,
) -> Result<()> {
    let Some(declared) = resolve_row(def, declarations)? else {
        return Ok(());
    };
    let missing = inferred.difference(&declared);
    if missing.is_empty() {
        return Ok(());
    }
    bail!(
        "function `{}` needs `{missing}` but its effect row is `{}`; widen it to `/ {}` \
         or handle the effect inside",
        def.name,
        declared.render_row(),
        inferred.render_row()
    )
}

/// Turns the names written in a row into an [`EffectSet`].
pub(super) fn resolve_row(
    def: &FunctionDef,
    declarations: &EffectDeclarations,
) -> Result<Option<EffectSet>> {
    let Some(names) = &def.declared_effects else {
        return Ok(None);
    };
    let mut set = EffectSet::empty();
    for name in names {
        if let Some(effect) = Effect::parse(name) {
            set.insert(effect);
            continue;
        }
        if declarations.contains(name) {
            set.insert_user(name.clone());
            continue;
        }
        bail!(
            "unknown effect `{name}` in the row of `{}`; declare it with `effect {name}` \
             or use one of {}",
            def.name,
            Effect::ALL
                .into_iter()
                .map(Effect::name)
                .collect::<alloc::vec::Vec<_>>()
                .join(", ")
        );
    }
    Ok(Some(set))
}
