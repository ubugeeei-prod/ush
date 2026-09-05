use anyhow::{Result, anyhow, bail};

use super::ast::{EnumDef, Type, VariantDef};
use crate::types::{AstString as String, Map as HashMap};

#[derive(Debug, Clone)]
pub(crate) struct Binding {
    pub ty: Type,
    pub storage: Storage,
}

#[derive(Debug, Clone)]
pub(crate) enum Storage {
    Primitive(String),
    Adt(String),
    Aggregate(String),
    Task(String),
}

pub(crate) type Env = HashMap<String, Binding>;
pub(crate) type EnumRegistry = HashMap<String, EnumDef>;

#[derive(Debug, Default)]
pub(crate) struct CodegenState {
    next_id: usize,
}

impl CodegenState {
    pub(crate) fn temp_var(&mut self, prefix: &str) -> String {
        format!("__ush_{prefix}_{}", self.next_handler_id()).into()
    }

    /// A fresh id for anything that needs a unique shell name.
    pub(crate) fn next_handler_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

pub(crate) fn lookup_variant<'a>(
    enums: &'a EnumRegistry,
    enum_name: &str,
    variant_name: &str,
) -> Result<&'a VariantDef> {
    let enum_def = enums
        .get(enum_name)
        .ok_or_else(|| anyhow!("unknown enum: {enum_name}"))?;
    enum_def
        .variants
        .iter()
        .find(|variant| variant.name == variant_name)
        .ok_or_else(|| anyhow!("unknown variant: {enum_name}::{variant_name}"))
}

pub(crate) fn expect_adt(ty: &Type) -> Result<&str> {
    match ty {
        Type::Adt(name) => Ok(name.as_str()),
        _ => bail!("expected ADT, found {ty:?}"),
    }
}
