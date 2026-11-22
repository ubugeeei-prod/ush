use anyhow::{Result, bail};

use super::super::{
    ast::{ExprFields, VariantExpr, VariantFields},
    codegen::FunctionRegistry,
    env::{Binding, CodegenState, EnumRegistry, Env, Storage, lookup_variant},
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;

use super::emit_value_to_target;

#[allow(clippy::too_many_arguments)]
pub(super) fn emit_variant(
    target: &str,
    variant: &VariantExpr,
    expected_enum: &str,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    if variant.enum_name != expected_enum {
        bail!("expected {expected_enum}, found {}", variant.enum_name);
    }
    let def = lookup_variant(enums, &variant.enum_name, &variant.variant_name)?;
    out.push_str(&format!(
        "{target}__tag='{}::{}'\n",
        variant.enum_name, variant.variant_name
    ));
    match (&def.fields, &variant.fields) {
        (VariantFields::Unit, ExprFields::Unit) => {}
        (VariantFields::Tuple(types), ExprFields::Tuple(values)) if types.len() == values.len() => {
            for (index, (ty, expr)) in types.iter().zip(values).enumerate() {
                emit_value_to_target(
                    &format!("{target}__{index}"),
                    expr,
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
        }
        (VariantFields::Struct(fields), ExprFields::Struct(values)) => {
            for value in values {
                if !fields.iter().any(|field| field.name == value.name) {
                    bail!("unknown field {}", value.name);
                }
            }
            for field in fields {
                let expr = values
                    .iter()
                    .find(|item| item.name == field.name)
                    .ok_or_else(|| anyhow::anyhow!("missing field {}", field.name))?;
                emit_value_to_target(
                    &format!("{target}__{}", field.name),
                    &expr.expr,
                    &field.ty,
                    env,
                    functions,
                    impls,
                    enums,
                    state,
                    inside_function,
                    out,
                )?;
            }
        }
        _ => bail!(
            "variant field shape mismatch for {}::{}",
            variant.enum_name,
            variant.variant_name
        ),
    }
    Ok(())
}

pub(super) fn emit_copy(
    target: &str,
    _enum_name: &str,
    binding: &Binding,
    _enums: &EnumRegistry,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) -> Result<()> {
    let Storage::Adt(source) = &binding.storage else {
        bail!("cannot copy primitive into ADT target");
    };
    crate::codegen::wire::copy(&binding.ty, source, target, state, out);
    Ok(())
}
