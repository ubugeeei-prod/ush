use anyhow::{Result, bail};

use super::{
    super::{
        ast::{Call, CallArg, Expr, MethodCall, Type},
        env::{EnumRegistry, Env},
    },
    FunctionRegistry,
    call_support::{function_for_call, resolve_call_args},
    calls::{call_expr_type, capture_call, ensure_value_type},
    infer,
};
use crate::traits::TraitImplRegistry;
use crate::types::{HeapVec as Vec, OutputString as String};

pub(super) fn infer_builtin_method(
    call: &MethodCall,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Option<Type>> {
    if let Some(ty) = super::list::infer_method(call, env, functions, impls, enums)? {
        return Ok(Some(ty));
    }
    let receiver_ty = infer(&call.receiver, env, functions, impls, enums)?;
    let Some(lowered) =
        lower_builtin_method_call(&call.receiver, &call.method, &call.args, &receiver_ty)
    else {
        return Ok(None);
    };
    let def = function_for_call(&lowered.name, functions)?;
    for (param, arg) in def.params.iter().zip(resolve_call_args(&lowered, def)?) {
        let actual = infer(arg, env, functions, impls, enums)?;
        if actual != param.ty {
            bail!(
                "type mismatch for `{}.{}`: expected {:?}, found {:?}",
                receiver_ty.render(),
                call.method,
                param.ty,
                actual
            );
        }
    }
    Ok(Some(call_expr_type(&lowered, functions)?))
}

pub(super) fn capture_builtin_method(
    receiver: &Expr,
    method: &str,
    args: &[Expr],
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<Option<String>> {
    let receiver_ty = infer(receiver, env, functions, impls, enums)?;
    let Some(lowered) = lower_builtin_method(receiver, method, args, &receiver_ty) else {
        return Ok(None);
    };
    let return_ty = call_expr_type(&lowered, functions)?;
    ensure_value_type(&return_ty)?;
    Ok(Some(capture_call(
        &lowered, env, functions, impls, enums, &return_ty,
    )?))
}

fn lower_builtin_method(
    receiver: &Expr,
    method: &str,
    args: &[Expr],
    receiver_ty: &Type,
) -> Option<Call> {
    let name = builtin_method_name(receiver_ty, method)?;
    let mut lowered_args = Vec::with_capacity(args.len() + 1);
    lowered_args.push(CallArg {
        label: None,
        expr: receiver.clone(),
    });
    lowered_args.extend(
        args.iter()
            .cloned()
            .map(|expr| CallArg { label: None, expr }),
    );
    Some(Call {
        name: name.into(),
        args: lowered_args,
        asynchronous: false,
    })
}

fn lower_builtin_method_call(
    receiver: &Expr,
    method: &str,
    args: &[CallArg],
    receiver_ty: &Type,
) -> Option<Call> {
    let name = builtin_method_name(receiver_ty, method)?;
    let mut lowered_args = Vec::with_capacity(args.len() + 1);
    lowered_args.push(CallArg {
        label: None,
        expr: receiver.clone(),
    });
    lowered_args.extend(args.iter().cloned());
    Some(Call {
        name: name.into(),
        args: lowered_args,
        asynchronous: false,
    })
}

fn builtin_method_name(receiver_ty: &Type, method: &str) -> Option<&'static str> {
    match (receiver_ty, method) {
        (Type::String, "resolve") => Some("std::path::resolve"),
        (Type::String, "join") => Some("std::path::join"),
        (Type::String, "dirname") => Some("std::path::dirname"),
        (Type::String, "basename") => Some("std::path::basename"),
        (Type::String, "exists") => Some("std::path::exists"),
        (Type::String, "is_file") => Some("std::path::is_file"),
        (Type::String, "is_dir") => Some("std::path::is_dir"),
        (Type::String, "mkdir_p") => Some("std::path::mkdir_p"),
        (Type::String, "read_text") => Some("std::fs::read_text"),
        (Type::String, "write_text") => Some("std::fs::write_text"),
        (Type::String, "append_text") => Some("std::fs::append_text"),
        (Type::String, "remove") => Some("std::fs::remove"),
        (Type::String, "copy") => Some("std::fs::copy"),
        (Type::String, "move") => Some("std::fs::move"),
        (Type::String, "sha256") => Some("std::fs::sha256"),
        (Type::String, "mime_type") => Some("std::fs::mime_type"),
        (Type::String, "starts_with") => Some("std::string::starts_with"),
        (Type::String, "ends_with") => Some("std::string::ends_with"),
        (Type::String, "is_match") => Some("std::regex::is_match"),
        (Type::String, "find") => Some("std::regex::find"),
        (Type::String, "capture") => Some("std::regex::capture"),
        (Type::String, "replace") => Some("std::string::replace"),
        (Type::String, "replace_regex") => Some("std::regex::replace"),
        (Type::String, "trim_prefix") => Some("std::string::trim_prefix"),
        (Type::String, "trim_suffix") => Some("std::string::trim_suffix"),
        _ => None,
    }
}
