//! `effect log(message: String) -> ()` — effect operation
//! declarations.

use anyhow::{Result, bail};

use super::{
    super::ast::{EffectDef, FunctionParam, StatementKind},
    params::parse_params,
    path::parse_identifier,
    returns::parse_function_return,
    signature::split_paren_form,
};
use crate::types::{AstString as String, HeapVec as Vec};

/// `effect log(message: String) -> ()`
pub(super) fn parse_effect(header: &str) -> Result<StatementKind> {
    let (name, params, return_type) = parse_effect_header(header.trim())?;
    Ok(StatementKind::Effect(EffectDef {
        name,
        params,
        return_type,
    }))
}

/// `effect log(message: String) -> ()`, or `effect log` for an
/// operation that takes and returns nothing.
pub(super) fn parse_effect_header(
    header: &str,
) -> Result<(String, Vec<FunctionParam>, Option<super::super::ast::Type>)> {
    match split_paren_form(header) {
        Some((name, inner, tail)) => {
            let (return_type, errors, effects) = parse_function_return(tail)?;
            if errors.is_some() {
                bail!("an effect operation cannot declare an error set");
            }
            if effects.is_some() {
                bail!("an effect operation cannot declare an effect row");
            }
            Ok((parse_identifier(name)?, parse_params(inner)?, return_type))
        }
        None => Ok((
            parse_identifier(header.trim())?,
            Vec::new(),
            Some(super::super::ast::Type::Unit),
        )),
    }
}
