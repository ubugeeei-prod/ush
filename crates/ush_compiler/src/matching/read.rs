use super::{
    Binding, CodegenState, EnumRegistry, Env, Expr, FunctionRegistry, OutputBuffer, Result,
    TraitImplRegistry, materialize_expr,
};

/// Read an existing binding directly when its value is consumed immediately.
#[allow(clippy::too_many_arguments)]
pub(crate) fn read_expr(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<Binding> {
    if let Expr::Var(name) = expr
        && let Some(binding) = env.get(name)
    {
        return Ok(binding.clone());
    }
    materialize_expr(
        expr,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )
}
