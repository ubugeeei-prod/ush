//! Moves a module's statements to its range of the source space and
//! anchors its source-relative paths at the module file.

use super::visit::{VisitMut, walk_expr, walk_statement};
use crate::ast::{Call, CallArg, Expr, Statement};

pub(super) fn relocate(program: &mut [Statement], base: usize, dir: &str) {
    let mut relocate = Relocate { base, dir };
    for statement in program {
        relocate.statement(statement);
    }
}

struct Relocate<'a> {
    base: usize,
    /// The module's directory relative to the root script's.
    dir: &'a str,
}

impl VisitMut for Relocate<'_> {
    fn statement(&mut self, statement: &mut Statement) {
        statement.line += self.base;
        walk_statement(self, statement);
    }

    fn expr(&mut self, expr: &mut Expr) {
        walk_expr(self, expr);
        // `from_source "x"` in lib/gh.ush means lib/x, so it becomes
        // `join (from_source "lib") "x"`; absolute paths still win.
        if !self.dir.is_empty()
            && let Expr::Call(call) = expr
            && call.name == "std::path::from_source"
            && let [path] = call.args.as_mut_slice()
        {
            let path = core::mem::replace(&mut path.expr, Expr::Unit);
            let directory = Expr::Call(Call {
                name: "std::path::from_source".into(),
                args: vec![CallArg {
                    label: None,
                    expr: Expr::String(self.dir.into()),
                }],
                asynchronous: false,
            });
            *expr = Expr::Call(Call {
                name: "std::path::join".into(),
                args: vec![
                    CallArg {
                        label: None,
                        expr: directory,
                    },
                    CallArg {
                        label: None,
                        expr: path,
                    },
                ],
                asynchronous: false,
            });
        }
    }
}
