//! The declarations every pass after linking looks names up in.

use anyhow::Result;

use super::{effects_lowering, enum_registry, functions, stdlib, uses_std};
use crate::{
    ast::{Statement, StatementKind},
    effects::{self, FunctionErrorRegistry},
    env::EnumRegistry,
    traits::{TraitImplRegistry, TraitRegistry, register_trait, register_trait_impl},
};

pub(super) struct Registries {
    pub functions: functions::FunctionRegistry,
    pub enums: EnumRegistry,
    pub trait_impls: TraitImplRegistry,
}

impl Registries {
    pub(super) fn build(program: &[Statement]) -> Result<Self> {
        let mut functions = functions::FunctionRegistry::default();
        let mut enums = EnumRegistry::default();
        let mut traits = TraitRegistry::default();
        let mut trait_impls = TraitImplRegistry::default();
        if uses_std::references(program) {
            stdlib::register_builtins(&mut functions)?;
        }
        for statement in program {
            match &statement.kind {
                StatementKind::Enum(def) => enum_registry::register_enum(def, &mut enums)?,
                StatementKind::Trait(def) => register_trait(def, &mut traits)?,
                StatementKind::Impl(item) => {
                    register_trait_impl(item, &traits, &mut trait_impls)?;
                }
                StatementKind::Function(def) => functions::register_function(def, &mut functions)?,
                // An effect operation is a call target like any other:
                // the declaration generates the dispatcher function that
                // `do name(…)` resolves to.
                StatementKind::Effect(def) => functions::register_function(
                    &effects_lowering::effect_dispatcher(def),
                    &mut functions,
                )?,
                _ => {}
            }
        }
        Ok(Self {
            functions,
            enums,
            trait_impls,
        })
    }
}

/// What every function's body can raise, with callers seeing written
/// rows where there are any.
pub(crate) fn error_rows(program: &[Statement]) -> Result<FunctionErrorRegistry> {
    let registries = Registries::build(program)?;
    let globals = functions::analyze_globals(
        program,
        &registries.functions,
        &registries.trait_impls,
        &registries.enums,
    )?;
    let (functions, impls, enums) = (
        &registries.functions,
        &registries.trait_impls,
        &registries.enums,
    );
    let rows = effects::analyze_function_errors(program, &globals, functions, impls, enums)?;
    effects::body_errors(program, &globals, functions, impls, enums, &rows)
}
