//! A mutable walk over every node of a file, for the linker's passes.
//!
//! Each pass overrides the nodes it cares about and calls the matching
//! `walk_*` function to keep descending.

use crate::ast::{
    Call, ComprehensionClause, Condition, Expr, ExprFields, FunctionDef, Pattern, PatternFields,
    Statement, StatementKind as S, Type, VariantFields,
};

pub(crate) trait VisitMut {
    fn statement(&mut self, statement: &mut Statement) {
        walk_statement(self, statement);
    }
    fn function(&mut self, def: &mut FunctionDef) {
        walk_function(self, def);
    }
    fn expr(&mut self, expr: &mut Expr) {
        walk_expr(self, expr);
    }
    fn call(&mut self, call: &mut Call) {
        walk_call(self, call);
    }
    fn pattern(&mut self, pattern: &mut Pattern) {
        walk_pattern(self, pattern);
    }
    fn ty(&mut self, ty: &mut Type) {
        walk_ty(self, ty);
    }
}

pub(crate) fn walk_block<V: VisitMut + ?Sized>(v: &mut V, body: &mut [Statement]) {
    for statement in body {
        v.statement(statement);
    }
}

pub(crate) fn walk_statement<V: VisitMut + ?Sized>(v: &mut V, statement: &mut Statement) {
    match &mut statement.kind {
        S::Function(def) => v.function(def),
        S::Enum(def) => {
            for variant in &mut def.variants {
                match &mut variant.fields {
                    VariantFields::Unit => {}
                    VariantFields::Tuple(types) => types.iter_mut().for_each(|ty| v.ty(ty)),
                    VariantFields::Struct(fields) => {
                        fields.iter_mut().for_each(|f| v.ty(&mut f.ty))
                    }
                }
            }
        }
        S::Impl(item) => {
            v.ty(&mut item.ty);
            item.methods
                .iter_mut()
                .for_each(|method| v.function(method));
        }
        S::Effect(def) => {
            for param in &mut def.params {
                v.ty(&mut param.ty);
            }
            if let Some(ty) = &mut def.return_type {
                v.ty(ty);
            }
        }
        S::Handle { body, handlers } => {
            walk_block(v, body);
            for handler in handlers {
                walk_block(v, &mut handler.body);
            }
        }
        S::Alias { value: e, .. }
        | S::Let { expr: e, .. }
        | S::Print(e)
        | S::Shell(e)
        | S::TryShell(e)
        | S::Raise(e)
        | S::Expr(e)
        | S::Return(e) => v.expr(e),
        S::Spawn { call, .. } | S::Call(call) | S::TryCall(call) => v.call(call),
        S::Fail { status, message } => {
            v.expr(status);
            v.expr(message);
        }
        S::Match { expr, arms, .. } => {
            v.expr(expr);
            for (pattern, arm) in arms {
                v.pattern(pattern);
                v.statement(arm);
            }
        }
        S::If { branch, .. } => {
            walk_condition(v, &mut branch.condition);
            walk_block(v, &mut branch.then_body);
            if let Some(body) = &mut branch.else_body {
                walk_block(v, body);
            }
        }
        S::While { condition, body } => {
            walk_condition(v, condition);
            walk_block(v, body);
        }
        S::For { iterable, body, .. } => {
            v.expr(iterable);
            walk_block(v, body);
        }
        S::Loop { body } => walk_block(v, body),
        S::Pipeline(pipeline) => pipeline.exprs_mut().for_each(|e| v.expr(e)),
        S::Export(bindings) => bindings.iter_mut().for_each(|b| v.expr(&mut b.value)),
        S::Scoped { scope, body } => {
            scope.exprs_mut().for_each(|e| v.expr(e));
            walk_block(v, body);
        }
        S::AllowFail(e) | S::Detach(e) => v.expr(e),
        S::Defer(body) => walk_block(v, body),
        S::Use(_)
        | S::Module(_)
        | S::UnitDef(_)
        | S::Trait(_)
        | S::Await { .. }
        | S::Break
        | S::Continue => {}
    }
}

