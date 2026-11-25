use alloc::boxed::Box;

use anyhow::{Result, bail};

use super::{
    super::{
        ast::{Expr, Statement, Type},
        effects::FunctionErrorRegistry,
        env::{CodegenState, EnumRegistry, Env},
        matching::{compile_pattern, emit_value_to_target, read_expr},
    },
    compile_runtime_primitive_expr,
    functions::FunctionRegistry,
    shared::{binding_for_name, push_line, push_output},
    statement::compile_statement,
    task_block::compile_async_block,
};
use crate::sourcemap::OutputBuffer;
use crate::traits::TraitImplRegistry;

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_let(
    name: &str,
    expr: &Expr,
    ty: &Type,
    env: &mut Env,
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    if let Expr::AsyncBlock(body) = expr {
        let binding = compile_async_block(
            body,
            env,
            globals,
            functions,
            impls,
            enums,
            function_errors,
            state,
            out,
        )?;
        super::binding::store_task(name, &binding, out);
        env.insert(name.into(), binding_for_name(name, binding.ty));
        return Ok(());
    }

    match ty {
        Type::String | Type::Int | Type::Bool | Type::Unit => {
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
            out.push_str(name);
            out.push('=');
            out.push_str(&rendered);
            out.push('\n');
        }
        Type::Adt(_) => emit_value_to_target(
            name,
            expr,
            ty,
            env,
            functions,
            impls,
            enums,
            state,
            inside_function,
            out,
        )?,
        Type::Tuple(_) | Type::List(_) => emit_value_to_target(
            name,
            expr,
            ty,
            env,
            functions,
            impls,
            enums,
            state,
            inside_function,
            out,
        )?,
        Type::Task(_) => bail!("task expressions must be bound before awaiting"),
    }
    env.insert(name.into(), binding_for_name(name, ty.clone()));
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_match(
    expr: &Expr,
    arms: &[(super::super::ast::Pattern, Box<Statement>)],
    returns_value: bool,
    env: &Env,
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    function_errors: &FunctionErrorRegistry,
    state: &mut CodegenState,
    return_type: Option<&Type>,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    if arms.is_empty() {
        bail!("match must have at least one arm");
    }
    let subject = read_expr(
        expr,
        env,
        functions,
        impls,
        enums,
        state,
        inside_function,
        out,
    )?;
    let words = case_words(arms, &subject, out)?;
    for (index, (pattern, arm)) in arms.iter().enumerate() {
        let plan = compile_pattern(pattern, &subject, env, enums)?;
        let indent = match &words {
            Some(words) => {
                push_line(out, &format!("{})", words[index]), 2);
                4
            }
            None => {
                out.push_str(if index == 0 { "if " } else { "elif " });
                out.push_str(&plan.condition);
                out.push_str("; then\n");
                2
            }
        };
        for line in &plan.prelude {
            push_line(out, line, indent);
        }
        let mut arm_env = plan.env;
        let body = compile_one(
            arm,
            returns_value,
            &mut arm_env,
            globals,
            functions,
            impls,
            enums,
            function_errors,
            state,
            return_type,
            inside_function,
        )?;
        push_output(out, &body, indent);
        if words.is_some() {
            push_line(out, ";;", indent);
        }
    }
    out.push_str(if words.is_some() { "esac\n" } else { "fi\n" });
    Ok(())
}

/// The `case` patterns of a String `match` whose every arm has one, after
/// opening the `case` statement; `None` keeps the `if` chain.
fn case_words(
    arms: &[(super::super::ast::Pattern, Box<Statement>)],
    subject: &crate::env::Binding,
    out: &mut OutputBuffer,
) -> Result<Option<crate::types::HeapVec<crate::types::OutputString>>> {
    let (Type::String, crate::env::Storage::Primitive(var)) = (&subject.ty, &subject.storage)
    else {
        return Ok(None);
    };
    let mut words = crate::types::HeapVec::with_capacity(arms.len());
    for (pattern, _) in arms {
        match crate::matching::case_word(pattern) {
            Some(word) => words.push(word?),
            None => return Ok(None),
        }
    }
    out.push_str(&format!("case \"${{{var}}}\" in\n"));
    Ok(Some(words))
}

#[allow(clippy::too_many_arguments)]
fn compile_one(
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
) -> Result<OutputBuffer> {
    let mut buffer = OutputBuffer::new(state.mapped);
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
