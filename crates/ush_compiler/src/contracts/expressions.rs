use super::{
    Checker, Locals,
    types::{Index, Ty},
};
use crate::{
    ast::{Expr, ExprFields, VariantFields},
    types::HeapVec as Vec,
};
use anyhow::{Result, anyhow, ensure};

impl Checker {
    pub(super) fn expr(&mut self, expr: &mut Expr, locals: &Locals) -> Result<Ty> {
        if let Expr::FunctionRef(n) = expr {
            let ty = self.function_ref(n, locals)?;
            if !self.symbolic {
                *expr = Expr::String(n.clone());
            }
            return Ok(ty);
        }
        Ok(match expr {
            Expr::FunctionRef(_) => unreachable!(),
            Expr::Comprehension(c) => self.comprehension(c, locals)?,
            Expr::Int(n) => Ty::integer(*n),
            Expr::Quantity(n, unit) => {
                ensure!(
                    self.units.contains(unit),
                    "unknown measurement unit: {unit}"
                );
                let mut ty = Ty::integer(*n);
                ty.args.push(Ty::named(unit));
                *expr = Expr::Int(*n);
                ty
            }
            Expr::String(_) | Expr::ShellCapture(_) => Ty::named("String"),
            Expr::Bool(_) => Ty::named("Bool"),
            Expr::Unit => Ty::named("()"),
            Expr::Var(n) => locals
                .get(n)
                .cloned()
                .ok_or_else(|| anyhow!("unknown variable: {n}"))?,
            Expr::Try(inner) => self.expr(inner, locals)?,
            Expr::Call(call) => self.call(call, locals)?,
            Expr::Add(items) => {
                let types: Vec<_> = items
                    .iter_mut()
                    .map(|e| self.expr(e, locals))
                    .collect::<Result<_>>()?;
                if types.iter().any(|t| t.name == "String") {
                    Ty::named("String")
                } else {
                    let first = types
                        .iter()
                        .find(|ty| !self.symbolic || !ty.name.starts_with("__T"));
                    let Some(first) = first else {
                        return Ok(types[0].clone());
                    };
                    let mut ty = first.clone();
                    ensure!(
                        ty.name == "Int" || (self.symbolic && ty.name.starts_with("__T")),
                        "addition requires integers of the same unit or a string"
                    );
                    for other in types.iter().filter(|t| !core::ptr::eq(*t, first)) {
                        if self.symbolic && other.name.starts_with("__T") {
                            ty = ty.widened();
                            continue;
                        }
                        ty.clone().widened().check(other)?;
                        let bounds = ty
                            .min
                            .checked_add(other.min)
                            .zip(ty.max.checked_add(other.max));
                        (ty.min, ty.max) = bounds.unwrap_or((i64::MIN, i64::MAX));
                    }
                    ty
                }
            }
            Expr::Compare { lhs, rhs, .. } => {
                self.expr(lhs, locals)?
                    .widened()
                    .check(&self.expr(rhs, locals)?)?;
                Ty::named("Bool")
            }
            Expr::List(items) => {
                let size = items.len();
                let mut items = items.iter_mut();
                let first = self
                    .expr(
                        items
                            .next()
                            .ok_or_else(|| anyhow!("cannot infer empty list element type"))?,
                        locals,
                    )?
                    .widened();
                for item in items {
                    first.check(&self.expr(item, locals)?)?;
                }
                let mut ty = Ty::app("List", vec![first]);
                ty.length = Some(Index::Literal(size as i64));
                ty
            }
            Expr::Tuple(items) => Ty::app(
                "Tuple",
                items
                    .iter_mut()
                    .map(|e| self.expr(e, locals))
                    .collect::<Result<_>>()?,
            ),
            Expr::Range { start, end } => {
                let a = self.expr(start, locals)?;
                let b = self.expr(end, locals)?;
                Ty::named("Int").check(&a)?;
                Ty::named("Int").check(&b)?;
                let mut t = Ty::app("List", vec![Ty::named("Int")]);
                if a.min == a.max && b.min == b.max {
                    t.length = Some(Index::Literal(b.min.saturating_sub(a.min).max(0)));
                }
                t
            }
            Expr::AsyncBlock(body) => {
                let values = self.body(body, &mut locals.clone())?;
                Ty::app(
                    "Task",
                    vec![values.first().cloned().unwrap_or_else(|| Ty::named("()"))],
                )
            }
            Expr::Variant(v) => {
                let fields =
                    crate::env::lookup_variant(&self.enums, &v.enum_name, &v.variant_name)?
                        .fields
                        .clone();
                match (&mut v.fields, fields) {
                    (ExprFields::Tuple(values), VariantFields::Tuple(types)) => {
                        for (v, t) in values.iter_mut().zip(types) {
                            Ty::from_type(&t).check(&self.expr(v, locals)?)?;
                        }
                    }
                    (ExprFields::Struct(values), VariantFields::Struct(types)) => {
                        for v in values {
                            if let Some(t) = types.iter().find(|f| f.name == v.name) {
                                Ty::from_type(&t.ty).check(&self.expr(&mut v.expr, locals)?)?;
                            }
                        }
                    }
                    _ => {}
                }
                Ty::named(&v.enum_name)
            }
            Expr::Field { base, name } => {
                let ty = self.expr(base, locals)?;
                if name == "await" && ty.name == "Task" {
                    return Ok(ty.args[0].clone());
                }
                let item = self
                    .enums
                    .get(&ty.name)
                    .and_then(|d| d.variants.first())
                    .ok_or_else(|| anyhow!("unknown record {}", ty.name))?;
                let VariantFields::Struct(fields) = &item.fields else {
                    return Err(anyhow!("expected a record"));
                };
                Ty::from_type(
                    &fields
                        .iter()
                        .find(|f| f.name == *name)
                        .ok_or_else(|| anyhow!("unknown field {name}"))?
                        .ty,
                )
            }
            Expr::MethodCall(call) => {
                self.expr(&mut call.receiver, locals)?;
                for a in &mut call.args {
                    self.expr(&mut a.expr, locals)?;
                }
                let env = locals
                    .iter()
                    .map(|(n, t)| (n.clone(), crate::codegen::binding_for_name(n, t.erase())))
                    .collect();
                Ty::from_type(&crate::codegen::infer(
                    expr,
                    &env,
                    &self.functions,
                    &self.impls,
                    &self.enums,
                )?)
            }
        })
    }
}
