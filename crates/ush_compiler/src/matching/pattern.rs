use anyhow::{Result, bail};

use crate::types::{AstString as NameString, OutputString as String};
use crate::{
    ast::{NamedPattern, Pattern, PatternFields, Type, VariantFields, VariantPattern},
    env::{Binding, EnumRegistry, Storage, lookup_variant},
};

use super::PatternPlan;

pub(super) fn bind_pattern(
    pattern: &Pattern,
    subject: &Binding,
    enums: &EnumRegistry,
    plan: &mut PatternPlan,
) -> Result<()> {
    match (pattern, &subject.ty, &subject.storage) {
        (Pattern::Wildcard, _, _) => {}
        (Pattern::Binding(name), _, Storage::Primitive(var)) => {
            // `${var-}` rather than `${var}`: the prelude runs before
            // the tag test, so a payload slot belonging to a variant
            // that did not match is still unset and `set -u` would
            // abort the script. The value is only ever read once the
            // condition has passed.
            plan.prelude.push(format!("{name}=\"${{{var}-}}\""));
            plan.env.insert(
                name.clone(),
                Binding {
                    ty: subject.ty.clone(),
                    storage: Storage::Primitive(name.clone()),
                },
            );
        }
        (Pattern::Binding(name), _, Storage::Adt(prefix)) => {
            plan.env.insert(
                name.clone(),
                Binding {
                    ty: subject.ty.clone(),
                    storage: Storage::Adt(prefix.clone()),
                },
            );
        }
        (Pattern::Binding(name), _, Storage::Aggregate(prefix)) => {
            plan.env.insert(
                name.clone(),
                Binding {
                    ty: subject.ty.clone(),
                    storage: Storage::Aggregate(prefix.clone()),
                },
            );
        }
        (Pattern::String(value), Type::String, Storage::Primitive(var)) => add_cond(
            plan,
            format!(
                "[ \"${{{var}}}\" = {} ]",
                super::super::util::shell_quote(value)
            ),
        ),
        (Pattern::Int(value), Type::Int, Storage::Primitive(var)) => {
            add_cond(plan, format!("[ \"${{{var}}}\" = \"{value}\" ]"))
        }
        (Pattern::Bool(value), Type::Bool, Storage::Primitive(var)) => add_cond(
            plan,
            format!(
                "[ \"${{{var}}}\" = \"{}\" ]",
                if *value { "true" } else { "false" }
            ),
        ),
        (Pattern::Unit, Type::Unit, _) => add_cond(plan, ":".into()),
        (Pattern::Variant(variant), Type::Adt(enum_name), Storage::Adt(prefix)) => {
            bind_variant_pattern(prefix, enum_name, variant, enums, plan)?
        }
        _ => bail!("pattern/type mismatch"),
    }
    Ok(())
}

fn bind_variant_pattern(
    prefix: &str,
    enum_name: &str,
    pattern: &VariantPattern,
    enums: &EnumRegistry,
    plan: &mut PatternPlan,
) -> Result<()> {
    if pattern.enum_name != enum_name {
        bail!(
            "pattern enum mismatch: expected {enum_name}, found {}",
            pattern.enum_name
        );
    }
    let def = lookup_variant(enums, enum_name, &pattern.variant_name)?;
    add_cond(
        plan,
        format!(
            "[ \"${{{prefix}__tag}}\" = '{}::{}' ]",
            enum_name, pattern.variant_name
        ),
    );
    match (&def.fields, &pattern.fields) {
        (VariantFields::Unit, PatternFields::Unit) => {}
        (VariantFields::Tuple(types), PatternFields::Tuple(patterns))
            if types.len() == patterns.len() =>
        {
            for (index, (ty, inner)) in types.iter().zip(patterns).enumerate() {
                bind_pattern(
                    inner,
                    &binding_for_child(ty.clone(), format!("{prefix}__{index}"))?,
                    enums,
                    plan,
                )?;
            }
        }
        (VariantFields::Struct(fields), PatternFields::Struct(patterns)) => {
            for NamedPattern { name, pattern } in patterns {
                let field = fields
                    .iter()
                    .find(|field| field.name == name)
                    .ok_or_else(|| anyhow::anyhow!("unknown field {name}"))?;
                bind_pattern(
                    pattern,
                    &binding_for_child(field.ty.clone(), format!("{prefix}__{name}"))?,
                    enums,
                    plan,
                )?;
            }
        }
        _ => bail!(
            "pattern shape mismatch for {enum_name}::{}",
            pattern.variant_name
        ),
    }
    Ok(())
}

fn binding_for_child(ty: Type, name: String) -> Result<Binding> {
    Ok(match ty {
        Type::String | Type::Int | Type::Bool | Type::Unit => Binding {
            ty,
            storage: Storage::Primitive(NameString::from(name)),
        },
        Type::Adt(_) => Binding {
            ty,
            storage: Storage::Adt(NameString::from(name)),
        },
        Type::Tuple(_) | Type::List(_) => Binding {
            ty,
            storage: Storage::Aggregate(NameString::from(name)),
        },
        Type::Task(_) => bail!("task handles are not supported inside ADT patterns"),
    })
}

fn add_cond(plan: &mut PatternPlan, condition: String) {
    plan.condition = if plan.condition == ":" {
        condition
    } else {
        format!("{} && {}", plan.condition, condition)
    };
}
