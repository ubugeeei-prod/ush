use super::{
    Checker,
    types::{Index, Ty},
    unify::Substitution,
};
use crate::{ast::Call, types::HeapVec as Vec};
use anyhow::{Result, bail, ensure};

impl Checker {
    pub(super) fn call(&mut self, call: &mut Call, locals: &super::Locals) -> Result<Ty> {
        if let Some(ty) = locals.get(&call.name).cloned() {
            return self.callback(call, &ty, locals);
        }
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
            let mut ordered: Vec<_> = ordered.into_iter().flatten().collect();
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
                if formal.name != "Fn" {
                    sub.unify(scheme, formal, actual)?;
                }
            }
            for ((_, formal), (actual, _)) in scheme.params.iter().zip(&mut ordered) {
                if formal.name == "Fn" {
                    self.contextual_callback(actual, &sub.apply(formal))?;
                    sub.unify(scheme, formal, actual)?;
                }
            }
            if scheme.variables.is_empty()
                && !self.symbolic
                && !scheme.params.iter().any(|p| p.1.name == "Fn")
            {
                let args = scheme.params.iter().map(|p| p.1.clone()).collect();
                let (name, ty) = self.instance(&def, args, Substitution::default())?;
                call.name = name;
                return Ok(sub.apply(&ty));
            }
            let concrete = scheme
                .params
                .iter()
                .zip(&ordered)
                .map(|((_, t), (actual, _))| {
                    let mut t = sub.apply(t);
                    if t.name == "Fn" {
                        t.function = actual.function.clone();
                    }
                    t
                })
                .collect();
            let (name, result) = self.instance(&def, concrete, sub)?;
            if !self.symbolic {
                call.name = name;
            }
            return Ok(result);
        }
        if let Some(cmd) = crate::commands::lookup(&call.name) {
            cmd.validate(args.len())?;
            for (arg, ty) in call.args.iter().zip(args) {
                ensure!(arg.label.is_none(), "command arguments are positional");
                if self.symbolic && ty.name.starts_with("__T") {
                    continue;
                }
                ensure!(
                    ty.name == "String" || (ty.name == "Int" && cmd.word != "cd"),
                    "invalid argument type for $bin::{}",
                    cmd.word
                );
            }
            return Ok(Ty::from_type(&cmd.ty()));
        }
        if call.name == "std::test::assert_eq" {
            ensure!(args.len() == 2, "assert_eq expects two arguments");
            ensure!(
                args[0].same_shape(&args[1]),
                "assert_eq arguments must have the same type"
            );
            return Ok(Ty::named("()"));
        }
        if call.name == "std::io::echo" {
            ensure!(
                call.args.iter().all(|arg| arg.label.is_none()),
                "echo arguments are positional"
            );
            return Ok(Ty::named("String"));
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
}
