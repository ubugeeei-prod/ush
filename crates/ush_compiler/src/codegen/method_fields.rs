use anyhow::{Result, anyhow, bail};

use super::super::{
    ast::{NamedFieldType, Type, VariantFields},
    env::{Binding, EnumRegistry, Storage},
};

pub(super) fn field_binding(base: &Binding, name: &str, enums: &EnumRegistry) -> Result<Binding> {
    let Storage::Adt(prefix) = &base.storage else {
        bail!("field access requires a struct-like value");
    };
    let field = struct_field(&base.ty, name, enums)?;
    Ok(binding_for_field(prefix, field.ty.clone(), name))
}

pub(super) fn struct_field<'a>(
    ty: &Type,
    name: &str,
    enums: &'a EnumRegistry,
) -> Result<&'a NamedFieldType> {
    let Type::Adt(enum_name) = ty else {
        bail!("field access requires a struct-like value");
    };
    let enum_def = enums
        .get(enum_name)
        .ok_or_else(|| anyhow!("unknown enum: {enum_name}"))?;
    let [variant] = enum_def.variants.as_slice() else {
        bail!("field access is only supported on `type` structs");
    };
    // A module's `type gh::Entry` keeps its variant name `Entry`.
    if Some(variant.name.as_str()) != enum_name.rsplit("::").next() {
        bail!("field access is only supported on `type` structs");
    }
    let VariantFields::Struct(fields) = &variant.fields else {
        bail!("field access requires a struct-like value");
    };
    fields
        .iter()
        .find(|field| field.name == name)
        .ok_or_else(|| anyhow!("unknown field `{name}` on `{enum_name}`"))
}

fn binding_for_field(prefix: &str, ty: Type, name: &str) -> Binding {
    let storage = match ty {
        Type::String | Type::Int | Type::Bool | Type::Unit => {
            Storage::Primitive(format!("{prefix}__{name}").into())
        }
        Type::Adt(_) => Storage::Adt(format!("{prefix}__{name}").into()),
        Type::Tuple(_) | Type::List(_) => Storage::Aggregate(format!("{prefix}__{name}").into()),
        Type::Task(_) => Storage::Task(format!("{prefix}__{name}").into(), Default::default()),
    };
    Binding { ty, storage }
}
