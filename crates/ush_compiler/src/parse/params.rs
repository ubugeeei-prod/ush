//! Parameter lists, and the `name(args) tail` shape every
//! declaration header shares.

use anyhow::{Result, anyhow};

use super::{
    super::{
        ast::FunctionParam,
        util::{parse_type, split_once_top_level, split_top_level},
    },
    attr,
    path::parse_identifier,
    signature::{attr_expr, attr_string},
};
use crate::types::HeapVec as Vec;

pub(super) fn parse_params(source: &str) -> Result<Vec<FunctionParam>> {
    split_top_level(source, ',')
        .into_iter()
        .filter(|part| !part.is_empty())
        .map(|part| {
            let (attrs, rest) = attr::parse_inline_attrs(part)?;
            let (name, ty) = split_once_top_level(rest, ':')
                .ok_or_else(|| anyhow!("invalid parameter: {part}"))?;
            Ok(FunctionParam {
                name: parse_identifier(name)?,
                ty: parse_type(ty).ok_or_else(|| anyhow!("invalid type: {ty}"))?,
                default: attr_expr(&attrs, "default")?,
                cli_alias: attr_string(&attrs, "alias")?,
            })
        })
        .collect()
}
