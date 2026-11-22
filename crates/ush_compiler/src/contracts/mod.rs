//! Checked source types are erased after call-site instantiation.
mod body;
mod expressions;
mod instantiate;
mod refine;
mod returns;
pub(crate) use returns::completes;
mod signature;
mod types;
mod unify;

use crate::{
    ast::{FunctionDef, Statement, StatementKind},
    codegen::FunctionRegistry,
    env::EnumRegistry,
    traits::TraitImplRegistry,
    types::{AstString as Name, HeapVec as Vec, Map, Set},
};
use anyhow::{Context, Result, ensure};
pub(crate) use signature::{Scheme, header};
use types::Ty;

type Locals = Map<Name, Ty>;
type InstanceKey = (Name, Vec<Ty>, Vec<(Name, types::Index)>);

struct Checker {
    templates: Map<Name, (usize, FunctionDef)>,
    functions: FunctionRegistry,
    enums: EnumRegistry,
    impls: TraitImplRegistry,
    units: Set<Name>,
    instances: Map<InstanceKey, (Name, Ty)>,
    generated: Vec<Statement>,
    globals: Locals,
    checking: Set<Name>,
    symbolic: bool,
    declared_failures: Option<crate::errors::ErrorSet>,
}

pub(crate) fn elaborate(mut program: Vec<Statement>) -> Result<Vec<Statement>> {
    if !program.iter().any(|s| {
        matches!(
            &s.kind,
            StatementKind::UnitDef(_)
                | StatementKind::Function(FunctionDef {
                    scheme: Some(_),
                    ..
                })
        )
    }) {
        return Ok(program);
    }
    crate::effects::analyze_function_effects(&program)?;
    let mut checker = Checker {
        templates: Map::default(),
        functions: Map::default(),
        enums: Map::default(),
        impls: TraitImplRegistry::default(),
        units: Set::default(),
        instances: Map::default(),
        generated: Vec::new(),
        globals: Map::default(),
        checking: Set::default(),
        symbolic: true,
        declared_failures: None,
    };
    crate::codegen::register_builtins(&mut checker.functions)?;
    let mut traits = crate::traits::TraitRegistry::default();
    for s in &program {
        match &s.kind {
            StatementKind::UnitDef(n) => ensure!(
                checker.units.insert(n.clone()),
                "duplicate measurement unit: {n}"
            ),
            StatementKind::Function(def) => {
                ensure!(
                    checker
                        .functions
                        .insert(def.name.clone(), def.clone())
                        .is_none(),
                    "duplicate function: {}",
                    def.name
                );
                if def.scheme.is_some() {
                    checker
                        .templates
                        .insert(def.name.clone(), (s.line, def.clone()));
                }
            }
            StatementKind::Enum(d) => {
                checker.enums.insert(d.name.clone(), d.clone());
            }
            StatementKind::Trait(d) => crate::traits::register_trait(d, &mut traits)?,
            StatementKind::Impl(d) => {
                crate::traits::register_trait_impl(d, &traits, &mut checker.impls)?
            }
            StatementKind::Effect(d) => {
                checker
                    .functions
                    .insert(d.name.clone(), crate::codegen::effect_dispatcher(d));
            }
            _ => {}
        }
    }
    let nominal = checker.enums.keys().cloned().collect();
    for (line, def) in checker.templates.values() {
        let scheme = def.scheme.as_ref().unwrap();
        for t in scheme
            .params
            .iter()
            .map(|p| &p.1)
            .chain(scheme.result.iter())
        {
            unify::validate_type(t, scheme, &checker.units, &nominal)
                .with_context(|| format!("line {line}"))?;
        }
    }
    let mut globals = Locals::default();
    // A symbolic pass checks every generic body, including unused definitions.
    for s in &program {
        if !matches!(s.kind, StatementKind::Function(_) | StatementKind::Impl(_)) {
            checker
                .statement(&mut s.clone(), &mut globals, &mut Vec::new(), false)
                .with_context(|| format!("line {}", s.line))?;
        }
    }
    checker.globals = globals;
    for s in &program {
        if let StatementKind::Function(def) = &s.kind
            && let Some(scheme) = &def.scheme
        {
            let args = scheme.params.iter().map(|pair| pair.1.clone()).collect();
            checker
                .instance(def, args, unify::Substitution::default())
                .with_context(|| format!("line {}", s.line))?;
        }
    }
    checker.symbolic = false;
    for s in &program {
        if let StatementKind::Function(def) = &s.kind
            && let Some(scheme) = &def.scheme
            && scheme.variables.is_empty()
        {
            let args = scheme.params.iter().map(|p| p.1.clone()).collect();
            checker
                .instance(def, args, unify::Substitution::default())
                .with_context(|| format!("line {}", s.line))?;
        }
    }
    program.retain(|s| {
        !matches!(
            &s.kind,
            StatementKind::UnitDef(_)
                | StatementKind::Function(FunctionDef {
                    scheme: Some(_),
                    ..
                })
        )
    });
    let mut globals = checker.globals.clone();
    for s in &mut program {
        match &mut s.kind {
            StatementKind::Function(_) | StatementKind::Impl(_) => {
                let methods = match &mut s.kind {
                    StatementKind::Function(def) => core::slice::from_mut(def),
                    StatementKind::Impl(item) => item.methods.as_mut_slice(),
                    _ => unreachable!(),
                };
                for def in methods {
                    let mut env = checker.globals.clone();
                    if let Some(ty) = &def.receiver {
                        env.insert("self".into(), Ty::from_type(ty));
                    }
                    env.extend(
                        def.params
                            .iter()
                            .map(|p| (p.name.clone(), Ty::from_type(&p.ty))),
                    );
                    let returns = checker.body(&mut def.body, &mut env)?;
                    if let Some(expected) = &def.return_type {
                        for actual in returns {
                            Ty::from_type(expected).check(&actual)?;
                        }
                    }
                }
            }
            _ => {
                checker
                    .statement(s, &mut globals, &mut Vec::new(), false)
                    .with_context(|| format!("line {}", s.line))?;
            }
        }
    }
    // Function definitions must be available before top-level calls reach them.
    let mut result = checker.generated;
    result.extend(program);
    Ok(result)
}
