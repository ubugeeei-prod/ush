use anyhow::{Context, Result, anyhow, ensure};

use super::{FunctionRegistry, binding_for_name, infer};
use crate::{
    ast::{MatchValue, StatementKind, Type},
    env::{EnumRegistry, Env},
    matching::{check_exhaustive, compile_pattern},
    traits::TraitImplRegistry,
};

pub(super) fn infer_value(
    value: &MatchValue,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Type> {
    let subject_ty = infer(&value.subject, env, functions, impls, enums)?;
    check_exhaustive(&subject_ty, &value.arms, enums)?;
    let subject = binding_for_name("__ush_match_value", subject_ty);
    let mut result: Option<Type> = None;
    for (pattern, arm) in &value.arms {
        let scope = compile_pattern(pattern, &subject, env, enums)?.env;
        if let StatementKind::Expr(expr) = &arm.kind {
            let ty = infer(expr, &scope, functions, impls, enums)
                .with_context(|| format!("line {}", arm.line))?;
            if let Some(expected) = &result {
                ensure!(
                    expected == &ty,
                    "line {}: match arm type mismatch: expected {}, found {}",
                    arm.line,
                    expected.render(),
                    ty.render()
                );
            } else {
                result = Some(ty);
            }
        }
    }
    result.ok_or_else(|| anyhow!("a match binding needs at least one value-producing arm"))
}
