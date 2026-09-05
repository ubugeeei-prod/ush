//! The inference walk: what a block, a statement, and an expression
//! touch.

use crate::ast::{
    Call, Condition, Expr, ExprFields, IfBranch, MethodCall, Statement, StatementKind,
};

use super::table::{builtin_call_effects, builtin_method_effects};
use super::{Effect, EffectDeclarations, EffectSet, FunctionEffectRegistry};

/// What the walk needs to answer a call: the rows of the program's
/// own functions, and which names are effect operations.
#[derive(Clone, Copy)]
pub(super) struct Context<'a> {
    pub registry: &'a FunctionEffectRegistry,
    pub declarations: &'a EffectDeclarations,
}

pub(super) fn block_effects(body: &[Statement], ctx: Context<'_>) -> EffectSet {
    let mut effects = EffectSet::empty();
    for statement in body {
        effects.merge(&statement_effects(statement, ctx));
    }
    effects
}

fn statement_effects(statement: &Statement, ctx: Context<'_>) -> EffectSet {
    match &statement.kind {
        // A nested definition contributes nothing until it is called.
        StatementKind::Use(_)
        | StatementKind::Enum(_)
        | StatementKind::Trait(_)
        | StatementKind::Impl(_)
        | StatementKind::Function(_)
        | StatementKind::Effect(_)
        | StatementKind::Break
        | StatementKind::Continue => EffectSet::empty(),

        // A handler answers the effect for the block it wraps, so the
        // row that escapes is the body's minus what was handled —
        // plus whatever the handler bodies themselves need.
        StatementKind::Handle { body, handlers } => {
            let mut handled = EffectSet::empty();
            for handler in handlers {
                handled.insert_user(handler.effect.clone());
            }
            let mut effects = block_effects(body, ctx).difference(&handled);
            for handler in handlers {
                effects.merge(&block_effects(&handler.body, ctx));
            }
            effects
        }

        StatementKind::Print(expr) => EffectSet::of(Effect::Io).union(&expr_effects(expr, ctx)),
        StatementKind::Shell(expr) | StatementKind::TryShell(expr) => {
            EffectSet::of(Effect::Exec).union(&expr_effects(expr, ctx))
        }
        StatementKind::Spawn { call, .. } => {
            EffectSet::of(Effect::Task).union(&call_effects(call, ctx))
        }
        StatementKind::Await { .. } => EffectSet::of(Effect::Task),

        StatementKind::Alias { value, .. }
        | StatementKind::Let { expr: value, .. }
        | StatementKind::Raise(value)
        | StatementKind::Expr(value)
        | StatementKind::Return(value) => expr_effects(value, ctx),

        StatementKind::Call(call) | StatementKind::TryCall(call) => call_effects(call, ctx),

        StatementKind::Match { expr, arms, .. } => {
            let mut effects = expr_effects(expr, ctx);
            for (_, body) in arms {
                effects.merge(&statement_effects(body, ctx));
            }
            effects
        }
        StatementKind::If { branch, .. } => branch_effects(branch, ctx),
        StatementKind::While { condition, body } => {
            condition_effects(condition, ctx).union(&block_effects(body, ctx))
        }
        StatementKind::For { iterable, body, .. } => {
            expr_effects(iterable, ctx).union(&block_effects(body, ctx))
        }
        StatementKind::Loop { body } => block_effects(body, ctx),
    }
}

fn branch_effects(branch: &IfBranch, ctx: Context<'_>) -> EffectSet {
    let mut effects = condition_effects(&branch.condition, ctx);
    effects.merge(&block_effects(&branch.then_body, ctx));
    // `elif` is parsed as a nested `if` inside `else_body`, so the
    // recursion is already covered by walking the else block.
    if let Some(otherwise) = &branch.else_body {
        effects.merge(&block_effects(otherwise, ctx));
    }
    effects
}

fn condition_effects(condition: &Condition, ctx: Context<'_>) -> EffectSet {
    match condition {
        Condition::Expr(expr) => expr_effects(expr, ctx),
        Condition::Let { expr, .. } => expr_effects(expr, ctx),
        Condition::And(parts) | Condition::Or(parts) => {
            let mut effects = EffectSet::empty();
            for part in parts {
                effects.merge(&condition_effects(part, ctx));
            }
            effects
        }
    }
}

fn expr_effects(expr: &Expr, ctx: Context<'_>) -> EffectSet {
    match expr {
        Expr::String(_) | Expr::Int(_) | Expr::Bool(_) | Expr::Unit | Expr::Var(_) => {
            EffectSet::empty()
        }
        Expr::Add(parts) | Expr::Tuple(parts) | Expr::List(parts) => {
            let mut effects = EffectSet::empty();
            for part in parts {
                effects.merge(&expr_effects(part, ctx));
            }
            effects
        }
        Expr::Compare { lhs, rhs, .. } => expr_effects(lhs, ctx).union(&expr_effects(rhs, ctx)),
        Expr::Range { start, end } => expr_effects(start, ctx).union(&expr_effects(end, ctx)),
        Expr::Try(inner) | Expr::Field { base: inner, .. } => expr_effects(inner, ctx),
        Expr::Call(call) => call_effects(call, ctx),
        Expr::MethodCall(call) => method_effects(call, ctx),
        Expr::Variant(variant) => match &variant.fields {
            ExprFields::Unit => EffectSet::empty(),
            ExprFields::Tuple(parts) => {
                let mut effects = EffectSet::empty();
                for part in parts {
                    effects.merge(&expr_effects(part, ctx));
                }
                effects
            }
            ExprFields::Struct(fields) => {
                let mut effects = EffectSet::empty();
                for field in fields {
                    effects.merge(&expr_effects(&field.expr, ctx));
                }
                effects
            }
        },
        Expr::AsyncBlock(body) => EffectSet::of(Effect::Task).union(&block_effects(body, ctx)),
    }
}

fn call_effects(call: &Call, ctx: Context<'_>) -> EffectSet {
    let mut effects = builtin_call_effects(&call.name);
    // `do log(…)` lowers to a call of the operation's own name, so
    // this is where performing an effect enters the row.
    if ctx.declarations.contains(&call.name) {
        effects.insert_user(call.name.clone());
    }
    if let Some(user) = ctx.registry.get(&call.name) {
        effects.merge(user);
    }
    if call.asynchronous {
        effects.insert(Effect::Task);
    }
    for arg in &call.args {
        effects.merge(&expr_effects(&arg.expr, ctx));
    }
    effects
}

fn method_effects(call: &MethodCall, ctx: Context<'_>) -> EffectSet {
    let mut effects = builtin_method_effects(&call.method);
    effects.merge(&expr_effects(&call.receiver, ctx));
    for arg in &call.args {
        effects.merge(&expr_effects(&arg.expr, ctx));
    }
    effects
}
