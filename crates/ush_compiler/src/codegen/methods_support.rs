use anyhow::{Result, anyhow, bail};

use super::{
    super::{
        ast::{Expr, FunctionDef, MethodCall, Type},
        env::{Binding, CodegenState, EnumRegistry, Env, Storage},
        matching::materialize_expr,
    },
    FunctionRegistry, compile_runtime_primitive_expr, infer,
    statement::compile_statement,
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;
use crate::types::{AstVec as Vec, OutputString as String};

#[allow(clippy::too_many_arguments)]
pub(super) fn bind_method_args(
    args: &[Expr],
    def: &FunctionDef,
    method_env: &mut Env,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<()> {
    for (param, arg) in def.params.iter().zip(args) {
        let actual = infer(arg, env, functions, impls, enums)?;
        if actual != param.ty {
            bail!("type mismatch for method parameter `{}`", param.name);
        }
        let value = materialize_named_value(
            &param.name,
            arg,
            &param.ty,
            env,
            functions,
            impls,
            enums,
            state,
            out,
        )?;
        method_env.insert(param.name.clone(), value);
    }
    Ok(())
}

pub(super) fn compile_method_body(
    def: &FunctionDef,
    env: &mut Env,
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
) -> Result<String> {
    let mut out = OutputBuffer::new(state.mapped);
    for (index, statement) in def.body.iter().enumerate() {
        compile_statement(
            statement,
            env,
            globals,
            functions,
            impls,
            enums,
            &Default::default(),
            state,
            def.return_type.as_ref(),
            true,
            index + 1 == def.body.len(),
            &mut out,
        )?;
    }
    Ok(out.into_compiled(None).shell)
}

pub(super) fn resolve_method_args<'a>(
    call: &'a MethodCall,
    def: &'a FunctionDef,
) -> Result<Vec<&'a Expr>> {
    if call.args.len() > def.params.len() {
        bail!(
            "method `{}` expects at most {} arguments",
            call.method,
            def.params.len()
        );
    }
    def.params
        .iter()
        .enumerate()
        .map(|(index, param)| {
            call.args
                .get(index)
                .map(|arg| &arg.expr)
                .or(param.default.as_ref())
                .ok_or_else(|| {
                    anyhow!(
                        "missing argument for method `{}`: {}",
                        call.method,
                        param.name
                    )
                })
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn materialize_named_value(
    name: &str,
    expr: &Expr,
    ty: &Type,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<Binding> {
    let prefix = state.temp_var(name);
    match ty {
        Type::String | Type::Int | Type::Bool | Type::Unit => {
            let rendered = compile_runtime_primitive_expr(
                expr, env, functions, impls, enums, state, true, out,
            )?;
            out.push_str(&prefix);
            out.push('=');
            out.push_str(&rendered);
            out.push('\n');
            Ok(Binding {
                ty: ty.clone(),
                storage: Storage::Primitive(prefix),
            })
        }
        Type::Adt(_) | Type::Tuple(_) | Type::List(_) => {
            copy_named_binding(expr, env, functions, impls, enums, state, out)
        }
        Type::Task(_) => bail!("task handles are not valid method arguments"),
    }
}

fn copy_named_binding(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<Binding> {
    let binding = materialize_expr(expr, env, functions, impls, enums, state, true, out)?;
    Ok(match binding.storage {
        Storage::Primitive(_) => binding,
        Storage::Adt(_) => Binding {
            ty: binding.ty,
            storage: Storage::Adt(binding_name(binding.storage).into()),
        },
        Storage::Aggregate(_) => Binding {
            ty: binding.ty,
            storage: Storage::Aggregate(binding_name(binding.storage).into()),
        },
        Storage::Task(_, _) => bail!("task handles are not valid method arguments"),
    })
}

fn binding_name(storage: Storage) -> String {
    match storage {
        Storage::Primitive(name)
        | Storage::Adt(name)
        | Storage::Aggregate(name)
        | Storage::Task(name, _) => name.into(),
    }
}
