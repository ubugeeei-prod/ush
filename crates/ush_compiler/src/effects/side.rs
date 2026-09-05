//! The side-effect system.
//!
//! `.ush` already tracks *which errors* a function can raise. This
//! pass tracks *what a function touches*: the filesystem, the
//! environment, the network, other processes, the terminal, and the
//! task scheduler. Effects are inferred bottom-up from the stdlib and
//! propagated through calls, so nothing has to be annotated for the
//! information to exist.
//!
//! Annotating is how you turn the information into a check. A
//! function that declares
//!
//! ```text
//! #[effects(fs, exec)]
//! fn deploy() -> String { ... }
//! ```
//!
//! is refused if it grows a network call later, and `#[pure]` says a
//! function may touch nothing at all. A declaration is an upper
//! bound: declaring more than you use is allowed and keeps a public
//! signature stable while an implementation shrinks.

mod infer;
mod kinds;
mod table;

#[cfg(test)]
mod tests;

use anyhow::{Result, bail};

use crate::{
    ast::{FunctionDef, Statement, StatementKind},
    types::{AstString as String, HeapVec as Vec, Map as HashMap},
};

use self::infer::block_effects;
pub use self::kinds::{Effect, EffectSet};

pub(crate) type FunctionEffectRegistry = HashMap<String, EffectSet>;

/// What one function was found to do, and what it claims to do.
#[derive(Clone, Debug)]
pub struct FunctionEffects {
    pub name: crate::types::OutputString,
    pub inferred: EffectSet,
    pub declared: Option<EffectSet>,
}

/// Infers effects for every function, then checks the declarations.
///
/// Inference is a fixpoint because functions call each other: each
/// round re-walks every body with the registry built so far, and the
/// walk stops when a round changes nothing. The set only ever grows,
/// and there are finitely many effects, so it terminates.
pub(crate) fn analyze_function_effects(program: &[Statement]) -> Result<FunctionEffectRegistry> {
    let mut registry = FunctionEffectRegistry::default();
    for def in function_defs(program) {
        registry.insert(def.name.clone(), EffectSet::empty());
    }

    for _ in 0..=registry.len() {
        let mut changed = false;
        for def in function_defs(program) {
            let inferred = block_effects(&def.body, &registry);
            if registry.get(&def.name) != Some(&inferred) {
                registry.insert(def.name.clone(), inferred);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    for def in function_defs(program) {
        let inferred = registry.get(&def.name).copied().unwrap_or_default();
        validate_declaration(def, inferred)?;
    }

    Ok(registry)
}

/// The per-function report behind `ush effects`.
pub(crate) fn describe_function_effects(program: &[Statement]) -> Result<Vec<FunctionEffects>> {
    let registry = analyze_function_effects(program)?;
    let mut reports = Vec::new();
    for def in function_defs(program) {
        reports.push(FunctionEffects {
            name: def.name.as_str().into(),
            inferred: registry.get(&def.name).copied().unwrap_or_default(),
            declared: declared_effects(def)?,
        });
    }
    reports.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(reports)
}

/// Effects of the statements outside any function.
pub(crate) fn top_level_effects(program: &[Statement]) -> Result<EffectSet> {
    let registry = analyze_function_effects(program)?;
    Ok(block_effects(program, &registry))
}

fn function_defs(program: &[Statement]) -> impl Iterator<Item = &FunctionDef> {
    program.iter().flat_map(|statement| match &statement.kind {
        StatementKind::Function(def) => core::slice::from_ref(def),
        StatementKind::Impl(item) => item.methods.as_slice(),
        _ => &[] as &[FunctionDef],
    })
}

fn validate_declaration(def: &FunctionDef, inferred: EffectSet) -> Result<()> {
    let Some(declared) = declared_effects(def)? else {
        return Ok(());
    };
    let missing = inferred.difference(declared);
    if missing.is_empty() {
        return Ok(());
    }
    bail!(
        "function `{}` performs `{missing}` but its effect row declares `{declared}`; \
         write `#[effects({})]` or drop the offending call",
        def.name,
        inferred
            .iter()
            .map(Effect::name)
            .collect::<alloc::vec::Vec<_>>()
            .join(", ")
    )
}

fn declared_effects(def: &FunctionDef) -> Result<Option<EffectSet>> {
    let mut declared = None;
    for attr in &def.attrs {
        match attr.name.as_str() {
            "pure" => {
                if !attr.args.is_empty() {
                    bail!("`#[pure]` takes no arguments");
                }
                declared = Some(declared.unwrap_or_else(EffectSet::empty));
            }
            "effects" => {
                let mut set = declared.unwrap_or_else(EffectSet::empty);
                for arg in &attr.args {
                    let Some(effect) = Effect::parse(arg.trim()) else {
                        bail!(
                            "unknown effect `{arg}` on `{}`; known effects are {}",
                            def.name,
                            Effect::ALL
                                .into_iter()
                                .map(Effect::name)
                                .collect::<alloc::vec::Vec<_>>()
                                .join(", ")
                        );
                    };
                    set.insert(effect);
                }
                declared = Some(set);
            }
            _ => {}
        }
    }
    Ok(declared)
}
