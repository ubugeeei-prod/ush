//! The side-effect system.
//!
//! `.ush` already tracks *which errors* a function can raise. This
//! pass tracks *what a function touches*: the filesystem, the
//! environment, the network, other processes, the terminal, and the
//! task scheduler. Effects are inferred bottom-up from the stdlib and
//! propagated through calls, so nothing has to be annotated for the
//! information to exist.
//!
//! Annotating is how you turn the information into a check. A
//! function that declares
//!
//! ```text
//! #[effects(fs, exec)]
//! fn deploy() -> String { ... }
//! ```
//!
//! is refused if it grows a network call later, and `#[pure]` says a
//! function may touch nothing at all. A declaration is an upper
//! bound: declaring more than you use is allowed and keeps a public
//! signature stable while an implementation shrinks.

use core::fmt;

use anyhow::{Result, bail};

use crate::{
    ast::{
        Call, Condition, Expr, ExprFields, FunctionDef, IfBranch, MethodCall, Statement,
        StatementKind,
    },
    types::{AstString as String, HeapVec as Vec, Map as HashMap},
};

/// What a function reaches for beyond its own arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Effect {
    /// Terminal output and input: `print`, and the interactive
    /// builtins.
    Io,
    /// The filesystem: `std::fs`, and the `std::path` helpers that
    /// look at what is actually on disk.
    Fs,
    /// Process environment: `std::env`, and `$PATH`-shaped helpers.
    Env,
    /// The network: `std::http`.
    Net,
    /// Other processes: `$ cmd`, `shell`, and `std::command`.
    Exec,
    /// Concurrency: `spawn`, `.await`, and `async` blocks.
    Task,
}

