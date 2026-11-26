//! The words of an external command call. A List argument gives each of its
//! elements a word of its own.

use crate::{
    ast::{Expr, Type},
    env::{Binding, Env, Storage},
    types::OutputString as String,
};

/// One argument of an external command: a single word, or every element of a
/// List, each its own word.
pub(super) enum CommandWord {
    One(String),
    Each(String),
}

/// The packed form of a List argument of Strings or Ints, which the statement
/// before the call has already bound to a variable.
pub(super) fn list_argument(expr: &Expr, ty: &Type, env: &Env) -> Option<String> {
    let Type::List(item) = ty else {
        return None;
    };
    if !matches!(**item, Type::String | Type::Int) {
        return None;
    }
    let Expr::Var(name) = expr else {
        return None;
    };
    match env.get(name)? {
        Binding {
            storage: Storage::Aggregate(prefix),
            ..
        } => Some(super::wire::encode(ty, prefix)),
        _ => None,
    }
}

/// Render the command. With a List among its arguments, the words go through
/// `ush_fn_std__run_words`, which gives every element an argv entry of its own.
pub(super) fn command_line(
    word: &str,
    predicate: bool,
    spreads: bool,
    words: &[CommandWord],
) -> String {
    let mut line = if predicate {
        String::from("ush_fn_std__test")
    } else if spreads {
        format!("ush_fn_std__run_words {}", crate::util::shell_quote(word))
    } else {
        format!("command {word}")
    };
    for word in words {
        line.push(' ');
        match word {
            CommandWord::One(value) if spreads => {
                line.push_str("s ");
                line.push_str(value);
            }
            CommandWord::One(value) => line.push_str(value),
            CommandWord::Each(list) => {
                line.push_str("l ");
                line.push_str(list);
            }
        }
    }
    line
}
