use super::super::{
    ast::Type,
    env::{Binding, Storage},
};
use crate::{sourcemap::OutputBuffer, types::AstString as NameString};

pub(crate) fn binding_for_name(name: &str, ty: Type) -> Binding {
    let storage = match ty {
        Type::Adt(_) => Storage::Adt(NameString::from(name)),
        Type::Tuple(_) | Type::List(_) => Storage::Aggregate(NameString::from(name)),
        Type::String | Type::Int | Type::Bool | Type::Unit => {
            Storage::Primitive(NameString::from(name))
        }
        Type::Task(_) => Storage::Task(NameString::from(name)),
    };
    Binding { ty, storage }
}

pub(crate) fn push_output(out: &mut OutputBuffer, nested: &OutputBuffer, indent: usize) {
    out.append_buffer(nested, indent);
}

pub(crate) fn push_line(out: &mut OutputBuffer, line: &str, indent: usize) {
    out.push_indent(indent);
    out.push_str(line);
    out.push('\n');
}

pub(crate) fn shell_function_name(name: &str) -> NameString {
    let mut rendered = NameString::from("ush_fn_");
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            rendered.push(ch);
        } else {
            rendered.push('_');
        }
    }
    rendered
}
