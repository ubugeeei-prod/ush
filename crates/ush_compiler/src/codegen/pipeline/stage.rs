//! What each written stage of a pipeline is, once names are resolved.

use anyhow::{Result, anyhow, bail};

use super::super::{
    FunctionRegistry,
    calls::call_return_type,
    infer,
    stdlib::pipe::{APPEND_TO, FROM_FILE, TO_FILE},
};
use crate::{
    ast::{Call, Expr, Pipeline, Stage, Type},
    env::{EnumRegistry, Env},
    traits::TraitImplRegistry,
    types::HeapVec as Vec,
};

pub(super) enum StageKind<'a> {
    /// A program: a `$bin::` command, `std::process::exec` or `output`.
    Command(&'a Call),
    /// A function returning `()`, run in its own subshell.
    Function(&'a Call),
    /// A String written to the next stage's stdin, byte for byte.
    Text(&'a Expr),
    /// `from_file path`: the next stage's stdin.
    Source(&'a Expr),
    /// `to_file path` or `append_to path`: the previous stage's stdout.
    Sink { path: &'a Expr, append: bool },
}

pub(super) struct Types<'a> {
    pub env: &'a Env,
    pub functions: &'a FunctionRegistry,
    pub impls: &'a TraitImplRegistry,
    pub enums: &'a EnumRegistry,
}

impl Types<'_> {
    fn infer(&self, expr: &Expr) -> Result<Type> {
        infer(expr, self.env, self.functions, self.impls, self.enums)
    }
}

pub(super) fn classify<'a>(
    pipeline: &'a Pipeline,
    captured: bool,
    types: &Types<'_>,
) -> Result<Vec<StageKind<'a>>> {
    let last = pipeline.stages.len() - 1;
    let kinds = pipeline
        .stages
        .iter()
        .enumerate()
        .map(|(index, stage)| {
            classify_stage(stage, index, last, types)
                .map_err(|error| anyhow!("pipeline stage {} `{}`: {error}", index + 1, stage.text))
        })
        .collect::<Result<Vec<_>>>()?;
    if captured && matches!(kinds.last(), Some(StageKind::Sink { .. })) {
        bail!(
            "a pipeline that ends in a file sink has no output to capture; \
             as a function's last line, end it with `;` to run it as a statement"
        );
    }
    Ok(kinds)
}

fn classify_stage<'a>(
    stage: &'a Stage,
    index: usize,
    last: usize,
    types: &Types<'_>,
) -> Result<StageKind<'a>> {
    if let Expr::Call(call) = &stage.expr {
        if call.asynchronous {
            bail!("stages already run concurrently, so a stage cannot be `async`");
        }
        match call.name.as_str() {
            FROM_FILE if index != 0 => bail!("`from_file` can only be the first stage"),
            FROM_FILE if stage.merge_stderr => bail!("`from_file` has no stderr to merge"),
            FROM_FILE => return Ok(StageKind::Source(path_arg(call, types)?)),
            TO_FILE | APPEND_TO if index != last => bail!("a file sink can only be the last stage"),
            TO_FILE | APPEND_TO => {
                let append = call.name == APPEND_TO;
                return Ok(StageKind::Sink {
                    path: path_arg(call, types)?,
                    append,
                });
            }
            "std::process::exec" | "std::process::output" => return Ok(StageKind::Command(call)),
            _ if crate::commands::lookup(&call.name).is_some() => {
                return Ok(StageKind::Command(call));
            }
            "format" => {}
            name if call_return_type(name, types.functions)? == Some(Type::Unit) => {
                return Ok(StageKind::Function(call));
            }
            _ => {}
        }
    }
    let ty = types.infer(&stage.expr)?;
    match (index, ty) {
        (0, Type::String) => Ok(StageKind::Text(&stage.expr)),
        (0, ty) => bail!(
            "the first stage feeds stdin, so it must be a command, a function returning `()`, \
             `from_file`, or a String; found {ty:?}"
        ),
        (_, ty) => bail!(
            "a later stage must be a command or a function returning `()`, found a {ty:?} value; \
             a function stage prints its output instead of returning it"
        ),
    }
}

fn path_arg<'a>(call: &'a Call, types: &Types<'_>) -> Result<&'a Expr> {
    let [arg] = call.args.as_slice() else {
        bail!("`{}` takes one path", call.name);
    };
    if arg.label.as_ref().is_some_and(|label| label != "path") {
        bail!("`{}` takes one path", call.name);
    }
    let ty = types.infer(&arg.expr)?;
    if ty != Type::String {
        bail!("a file path must be a String, found {ty:?}");
    }
    Ok(&arg.expr)
}
