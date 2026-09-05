//! The effect system.
//!
//! Modelled on [Effekt]: a function's *effect row* is what its body
//! still needs from its caller, effects propagate outward through
//! calls until something handles them, and a handler discharges an
//! effect for the block it wraps.
//!
//! ```text
//! effect log(message: String) -> ()
//!
//! fn greet(name: String) -> String / { log } {
//!   do log("greeting " + name)
//!   "hello " + name
//! }
//!
//! fn main() -> () {
//!   try {
//!     print greet("ubu")
//!   } with log { (message) =>
//!     print "[log] " + message
//!   }
//! }
//! ```
//!
//! Two kinds of effect share the row. **Built-in** effects (`io`,
//! `fs`, `env`, `net`, `exec`, `task`) are inferred from what the
//! generated shell does and cannot be handled — no handler takes
//! back a file that was written. **User** effects are declared,
//! performed with `do`, and discharged by `try … with`.
//!
//! Inference runs whether or not a row is written. Writing one turns
//! the inference into a check, and the row is an upper bound:
//! declaring more than the body needs is allowed, so a signature can
//! stay stable while an implementation shrinks.
//!
//! [Effekt]: https://effekt-lang.org

mod infer;
mod kinds;
mod table;

#[cfg(test)]
mod tests;

use anyhow::{Result, bail};

use crate::{
    ast::{EffectDef, FunctionDef, Statement, StatementKind},
    types::{AstString as String, HeapVec as Vec, Map as HashMap, Set as HashSet},
};

use self::infer::{Context, block_effects};
pub use self::kinds::{Effect, EffectSet};

pub(crate) type FunctionEffectRegistry = HashMap<String, EffectSet>;

/// The `effect` declarations a program brought into scope.
pub(crate) type EffectDeclarations = HashSet<String>;

/// What one function was found to need, and what it claims to need.
#[derive(Clone, Debug)]
pub struct FunctionEffects {
    pub name: crate::types::OutputString,
    pub inferred: EffectSet,
    pub declared: Option<EffectSet>,
}

/// Everything the effect pass knows about a program.
pub(crate) struct ProgramEffects {
    pub functions: FunctionEffectRegistry,
    pub declarations: EffectDeclarations,
}

/// Infers rows for every function, then checks them.
///
/// Inference is a fixpoint because functions call each other: each
/// round re-walks every body with the registry built so far, and it
/// stops when a round changes nothing. Rows only grow and there are
/// finitely many effects, so it terminates.
pub(crate) fn analyze_function_effects(program: &[Statement]) -> Result<ProgramEffects> {
    let declarations = collect_declarations(program)?;
    let mut registry = FunctionEffectRegistry::default();
    for def in function_defs(program) {
        registry.insert(def.name.clone(), EffectSet::empty());
    }

    for _ in 0..=registry.len() {
        let mut changed = false;
        for def in function_defs(program) {
            let inferred = block_effects(
                &def.body,
                Context {
                    registry: &registry,
                    declarations: &declarations,
                },
            );
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
        let inferred = registry.get(&def.name).cloned().unwrap_or_default();
        validate_declaration(def, &inferred, &declarations)?;
    }

    // A user effect that reaches the top of the program was never
    // handled. Effekt makes the same demand of `main`: an entry point
    // has no caller left to answer it.
    let top_level = block_effects(
        program,
        Context {
            registry: &registry,
            declarations: &declarations,
        },
    );
    if let Some(name) = top_level.user_effects().next() {
        bail!(
            "effect `{name}` is performed but never handled; wrap the call in \
             `try {{ … }} with {name} {{ … }}`"
        );
    }

    Ok(ProgramEffects {
        functions: registry,
        declarations,
    })
}

/// The per-function report behind `ush effects`.
pub(crate) fn describe_function_effects(program: &[Statement]) -> Result<Vec<FunctionEffects>> {
    let analysis = analyze_function_effects(program)?;
    let mut reports = Vec::new();
    for def in function_defs(program) {
        reports.push(FunctionEffects {
            name: def.name.as_str().into(),
            inferred: analysis
                .functions
                .get(&def.name)
                .cloned()
                .unwrap_or_default(),
            declared: resolve_row(def, &analysis.declarations)?,
        });
    }
    reports.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(reports)
}

/// The row of the statements outside any function.
pub(crate) fn top_level_effects(program: &[Statement]) -> Result<EffectSet> {
    let analysis = analyze_function_effects(program)?;
    Ok(block_effects(
        program,
        Context {
            registry: &analysis.functions,
            declarations: &analysis.declarations,
        },
    ))
}

fn collect_declarations(program: &[Statement]) -> Result<EffectDeclarations> {
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

fn function_defs(program: &[Statement]) -> impl Iterator<Item = &FunctionDef> {
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

fn validate_declaration(
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
fn resolve_row(def: &FunctionDef, declarations: &EffectDeclarations) -> Result<Option<EffectSet>> {
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
