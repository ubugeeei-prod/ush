use super::{Checker, Locals, types::Ty};
use crate::ast::{Comprehension, ComprehensionClause};
use anyhow::{Result, ensure};
impl Checker {
    pub(super) fn comprehension(&mut self, c: &mut Comprehension, locals: &Locals) -> Result<Ty> {
        let mut inner = locals.clone();
        for clause in &mut c.clauses {
            match clause {
                ComprehensionClause::For { name, iterable } => {
                    let ty = self.expr(iterable, &inner)?;
                    ensure!(
                        ty.name == "List" && ty.args.len() == 1,
                        "comprehension expects a list or range"
                    );
                    inner.insert(name.clone(), ty.args[0].clone());
                }
                ComprehensionClause::If(e) => {
                    Ty::named("Bool").check(&self.expr(e, &inner)?)?;
                    super::refine::narrow(e, &mut inner);
                }
            }
        }
        Ok(Ty::app(
            "List",
            vec![self.expr(&mut c.value, &inner)?.widened()],
        ))
    }
}
