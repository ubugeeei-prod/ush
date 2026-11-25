use anyhow::{Result, bail};

use super::{
    super::{
        ast::{Expr, FunctionDef, Statement, StatementKind},
        effects::FunctionErrorRegistry,
        env::{CodegenState, EnumRegistry, Env},
    },
    calls::ensure_signature,
    shared::{binding_for_name, push_line, push_output},
    statement::compile_statement,
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;
use crate::types::{
    AstString as NameString, Map as HashMap, OutputString as String, Set as HashSet,
};

pub(crate) type FunctionRegistry = HashMap<NameString, FunctionDef>;

pub(crate) fn register_function(def: &FunctionDef, functions: &mut FunctionRegistry) -> Result<()> {
    ensure_function_attrs(def)?;
    if functions.contains_key(&def.name) {
        bail!("duplicate function: {}", def.name);
    }
    let mut cli_aliases = HashSet::with_hasher(Default::default());
    for (index, param) in def.params.iter().enumerate() {
        if def.params[..index]
            .iter()
            .any(|other| other.name == param.name)
        {
            bail!("duplicate parameter: {}::{}", def.name, param.name);
        }
        if let Some(alias) = &param.cli_alias {
            if alias.len() != 1 || !alias.chars().all(|ch| ch.is_ascii_alphanumeric()) {
                bail!(
                    "parameter alias must be a single ASCII letter or digit: {}::{}",
                    def.name,
                    param.name
                );
            }
            if !cli_aliases.insert(alias.clone()) {
                bail!("duplicate parameter alias: {}::{alias}", def.name);
            }
        }
    }
    functions.insert(def.name.clone(), def.clone());
    Ok(())
}

pub(crate) fn analyze_globals(
    program: &[Statement],
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Env> {
    let mut env = Env::default();
    for statement in program {
        let (name, ty) = match &statement.kind {
            StatementKind::Let { name, expr } => {
                (name, super::infer(expr, &env, functions, impls, enums)?)
            }
            StatementKind::Spawn { name, call } => (
                name,
                crate::ast::Type::Task(alloc::boxed::Box::new(super::calls::call_expr_type(
                    call, functions,
                )?)),
            ),
            StatementKind::Await { name, task } => {
                let Some(crate::ast::Type::Task(ty)) = env.get(task).map(|b| &b.ty) else {
                    continue;
                };
                (name, (**ty).clone())
            }
            _ => continue,
        };
        env.insert(name.clone(), binding_for_name(name, ty));
    }
    Ok(env)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_function(
    def: &FunctionDef,
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<()> {
    ensure_signature(def)?;
    if def
        .return_type
        .as_ref()
        .is_some_and(|ty| *ty != crate::ast::Type::Unit)
        && !crate::contracts::completes(&def.body, true)
    {
        bail!("function {} can finish without returning a value", def.name);
    }
    if let Some(errors) = function_errors
        .get(&def.name)
        .filter(|errors| !errors.is_empty())
    {
        out.push_str("# raises: ");
        out.push_str(&errors.render());
        out.push('\n');
    }
    out.push_str(&super::shared::shell_function_name(&def.name));
    out.push_str("() {\n");

    let mut env = globals.clone();
    for (index, param) in def.params.iter().enumerate() {
        let storage = param_storage(&def.name, index);
        if param.ty.is_structured() {
            super::wire::register(&param.ty, state);
            let mut unpack = OutputBuffer::new(state.mapped);
            super::wire::decode(
                &param.ty,
                &storage,
                &format!("\"${{{}}}\"", index + 1),
                &mut unpack,
            );
            push_output(out, &unpack, 2);
        } else {
            push_line(out, &format!("{storage}=\"${{{}}}\"", index + 1), 2);
            // Preserve source names for embedded shell, even when typed code uses only storage.
            push_line(out, "# shellcheck disable=SC2034", 2);
            push_line(out, &format!("{}=\"${{{storage}}}\"", param.name), 2);
        }
        env.insert(
            param.name.clone(),
            binding_for_name(&storage, param.ty.clone()),
        );
    }

    let body = compile_many(
        &def.body,
        &mut env,
        globals,
        functions,
        impls,
        enums,
        function_errors,
        state,
        def.return_type.as_ref(),
    )?;
    if body.as_str().contains("__ush_jobs=") {
        out.push_str("  (\n    __ush_task_scope\n");
        push_output(out, &body, 4);
        out.push_str("  )\n");
    } else if body.is_empty() {
        push_line(out, ":", 2);
    } else {
        push_output(out, &body, 2);
    }
    out.push_str("}\n\n");
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn compile_many(
    statements: &[Statement],
    env: &mut Env,
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
    state: &mut CodegenState,
    return_type: Option<&super::super::ast::Type>,
) -> Result<OutputBuffer> {
    let mut buffer = OutputBuffer::new(state.mapped);
    for (index, statement) in statements.iter().enumerate() {
        compile_statement(
            statement,
            env,
            globals,
            functions,
            impls,
            enums,
            function_errors,
            state,
            return_type,
            true,
            index + 1 == statements.len(),
            &mut buffer,
        )?;
    }
    Ok(buffer)
}

fn param_storage(function_name: &str, index: usize) -> String {
    format!(
        "__{}_arg_{}",
        super::shared::shell_function_name(function_name),
        index
    )
}

fn ensure_function_attrs(def: &FunctionDef) -> Result<()> {
    let mut seen = HashSet::with_hasher(Default::default());
    for attr in &def.attrs {
        if !seen.insert(attr.name.clone()) {
            bail!("duplicate function attribute: {}::{}", def.name, attr.name);
        }
        match (attr.name.as_str(), &attr.value) {
            ("alias" | "test", Some(Expr::String(_))) => {}
            ("alias", Some(_)) => bail!("function attribute `alias` expects a string literal"),
            ("alias", None) => bail!("function attribute `alias` requires a string literal"),
            (other, _) => bail!("unsupported function attribute on `{}`: {other}", def.name),
        }
    }
    Ok(())
}
