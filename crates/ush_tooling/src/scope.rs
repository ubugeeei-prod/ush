//! A tolerant lexical index, shared by navigation and rename. Never parses strings as code.
use crate::{
    semantic::semantic_tokens,
    token::{SemanticToken, SemanticTokenKind},
};

pub(crate) struct Index<'a> {
    pub tokens: Vec<SemanticToken>,
    pub names: Vec<&'a str>,
    pub resolved: Vec<Option<usize>>,
    pub external: Vec<bool>,
}

impl<'a> Index<'a> {
    pub fn new(source: &'a str) -> Self {
        let tokens = semantic_tokens(source);
        let lines: Vec<_> = source.lines().collect();
        let names: Vec<_> = tokens
            .iter()
            .map(|t| {
                if matches!(
                    t.kind,
                    SemanticTokenKind::String
                        | SemanticTokenKind::Comment
                        | SemanticTokenKind::Decorator
                ) {
                    ""
                } else {
                    &lines[t.line as usize][t.start as usize..(t.start + t.length) as usize]
                }
            })
            .collect();
        let mut parents = vec![0];
        let mut scopes = Vec::with_capacity(tokens.len());
        let mut ends = vec![tokens.len()];
        let mut scope = 0;
        let mut opens = std::collections::HashMap::new();
        for (i, name) in names.iter().enumerate() {
            scopes.push(scope);
            if matches!(*name, "{" | "[") {
                let child = parents.len();
                parents.push(scope);
                ends.push(tokens.len());
                opens.insert(i, child);
                scope = child;
            } else if matches!(*name, "}" | "]") && scope != 0 {
                ends[scope] = i;
                scope = parents[scope];
            }
        }
        let mut declarations = Vec::new();
        let mut excluded = std::collections::HashMap::new();
        // (token, scope, visibility offset); function/type names may be used before declaration.
        for i in 0..tokens.len().saturating_sub(1) {
            if matches!(
                names[i],
                "fn" | "enum" | "trait" | "type" | "unit" | "effect" | "alias"
            ) && identifier(tokens[i + 1].kind)
            {
                declarations.push((i + 1, scopes[i], 0));
            }
            if matches!(names[i], "let" | "const" | "as") && identifier(tokens[i + 1].kind) {
                let after = (i + 2..tokens.len())
                    .find(|&j| tokens[j].line > tokens[i].line)
                    .unwrap_or(tokens.len());
                declarations.push((i + 1, scopes[i], after));
            }
            if names[i] == "fn"
                && let Some(body) = (i + 2..tokens.len())
                    .find(|&j| matches!(names[j], "{" | "}"))
                    .filter(|&j| names[j] == "{")
            {
                let child = opens[&body];
                // Parameters and generic variables belong to this function, including its signature.
                for j in i + 2..body {
                    scopes[j] = child;
                    if identifier(tokens[j].kind)
                        && (names.get(j + 1) == Some(&":")
                            || (tokens[j].kind == SemanticTokenKind::Type
                                && matches!(names[j - 1], "<" | ",")
                                && !names[i + 2..j].contains(&"(")))
                    {
                        declarations.push((j, child, 0));
                    }
                }
            }
            if names[i] == "for" && identifier(tokens[i + 1].kind) {
                let current = scopes[i];
                // A comprehension binds across its result expression; a loop binds only its body.
                if opens
                    .iter()
                    .any(|(&open, &child)| child == current && names[open] == "[")
                {
                    declarations.push((i + 1, current, 0));
                    let first = (0..=i)
                        .find(|&j| scopes[j] == current && names[j] == "for")
                        .unwrap();
                    let end = (i + 2..ends[current])
                        .find(|&j| scopes[j] == current && matches!(names[j], "for" | "if"))
                        .unwrap_or(ends[current]);
                    excluded.insert(i + 1, first..end);
                } else if let Some(body) = (i + 2..tokens.len()).find(|&j| names[j] == "{") {
                    declarations.push((i + 1, opens[&body], 0));
                    scopes[i + 1] = opens[&body];
                }
            }
        }
        let mut modules = std::collections::HashMap::new();
        for i in 0..names.len().saturating_sub(4) {
            if names[i] == "extern" && names[i + 2] == "as" && names[i + 4] == "{" {
                modules.insert(opens[&(i + 4)], names[i + 3]);
            }
        }
        let imported: Vec<_> = (0..names.len().saturating_sub(3))
            .filter(|&i| names[i] == "use" && names[i + 2] == "::")
            .map(|i| (names[i + 1], names[i + 3], scopes[i]))
            .collect();
        let exports = declarations.clone();
        for &(decl, scope, _) in &exports {
            if let Some(module) = modules.get(&scope) {
                for &(import, item, parent) in &imported {
                    if import == *module && (item == "*" || item == names[decl]) {
                        declarations.push((decl, parent, 0));
                    }
                }
            }
        }
        let mut resolved = vec![None; tokens.len()];
        let mut external = vec![false; tokens.len()];
        let mut by_name = std::collections::HashMap::<_, Vec<_>>::new();
        for &(decl, scope, visible) in &declarations {
            resolved[decl] = Some(decl);
            by_name
                .entry((scope, names[decl]))
                .or_default()
                .push((decl, visible));
            external[decl] |= modules.contains_key(&scope);
        }
        for i in 0..tokens.len() {
            if !identifier(tokens[i].kind) {
                continue;
            }
            if resolved[i].is_some() {
                continue;
            }
            // Qualified members must never resolve to an unrelated unqualified local.
            if i > 0 && matches!(names[i - 1], "." | "::") {
                if i >= 2 && names[i - 1] == "::" {
                    resolved[i] = exports
                        .iter()
                        .find(|&&(decl, s, _)| {
                            names[decl] == names[i] && modules.get(&s) == Some(&names[i - 2])
                        })
                        .map(|&(decl, _, _)| decl);
                }
                continue;
            }
            let mut scope = scopes[i];
            loop {
                if let Some(&(decl, _)) = by_name.get(&(scope, names[i])).and_then(|ds| {
                    ds.iter().rev().find(|&&(decl, visible)| {
                        visible <= i && !excluded.get(&decl).is_some_and(|range| range.contains(&i))
                    })
                }) {
                    resolved[i] = Some(decl);
                    break;
                }
                if scope == 0 {
                    break;
                }
                scope = parents[scope];
            }
        }
        Self {
            tokens,
            names,
            resolved,
            external,
        }
    }

    pub fn at(&self, line: u32, byte: u32) -> Option<usize> {
        self.tokens
            .iter()
            .position(|t| t.line == line && t.start <= byte && byte < t.start + t.length)
    }
}

pub(crate) fn identifier(kind: SemanticTokenKind) -> bool {
    matches!(
        kind,
        SemanticTokenKind::Variable
            | SemanticTokenKind::Function
            | SemanticTokenKind::Type
            | SemanticTokenKind::Property
    )
}
