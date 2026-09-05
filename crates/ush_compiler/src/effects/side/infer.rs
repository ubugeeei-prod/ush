//! The inference walk: what a block, a statement, and an expression
//! touch.

use crate::ast::{
    Call, Condition, Expr, ExprFields, IfBranch, MethodCall, Statement, StatementKind,
};

use super::table::{builtin_call_effects, builtin_method_effects};
use super::{Effect, EffectSet, FunctionEffectRegistry};

pub(super) fn block_effects(body: &[Statement], registry: &FunctionEffectRegistry) -> EffectSet {
    let mut effects = EffectSet::empty();
    for statement in body {
        effects.merge(statement_effects(statement, registry));
    }
    effects
}

fn statement_effects(statement: &Statement, registry: &FunctionEffectRegistry) -> EffectSet {
    match &statement.kind {
        // A nested definition contributes nothing until it is called.
        StatementKind::Use(_)
        | StatementKind::Enum(_)
        | StatementKind::Trait(_)
        | StatementKind::Impl(_)
        | StatementKind::Function(_)
        | StatementKind::Break
        | StatementKind::Continue => EffectSet::empty(),

        StatementKind::Print(expr) => EffectSet::of(Effect::Io).union(expr_effects(expr, registry)),
        StatementKind::Shell(expr) | StatementKind::TryShell(expr) => {
            EffectSet::of(Effect::Exec).union(expr_effects(expr, registry))
        }
        StatementKind::Spawn { call, .. } => {
            EffectSet::of(Effect::Task).union(call_effects(call, registry))
        }
        StatementKind::Await { .. } => EffectSet::of(Effect::Task),

        StatementKind::Alias { value, .. }
        | StatementKind::Let { expr: value, .. }
        | StatementKind::Raise(value)
        | StatementKind::Expr(value)
        | StatementKind::Return(value) => expr_effects(value, registry),

        StatementKind::Call(call) | StatementKind::TryCall(call) => call_effects(call, registry),

        StatementKind::Match { expr, arms, .. } => {
            let mut effects = expr_effects(expr, registry);
            for (_, body) in arms {
                effects.merge(statement_effects(body, registry));
            }
            effects
        }
        StatementKind::If { branch, .. } => branch_effects(branch, registry),
        StatementKind::While { condition, body } => {
            condition_effects(condition, registry).union(block_effects(body, registry))
        }
        StatementKind::For { iterable, body, .. } => {
            expr_effects(iterable, registry).union(block_effects(body, registry))
        }
        StatementKind::Loop { body } => block_effects(body, registry),
    }
}

fn branch_effects(branch: &IfBranch, registry: &FunctionEffectRegistry) -> EffectSet {
    let mut effects = condition_effects(&branch.condition, registry);
    effects.merge(block_effects(&branch.then_body, registry));
    // `elif` is parsed as a nested `if` inside `else_body`, so the
    // recursion is already covered by walking the else block.
    if let Some(otherwise) = &branch.else_body {
        effects.merge(block_effects(otherwise, registry));
    }
    effects
}

fn condition_effects(condition: &Condition, registry: &FunctionEffectRegistry) -> EffectSet {
    match condition {
        Condition::Expr(expr) => expr_effects(expr, registry),
        Condition::Let { expr, .. } => expr_effects(expr, registry),
        Condition::And(parts) | Condition::Or(parts) => {
            let mut effects = EffectSet::empty();
            for part in parts {
                effects.merge(condition_effects(part, registry));
            }
            effects
        }
    }
}

fn expr_effects(expr: &Expr, registry: &FunctionEffectRegistry) -> EffectSet {
    match expr {
        Expr::String(_) | Expr::Int(_) | Expr::Bool(_) | Expr::Unit | Expr::Var(_) => {
            EffectSet::empty()
        }
        Expr::Add(parts) | Expr::Tuple(parts) | Expr::List(parts) => {
            let mut effects = EffectSet::empty();
            for part in parts {
                effects.merge(expr_effects(part, registry));
            }
            effects
        }
        Expr::Compare { lhs, rhs, .. } => {
            expr_effects(lhs, registry).union(expr_effects(rhs, registry))
        }
        Expr::Range { start, end } => {
            expr_effects(start, registry).union(expr_effects(end, registry))
        }
        Expr::Try(inner) | Expr::Field { base: inner, .. } => expr_effects(inner, registry),
        Expr::Call(call) => call_effects(call, registry),
        Expr::MethodCall(call) => method_effects(call, registry),
        Expr::Variant(variant) => match &variant.fields {
            ExprFields::Unit => EffectSet::empty(),
            ExprFields::Tuple(parts) => {
                let mut effects = EffectSet::empty();
                for part in parts {
                    effects.merge(expr_effects(part, registry));
                }
                effects
            }
            ExprFields::Struct(fields) => {
                let mut effects = EffectSet::empty();
                for field in fields {
                    effects.merge(expr_effects(&field.expr, registry));
                }
                effects
            }
        },
        Expr::AsyncBlock(body) => EffectSet::of(Effect::Task).union(block_effects(body, registry)),
    }
}

fn call_effects(call: &Call, registry: &FunctionEffectRegistry) -> EffectSet {
    let mut effects = builtin_call_effects(&call.name);
    if let Some(user) = registry.get(&call.name) {
        effects.merge(*user);
    }
    if call.asynchronous {
        effects.insert(Effect::Task);
    }
    for arg in &call.args {
        effects.merge(expr_effects(&arg.expr, registry));
    }
    effects
}

fn method_effects(call: &MethodCall, registry: &FunctionEffectRegistry) -> EffectSet {
    let mut effects = builtin_method_effects(&call.method);
    effects.merge(expr_effects(&call.receiver, registry));
    for arg in &call.args {
        effects.merge(expr_effects(&arg.expr, registry));
    }
    effects
}
