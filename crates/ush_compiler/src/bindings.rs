//! Check binding permissions before specialization, including unused generic bodies.
mod expressions;

use crate::{
    ast::{
        BindingMode, Condition, FunctionDef, Pattern, PatternFields, Statement, StatementKind as S,
    },
    types::{AstString as Name, Map},
};
use anyhow::{Context, Result, bail};

type Scope = Map<Name, bool>;

pub(crate) fn validate(program: &[Statement]) -> Result<()> {
    let mut globals = Scope::default();
    for s in program {
        if let S::Let { name, .. } | S::Spawn { name, .. } | S::Await { name, .. } = &s.kind
            && s.binding != BindingMode::Assign
        {
            globals.insert(name.clone(), s.binding == BindingMode::Mutable);
        }
    }
    block(program, &mut Scope::default(), &globals)
}

fn block(body: &[Statement], scope: &mut Scope, globals: &Scope) -> Result<()> {
    for s in body {
        statement(s, scope, globals).with_context(|| format!("line {}", s.line))?;
    }
    Ok(())
}

fn bind(s: &Statement, name: &Name, scope: &mut Scope) -> Result<()> {
    match s.binding {
        BindingMode::Assign => match scope.get(name) {
            Some(true) => {}
            Some(false) => bail!(
                "cannot assign to immutable binding `{name}`; declare it with `let mut {name} = ...`"
            ),
            None => bail!(
                "cannot assign to undeclared binding `{name}`; use `let {name} = ...` to declare it"
            ),
        },
        mode => {
            scope.insert(name.clone(), mode == BindingMode::Mutable);
        }
    }
    Ok(())
}

fn function(def: &FunctionDef, globals: &Scope) -> Result<()> {
    let mut scope = globals.clone();
    if def.receiver.is_some() {
        scope.insert("self".into(), false);
    }
    for p in &def.params {
        if let Some(e) = &p.default {
            expressions::expr(e, &scope, globals)?;
        }
        scope.insert(p.name.clone(), false);
    }
    if let Some(scheme) = &def.scheme {
        for name in scheme.variables.keys() {
            scope.insert(name.clone(), false);
        }
    }
    block(&def.body, &mut scope, globals)
}

fn statement(s: &Statement, scope: &mut Scope, globals: &Scope) -> Result<()> {
    use expressions::{call, expr};
    match &s.kind {
        S::Let { name, expr: value } => {
            expr(value, scope, globals)?;
            bind(s, name, scope)?;
        }
        S::Spawn { name, call: value } => {
            call(value, scope, globals)?;
            bind(s, name, scope)?;
        }
        S::Await { name, .. } => bind(s, name, scope)?,
        S::Function(def) => function(def, globals)?,
        S::Impl(item) => {
            for def in &item.methods {
                function(def, globals)?;
            }
        }
        S::Alias { name, value } => {
            expr(value, scope, globals)?;
            scope.insert(name.clone(), false);
        }
        S::Print(e) | S::Shell(e) | S::TryShell(e) | S::Raise(e) | S::Expr(e) | S::Return(e) => {
            expr(e, scope, globals)?
        }
        S::Fail { status, message } => {
            expr(status, scope, globals)?;
            expr(message, scope, globals)?;
        }
        S::Call(c) | S::TryCall(c) => call(c, scope, globals)?,
        S::If { branch, .. } => {
            let mut inner = scope.clone();
            condition(&branch.condition, &mut inner, globals)?;
            block(&branch.then_body, &mut inner, globals)?;
            if let Some(body) = &branch.else_body {
                block(body, &mut scope.clone(), globals)?;
            }
        }
        S::While { condition: c, body } => {
            let mut inner = scope.clone();
            condition(c, &mut inner, globals)?;
            block(body, &mut inner, globals)?;
        }
        S::For {
            name,
            iterable,
            body,
        } => {
            expr(iterable, scope, globals)?;
            let mut inner = scope.clone();
            inner.insert(name.clone(), false);
            block(body, &mut inner, globals)?;
        }
        S::Loop { body } => block(body, &mut scope.clone(), globals)?,
        S::Match {
            expr: value, arms, ..
        } => {
            expr(value, scope, globals)?;
            for (p, arm) in arms {
                let mut inner = scope.clone();
                pattern(p, &mut inner);
                statement(arm, &mut inner, globals)
                    .with_context(|| format!("line {}", arm.line))?;
            }
        }
        S::Handle { body, handlers } => {
            block(body, &mut scope.clone(), globals)?;
            for h in handlers {
                let mut inner = scope.clone();
                for name in &h.params {
                    inner.insert(name.clone(), false);
                }
                block(&h.body, &mut inner, globals)?;
            }
        }
        S::Pipeline(p) => {
            for e in p.exprs() {
                expr(e, scope, globals)?;
            }
        }
        S::Export(bs) => {
            for b in bs {
                expr(&b.value, scope, globals)?;
            }
        }
        S::Scoped { scope: kind, body } => {
            for e in kind.exprs() {
                expr(e, scope, globals)?;
            }
            block(body, &mut scope.clone(), globals)?;
        }
        S::AllowFail(e) => expr(e, scope, globals)?,
        S::Defer(body) => block(body, &mut scope.clone(), globals)?,
        S::Use(_)
        | S::Module(_)
        | S::UnitDef(_)
        | S::Enum(_)
        | S::Trait(_)
        | S::Effect(_)
        | S::Break
        | S::Continue => {}
    }
    Ok(())
}

fn condition(c: &Condition, scope: &mut Scope, globals: &Scope) -> Result<()> {
    match c {
        Condition::Expr(e) => expressions::expr(e, scope, globals)?,
        Condition::Let { pattern: p, expr } => {
            expressions::expr(expr, scope, globals)?;
            pattern(p, scope);
        }
        Condition::And(cs) => {
            for c in cs {
                condition(c, scope, globals)?;
            }
        }
        Condition::Or(cs) => {
            for c in cs {
                condition(c, &mut scope.clone(), globals)?;
            }
        }
        Condition::Not(c) => condition(c, &mut scope.clone(), globals)?,
    }
    Ok(())
}

fn pattern(p: &Pattern, scope: &mut Scope) {
    match p {
        Pattern::Binding(n) => {
            scope.insert(n.clone(), false);
        }
        Pattern::Variant(v) => match &v.fields {
            PatternFields::Tuple(ps) => {
                for p in ps {
                    pattern(p, scope);
                }
            }
            PatternFields::Struct(ps) => {
                for p in ps {
                    pattern(&p.pattern, scope);
                }
            }
            PatternFields::Unit => {}
        },
        _ => {}
    }
}
