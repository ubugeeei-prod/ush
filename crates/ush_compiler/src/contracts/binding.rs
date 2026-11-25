use super::{Locals, types::Ty};
use crate::{ast::BindingMode, types::AstString as Name};
use anyhow::{Context, Result, anyhow};

pub(super) fn bind(mode: BindingMode, name: &Name, ty: Ty, locals: &mut Locals) -> Result<()> {
    if mode == BindingMode::Assign {
        let expected = locals
            .get(name)
            .ok_or_else(|| anyhow!("undeclared binding `{name}`"))?;
        let expected = widen(expected.clone());
        expected
            .check(&ty)
            .with_context(|| format!("assignment to `{name}`"))?;
        locals.insert(name.clone(), expected);
    } else {
        // Mutable values cannot retain facts derived only from their initializer.
        let ty = if mode == BindingMode::Mutable {
            widen(ty)
        } else {
            ty
        };
        locals.insert(name.clone(), ty);
    }
    Ok(())
}

fn widen(mut ty: Ty) -> Ty {
    ty.min = i64::MIN;
    ty.max = i64::MAX;
    if ty.name == "List" {
        ty.length = None;
    }
    ty.args = ty.args.into_iter().map(widen).collect();
    ty
}
