use super::{Call, Expr, Name, Resolver, Vec};
impl Resolver {
    pub(super) fn qualified_call(&self, expr: &mut Expr) {
        if let Expr::Variant(v) = expr {
            let name: Name = format!("{}::{}", v.enum_name, v.variant_name).into();
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
