//! Positional arguments and task files carry structured values as hex-encoded
//! scalar cells. Generate a traversal only for shapes that cross this boundary.
use crate::{
    ast::{Type, VariantFields},
    env::{CodegenState, EnumRegistry},
    sourcemap::OutputBuffer,
    types::{HeapVec as Vec, OutputString as String, Set},
};
use alloc::string::ToString;
use anyhow::{Result, anyhow, bail};

fn layout(ty: &Type) -> String {
    match ty {
        Type::Adt(n) => format!("record_{}_{}", n.len(), n),
        Type::List(t) => format!("list_{}", layout(t)),
        Type::Tuple(ts) => format!(
            "tuple_{}_{}",
            ts.len(),
            ts.iter().map(layout).collect::<Vec<_>>().join("_")
        ),
        _ => "word".into(),
    }
}

fn flat(ty: &Type) -> bool {
    match ty {
        Type::List(item) => item.is_scalar(),
        Type::Tuple(items) => items.iter().all(Type::is_scalar),
        _ => false,
    }
}

pub(super) fn encode(ty: &Type, prefix: &str) -> String {
    if flat(ty) {
        return format!("\"$(ush_fn_std__pack {prefix})\"");
    }
    format!(
        "\"$(__ush_walk_{} ush_fn_std__write_cell {prefix})\"",
        layout(ty)
    )
}

pub(crate) fn decode(ty: &Type, prefix: &str, word: &str, out: &mut OutputBuffer) {
    if flat(ty) {
        out.push_str(&format!("ush_fn_std__unpack {prefix} {word}\n"));
    } else {
        out.push_str(&format!(
            "__ush_cells={word}\n__ush_walk_{} ush_fn_std__read_cell {prefix}\n",
            layout(ty)
        ));
    }
}

pub(crate) fn register(ty: &Type, state: &mut CodegenState) {
    if !ty.is_scalar() && !flat(ty) && !state.wire_types.contains(ty) {
        state.wire_types.push(ty.clone());
    }
}

pub(crate) fn copy(
    ty: &Type,
    source: &str,
    target: &str,
    state: &mut CodegenState,
    out: &mut OutputBuffer,
) {
    register(ty, state);
    decode(ty, target, &encode(ty, source), out);
}

pub(super) fn emit(types: Vec<Type>, enums: &EnumRegistry, out: &mut OutputBuffer) -> Result<()> {
    let mut pending = types;
    let mut emitted = Set::default();
    while let Some(ty) = pending.pop() {
        let name = layout(&ty);
        if !emitted.insert(name.clone()) {
            continue;
        }
        out.push_str(&format!("__ush_walk_{name}() {{\n"));
        match ty {
            Type::Adt(name) => {
                let def = enums
                    .get(&name)
                    .ok_or_else(|| anyhow!("unknown enum: {name}"))?;
                out.push_str("  \"$1\" \"$2__tag\" || return \"$?\"\n  eval \"__ush_tag=\\${${2}__tag}\"\n  case $__ush_tag in\n");
                for variant in &def.variants {
                    out.push_str(&format!("    '{}::{}')\n", name, variant.name));
                    match &variant.fields {
                        VariantFields::Unit => {}
                        VariantFields::Tuple(ts) => {
                            for (index, t) in ts.iter().enumerate() {
                                field(out, &mut pending, t, &index.to_string(), "      ");
                            }
                        }
                        VariantFields::Struct(fs) => {
                            for f in fs {
                                field(out, &mut pending, &f.ty, &f.name, "      ");
                            }
                        }
                    }
                    out.push_str("      ;;\n");
                }
                out.push_str("    *) return 1 ;;\n  esac\n");
            }
            Type::Tuple(ts) => {
                if ts.is_empty() {
                    out.push_str("  :\n");
                }
                for (index, t) in ts.iter().enumerate() {
                    field(out, &mut pending, t, &index.to_string(), "  ");
                }
            }
            Type::List(t) => {
                out.push_str("  \"$1\" \"$2__len\" || return \"$?\"\n  eval \"__ush_count=\\${${2}__len}\"\n  case $__ush_count in ''|*[!0-9]*) return 1 ;; esac\n  set -- \"$1\" \"$2\" \"$__ush_count\" 0\n  while [ \"$4\" -lt \"$3\" ]; do\n");
                field(out, &mut pending, &t, "$4", "    ");
                out.push_str("    set -- \"$1\" \"$2\" \"$3\" \"$(($4 + 1))\"\n  done\n");
            }
            _ => bail!("unsupported structured value: {}", ty.render()),
        }
        out.push_str("}\n\n");
    }
    Ok(())
}

fn field(out: &mut OutputBuffer, pending: &mut Vec<Type>, ty: &Type, suffix: &str, indent: &str) {
    out.push_str(indent);
    if !ty.is_scalar() {
        out.push_str(&format!("__ush_walk_{} ", layout(ty)));
        pending.push(ty.clone());
    }
    out.push_str(&format!("\"$1\" \"$2__{suffix}\" || return \"$?\"\n"));
}