impl Effect {
    pub const ALL: [Self; 6] = [
        Self::Io,
        Self::Fs,
        Self::Env,
        Self::Net,
        Self::Exec,
        Self::Task,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Io => "io",
            Self::Fs => "fs",
            Self::Env => "env",
            Self::Net => "net",
            Self::Exec => "exec",
            Self::Task => "task",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|effect| effect.name() == name)
    }

    const fn bit(self) -> u8 {
        1 << (self as u8)
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A set of [`Effect`]s, held as a bitset.
///
/// The inference walk unions these once per expression node, so the
/// representation is one byte and every operation is a mask.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EffectSet(u8);

impl EffectSet {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn of(effect: Effect) -> Self {
        Self(effect.bit())
    }

    pub fn insert(&mut self, effect: Effect) {
        self.0 |= effect.bit();
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn merge(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// The effects in `self` that `other` does not cover.
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    pub const fn contains(self, effect: Effect) -> bool {
        self.0 & effect.bit() != 0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn iter(self) -> impl Iterator<Item = Effect> {
        Effect::ALL.into_iter().filter(move |e| self.contains(*e))
    }
}

impl fmt::Display for EffectSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return f.write_str("pure");
        }
        let mut first = true;
        for effect in self.iter() {
            if !first {
                f.write_str(", ")?;
            }
            f.write_str(effect.name())?;
            first = false;
        }
        Ok(())
    }
}

impl FromIterator<Effect> for EffectSet {
    fn from_iter<I: IntoIterator<Item = Effect>>(effects: I) -> Self {
        let mut set = Self::empty();
        for effect in effects {
            set.insert(effect);
        }
        set
    }
}

pub(crate) type FunctionEffectRegistry = HashMap<String, EffectSet>;

/// What one function was found to do, and what it claims to do.
#[derive(Clone, Debug)]
pub struct FunctionEffects {
    pub name: crate::types::OutputString,
    pub inferred: EffectSet,
    pub declared: Option<EffectSet>,
}

/// Infers effects for every function, then checks the declarations.
///
/// Inference is a fixpoint because functions call each other: each
/// round re-walks every body with the registry built so far, and the
/// walk stops when a round changes nothing. The set only ever grows,
/// and there are finitely many effects, so it terminates.
pub(crate) fn analyze_function_effects(program: &[Statement]) -> Result<FunctionEffectRegistry> {
    let mut registry = FunctionEffectRegistry::default();
    for def in function_defs(program) {
        registry.insert(def.name.clone(), EffectSet::empty());
    }

    for _ in 0..=registry.len() {
        let mut changed = false;
        for def in function_defs(program) {
            let inferred = block_effects(&def.body, &registry);
            if registry.get(&def.name) != Some(&inferred) {
                registry.insert(def.name.clone(), inferred);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    for def in function_defs(program) {
        let inferred = registry.get(&def.name).copied().unwrap_or_default();
        validate_declaration(def, inferred)?;
    }

    Ok(registry)
}

/// The per-function report behind `ush effects`.
pub(crate) fn describe_function_effects(program: &[Statement]) -> Result<Vec<FunctionEffects>> {
    let registry = analyze_function_effects(program)?;
    let mut reports = Vec::new();
    for def in function_defs(program) {
        reports.push(FunctionEffects {
            name: def.name.as_str().into(),
            inferred: registry.get(&def.name).copied().unwrap_or_default(),
            declared: declared_effects(def)?,
        });
    }
    reports.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(reports)
}

/// Effects of the statements outside any function.
pub(crate) fn top_level_effects(program: &[Statement]) -> Result<EffectSet> {
    let registry = analyze_function_effects(program)?;
    Ok(block_effects(program, &registry))
}

fn function_defs(program: &[Statement]) -> impl Iterator<Item = &FunctionDef> {
    program.iter().flat_map(|statement| match &statement.kind {
        StatementKind::Function(def) => core::slice::from_ref(def),
        StatementKind::Impl(item) => item.methods.as_slice(),
        _ => &[] as &[FunctionDef],
    })
}

fn validate_declaration(def: &FunctionDef, inferred: EffectSet) -> Result<()> {
    let Some(declared) = declared_effects(def)? else {
        return Ok(());
    };
    let missing = inferred.difference(declared);
    if missing.is_empty() {
        return Ok(());
    }
    bail!(
        "function `{}` performs `{missing}` but its effect row declares `{declared}`; \
         write `#[effects({})]` or drop the offending call",
        def.name,
        inferred
            .iter()
            .map(Effect::name)
            .collect::<alloc::vec::Vec<_>>()
            .join(", ")
    )
}

fn declared_effects(def: &FunctionDef) -> Result<Option<EffectSet>> {
    let mut declared = None;
    for attr in &def.attrs {
        match attr.name.as_str() {
            "pure" => {
                if !attr.args.is_empty() {
                    bail!("`#[pure]` takes no arguments");
                }
                declared = Some(declared.unwrap_or_else(EffectSet::empty));
            }
            "effects" => {
                let mut set = declared.unwrap_or_else(EffectSet::empty);
                for arg in &attr.args {
                    let Some(effect) = Effect::parse(arg.trim()) else {
                        bail!(
                            "unknown effect `{arg}` on `{}`; known effects are {}",
                            def.name,
                            Effect::ALL
                                .into_iter()
                                .map(Effect::name)
                                .collect::<alloc::vec::Vec<_>>()
                                .join(", ")
                        );
                    };
                    set.insert(effect);
                }
                declared = Some(set);
            }
            _ => {}
        }
    }
    Ok(declared)
}

fn block_effects(body: &[Statement], registry: &FunctionEffectRegistry) -> EffectSet {
    let mut effects = EffectSet::empty();
    for statement in body {
        effects.merge(statement_effects(statement, registry));
    }
    effects
}

fn statement_effects(statement: &Statement, registry: &FunctionEffectRegistry) -> EffectSet {
    match &statement.kind {
        // A nested definition contributes nothing until it is called.
        StatementKind::Use(_)
        | StatementKind::Enum(_)
        | StatementKind::Trait(_)
        | StatementKind::Impl(_)
        | StatementKind::Function(_)
        | StatementKind::Break
        | StatementKind::Continue => EffectSet::empty(),

        StatementKind::Print(expr) => EffectSet::of(Effect::Io).union(expr_effects(expr, registry)),
        StatementKind::Shell(expr) | StatementKind::TryShell(expr) => {
            EffectSet::of(Effect::Exec).union(expr_effects(expr, registry))
        }
        StatementKind::Spawn { call, .. } => {
            EffectSet::of(Effect::Task).union(call_effects(call, registry))
        }
        StatementKind::Await { .. } => EffectSet::of(Effect::Task),

        StatementKind::Alias { value, .. }
        | StatementKind::Let { expr: value, .. }
        | StatementKind::Raise(value)
        | StatementKind::Expr(value)
        | StatementKind::Return(value) => expr_effects(value, registry),

        StatementKind::Call(call) | StatementKind::TryCall(call) => call_effects(call, registry),

        StatementKind::Match { expr, arms, .. } => {
            let mut effects = expr_effects(expr, registry);
            for (_, body) in arms {
                effects.merge(statement_effects(body, registry));
            }
            effects
        }
        StatementKind::If { branch, .. } => branch_effects(branch, registry),
        StatementKind::While { condition, body } => {
            condition_effects(condition, registry).union(block_effects(body, registry))
        }
        StatementKind::For { iterable, body, .. } => {
            expr_effects(iterable, registry).union(block_effects(body, registry))
        }
        StatementKind::Loop { body } => block_effects(body, registry),
    }
}

fn branch_effects(branch: &IfBranch, registry: &FunctionEffectRegistry) -> EffectSet {
    let mut effects = condition_effects(&branch.condition, registry);
    effects.merge(block_effects(&branch.then_body, registry));
    // `elif` is parsed as a nested `if` inside `else_body`, so the
    // recursion is already covered by walking the else block.
    if let Some(otherwise) = &branch.else_body {
        effects.merge(block_effects(otherwise, registry));
    }
    effects
}

fn condition_effects(condition: &Condition, registry: &FunctionEffectRegistry) -> EffectSet {
    match condition {
        Condition::Expr(expr) => expr_effects(expr, registry),
        Condition::Let { expr, .. } => expr_effects(expr, registry),
        Condition::And(parts) | Condition::Or(parts) => {
            let mut effects = EffectSet::empty();
            for part in parts {
                effects.merge(condition_effects(part, registry));
            }
            effects
        }
    }
}

fn expr_effects(expr: &Expr, registry: &FunctionEffectRegistry) -> EffectSet {
    match expr {
        Expr::String(_) | Expr::Int(_) | Expr::Bool(_) | Expr::Unit | Expr::Var(_) => {
            EffectSet::empty()
        }
        Expr::Add(parts) | Expr::Tuple(parts) | Expr::List(parts) => {
            let mut effects = EffectSet::empty();
            for part in parts {
                effects.merge(expr_effects(part, registry));
            }
            effects
        }
        Expr::Compare { lhs, rhs, .. } => {
            expr_effects(lhs, registry).union(expr_effects(rhs, registry))
        }
        Expr::Range { start, end } => {
            expr_effects(start, registry).union(expr_effects(end, registry))
        }
        Expr::Try(inner) | Expr::Field { base: inner, .. } => expr_effects(inner, registry),
        Expr::Call(call) => call_effects(call, registry),
        Expr::MethodCall(call) => method_effects(call, registry),
        Expr::Variant(variant) => match &variant.fields {
            ExprFields::Unit => EffectSet::empty(),
            ExprFields::Tuple(parts) => {
                let mut effects = EffectSet::empty();
                for part in parts {
                    effects.merge(expr_effects(part, registry));
                }
                effects
            }
            ExprFields::Struct(fields) => {
                let mut effects = EffectSet::empty();
                for field in fields {
                    effects.merge(expr_effects(&field.expr, registry));
                }
                effects
            }
        },
        Expr::AsyncBlock(body) => EffectSet::of(Effect::Task).union(block_effects(body, registry)),
    }
}

fn call_effects(call: &Call, registry: &FunctionEffectRegistry) -> EffectSet {
    let mut effects = builtin_call_effects(&call.name);
    if let Some(user) = registry.get(&call.name) {
        effects.merge(*user);
    }
    if call.asynchronous {
        effects.insert(Effect::Task);
    }
    for arg in &call.args {
        effects.merge(expr_effects(&arg.expr, registry));
    }
    effects
}

fn method_effects(call: &MethodCall, registry: &FunctionEffectRegistry) -> EffectSet {
    let mut effects = builtin_method_effects(&call.method);
    effects.merge(expr_effects(&call.receiver, registry));
    for arg in &call.args {
        effects.merge(expr_effects(&arg.expr, registry));
    }
    effects
}

/// The stdlib's effect table, keyed by fully qualified name.
///
/// `std::string` and `std::regex` are absent on purpose: they are
/// pure text transformations. `std::path` is split, because half of
/// it is path algebra and the other half asks the filesystem or the
/// environment what is actually there.
fn builtin_call_effects(name: &str) -> EffectSet {
    if let Some(rest) = name.strip_prefix("std::") {
        return match rest {
            _ if rest.starts_with("fs::") => EffectSet::of(Effect::Fs),
            _ if rest.starts_with("http::") => EffectSet::of(Effect::Net),
            _ if rest.starts_with("command::") => EffectSet::of(Effect::Exec),
            _ if rest.starts_with("env::") => EffectSet::of(Effect::Env),
            "path::cwd" | "path::from_cwd" | "path::source_root" | "path::from_source" => {
                EffectSet::of(Effect::Env)
            }
            "path::home" | "path::prepend_env" => EffectSet::of(Effect::Env),
            "path::exists" | "path::is_dir" | "path::is_file" | "path::mkdir_p"
            | "path::tmpfile" | "path::resolve" => EffectSet::of(Effect::Fs),
            _ => EffectSet::empty(),
        };
    }
    EffectSet::empty()
}

/// The same table for method syntax, which lowers to the same
/// builtins with the receiver as the first argument.
fn builtin_method_effects(method: &str) -> EffectSet {
    match method {
        "append_text" | "copy" | "mime_type" | "mkdir_p" | "move" | "read_text" | "remove"
        | "resolve" | "sha256" | "tmpfile" | "write_text" | "exists" | "is_dir" | "is_file" => {
            EffectSet::of(Effect::Fs)
        }
        "capture" | "capture_stderr" | "run" | "status" => EffectSet::of(Effect::Exec),
        "download" | "get_bytes" => EffectSet::of(Effect::Net),
        _ => EffectSet::empty(),
    }
}

#[cfg(test)]
mod tests;
