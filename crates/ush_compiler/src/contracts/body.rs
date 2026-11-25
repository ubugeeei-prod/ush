use super::{Checker, Locals, types::Ty};
use crate::ast::{Condition, Pattern, Statement, StatementKind as S, VariantFields};
use crate::types::HeapVec as Vec;
use anyhow::{Context, Result, anyhow};

impl Checker {
    pub(super) fn body(&mut self, body: &mut [Statement], locals: &mut Locals) -> Result<Vec<Ty>> {
        let mut returns = Vec::new();
        let len = body.len();
        for (i, s) in body.iter_mut().enumerate() {
            self.statement(s, locals, &mut returns, i + 1 == len)
                .with_context(|| format!("line {}", s.line))?;
        }
        Ok(returns)
    }
    pub(super) fn statement(
        &mut self,
        s: &mut Statement,
        locals: &mut Locals,
        returns: &mut Vec<Ty>,
        tail: bool,
    ) -> Result<()> {
        match &mut s.kind {
            S::Let { name, expr } => {
                let t = self.expr(expr, locals)?;
                locals.insert(name.clone(), t);
            }
            S::Return(expr) => returns.push(self.expr(expr, locals)?),
            S::Expr(expr) => {
                let t = self.expr(expr, locals)?;
                if tail {
                    returns.push(t);
                }
            }
            S::Spawn { name, call } => {
                let t = self.call(call, locals)?;
                locals.insert(name.clone(), Ty::app("Task", vec![t]));
            }
            S::Await { name, task } => {
                let t = locals
                    .get(task)
                    .filter(|t| t.name == "Task")
                    .ok_or_else(|| anyhow!("unknown task: {task}"))?
                    .args[0]
                    .clone();
                locals.insert(name.clone(), t);
            }
            S::Print(e) | S::Shell(e) | S::TryShell(e) | S::Alias { value: e, .. } => {
                self.expr(e, locals)?;
            }
            S::Raise(e) => {
                let ty = self.expr(e, locals)?;
                if self.symbolic && !self.checking.is_empty() {
                    self.check_declared_failure(crate::errors::ErrorType::Known(ty.name))?;
                }
            }
            S::Fail { status, message } => {
                if self.symbolic && !self.checking.is_empty() {
                    self.check_declared_failure(crate::errors::ErrorType::Unknown)?;
                }
                Ty::named("Int").check(&self.expr(status, locals)?)?;
                Ty::named("String").check(&self.expr(message, locals)?)?;
            }
            S::Call(c) | S::TryCall(c) => {
                self.call(c, locals)?;
            }
            S::If {
                branch,
                returns_value,
            } => {
                let mut yes = locals.clone();
                self.condition(&mut branch.condition, &mut yes)?;
                self.branch(
                    &mut branch.then_body,
                    &mut yes,
                    returns,
                    *returns_value && tail,
                )?;
                if let Some(other) = &mut branch.else_body {
                    self.branch(other, &mut locals.clone(), returns, *returns_value && tail)?;
                }
            }
            S::Match {
                expr,
                arms,
                returns_value,
            } => {
                let ty = self.expr(expr, locals)?;
                for (p, arm) in arms {
                    let mut arm_env = locals.clone();
                    self.pattern(p, &ty, &mut arm_env)?;
                    self.statement(arm, &mut arm_env, returns, *returns_value && tail)?;
                }
            }
            S::While { condition, body } => {
                let mut inner = locals.clone();
                self.condition(condition, &mut inner)?;
                self.branch(body, &mut inner, returns, false)?;
            }
            S::For {
                name,
                iterable,
                body,
            } => {
                let ty = self.expr(iterable, locals)?;
                let mut inner = locals.clone();
                let item = ty
                    .args
                    .first()
                    .ok_or_else(|| anyhow!("expected iterable"))?;
                inner.insert(name.clone(), item.clone());
                self.branch(body, &mut inner, returns, false)?;
            }
            S::Loop { body } => self.branch(body, &mut locals.clone(), returns, false)?,
            S::Pipeline(pipeline) => {
                for stage in pipeline.exprs_mut() {
                    self.expr(stage, locals)?;
                }
            }
            S::Export(bindings) => {
                for binding in bindings {
                    Ty::named("String").check(&self.expr(&mut binding.value, locals)?)?;
                }
            }
            S::Scoped { scope, body } => {
                for value in scope.exprs_mut() {
                    Ty::named("String").check(&self.expr(value, locals)?)?;
                }
                self.branch(body, &mut locals.clone(), returns, false)?;
            }
            S::Handle { body, handlers } => {
                self.branch(body, &mut locals.clone(), returns, false)?;
                for h in handlers {
                    let mut inner = locals.clone();
                    if let Some(f) = self.functions.get(&h.effect) {
                        for (n, p) in h.params.iter().zip(&f.params) {
                            inner.insert(n.clone(), Ty::from_type(&p.ty));
                        }
                    }
                    self.body(&mut h.body, &mut inner)?;
                }
            }
            S::Function(_)
            | S::Effect(_)
            | S::Enum(_)
            | S::Trait(_)
            | S::Impl(_)
            | S::Use(_)
            | S::Module(_)
            | S::UnitDef(_)
            | S::Break
            | S::Continue => {}
        }
        Ok(())
    }
    fn check_declared_failure(&self, error: crate::errors::ErrorType) -> Result<()> {
        anyhow::ensure!(
            self.declared_failures
                .as_ref()
                .is_some_and(|row| row.iter().any(|e| e == &error)),
            "failure {error:?} is missing from the function's error row"
        );
        Ok(())
    }
    fn branch(
        &mut self,
        body: &mut [Statement],
        env: &mut Locals,
        ret: &mut Vec<Ty>,
        tail: bool,
    ) -> Result<()> {
        let len = body.len();
        for (i, s) in body.iter_mut().enumerate() {
            self.statement(s, env, ret, tail && i + 1 == len)?;
        }
        Ok(())
    }
    fn condition(&mut self, c: &mut Condition, env: &mut Locals) -> Result<()> {
        match c {
            Condition::Expr(e) => {
                self.expr(e, env)?;
                super::refine::narrow(e, env);
            }
            Condition::Let { pattern, expr } => {
                let t = self.expr(expr, env)?;
                self.pattern(pattern, &t, env)?;
            }
            Condition::And(cs) => {
                for c in cs {
                    self.condition(c, env)?;
                }
            }
            Condition::Or(cs) => {
                for c in cs {
                    self.condition(c, &mut env.clone())?;
                }
            }
        }
        Ok(())
    }
    fn pattern(&self, p: &Pattern, t: &Ty, env: &mut Locals) -> Result<()> {
        use crate::ast::PatternFields;
        match p {
            Pattern::Binding(n) => {
                env.insert(n.clone(), t.clone());
            }
            Pattern::Variant(v) => {
                let fields =
                    &crate::env::lookup_variant(&self.enums, &v.enum_name, &v.variant_name)?.fields;
                match (&v.fields, fields) {
                    (PatternFields::Tuple(ps), VariantFields::Tuple(ts)) => {
                        for (p, t) in ps.iter().zip(ts) {
                            self.pattern(p, &Ty::from_type(t), env)?;
                        }
                    }
                    (PatternFields::Struct(ps), VariantFields::Struct(ts)) => {
                        for p in ps {
                            if let Some(t) = ts.iter().find(|t| t.name == p.name) {
                                self.pattern(&p.pattern, &Ty::from_type(&t.ty), env)?;
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
}
