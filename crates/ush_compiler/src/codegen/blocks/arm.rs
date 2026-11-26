use super::{compile_let, compile_statement};
use crate::{
    ast::{Statement, Type},
    codegen::FunctionRegistry,
    effects::FunctionErrorRegistry,
    env::{CodegenState, EnumRegistry, Env},
    sourcemap::OutputBuffer,
    traits::TraitImplRegistry,
};
use anyhow::Result;

#[allow(clippy::too_many_arguments)]
pub(super) fn compile_one(
    statement: &Statement,
    tail_position: bool,
    env: &mut Env,
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
    state: &mut CodegenState,
    return_type: Option<&Type>,
    inside_function: bool,
    destination: Option<(&str, &Type)>,
) -> Result<OutputBuffer> {
    let mut buffer = OutputBuffer::new(state.mapped);
    if let Some((name, ty)) = destination
        && let crate::ast::StatementKind::Expr(expr) = &statement.kind
    {
        buffer.set_origin(Some(statement.line));
        crate::codegen::io::track_statement(statement, state, &mut buffer);
        compile_let(
            name,
            expr,
            ty,
            env,
            globals,
            functions,
            impls,
            enums,
            function_errors,
            state,
            inside_function,
            &mut buffer,
        )?;
        return Ok(buffer);
    }
    compile_statement(
        statement,
        env,
        globals,
        functions,
        impls,
        enums,
        function_errors,
        state,
        return_type,
        inside_function,
        tail_position,
        &mut buffer,
    )?;
    if buffer.is_empty() {
        buffer.push_str(":\n");
    }
    Ok(buffer)
}
