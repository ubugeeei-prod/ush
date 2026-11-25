//! Keeps only the module functions a program can reach, the way the
//! standard library links only the helpers a program calls.

use super::visit::{VisitMut, walk_call, walk_expr, walk_statement};
use crate::{
    ast::{Call, Expr, Statement, StatementKind as S},
    types::{AstString as Name, HeapVec as Vec, Map, Set},
};

/// Drops module functions that neither the root script nor another
/// kept item calls. Types, impls and effects stay: they emit no code
/// unless used, and impl methods are found by type, not by name.
pub(super) fn shake(modules: &mut Vec<Statement>, root: &mut [Statement]) {
    let index: Map<Name, usize> = modules
        .iter()
        .enumerate()
        .filter_map(|(at, statement)| match &statement.kind {
            S::Function(def) => Some((def.name.clone(), at)),
            _ => None,
        })
        .collect();
    let mut refs = References::default();
    root.iter_mut()
        .for_each(|statement| refs.statement(statement));
    for statement in modules.iter_mut() {
        if !matches!(statement.kind, S::Function(_)) {
            refs.statement(statement);
        }
    }
    let mut kept = Set::default();
    while let Some(name) = refs.pending.pop() {
        if let Some(&at) = index.get(&name)
            && kept.insert(at)
        {
            refs.statement(&mut modules[at]);
        }
    }
    let mut at = 0;
    modules.retain(|statement| {
        let keep = !matches!(statement.kind, S::Function(_)) || kept.contains(&at);
        at += 1;
        keep
    });
}

#[derive(Default)]
struct References {
    seen: Set<Name>,
    pending: Vec<Name>,
}

impl References {
    fn note(&mut self, name: &Name) {
        if self.seen.insert(name.clone()) {
            self.pending.push(name.clone());
        }
    }
}

impl VisitMut for References {
    fn statement(&mut self, statement: &mut Statement) {
        if let S::Handle { handlers, .. } = &statement.kind {
            for handler in handlers {
                self.note(&handler.effect);
            }
        }
        walk_statement(self, statement);
    }

    fn call(&mut self, call: &mut Call) {
        self.note(&call.name);
        walk_call(self, call);
    }

    fn expr(&mut self, expr: &mut Expr) {
        if let Expr::FunctionRef(name) = expr {
            self.note(name);
        }
        walk_expr(self, expr);
    }
}
