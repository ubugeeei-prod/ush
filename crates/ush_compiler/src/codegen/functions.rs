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
        if let StatementKind::Let { name, expr } = &statement.kind {
            let ty = super::infer(expr, &env, functions, impls, enums)?;
            env.insert(name.clone(), binding_for_name(name, ty));
        }
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
    if let Some(errors) = function_errors
        .get(&def.name)
        .filter(|errors| !errors.is_empty())
    {
        out.push_str("# raises: ");
        out.push_str(&errors.render());
        out.push('\n');
    }
    out.push_str("ush_fn_");
    out.push_str(&def.name);
    out.push_str("() {\n");

    let mut env = globals.clone();
    for (index, param) in def.params.iter().enumerate() {
        let storage = param_storage(&def.name, index);
        push_line(out, &format!("{storage}=\"${}\"", index + 1), 2);
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
    if body.is_empty() {
        push_line(out, ":", 2);
    } else {
        push_output(out, &body, 2);
    }
    out.push_str("}\n\n");
    Ok(())
}

pub(crate) fn push_wait_footer(out: &mut OutputBuffer) {
    out.push_str("\nif [ -n \"$__ush_jobs\" ]; then\n");
    out.push_str("  for __ush_job in $__ush_jobs; do\n");
    out.push_str("    wait \"$__ush_job\" 2>/dev/null || true\n");
    out.push_str("  done\n");
    out.push_str("fi\n");
    out.push_str("if [ -n \"$__ush_task_files\" ]; then\n");
    out.push_str("  for __ush_task_file in $__ush_task_files; do\n");
    out.push_str("    rm -f \"$__ush_task_file\"\n");
    out.push_str("  done\n");
    out.push_str("fi\n");
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
    let mut buffer = OutputBuffer::default();
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
    format!("__ush_fn_{}_arg_{}", function_name, index)
}

fn ensure_function_attrs(def: &FunctionDef) -> Result<()> {
    let mut seen = HashSet::with_hasher(Default::default());
    for attr in &def.attrs {
        if !seen.insert(attr.name.clone()) {
            bail!("duplicate function attribute: {}::{}", def.name, attr.name);
        }
        match (attr.name.as_str(), &attr.value) {
            ("alias", Some(Expr::String(_))) => {}
            ("alias", Some(_)) => bail!("function attribute `alias` expects a string literal"),
            ("alias", None) => bail!("function attribute `alias` requires a string literal"),
            (other, _) => bail!("unsupported function attribute on `{}`: {other}", def.name),
        }
    }
    Ok(())
}
