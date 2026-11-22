use alloc::string::ToString;

use anyhow::{Result, bail};

use super::{
    super::{
        ast::{FunctionDef, Type},
        env::{EnumRegistry, Env},
    },
    compile_primitive_expr,
    functions::FunctionRegistry,
    infer,
};
use crate::{
    sourcemap::OutputBuffer,
    traits::TraitImplRegistry,
    types::{HeapVec as Vec, OutputString as String},
};

pub(crate) fn completion_candidates(def: &FunctionDef) -> Vec<String> {
    let mut values = Vec::new();
    for param in &def.params {
        values.push(format!("--{}", param.name));
        if let Some(alias) = &param.cli_alias {
            values.push(format!("-{alias}"));
        }
    }
    values
}

pub(crate) fn push_bin_entry(
    out: &mut OutputBuffer,
    def: &FunctionDef,
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<()> {
    out.push_str("__ush_run_bin() {\n");
    out.push_str("  __ush_bin_pos='0'\n");
    for param in &def.params {
        let var = cli_var(&param.name);
        let seen = cli_seen(&param.name);
        out.push_str("  ");
        out.push_str(&seen);
        out.push_str("='0'\n");
        out.push_str("  ");
        out.push_str(&var);
        out.push('=');
        out.push_str(&default_value(param, globals, functions, impls, enums)?);
        out.push('\n');
    }
    out.push_str("  while [ \"$#\" -gt 0 ]; do\n");
    out.push_str("    case \"$1\" in\n");
    for param in &def.params {
        push_named_case(out, param)?;
    }
    out.push_str("      --)\n");
    out.push_str("        shift\n");
    out.push_str("        break\n");
    out.push_str("        ;;\n");
    out.push_str("      *)\n");
    out.push_str("        case \"$__ush_bin_pos\" in\n");
    for (index, param) in def.params.iter().enumerate() {
        out.push_str("          '");
        out.push_str(&index.to_string());
        out.push_str("') ");
        assign_value(out, param, "\"$1\"");
        out.push_str(" ;;\n");
    }
    out.push_str("          *) printf '%s\\n' \"unexpected argument: $1\" >&2; return 2 ;;\n");
    out.push_str("        esac\n");
    out.push_str("        __ush_bin_pos=$((__ush_bin_pos + 1))\n");
    out.push_str("        ;;\n");
    out.push_str("    esac\n");
    out.push_str("    shift\n");
    out.push_str("  done\n");
    for param in &def.params {
        if requires_value(param) {
            out.push_str("  [ \"$");
            out.push_str(&cli_seen(&param.name));
            out.push_str("\" = '1' ] || { printf '%s\\n' \"missing argument: ");
            out.push_str(&param.name);
            out.push_str("\" >&2; return 2; }\n");
        }
    }
    for param in &def.params {
        if param.ty != Type::Int {
            continue;
        }
        let var = cli_var(&param.name);
        let (min, max) = def
            .scheme
            .as_ref()
            .and_then(|s| s.integer_bounds(&param.name))
            .unwrap_or((i64::MIN, i64::MAX));
        out.push_str(&format!("  __ush_digits=${{{var}#-}}\n  case $__ush_digits in ''|*[!0-9]*) printf '%s\\n' 'invalid integer for {}' >&2; return 2 ;; esac\n",param.name));
        out.push_str(&format!("  while [ \"${{__ush_digits#0}}\" != \"$__ush_digits\" ]; do __ush_digits=${{__ush_digits#0}}; done\n  case \"${var}\" in -*) {var}=-${{__ush_digits:-0}} ;; *) {var}=${{__ush_digits:-0}} ;; esac\n"));
        out.push_str(&format!("  if ! [ \"${var}\" -ge '{min}' ] 2>/dev/null || ! [ \"${var}\" -le '{max}' ] 2>/dev/null; then printf '%s\\n' '{} is outside its integer bounds' >&2; return 2; fi\n", param.name));
    }
    out.push_str("  ush_fn_bin");
    for param in &def.params {
        out.push(' ');
        out.push('"');
        out.push('$');
        out.push('{');
        out.push_str(&cli_var(&param.name));
        out.push('}');
        out.push('"');
    }
    out.push('\n');
    out.push_str("}\n\n__ush_run_bin \"$@\"\n");
    Ok(())
}

fn push_named_case(out: &mut OutputBuffer, param: &crate::ast::FunctionParam) -> Result<()> {
    let long = format!("--{}", param.name);
    match param.ty {
        Type::Bool => {
            push_case_pattern(out, &long);
            if let Some(alias) = &param.cli_alias {
                out.push('|');
                out.push_str(&format!("'-{alias}'"));
            }
            out.push_str(")\n        ");
            assign_value(out, param, "'true'");
            out.push_str("\n        ;;\n");
        }
        Type::String | Type::Int => {
            push_case_pattern(out, &long);
            if let Some(alias) = &param.cli_alias {
                out.push('|');
                out.push_str(&format!("'-{alias}'"));
            }
            out.push_str(")\n        shift\n        [ \"$#\" -gt 0 ] || { printf '%s\\n' \"missing value for ");
            out.push_str(&param.name);
            out.push_str("\" >&2; return 2; }\n        ");
            assign_value(out, param, "\"$1\"");
            out.push_str("\n        ;;\n");
            out.push_str("      '");
            out.push_str(&long);
            out.push_str("='*)\n        ");
            assign_value(out, param, &format!("\"${{1#{}=}}\"", long));
            out.push_str("\n        ;;\n");
        }
        Type::Unit | Type::Adt(_) | Type::Tuple(_) | Type::List(_) | Type::Task(_) => {
            bail!("bin parameters only support String, Int, and Bool")
        }
    }
    Ok(())
}

fn default_value(
    param: &crate::ast::FunctionParam,
    globals: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
) -> Result<String> {
    if let Some(default) = &param.default {
        let actual = infer(default, globals, functions, impls, enums)?;
        if actual != param.ty {
            bail!(
                "default value type mismatch for `{}`: expected {:?}, found {:?}",
                param.name,
                param.ty,
                actual
            );
        }
        return compile_primitive_expr(default, globals, functions, impls, enums);
    }
    Ok(match param.ty {
        Type::Bool => "'false'".into(),
        _ => "''".into(),
    })
}

fn requires_value(param: &crate::ast::FunctionParam) -> bool {
    param.default.is_none() && !matches!(param.ty, Type::Bool)
}

fn assign_value(out: &mut OutputBuffer, param: &crate::ast::FunctionParam, value: &str) {
    out.push_str(&cli_var(&param.name));
    out.push('=');
    out.push_str(value);
    out.push_str("; ");
    out.push_str(&cli_seen(&param.name));
    out.push_str("='1'");
}

fn push_case_pattern(out: &mut OutputBuffer, value: &str) {
    out.push_str("      '");
    out.push_str(value);
    out.push('\'');
}

fn cli_var(name: &str) -> String {
    format!("__ush_bin_{}", name)
}

fn cli_seen(name: &str) -> String {
    format!("__ush_bin_{}_seen", name)
}
