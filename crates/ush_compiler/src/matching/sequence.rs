use anyhow::{Result, bail};

use super::super::{
    ast::{Expr, Type},
    codegen::{FunctionRegistry, compile_runtime_primitive_expr},
    env::{Binding, CodegenState, EnumRegistry, Env, Storage},
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;

use super::emit_value_to_target;

#[allow(clippy::too_many_arguments)]
pub(super) fn emit_sequence(
    target: &str,
    expr: &Expr,
    expected: &Type,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    match (expected, expr) {
        (Type::Tuple(types), Expr::Tuple(items)) if types.len() == items.len() => {
            out.push_str(&format!("{target}__len='{}'\n", items.len()));
            for (index, (ty, item)) in types.iter().zip(items).enumerate() {
                emit_value_to_target(
                    &format!("{target}__{index}"),
                    item,
                    ty,
                    env,
                    functions,
                    impls,
                    enums,
                    state,
                    inside_function,
                    out,
                )?;
            }
            Ok(())
        }
        (Type::List(item_ty), Expr::List(items)) => {
            out.push_str(&format!("{target}__len='{}'\n", items.len()));
            for (index, item) in items.iter().enumerate() {
                emit_value_to_target(
                    &format!("{target}__{index}"),
                    item,
                    item_ty,
                    env,
                    functions,
                    impls,
                    enums,
                    state,
                    inside_function,
                    out,
                )?;
            }
            Ok(())
        }
        (Type::List(item_ty), Expr::Range { start, end }) if **item_ty == Type::Int => {
            let start = compile_runtime_primitive_expr(
                start,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?;
            let end = compile_runtime_primitive_expr(
                end,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?;
            let index = state.temp_var("range_index");
            let current = state.temp_var("range_current");
            out.push_str(&format!("{target}__len=$(({end} - {start}))\n"));
            out.push_str(&format!("{index}=0\n{current}={start}\n"));
            out.push_str(&format!("while [ \"$${current}\" -lt {end} ]; do\n").replace("$$", "$"));
            out.push_str(
                &format!("  eval \"{target}__$${index}=\\\"$${current}\\\"\"\n").replace("$$", "$"),
            );
            out.push_str(&format!("  {index}=$(({index} + 1))\n"));
            out.push_str(&format!("  {current}=$(({current} + 1))\n"));
            out.push_str("done\n");
            Ok(())
        }
        (Type::Tuple(_), Expr::Var(name)) | (Type::List(_), Expr::Var(name)) => {
            let binding = env
                .get(name)
                .ok_or_else(|| anyhow::anyhow!("unknown variable: {name}"))?;
            copy_sequence(target, binding, enums, state, out)
        }
        _ => bail!("expected structured value for {target}"),
    }
}

pub(super) fn copy_sequence(
    target: &str,
    binding: &Binding,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<()> {
    let Storage::Aggregate(source) = &binding.storage else {
        bail!("cannot copy non-structured value into {target}");
    };
    match &binding.ty {
        Type::Tuple(types) => {
            out.push_str(&format!("{target}__len='{}'\n", types.len()));
            for (index, ty) in types.iter().enumerate() {
                copy_item(
                    &format!("{source}__{index}"),
                    &format!("{target}__{index}"),
                    ty,
                    enums,
                    state,
                    out,
                )?;
            }
            Ok(())
        }
        Type::List(item_ty) => copy_list(source, target, item_ty, enums, state, out),
        _ => bail!("expected structured binding"),
    }
}

fn copy_list(
    source: &str,
    target: &str,
    item_ty: &Type,
    _enums: &EnumRegistry,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<()> {
    match item_ty {
        Type::String | Type::Int | Type::Bool | Type::Unit => {
            let index = state.temp_var("copy_index");
            let value = state.temp_var("copy_value");
            out.push_str(&format!("{target}__len=\"${{{source}__len}}\"\n"));
            out.push_str(&format!("{index}=0\n"));
            out.push_str(
                &format!("while [ \"$${index}\" -lt \"${{{source}__len}}\" ]; do\n")
                    .replace("$$", "$"),
            );
            out.push_str(
                &format!("  eval \"{value}=\\\"\\${{{source}__$${index}}}\\\"\"\n")
                    .replace("$$", "$"),
            );
            out.push_str(
                &format!("  eval \"{target}__$${index}=\\\"$${value}\\\"\"\n").replace("$$", "$"),
            );
            out.push_str(&format!("  {index}=$(({index} + 1))\n"));
            out.push_str("done\n");
            Ok(())
        }
        _ => {
            crate::codegen::wire::copy(
                &Type::List(alloc::boxed::Box::new(item_ty.clone())),
                source,
                target,
                state,
                out,
            );
            Ok(())
        }
    }
}

fn copy_item(
    source: &str,
    target: &str,
    ty: &Type,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<()> {
    match ty {
        Type::String | Type::Int | Type::Bool | Type::Unit => {
            out.push_str(&format!("{target}=\"${{{source}}}\"\n"));
            Ok(())
        }
        Type::Adt(_) => super::emit::emit_copy(
            target,
            ty.render().as_str(),
            &Binding {
                ty: ty.clone(),
                storage: Storage::Adt(source.into()),
            },
            enums,
            state,
            out,
        ),
        Type::Tuple(_) | Type::List(_) => copy_sequence(
            target,
            &Binding {
                ty: ty.clone(),
                storage: Storage::Aggregate(source.into()),
            },
            enums,
            state,
            out,
        ),
        Type::Task(_) => bail!("task handles cannot live inside structured values"),
    }
}
