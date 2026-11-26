use std::collections::HashMap;

use crate::{SemanticToken, SemanticTokenKind as Kind};

/// Give scalar match arms their own lexical scope. Destructuring remains
/// conservative until its bindings can be indexed with the same guarantees.
pub(super) fn index(
    tokens: &[SemanticToken],
    names: &[&str],
    opens: &HashMap<usize, usize>,
    parents: &mut Vec<usize>,
    ends: &mut Vec<usize>,
    scopes: &mut [usize],
    declarations: &mut Vec<(usize, usize, usize)>,
) -> bool {
    let mut unsupported = names.windows(2).any(|p| p == ["if", "let"]);
    for start in (0..names.len()).filter(|&i| names[i] == "match") {
        let Some(open) = (start + 1..names.len())
            .take_while(|&i| tokens[i].line == tokens[start].line)
            .find(|&i| names[i] == "{")
        else {
            unsupported = true;
            continue;
        };
        let parent = opens[&open];
        let end = ends[parent];
        if end == tokens.len() {
            unsupported = true;
        }
        let mut first = open + 1;
        while first < end {
            let after = (first + 1..end)
                .find(|&i| tokens[i].line > tokens[first].line)
                .unwrap_or(end);
            if tokens[first].kind == Kind::Comment {
                first = after;
                continue;
            }
            let Some(arrow) = (first..after).find(|&i| names[i] == "=>") else {
                unsupported = true;
                first = after;
                continue;
            };
            let binding = (arrow == first + 1
                && super::identifier(tokens[first].kind)
                && names[first] != "_")
                .then_some(first);
            if binding.is_none()
                && !(first..arrow).all(|i| {
                    matches!(tokens[i].kind, Kind::String | Kind::Number)
                        || matches!(names[i], "true" | "false" | "_" | "glob" | "|" | "-")
                })
            {
                unsupported = true;
            }
            let arm = parents.len();
            parents.push(parent);
            ends.push(after);
            for (i, scope) in scopes.iter_mut().enumerate().take(after).skip(first) {
                if *scope == parent {
                    *scope = arm;
                }
                if let Some(&child) = opens.get(&i)
                    && parents[child] == parent
                {
                    parents[child] = arm;
                }
            }
            for (decl, scope, _) in declarations.iter_mut() {
                if *scope == parent && (first..after).contains(decl) {
                    *scope = arm;
                }
            }
            if let Some(binding) = binding {
                declarations.push((binding, arm, first));
            }
            first = after;
        }
    }
    unsupported
}
