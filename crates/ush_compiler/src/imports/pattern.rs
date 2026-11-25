use crate::{
    ast::Pattern,
    types::{AstString as Name, Set},
};
pub(super) fn bind_pattern(p: &Pattern, scope: &mut Set<Name>) {
    match p {
        Pattern::Binding(n) => {
            scope.insert(n.clone());
        }
        Pattern::Variant(v) => match &v.fields {
            crate::ast::PatternFields::Tuple(ps) => {
                for p in ps {
                    bind_pattern(p, scope);
                }
            }
            crate::ast::PatternFields::Struct(ps) => {
                for p in ps {
                    bind_pattern(&p.pattern, scope);
                }
            }
            _ => {}
        },
        _ => {}
    }
}
