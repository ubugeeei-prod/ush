mod control;
mod items;
mod ty;

use alloc::boxed::Box;

use crate::types::{AstString as String, HeapVec as Vec};
pub(crate) use control::{Condition, IfBranch};
pub(crate) use items::{
    Attribute, Call, CallArg, EffectDef, EffectHandler, EnumDef, ExprFields, FunctionDef,
    FunctionParam, MethodCall, NamedExpr, NamedFieldType, NamedPattern, PatternFields, TraitDef,
    TraitImpl, UseItem, VariantDef, VariantExpr, VariantFields, VariantPattern,
};
pub(crate) use ty::{CompareOp, Type};

#[derive(Debug, Clone)]
pub(crate) struct Statement {
    pub line: usize,
    pub kind: StatementKind,
}

impl Statement {
    pub(crate) fn new(line: usize, kind: StatementKind) -> Self {
        Self { line, kind }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum StatementKind {
    Use(Vec<UseItem>),
    Enum(EnumDef),
    Trait(TraitDef),
    Impl(TraitImpl),
    Function(FunctionDef),
    Effect(EffectDef),
    /// `try { … } with log { (m) => … }`
    Handle {
        body: Vec<Statement>,
        handlers: Vec<EffectHandler>,
    },
    Alias {
        name: String,
        value: Expr,
    },
    Let {
        name: String,
        expr: Expr,
    },
    Spawn {
        name: String,
        call: Call,
    },
    Await {
        name: String,
        task: String,
    },
    Print(Expr),
    Shell(Expr),
    TryShell(Expr),
    Raise(Expr),
    Expr(Expr),
    Call(Call),
    TryCall(Call),
    Return(Expr),
    Match {
        expr: Expr,
        arms: Vec<(Pattern, Box<Statement>)>,
        returns_value: bool,
    },
    If {
        branch: IfBranch,
        returns_value: bool,
    },
    While {
        condition: Condition,
        body: Vec<Statement>,
    },
    For {
        name: String,
        iterable: Expr,
        body: Vec<Statement>,
    },
    Loop {
        body: Vec<Statement>,
    },
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    String(String),
    Int(i64),
    Bool(bool),
    Unit,
    Var(String),
    Add(Vec<Expr>),
    Compare {
        lhs: Box<Expr>,
        op: CompareOp,
        rhs: Box<Expr>,
    },
    Try(Box<Expr>),
    Call(Call),
    MethodCall(MethodCall),
    Field {
        base: Box<Expr>,
        name: String,
    },
    Variant(VariantExpr),
    Tuple(Vec<Expr>),
    List(Vec<Expr>),
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
    },
    AsyncBlock(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub(crate) enum Pattern {
    Wildcard,
    Binding(String),
    String(String),
    Int(i64),
    Bool(bool),
    Unit,
    Variant(VariantPattern),
}

pub(crate) type MatchArm = (Pattern, Box<Statement>);
