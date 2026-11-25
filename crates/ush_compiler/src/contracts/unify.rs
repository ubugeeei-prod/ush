use super::{
    signature::{Kind, Scheme},
    types::{Index, Ty},
};
use crate::types::{AstString as Name, Map, Set};
use anyhow::{Result, bail, ensure};

#[derive(Default, Clone)]
pub(super) struct Substitution {
    pub types: Map<Name, Ty>,
    pub constructors: Map<Name, (Name, Option<Index>)>,
    pub indices: Map<Name, Index>,
}

impl Substitution {
    pub fn apply(&self, ty: &Ty) -> Ty {
        if let Some(found) = self.types.get(&ty.name) {
            return found.clone();
        }
        let mut ty = ty.clone();
        if let Some((name, size)) = self.constructors.get(&ty.name) {
            ty.name = name.clone();
            ty.length = size.clone();
        }
        if let Some(Index::Name(n)) = &ty.length
            && let Some(index) = self.indices.get(n)
        {
            ty.length = Some(index.clone());
        }
        ty.args = ty.args.iter().map(|a| self.apply(a)).collect();
        ty
    }
    pub fn unify(&mut self, scheme: &Scheme, expected: &Ty, actual: &Ty) -> Result<()> {
        match scheme.variables.get(&expected.name) {
            Some(Kind::Type(0)) => {
                ensure!(
                    expected.args.is_empty(),
                    "type variable does not accept arguments"
                );
                let found = actual.clone().widened();
                if let Some(old) = self.types.get(&expected.name) {
                    old.check(&found)?;
                } else {
                    self.types.insert(expected.name.clone(), found);
                }
                return Ok(());
            }
            Some(Kind::Type(arity)) => {
                ensure!(
                    *arity == actual.args.len() && *arity == expected.args.len(),
                    "constructor kind mismatch for {}",
                    expected.name
                );
                if let Some(old) = self.constructors.insert(
                    expected.name.clone(),
                    (actual.name.clone(), actual.length.clone()),
                ) {
                    ensure!(
                        old == (actual.name.clone(), actual.length.clone()),
                        "inconsistent type constructor for {}",
                        expected.name
                    );
                }
            }
            _ => ensure!(
                expected.name == actual.name,
                "expected {}, found {}",
                expected.describe(),
                actual.describe()
            ),
        }
        if let Some(Index::Name(n)) = &expected.length {
            let size = actual
                .length
                .clone()
                .ok_or_else(|| anyhow::anyhow!("cannot infer array length {n}"))?;
            ensure!(
                self.indices.contains_key(n) || scheme.variables.get(n) == Some(&Kind::Const),
                "cannot prove the value of length parameter {n}"
            );
            if let Some(old) = self.indices.insert(n.clone(), size.clone()) {
                ensure!(old == size, "inconsistent array length {n}");
            }
        }
        ensure!(
            expected.args.len() == actual.args.len(),
            "type constructor arity mismatch"
        );
        for (a, b) in expected.args.iter().zip(&actual.args) {
            self.unify(scheme, a, b)?;
        }
        self.apply(expected).check(actual)
    }
}

pub(super) fn validate_type(
    ty: &Ty,
    scheme: &Scheme,
    units: &Set<Name>,
    nominal: &Set<Name>,
) -> Result<()> {
    if let Some(Index::Name(n)) = &ty.length {
        ensure!(
            scheme.variables.get(n) == Some(&Kind::Const)
                || scheme
                    .params
                    .iter()
                    .any(|(name, t)| name == n && t.name == "Int"),
            "undeclared length index {n}"
        );
    }
    let arity = match ty.name.as_str() {
        "Int" if !ty.args.is_empty() => {
            ensure!(
                ty.args.len() == 1
                    && ty.args[0].args.is_empty()
                    && units.contains(&ty.args[0].name),
                "unknown measurement unit in {}",
                ty.describe()
            );
            return Ok(());
        }
        "Int" | "String" | "Bool" | "()" => 0,
        "List" | "Task" => 1,
        "Tuple" => ty.args.len(),
        "Fn" if !ty.args.is_empty() => ty.args.len(),
        name if nominal.contains(name) => 0,
        _ => match scheme.variables.get(&ty.name) {
            Some(Kind::Type(n)) => *n,
            _ => bail!("unknown type or type constructor: {}", ty.name),
        },
    };
    ensure!(
        arity == ty.args.len(),
        "kind mismatch: {} expects {arity} type arguments",
        ty.name
    );
    for item in &ty.args {
        validate_type(item, scheme, units, nominal)?;
    }
    Ok(())
}
