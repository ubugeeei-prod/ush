use super::{
    super::{
        ast::{Call, Expr},
        env::{Binding, CodegenState, EnumRegistry, Env, Storage},
    },
    FunctionRegistry, runtime_member,
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;
use alloc::boxed::Box;
use anyhow::Result;
#[derive(Clone, Copy)]
pub(super) enum FailureMode {
    Abort,
    Propagate,
}
#[allow(clippy::too_many_arguments)]
pub(super) fn hoist_expr(
    expr: &Expr,
    env: &mut Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    mode: FailureMode,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<Expr> {
    Ok(match expr {
        Expr::ShellCapture(command) => {
            let temp = state.temp_var("stdout");
            out.push_str(&format!("{temp}=\"$(\n"));
            out.push_shell_source(command);
            out.push_str(")\"");
            super::io::push_failure_guard(out, inside_function);
            env.insert(
                temp.clone(),
                Binding {
                    ty: crate::ast::Type::String,
                    storage: Storage::Primitive(temp.clone()),
                },
            );
            Expr::Var(temp)
        }
        Expr::Env(_) | Expr::Index { .. } | Expr::MethodCall(_)
            if super::list::hoists(expr, env, functions, impls, enums) =>
        {
            let low = super::list::Lowering::new(functions, impls, enums, inside_function);
            super::list::hoist(expr, env, &low, state, out)?
        }
        Expr::Add(parts) | Expr::Template(parts) => {
            let hoisted = parts
                .iter()
                .map(|part| {
                    hoist_expr(
                        part,
                        env,
                        functions,
                        impls,
                        enums,
                        state,
                        mode,
                        inside_function,
                        out,
                    )
                })
                .collect::<Result<_>>()?;
            if matches!(expr, Expr::Add(_)) {
                Expr::Add(hoisted)
            } else {
                Expr::Template(hoisted)
            }
        }
        Expr::Compare { lhs, op, rhs } => Expr::Compare {
            lhs: Box::new(hoist_expr(
                lhs,
                env,
                functions,
                impls,
                enums,
                state,
                mode,
                inside_function,
                out,
            )?),
            op: *op,
            rhs: Box::new(hoist_expr(
                rhs,
                env,
                functions,
                impls,
                enums,
                state,
                mode,
                inside_function,
                out,
            )?),
        },
        Expr::Try(inner) => hoist_expr(
            inner,
            env,
            functions,
            impls,
            enums,
            state,
            FailureMode::Propagate,
            inside_function,
            out,
        )?,
        Expr::Field { base, name } => runtime_member::hoist_field_access(
            base,
            name,
            env,
            functions,
            impls,
            enums,
            state,
            inside_function,
            out,
        )?,
        Expr::MethodCall(call) => runtime_member::hoist_method_capture(
            call,
            env,
            functions,
            impls,
            enums,
            state,
            matches!(mode, FailureMode::Propagate),
            inside_function,
            out,
        )?,
        Expr::Call(call) if call.name == "format" => runtime_member::hoist_format_capture(
            call,
            env,
            functions,
            impls,
            enums,
            state,
            matches!(mode, FailureMode::Propagate),
            inside_function,
            out,
        )?,
        Expr::StatusOf(command) => super::process::hoist_status(
            command,
            env,
            functions,
            impls,
            enums,
            state,
            inside_function,
            out,
        )?,
        Expr::Pipeline(pipeline) => super::pipeline::hoist_capture(
            pipeline,
            env,
            functions,
            impls,
            enums,
            state,
            inside_function,
            out,
        )?,
        Expr::Call(call) => super::call_capture::hoist_call_capture(
            call,
            env,
            functions,
            impls,
            enums,
            state,
            mode,
            inside_function,
            out,
        )?,
        other => other.clone(),
    })
}
#[allow(clippy::too_many_arguments)]
pub(super) fn hoist_call(
    call: &Call,
    env: &mut Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    mode: FailureMode,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<Call> {
    let mut prepared = call.clone();
    for arg in &mut prepared.args {
        if call.name == "std::io::echo"
            && super::infer(&arg.expr, env, functions, impls, enums)?.is_structured()
        {
            let format = Call {
                name: "format".into(),
                args: vec![arg.clone()],
                asynchronous: false,
            };
            arg.expr = runtime_member::hoist_format_capture(
                &format,
                env,
                functions,
                impls,
                enums,
                state,
                matches!(mode, FailureMode::Propagate),
                inside_function,
                out,
            )?;
            continue;
        }
        if super::infer(&arg.expr, env, functions, impls, enums)?.is_structured() {
            let binding = crate::matching::materialize_expr(
                &arg.expr,
                env,
                functions,
                impls,
                enums,
                state,
                inside_function,
                out,
            )?;
            let temp = state.temp_var("argument");
            env.insert(temp.clone(), binding);
            arg.expr = Expr::Var(temp);
            continue;
        }
        arg.expr = hoist_expr(
            &arg.expr,
            env,
            functions,
            impls,
            enums,
            state,
            mode,
            inside_function,
            out,
        )?;
    }
    Ok(prepared)
}
