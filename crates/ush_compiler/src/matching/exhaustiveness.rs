use anyhow::{Result, bail};

use crate::ast::{Pattern, Type};
use crate::env::EnumRegistry;
use crate::types::{AstString as Name, HeapVec as Vec};

pub(crate) fn check_exhaustive(
    subject_ty: &Type,
    arms: &[(Pattern, alloc::boxed::Box<crate::ast::Statement>)],
    enums: &EnumRegistry,
) -> Result<()> {
    let mut patterns = Vec::new();
    for (pattern, _) in arms {
        flatten(pattern, &mut patterns);
    }
    if patterns.iter().any(|pattern| match pattern {
        Pattern::Wildcard | Pattern::Binding(_) => true,
        Pattern::Glob(glob) => glob == "*",
        _ => false,
    }) {
        return Ok(());
    }
    let arms = patterns.as_slice();

    match subject_ty {
        Type::Adt(enum_name) => check_adt(enum_name, arms, enums),
        Type::Unit => {
            if arms.iter().any(|pattern| matches!(pattern, Pattern::Unit)) {
                Ok(())
            } else {
                bail!("non-exhaustive `match` on Unit: add `()` or a wildcard arm",);
            }
        }
        Type::Bool => check_bool(arms),
        Type::String | Type::Int => bail!(
            "non-exhaustive `match` on {}: literal arms cannot cover the whole type, \
             add a wildcard `_` arm",
            type_label(subject_ty),
        ),
        Type::Tuple(_) | Type::List(_) => bail!(
            "non-exhaustive `match` on {}: add a wildcard `_` arm",
            type_label(subject_ty),
        ),
        Type::Task(_) => bail!("cannot `match` on a task handle"),
    }
}

/// Alternatives of a `|` pattern count as arms of their own.
fn flatten<'a>(pattern: &'a Pattern, out: &mut Vec<&'a Pattern>) {
    match pattern {
        Pattern::Or(alternatives) => alternatives.iter().for_each(|alt| flatten(alt, out)),
        other => out.push(other),
    }
}

fn check_adt(enum_name: &str, arms: &[&Pattern], enums: &EnumRegistry) -> Result<()> {
    let def = enums
        .get(enum_name)
        .ok_or_else(|| anyhow::anyhow!("unknown enum: {enum_name}"))?;

    let mut covered: Vec<Name> = Vec::new();
    for pattern in arms {
        if let Pattern::Variant(variant) = pattern {
            if variant.enum_name != enum_name {
                bail!(
                    "pattern enum mismatch: expected {enum_name}, found {}",
                    variant.enum_name,
                );
            }
            if !covered.contains(&variant.variant_name) {
                covered.push(variant.variant_name.clone());
            }
        }
    }

    let mut missing: Vec<Name> = Vec::new();
    for variant in &def.variants {
        if !covered.contains(&variant.name) {
            missing.push(variant.name.clone());
        }
    }

    if !missing.is_empty() {
        let formatted = missing
            .iter()
            .map(|name| format!("{enum_name}::{name}"))
            .collect::<alloc::vec::Vec<_>>()
            .join(", ");
        bail!("non-exhaustive `match` on enum {enum_name}: missing variant(s): {formatted}",);
    }

    Ok(())
}

fn check_bool(arms: &[&Pattern]) -> Result<()> {
    let mut has_true = false;
    let mut has_false = false;
    for pattern in arms {
        if let Pattern::Bool(value) = pattern {
            if *value {
                has_true = true;
            } else {
                has_false = true;
            }
        }
    }
    if has_true && has_false {
        Ok(())
    } else {
        let missing = if has_true { "false" } else { "true" };
        bail!("non-exhaustive `match` on Bool: missing `{missing}` arm or wildcard",);
    }
}

fn type_label(ty: &Type) -> &'static str {
    match ty {
        Type::String => "String",
        Type::Int => "Int",
        Type::Bool => "Bool",
        Type::Unit => "Unit",
        Type::Adt(_) => "enum",
        Type::Tuple(_) => "tuple",
        Type::List(_) => "List",
        Type::Task(_) => "Task",
    }
}