pub(crate) fn walk_function<V: VisitMut + ?Sized>(v: &mut V, def: &mut FunctionDef) {
    if let Some(receiver) = &mut def.receiver {
        v.ty(receiver);
    }
    for param in &mut def.params {
        v.ty(&mut param.ty);
        if let Some(default) = &mut param.default {
            v.expr(default);
        }
    }
    if let Some(ty) = &mut def.return_type {
        v.ty(ty);
    }
    walk_block(v, &mut def.body);
}

pub(crate) fn walk_condition<V: VisitMut + ?Sized>(v: &mut V, condition: &mut Condition) {
    match condition {
        Condition::Expr(expr) => v.expr(expr),
        Condition::Let { pattern, expr } => {
            v.pattern(pattern);
            v.expr(expr);
        }
        Condition::And(items) | Condition::Or(items) => {
            items.iter_mut().for_each(|item| walk_condition(v, item));
        }
        Condition::Not(inner) => walk_condition(v, inner),
    }
}

pub(crate) fn walk_call<V: VisitMut + ?Sized>(v: &mut V, call: &mut Call) {
    for arg in &mut call.args {
        v.expr(&mut arg.expr);
    }
}

pub(crate) fn walk_expr<V: VisitMut + ?Sized>(v: &mut V, expr: &mut Expr) {
    match expr {
        Expr::Comprehension(c) => {
            for clause in &mut c.clauses {
                match clause {
                    ComprehensionClause::For { iterable, .. } => v.expr(iterable),
                    ComprehensionClause::If(e) => v.expr(e),
                }
            }
            v.expr(&mut c.value);
        }
        Expr::Call(call) => v.call(call),
        Expr::MethodCall(call) => {
            v.expr(&mut call.receiver);
            for arg in &mut call.args {
                v.expr(&mut arg.expr);
            }
        }
        Expr::Add(items) | Expr::Tuple(items) | Expr::List(items) | Expr::Template(items) => {
            items.iter_mut().for_each(|item| v.expr(item));
        }
        Expr::Compare { lhs, rhs, .. }
        | Expr::Range {
            start: lhs,
            end: rhs,
        }
        | Expr::Index {
            base: lhs,
            index: rhs,
        } => {
            v.expr(lhs);
            v.expr(rhs);
        }
        Expr::Try(inner) | Expr::Field { base: inner, .. } | Expr::StatusOf(inner) => v.expr(inner),
        Expr::AsyncBlock(body) => walk_block(v, body),
        Expr::Match(value) => {
            v.expr(&mut value.subject);
            for (pattern, arm) in &mut value.arms {
                v.pattern(pattern);
                v.statement(arm);
            }
        }
        Expr::Pipeline(pipeline) => pipeline.exprs_mut().for_each(|e| v.expr(e)),
        Expr::Variant(variant) => match &mut variant.fields {
            ExprFields::Unit => {}
            ExprFields::Tuple(items) => items.iter_mut().for_each(|item| v.expr(item)),
            ExprFields::Struct(items) => items.iter_mut().for_each(|item| v.expr(&mut item.expr)),
        },
        Expr::FunctionRef(_)
        | Expr::ShellCapture(_)
        | Expr::Quantity(..)
        | Expr::String(_)
        | Expr::Int(_)
        | Expr::Bool(_)
        | Expr::Unit
        | Expr::Var(_)
        | Expr::Env(_) => {}
    }
}

pub(crate) fn walk_pattern<V: VisitMut + ?Sized>(v: &mut V, pattern: &mut Pattern) {
    if let Pattern::Or(alternatives) = pattern {
        alternatives
            .iter_mut()
            .for_each(|alternative| v.pattern(alternative));
    }
    if let Pattern::Variant(variant) = pattern {
        match &mut variant.fields {
            PatternFields::Unit => {}
            PatternFields::Tuple(items) => items.iter_mut().for_each(|item| v.pattern(item)),
            PatternFields::Struct(items) => {
                items
                    .iter_mut()
                    .for_each(|item| v.pattern(&mut item.pattern));
            }
        }
    }
}

pub(crate) fn walk_ty<V: VisitMut + ?Sized>(v: &mut V, ty: &mut Type) {
    match ty {
        Type::Tuple(items) => items.iter_mut().for_each(|item| v.ty(item)),
        Type::List(inner) | Type::Task(inner) => v.ty(inner),
        Type::String | Type::Int | Type::Bool | Type::Unit | Type::Adt(_) => {}
    }
}
