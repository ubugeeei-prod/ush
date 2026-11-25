//! Interpolation, list indexing and list concatenation.

use super::{
    Checker, Locals,
    types::{Index, Ty},
};
use crate::ast::Expr;
use anyhow::{Result, bail, ensure};

impl Checker {
    pub(super) fn template(&mut self, parts: &mut [Expr], locals: &Locals) -> Result<Ty> {
        for part in parts {
            let ty = self.expr(part, locals)?;
            let generic = self.symbolic && ty.name.starts_with("__T");
            ensure!(
                generic
                    || (ty.args.is_empty()
                        && matches!(ty.name.as_str(), "String" | "Int" | "Bool")),
                "cannot interpolate a {}: only String, Int and Bool values can appear in `${{…}}`",
                ty.describe()
            );
        }
        Ok(Ty::named("String"))
    }

    /// `list[index]`: an index the checker can prove out of range is an
    /// error here; any other index is checked when the program runs.
    pub(super) fn index(
        &mut self,
        base: &mut Expr,
        index: &mut Expr,
        locals: &Locals,
    ) -> Result<Ty> {
        let list = self.expr(base, locals)?;
        let at = self.expr(index, locals)?;
        ensure!(
            list.name == "List" && list.args.len() == 1,
            "only a List can be indexed, found {}",
            list.describe()
        );
        Ty::named("Int").check(&at)?;
        if at.max < 0 {
            bail!("index {} is negative: list indexes start at 0", at.max);
        }
        if let Some(Index::Literal(length)) = list.length
            && at.min >= length
        {
            bail!(
                "index {} is out of range for a list of length {length}",
                at.min
            );
        }
        Ok(list.args[0].clone())
    }

    /// `a + b` on lists: one element type, and a known length when every
    /// part has one.
    pub(super) fn concat(types: &[Ty]) -> Result<Ty> {
        let first = &types[0];
        let mut length = Some(0i64);
        for ty in types {
            ensure!(
                ty.name == "List" && ty.args.len() == 1,
                "`+` joins lists only with lists, found {}",
                ty.describe()
            );
            first.args[0]
                .clone()
                .widened()
                .check(&ty.args[0].clone().widened())?;
            length = match (length, &ty.length) {
                (Some(total), Some(Index::Literal(n))) => total.checked_add(*n),
                _ => None,
            };
        }
        let mut ty = Ty::app("List", vec![first.args[0].clone().widened()]);
        ty.length = length.map(Index::Literal);
        Ok(ty)
    }
}
