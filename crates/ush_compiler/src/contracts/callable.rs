use super::{Checker, Locals, types::Ty};
use crate::{
    ast::{Call, FunctionDef},
    types::{AstString as Name, HeapVec as Vec},
};
use anyhow::{Result, anyhow, ensure};

impl Checker {
    pub(super) fn contextual_callback(&mut self, actual: &mut Ty, expected: &Ty) -> Result<()> {
        let Some(name) = &actual.function else {
            return Ok(());
        };
        let Some((_, def)) = self.templates.get(name).cloned() else {
            return Ok(());
        };
        let scheme = def.scheme.as_ref().unwrap();
        if scheme.variables.is_empty() {
            return Ok(());
        }
        ensure!(
            expected.args.len() == scheme.params.len() + 1,
            "callback arity mismatch"
        );
        let mut sub = super::unify::Substitution::default();
        for ((_, formal), concrete) in scheme.params.iter().zip(&expected.args) {
            sub.unify(scheme, formal, concrete)?;
        }
        let params: Vec<_> = scheme.params.iter().map(|p| sub.apply(&p.1)).collect();
        let (target, result) = self.instance(&def, params.clone(), sub)?;
        actual.args = params;
        actual.args.push(result);
        actual.function = Some(target);
        Ok(())
    }
    pub(super) fn function_ref(&mut self, name: &Name, locals: &Locals) -> Result<Ty> {
        if let Some(ty) = locals.get(name) {
            ensure!(ty.name == "Fn", "{name} is not a function");
            return Ok(ty.clone());
        }
        let def = self
            .functions
            .get(name)
            .ok_or_else(|| anyhow!("unknown function: {name}"))?;
        let mut ty = signature(def)?;
        ty.function = Some(name.clone());
        Ok(ty)
    }
    pub(super) fn callback(&mut self, call: &mut Call, ty: &Ty, locals: &Locals) -> Result<Ty> {
        ensure!(ty.name == "Fn", "{} is not a function", call.name);
        if let Some(target) = &ty.function
            && self
                .templates
                .get(target)
                .is_some_and(|(_, f)| f.scheme.as_ref().is_some_and(|s| !s.variables.is_empty()))
        {
            call.name = target.clone();
            let mut scope = locals.clone();
            scope.remove(target);
            return self.call(call, &scope);
        }
        let (result, params) = ty
            .args
            .split_last()
            .ok_or_else(|| anyhow!("invalid Fn type"))?;
        ensure!(
            params.len() == call.args.len(),
            "wrong number of arguments for {}",
            call.name
        );
        for (param, arg) in params.iter().zip(&mut call.args) {
            ensure!(
                arg.label.is_none(),
                "function value arguments are positional"
            );
            param.check(&self.expr(&mut arg.expr.clone(), locals)?)?;
        }
        if let Some(target) = &ty.function {
            call.name = target.clone();
            // Arguments were checked above; dispatch directly without treating a
            // same-named local binding as another callback.
            let mut scope = locals.clone();
            scope.remove(target);
            let actual = self.call(call, &scope)?;
            result.check(&actual)?;
            return Ok(actual);
        }
        ensure!(
            self.symbolic,
            "cannot determine function value for {}",
            call.name
        );
        Ok(result.clone())
    }
}
fn signature(def: &FunctionDef) -> Result<Ty> {
    let (mut args, result): (Vec<_>, _) = if let Some(scheme) = &def.scheme {
        (
            scheme.params.iter().map(|p| p.1.clone()).collect(),
            scheme.result.clone(),
        )
    } else {
        (
            def.params.iter().map(|p| Ty::from_type(&p.ty)).collect(),
            def.return_type.as_ref().map(Ty::from_type),
        )
    };
    args.push(result.ok_or_else(|| {
        anyhow!(
            "function reference {} requires an annotated return type",
            def.name
        )
    })?);
    Ok(Ty::app("Fn", args))
}
