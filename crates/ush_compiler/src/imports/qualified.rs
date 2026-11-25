use super::{Call, Expr, Name, Resolver, Vec};
impl Resolver<'_> {
    /// `helpers::greet "x"` and `gh::build` parse as enum variants;
    /// they are calls when the path names a function.
    pub(super) fn qualified_call(&self, expr: &mut Expr) {
        if let Expr::Variant(v) = expr {
            let mut name: Name = format!("{}::{}", v.enum_name, v.variant_name).into();
            if matches!(self.scope.qualify(&name), Some(Ok(item)) if !item.kind.is_value()) {
                // `gh::Config { … }` builds a record of a module type.
                return;
            }
            self.name(&mut name);
            if self.known.contains(&name) {
                let args = match &v.fields {
                    crate::ast::ExprFields::Unit => Some(Vec::new()),
                    crate::ast::ExprFields::Tuple(es) => Some(
                        es.iter()
                            .cloned()
                            .map(|expr| crate::ast::CallArg { label: None, expr })
                            .collect(),
                    ),
                    _ => None,
                };
                if let Some(args) = args {
                    *expr = Expr::Call(Call {
                        name,
                        args,
                        asynchronous: false,
                    });
                }
            }
        }
    }
}
