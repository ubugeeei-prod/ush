use crate::{
    ast::Type,
    types::{AstString as Name, HeapVec as Vec},
};
use alloc::boxed::Box;
use anyhow::{Result, bail};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Index {
    Literal(i64),
    Name(Name),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Ty {
    pub name: Name,
    pub function: Option<Name>,
    pub args: Vec<Ty>,
    pub length: Option<Index>,
    pub min: i64,
    pub max: i64,
}

impl Ty {
    pub fn named(name: &str) -> Self {
        Self {
            name: name.into(),
            function: None,
            args: Vec::new(),
            length: None,
            min: i64::MIN,
            max: i64::MAX,
        }
    }
    pub fn app(name: &str, args: Vec<Self>) -> Self {
        Self {
            args,
            ..Self::named(name)
        }
    }
    pub fn integer(n: i64) -> Self {
        Self {
            min: n,
            max: n,
            ..Self::named("Int")
        }
    }
    pub fn erase(&self) -> Type {
        match self.name.as_str() {
            "Int" => Type::Int,
            "String" | "Fn" => Type::String,
            "Bool" => Type::Bool,
            "()" => Type::Unit,
            "List" if self.args.len() == 1 => Type::List(Box::new(self.args[0].erase())),
            "Task" if self.args.len() == 1 => Type::Task(Box::new(self.args[0].erase())),
            "Tuple" => Type::Tuple(self.args.iter().map(Self::erase).collect()),
            name => Type::Adt(name.into()),
        }
    }
    pub fn from_type(ty: &Type) -> Self {
        match ty {
            Type::List(t) => Self::app("List", vec![Self::from_type(t)]),
            Type::Task(t) => Self::app("Task", vec![Self::from_type(t)]),
            Type::Tuple(ts) => Self::app("Tuple", ts.iter().map(Self::from_type).collect()),
            _ => Self::named(&ty.render()),
        }
    }
    pub fn widened(mut self) -> Self {
        self.min = i64::MIN;
        self.max = i64::MAX;
        self
    }
    pub fn describe(&self) -> Name {
        if self.args.is_empty() {
            return self.name.clone();
        }
        format!(
            "{}<{}>",
            self.name,
            self.args
                .iter()
                .map(Self::describe)
                .collect::<Vec<_>>()
                .join(", ")
        )
        .into()
    }
    pub fn same_shape(&self, other: &Self) -> bool {
        self.name == other.name
            && self.args.len() == other.args.len()
            && self
                .args
                .iter()
                .zip(&other.args)
                .all(|(a, b)| a.same_shape(b))
    }
    pub fn check(&self, found: &Self) -> Result<()> {
        if self.function.is_some() && self.function != found.function {
            bail!("runtime selection between different function values is not supported");
        }
        if self.name != found.name || self.args.len() != found.args.len() {
            bail!("expected {}, found {}", self.describe(), found.describe());
        }
        if self.length.is_some() && self.length != found.length {
            bail!(
                "array length mismatch: expected {:?}, found {:?}",
                self.length,
                found.length
            );
        }
        if self.min > found.min || self.max < found.max {
            bail!(
                "cannot prove integer refinement (required {}..={})",
                self.min,
                self.max
            );
        }
        for (a, b) in self.args.iter().zip(&found.args) {
            a.check(b)?;
        }
        Ok(())
    }
}
