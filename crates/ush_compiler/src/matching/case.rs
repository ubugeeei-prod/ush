//! A `match` on a String whose arms are literals, globs, `_` or a binding
//! lowers to one POSIX `case` statement.

use anyhow::Result;

use crate::{ast::Pattern, types::OutputString as String, util::shell_value_quote};

/// The `case` pattern list for `pattern`, or `None` when the pattern needs
/// the general `if` lowering.
pub(crate) fn case_word(pattern: &Pattern) -> Option<Result<String>> {
    Some(match pattern {
        Pattern::String(text) => Ok(shell_value_quote(text)),
        Pattern::Glob(glob) => crate::glob::lower(glob),
        Pattern::Wildcard | Pattern::Binding(_) => Ok("*".into()),
        Pattern::Or(alternatives) => {
            let mut words = alloc::vec::Vec::with_capacity(alternatives.len());
            for alternative in alternatives {
                match case_word(alternative)? {
                    Ok(word) => words.push(word),
                    Err(error) => return Some(Err(error)),
                }
            }
            Ok(words.join("|"))
        }
        _ => return None,
    })
}
