use super::{Name, Resolver, Set, bind_pattern};
impl Resolver<'_> {
    pub(super) fn condition(&self, condition: &mut crate::ast::Condition, scope: &mut Set<Name>) {
        use crate::ast::Condition;
        match condition {
            Condition::Expr(e) => self.expr(e, scope),
            Condition::Let { pattern, expr } => {
                self.expr(expr, scope);
                bind_pattern(pattern, scope);
            }
            Condition::And(cs) => {
                for c in cs {
                    self.condition(c, scope);
                }
            }
            Condition::Not(c) => self.condition(c, &mut scope.clone()),
            Condition::Or(cs) => {
                for c in cs {
                    self.condition(c, &mut scope.clone());
                }
            }
        }
    }
}
