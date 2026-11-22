use super::{
    Checker,
    types::{Index, Ty},
    unify::Substitution,
};
use crate::{
    ast::{Call, FunctionDef, Statement, StatementKind},
    types::{AstString as Name, HeapVec as Vec},
};
use anyhow::{Result, bail, ensure};

impl Checker {
    pub(super) fn call(&mut self, call: &mut Call, locals: &super::Locals) -> Result<Ty> {
        let mut args = Vec::new();
        for arg in &mut call.args {
            args.push(self.expr(&mut arg.expr, locals)?);
        }
        if let Some((_, def)) = self.templates.get(&call.name).cloned() {
            let scheme = def.scheme.as_ref().unwrap();
            ensure!(
                args.len() == scheme.params.len(),
                "wrong number of arguments for {}",
                def.name
            );
            let mut sub = Substitution::default();
            let mut ordered = vec![None; args.len()];
            let mut next = 0;
            for (arg, actual) in call.args.iter().zip(args) {
                let i = if let Some(label) = &arg.label {
                    scheme
                        .params
                        .iter()
                        .position(|p| p.0 == *label)
                        .ok_or_else(|| anyhow::anyhow!("unknown argument label {label}"))?
                } else {
                    while next < ordered.len() && ordered[next].is_some() {
                        next += 1;
                    }
                    next
                };
                ensure!(
                    i < ordered.len() && ordered[i].is_none(),
                    "duplicate argument"
                );
                ordered[i] = Some((actual, &arg.expr));
            }
            let ordered: Vec<_> = ordered.into_iter().flatten().collect();
            for ((name, _), (actual, expr)) in scheme.params.iter().zip(&ordered) {
                if actual.name == "Int" {
                    let index = if actual.min == actual.max {
                        Some(Index::Literal(actual.min))
                    } else if let crate::ast::Expr::Var(var) = expr {
                        Some(Index::Name(var.clone()))
                    } else {
                        None
                    };
                    if let Some(value) = index {
                        sub.indices.insert(name.clone(), value);
                    }
                }
            }
            for ((_, formal), (actual, _)) in scheme.params.iter().zip(&ordered) {
                sub.unify(scheme, formal, actual)?;
            }
            if scheme.variables.is_empty() && !self.symbolic {
                let args = scheme.params.iter().map(|p| p.1.clone()).collect();
                let (name, ty) = self.instance(&def, args, Substitution::default())?;
                call.name = name;
                return Ok(sub.apply(&ty));
            }
            let concrete = scheme.params.iter().map(|(_, t)| sub.apply(t)).collect();
            let (name, result) = self.instance(&def, concrete, sub)?;
            if !self.symbolic {
                call.name = name;
            }
            return Ok(result);
        }
        if let Some(cmd) = crate::commands::lookup(&call.name) {
            cmd.validate(args.len())?;
            return Ok(Ty::from_type(&cmd.ty()));
        }
        if call.name == "format" {
            return Ok(Ty::named("String"));
        }
        let Some(def) = self.functions.get(&call.name) else {
            bail!("unknown function: {}", call.name);
        };
        for (i, (arg, actual)) in call.args.iter().zip(args).enumerate() {
            let param = arg
                .label
                .as_ref()
                .and_then(|n| def.params.iter().find(|p| p.name == *n))
                .or_else(|| def.params.get(i));
            if let Some(param) = param
                && !(self.symbolic && actual.name.starts_with("__T"))
            {
                Ty::from_type(&param.ty).check(&actual)?;
            }
        }
        Ok(def
            .return_type
            .as_ref()
            .map_or_else(|| Ty::named("()"), Ty::from_type))
    }

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
        let name: Name = if self.symbolic || scheme.variables.is_empty() {
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
        let returns = self.body(&mut def.body, &mut locals)?;
        let inferred = result.unwrap_or_else(|| {
            returns
                .first()
                .cloned()
                .unwrap_or_else(|| Ty::named("()"))
                .widened()
        });
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
