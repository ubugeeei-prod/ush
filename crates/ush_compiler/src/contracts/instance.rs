use super::{
    Checker,
    types::{Index, Ty},
    unify::Substitution,
};
use crate::{
    ast::{FunctionDef, Statement, StatementKind},
    types::{AstString as Name, HeapVec as Vec},
};
use anyhow::{Result, bail, ensure};
impl Checker {
    pub(super) fn instance(
        &mut self,
        original: &FunctionDef,
        args: Vec<Ty>,
        sub: Substitution,
    ) -> Result<(Name, Ty)> {
        let scheme = original.scheme.as_ref().unwrap();
        let result = scheme.result.as_ref().map(|t| sub.apply(t));
        let mut indices: Vec<_> = sub
            .indices
            .iter()
            .filter(|(n, _)| scheme.variables.contains_key(*n))
            .map(|(n, i)| (n.clone(), i.clone()))
            .collect();
        indices.sort_by(|a, b| a.0.cmp(&b.0));
        let key = (original.name.clone(), args.clone(), indices);
        if !self.symbolic
            && let Some(done) = self.instances.get(&key)
        {
            return Ok(done.clone());
        }
        if !self.checking.insert(original.name.clone()) {
            if self.symbolic
                && let Some(ty) = result
            {
                return Ok((original.name.clone(), ty));
            }
            bail!(
                "recursive specialization of {} requires a stable annotated return type",
                original.name
            );
        }
        let mut def = original.clone();
        let name: Name = if self.symbolic
            || (scheme.variables.is_empty() && !args.iter().any(|t| t.function.is_some()))
        {
            def.name.clone()
        } else {
            format!("__ush_inst_{}_{}", def.name, self.instances.len()).into()
        };
        if !self.symbolic
            && let Some(ty) = &result
        {
            self.instances
                .insert(key.clone(), (name.clone(), ty.clone()));
        }
        let mut locals = self.globals.clone();
        for (param, t) in def.params.iter_mut().zip(&args) {
            locals.insert(param.name.clone(), t.clone());
            param.ty = t.erase();
        }
        for (n, kind) in &scheme.variables {
            if *kind == super::signature::Kind::Const {
                let ty = match sub.indices.get(n) {
                    Some(Index::Literal(v)) => Ty::integer(*v),
                    _ if self.symbolic => Ty::named("Int"),
                    _ => bail!("cannot infer const parameter {n}"),
                };
                if let Some(Index::Literal(value)) = sub.indices.get(n)
                    && !self.symbolic
                {
                    def.body.insert(
                        0,
                        Statement::new(
                            self.templates[&original.name].0,
                            StatementKind::Let {
                                name: n.clone(),
                                expr: crate::ast::Expr::Int(*value),
                            },
                        ),
                    );
                }
                locals.insert(n.clone(), ty);
            }
        }
        let parent = core::mem::replace(
            &mut self.declared_failures,
            original.declared_errors.clone(),
        );
        let returns = self.body(&mut def.body, &mut locals)?;
        self.declared_failures = parent;
        let mut inferred = result.unwrap_or_else(|| {
            returns
                .first()
                .cloned()
                .unwrap_or_else(|| Ty::named("()"))
                .widened()
        });
        ensure!(
            inferred.name == "()" || super::completes(&def.body, true),
            "function {} can finish without returning {}",
            original.name,
            inferred.describe()
        );
        if inferred.name == "Fn" {
            let target = returns.first().and_then(|t| t.function.clone());
            ensure!(
                self.symbolic || (target.is_some() && returns.iter().all(|t| t.function == target)),
                "returned function must resolve to one named function"
            );
            inferred.function = target;
        }
        for ty in returns {
            inferred.check(&ty)?;
        }
        self.checking.remove(&original.name);
        if !self.symbolic {
            def.name = name.clone();
            def.return_type = Some(inferred.erase());
            if def.name != "bin" {
                def.scheme = None;
            }
            self.functions.insert(name.clone(), def.clone());
            self.generated.push(Statement::new(
                self.templates[&original.name].0,
                StatementKind::Function(def),
            ));
            self.instances.insert(key, (name.clone(), inferred.clone()));
        }
        Ok((name, inferred))
    }
}
