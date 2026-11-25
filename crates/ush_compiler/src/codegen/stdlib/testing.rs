use super::{builtin, param};
use crate::codegen::{FunctionRegistry, compile_primitive_expr, infer};
use crate::{
    ast::{Call, Expr, FunctionDef, Type},
    env::{EnumRegistry, Env, Storage},
    traits::TraitImplRegistry,
    types::{HeapVec as Vec, OutputString},
};
use anyhow::{Result, ensure};
pub(super) fn definitions() -> Vec<FunctionDef> {
    vec![
        builtin(
            "std::test::assert",
            vec![param("condition", Type::Bool)],
            Some(Type::Unit),
        ),
        builtin(
            "std::test::assert_eq",
            vec![
                param("actual", Type::String),
                param("expected", Type::String),
            ],
            Some(Type::Unit),
        ),
    ]
}
pub(crate) fn render_eq(
    call: &Call,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<OutputString> {
    let args = crate::codegen::call_support::resolve_call_args(call, &functions[&call.name])?;
    let ty = infer(args[0], env, functions, impls, enums)?;
    ensure!(
        ty == infer(args[1], env, functions, impls, enums)?,
        "assert_eq arguments must have the same type"
    );
    ensure!(
        !matches!(ty, Type::Task(_)),
        "assert_eq cannot compare task handles"
    );
    let words = args
        .iter()
        .map(|arg| {
            if let Expr::Var(n) = arg
                && let Some(binding) = env.get(n)
                && let Storage::Aggregate(prefix) | Storage::Adt(prefix) = &binding.storage
            {
                return Ok(crate::codegen::wire::encode(&ty, prefix));
            }
            compile_primitive_expr(arg, env, functions, impls, enums)
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(format!("ush_fn_std__test__assert_eq {}", words.join(" ")))
}
