use alloc::boxed::Box;

use crate::errors::ErrorSet;
use crate::types::{AstString as String, HeapVec as Vec};

use super::Type;

#[derive(Debug, Clone)]
pub(crate) struct TraitDef {
    pub name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct TraitImpl {
    pub trait_name: Option<String>,
    pub ty: Type,
    pub methods: Vec<FunctionDef>,
}

#[derive(Debug, Clone)]
pub(crate) struct Attribute {
    pub name: String,
    pub value: Option<super::Expr>,
    /// The argument list exactly as written, split on top-level
    /// commas. Attributes such as `#[effects(fs, exec)]` are lists of
    /// names rather than a single expression, so the raw text is kept
    /// alongside the parsed `value`.
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct UseItem {
    pub path: String,
    pub alias: String,
}

#[derive(Debug, Clone)]
pub(crate) struct FunctionDef {
    pub attrs: Vec<Attribute>,
    pub name: String,
    pub receiver: Option<Type>,
    pub params: Vec<FunctionParam>,
    pub return_type: Option<Type>,
    pub declared_errors: Option<ErrorSet>,
    pub body: Vec<super::Statement>,
}

#[derive(Debug, Clone)]
pub(crate) struct Call {
    pub name: String,
    pub args: Vec<CallArg>,
    pub asynchronous: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct MethodCall {
    pub receiver: Box<super::Expr>,
    pub method: String,
    pub args: Vec<CallArg>,
}

#[derive(Debug, Clone)]
pub(crate) struct FunctionParam {
    pub name: String,
    pub ty: Type,
    pub default: Option<super::Expr>,
    pub cli_alias: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct CallArg {
    pub label: Option<String>,
    pub expr: super::Expr,
}

#[derive(Debug, Clone)]
pub(crate) struct EnumDef {
    pub name: String,
    pub variants: Vec<VariantDef>,
}

#[derive(Debug, Clone)]
pub(crate) struct VariantDef {
    pub name: String,
    pub fields: VariantFields,
}

#[derive(Debug, Clone)]
pub(crate) enum VariantFields {
    Unit,
    Tuple(Vec<Type>),
    Struct(Vec<NamedFieldType>),
}

#[derive(Debug, Clone)]
pub(crate) struct NamedFieldType {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub(crate) struct VariantExpr {
    pub enum_name: String,
    pub variant_name: String,
    pub fields: ExprFields,
}

#[derive(Debug, Clone)]
pub(crate) enum ExprFields {
    Unit,
    Tuple(Vec<super::Expr>),
    Struct(Vec<NamedExpr>),
}

#[derive(Debug, Clone)]
pub(crate) struct NamedExpr {
    pub name: String,
    pub expr: super::Expr,
}

#[derive(Debug, Clone)]
pub(crate) struct VariantPattern {
    pub enum_name: String,
    pub variant_name: String,
    pub fields: PatternFields,
}

#[derive(Debug, Clone)]
pub(crate) enum PatternFields {
    Unit,
    Tuple(Vec<super::Pattern>),
    Struct(Vec<NamedPattern>),
}

#[derive(Debug, Clone)]
pub(crate) struct NamedPattern {
    pub name: String,
    pub pattern: super::Pattern,
}
