mod analyze;
mod bindings;
mod comprehension;
mod control;
mod declared_errors;
mod handled;
mod loops;
mod matching;
mod shell;
mod side;
mod support;

use anyhow::Result;

use crate::traits::TraitImplRegistry;
use crate::{
    ast::{Statement, StatementKind},
    codegen::FunctionRegistry,
    env::{EnumRegistry, Env},
    errors::{ErrorSet, ErrorType},
    types::{AstString as String, Map as HashMap},
};
use declared_errors::{exposed_errors, validate_function_errors};

pub use side::{Effect, EffectSet, FunctionEffects};
pub(crate) use side::{analyze_function_effects, describe_function_effects, top_level_effects};

pub(crate) type FunctionErrorRegistry = HashMap<String, ErrorSet>;
type TaskErrorRegistry = HashMap<String, ErrorSet>;

fn analyze_function(
    def: &crate::ast::FunctionDef,
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
) -> Result<ErrorSet> {
    let mut env = globals.clone();
    let mut tasks = TaskErrorRegistry::default();
    for param in &def.params {
        env.insert(
            param.name.clone(),
            support::binding_for_type(&param.name, param.ty.clone()),
        );
    }
    analyze::block_errors(
        &def.body,
        &mut env,
        &mut tasks,
        functions,
        impls,
        enums,
        function_errors,
    )
}

pub(crate) fn analyze_function_errors(
    program: &[Statement],
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<FunctionErrorRegistry> {
    let mut registry = FunctionErrorRegistry::default();
    for statement in program {
        if let StatementKind::Function(def) = &statement.kind {
            registry.insert(def.name.clone(), exposed_errors(def, &ErrorSet::default()));
        }
    }

    for _ in 0..=registry.len() {
        let mut changed = false;
        for statement in program {
            let StatementKind::Function(def) = &statement.kind else {
                continue;
            };
            let inferred = analyze_function(def, globals, functions, impls, enums, &registry)?;
            let errors = exposed_errors(def, &inferred);
            if registry.get(&def.name) != Some(&errors) {
                registry.insert(def.name.clone(), errors);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    for statement in program {
        let StatementKind::Function(def) = &statement.kind else {
            continue;
        };
        let inferred = analyze_function(def, globals, functions, impls, enums, &registry)?;
        validate_function_errors(def, &inferred, enums)?;
    }

    // Re-walk the top-level program so checks that live inside the
    // effects pass (currently: match exhaustiveness) also fire on
    // statements outside of any function.
    let mut top_env = globals.clone();
    let mut top_tasks = TaskErrorRegistry::default();
    analyze::block_errors(
        program,
        &mut top_env,
        &mut top_tasks,
        functions,
        impls,
        enums,
        &registry,
    )?;

    Ok(registry)
}

fn callee_errors(name: &str, registry: &FunctionErrorRegistry) -> ErrorSet {
    let mut errors = registry.get(name).cloned().unwrap_or_default();
    if crate::commands::lookup(name).is_some()
        || crate::codegen::host_required(name)
        || name.starts_with("std::test::")
        || name.starts_with("std::os::")
        || name.starts_with("std::io::read_")
        || matches!(
            name,
            "std::process::exec"
                | "std::process::arg"
                | "std::process::exit"
                | "std::process::output"
                | "std::string::split"
                | "std::string::to_int"
                | "std::fs::temp_dir"
                | "std::fs::is_symlink"
                | "std::fs::is_fifo"
                | "std::fs::is_readable"
                | "std::fs::is_writable"
                | "std::fs::is_executable"
                | "std::fs::mkdir"
                | "std::fs::rmdir"
                | "std::fs::touch"
                | "std::fs::remove_tree"
                | "std::fs::read_dir"
                | "std::fs::read_lines"
                | "std::fs::symlink"
                | "std::fs::hard_link"
                | "std::fs::chmod"
                | "std::fs::size"
                | "std::process::chdir"
                | "std::process::sleep"
                | "std::string::repeat"
                | "std::string::count"
                | "std::int::abs"
                | "std::int::clamp"
                | "std::terminal::is_tty"
        )
    {
        errors.insert(ErrorType::Unknown);
    }
    errors
}
