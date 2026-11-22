use alloc::boxed::Box;

use crate::types::{AstString as String, HeapVec as Vec};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Type {
    String,
    Int,
    Bool,
    Unit,
    Adt(String),
    Tuple(Vec<Type>),
    List(Box<Type>),
    Task(Box<Type>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompareOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Type {
    pub(crate) fn is_scalar(&self) -> bool {
        matches!(self, Self::Int | Self::String | Self::Unit | Self::Bool)
    }
    pub(crate) fn is_structured(&self) -> bool {
        matches!(self, Self::Tuple(_) | Self::List(_) | Self::Adt(_))
    }
    pub(crate) fn render(&self) -> String {
        match self {
            Self::String => "String".into(),
            Self::Int => "Int".into(),
            Self::Bool => "Bool".into(),
            Self::Unit => "()".into(),
            Self::Adt(name) => name.clone(),
            Self::Tuple(items) => format!(
                "({})",
                items
                    .iter()
                    .map(Type::render)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .into(),
            Self::List(inner) => format!("[{}]", inner.render()).into(),
            Self::Task(inner) => format!("Task<{}>", inner.render()).into(),
        }
    }
}
