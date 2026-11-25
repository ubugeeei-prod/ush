mod control;
mod items;
mod shell;
mod ty;

use alloc::boxed::Box;

use crate::types::{AstString as String, HeapVec as Vec};
pub(crate) use control::{Condition, IfBranch};
pub(crate) use items::{
    Attribute, Call, CallArg, EffectDef, EffectHandler, EnumDef, ExprFields, FunctionDef,
    FunctionParam, MethodCall, ModuleDecl, NamedExpr, NamedFieldType, NamedPattern, PatternFields,
    TraitDef, TraitImpl, UseItem, VariantDef, VariantExpr, VariantFields, VariantPattern,
};
pub(crate) use shell::{EnvBinding, Pipeline, Scope, Stage};
pub(crate) use ty::{CompareOp, Type};

#[derive(Debug, Clone)]
pub(crate) struct Statement {
    /// A line in the program's source space. The root script keeps its own
    /// line numbers; each module follows it at an offset, so a copied `line`
    /// still names the right file. [`crate::modules::SourceSpace`] maps it
    /// back to a file and a line within it.
    pub line: usize,
    pub kind: StatementKind,
    /// Binding intent for Let, Spawn, and Await; ignored on other statements.
    pub binding: BindingMode,
    /// `pub` on a top-level declaration: other files may import it.
    pub public: bool,
}

impl Statement {
    pub(crate) fn new(line: usize, kind: StatementKind) -> Self {
        Self {
            line,
            kind,
            binding: BindingMode::Immutable,
            public: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BindingMode {
    Immutable,
    Mutable,
    Assign,
}

#[derive(Debug, Clone)]
pub(crate) enum StatementKind {
    Use(Vec<UseItem>),
    /// `mod "lib/gh.ush" as gh`
    Module(ModuleDecl),
    UnitDef(String),
    Enum(EnumDef),
    Trait(TraitDef),
    Impl(TraitImpl),
    Function(FunctionDef),
    Effect(EffectDef),
    /// `try { … } with log(m) { … }`
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
    Fail {
        status: Expr,
        message: Expr,
    },
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
    /// `a | b`: the last stage's stdout streams to the script's.
    Pipeline(Pipeline),
    /// `export NAME = value, …` for the rest of the process scope.
    Export(Vec<EnvBinding>),
    /// `cd path { … }` and `export NAME = value { … }`.
    Scoped {
        scope: Scope,
        body: Vec<Statement>,
    },
    /// `allow_fail command`: run it, streaming, and continue whatever its
    /// exit status.
    AllowFail(Expr),
    /// `defer { … }`: run the block when the enclosing function returns or
    /// the script exits, last registered first.
    Defer(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    FunctionRef(String),
    Comprehension(Box<Comprehension>),
    ShellCapture(String),
    Quantity(i64, String),
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
    /// `a | b` in value position: the last stage's captured stdout.
    Pipeline(Box<Pipeline>),
    /// `status_of command`: run it, streaming, and evaluate to its exit
    /// status as an `Int` instead of failing.
    StatusOf(Box<Expr>),
    /// `"text ${expr} text"`: literal pieces are `Expr::String`, the
    /// others are interpolated expressions, joined left to right.
    Template(Vec<Expr>),
    /// `$NAME`: a required, non-empty environment variable.
    Env(String),
    /// `list[index]`
    Index {
        base: Box<Expr>,
        index: Box<Expr>,
    },
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
    /// `glob "*.ts"`: a POSIX `case` pattern over a String.
    Glob(String),
    /// `a | b`: matches when any alternative matches; binds nothing.
    Or(Vec<Pattern>),
}

pub(crate) type MatchArm = (Pattern, Box<Statement>);

#[derive(Debug, Clone)]
pub(crate) struct Comprehension {
    pub value: Expr,
    pub clauses: Vec<ComprehensionClause>,
}
#[derive(Debug, Clone)]
pub(crate) enum ComprehensionClause {
    For { name: String, iterable: Expr },
    If(Expr),
}
