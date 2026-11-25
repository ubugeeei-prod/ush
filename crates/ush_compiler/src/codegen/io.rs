use anyhow::{Result, bail};

use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;

use super::{
    super::{
        ast::{Expr, Type},
        env::{CodegenState, EnumRegistry, Env},
    },
    FunctionRegistry, compile_runtime_primitive_expr, infer,
    methods::compile_display_expr,
};

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_print(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    let rendered = compile_display_expr(
        expr,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )?;
    out.push_str("printf '%s\\n' ");
    out.push_str(&rendered);
    out.push('\n');
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_shell(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    propagate: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    if infer(expr, env, functions, impls, enums)? != Type::String {
        bail!("shell statements must evaluate to string");
    }
    // A function invoked on the left of || inherits POSIX errexit suppression.
    // Check shell boundaries explicitly so ? cannot resume after a failure.
    let guard = propagate || inside_function;
    if guard {
        out.push_str("{\n");
    }
    if let Expr::String(value) = expr {
        out.push_shell_source(value);
    } else {
        let rendered = compile_runtime_primitive_expr(
            expr,
            env,
            functions,
            impls,
            enums,
            state,
            inside_function,
            out,
        )?;
        out.push_str("eval ");
        out.push_str(&rendered);
        out.push('\n');
    }
    if guard {
        out.push('}');
        push_failure_guard(out, inside_function);
    }
    Ok(())
}

pub(crate) fn compile_raise(
    expr: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    let Type::Adt(name) = infer(expr, env, functions, impls, enums)? else {
        bail!("raise expects an error ADT value");
    };
    let label = if let Expr::Variant(variant) = expr {
        format!("{name}::{}", variant.variant_name)
    } else {
        format!("{name}")
    };
    out.push_str(&format!(
        "__ush_report 1 {}\n",
        crate::util::shell_quote(&label)
    ));
    out.push_str(if inside_function {
        "return 1\n"
    } else {
        "exit 1\n"
    });
    Ok(())
}

pub(super) fn track_statement(
    statement: &crate::ast::Statement,
    state: &CodegenState,
    out: &mut OutputBuffer,
) {
    use crate::ast::StatementKind as S;
    if !matches!(
        statement.kind,
        S::Function(_)
            | S::Enum(_)
            | S::Trait(_)
            | S::Use(_)
            | S::Impl(_)
            | S::Effect(_)
            | S::UnitDef(_)
    ) && let Some(text) = state.source_lines.get(statement.line.saturating_sub(1))
    {
        let quoted = crate::util::shell_quote(text.trim_end());
        match crate::modules::locate(&state.modules, statement.line) {
            // A module statement also names its file for failure reports.
            Some((file, line)) => out.push_str(&format!(
                "__ush_at {line} {quoted} {}\n",
                crate::util::shell_quote(&file.origin)
            )),
            None => out.push_str(&format!("__ush_at {} {quoted}\n", statement.line)),
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn compile_fail(
    status: &Expr,
    message: &Expr,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    if super::infer(status, env, functions, impls, enums)? != Type::Int
        || super::infer(message, env, functions, impls, enums)? != Type::String
    {
        anyhow::bail!("fail expects an integer exit status and a string message");
    }
    if let Expr::Int(n) = status
        && !(1..=255).contains(n)
    {
        anyhow::bail!("fail status must be between 1 and 255");
    }
    let code = super::compile_runtime_primitive_expr(
        status,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )?;
    let message = super::compile_runtime_primitive_expr(
        message,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )?;
    out.push_str(&format!("__ush_status={code}\nif [ \"$__ush_status\" -lt 1 ] || [ \"$__ush_status\" -gt 255 ]; then __ush_status=1; fi\n__ush_report \"$__ush_status\" {message}\n"));
    out.push_str(if inside_function {
        "return \"$__ush_status\"\n"
    } else {
        "exit \"$__ush_status\"\n"
    });
    Ok(())
}

pub(super) fn push_failure_guard(out: &mut OutputBuffer, inside_function: bool) {
    out.push_str(" || { __ush_status=$?; __ush_report \"$__ush_status\"; ");
    out.push_str(if inside_function { "return" } else { "exit" });
    out.push_str(" \"$__ush_status\"; }\n");
}

pub(super) fn track_shell_body(
    statement: &crate::ast::Statement,
    state: &CodegenState,
    out: &mut OutputBuffer,
) {
    if matches!(
        &statement.kind,
        crate::ast::StatementKind::Shell(crate::ast::Expr::String(_))
            | crate::ast::StatementKind::TryShell(crate::ast::Expr::String(_))
    ) && state
        .source_lines
        .get(statement.line.saturating_sub(1))
        .is_some_and(|line| matches!(line.trim_start(), "shell \"\"\""))
    {
        out.set_origin(Some(statement.line + 1));
    }
}
