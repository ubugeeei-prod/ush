use anyhow::Result;

use super::{
    super::{
        ast::StatementKind,
        effects::FunctionErrorRegistry,
        env::{CodegenState, Env},
    },
    control::{ControlContext, compile_for, compile_if, compile_loop, compile_while},
    functions::FunctionRegistry,
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;
use crate::{ast::Type, env::EnumRegistry};

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_control_statement(
    kind: &StatementKind,
    env: &mut Env,
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
    state: &mut CodegenState,
    return_type: Option<&Type>,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Option<Result<()>> {
    let mut ctx = ControlContext {
        globals,
        functions,
        impls,
        enums,
        function_errors,
        state,
        return_type,
        inside_function,
    };
    Some(match kind {
        StatementKind::If {
            branch,
            returns_value,
        } => compile_if(branch, *returns_value, env, &mut ctx, out),
        StatementKind::While { condition, body } => looped(&mut ctx, |ctx| {
            compile_while(condition, body, env, ctx, out)
        }),
        StatementKind::For {
            name,
            iterable,
            body,
        } => looped(&mut ctx, |ctx| {
            compile_for(name, iterable, env, body, ctx, out)
        }),
        StatementKind::Loop { body } => looped(&mut ctx, |ctx| compile_loop(body, env, ctx, out)),
        StatementKind::Break | StatementKind::Continue => {
            super::scoped::unwind_to_loop(ctx.state, out);
            let keyword = if matches!(kind, StatementKind::Break) {
                "break"
            } else {
                "continue"
            };
            out.push_str(keyword);
            out.push('\n');
            Ok(())
        }
        StatementKind::Pipeline(pipeline) => {
            super::pipeline::compile_statement(pipeline, env, &mut ctx, out)
        }
        StatementKind::Export(bindings) => {
            super::scoped::compile_export(bindings, env, &mut ctx, out)
        }
        StatementKind::Scoped { scope, body } => {
            super::scoped::compile_scoped(scope, body, env, &mut ctx, out)
        }
        _ => return None,
    })
}

/// Compile a loop with its body one loop deeper, which is how `break`
/// and `continue` know which scoped blocks they leave.
fn looped(
    ctx: &mut ControlContext<'_>,
    compile: impl FnOnce(&mut ControlContext<'_>) -> Result<()>,
) -> Result<()> {
    ctx.state.loop_depth += 1;
    let result = compile(ctx);
    ctx.state.loop_depth -= 1;
    result
}
