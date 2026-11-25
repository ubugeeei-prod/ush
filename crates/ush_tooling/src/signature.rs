//! Signature help for both adjacent-parenthesis and shell-style calls.
mod scan;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<String>,
    pub label: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureHelp {
    pub signature: FunctionSignature,
    pub active_parameter: u32,
}
pub fn function_signatures(source: &str) -> Vec<FunctionSignature> {
    let index = crate::scope::Index::new(source);
    let mut out = Vec::new();
    for i in 0..index.names.len().saturating_sub(2) {
        if index.names[i] != "fn" {
            continue;
        }
        let name = index.names[i + 1];
        let Some(open) = (i + 2..index.names.len())
            .take_while(|&j| !matches!(index.names[j], "fn" | "{" | "}" | ";"))
            .find(|&j| index.names[j] == "(")
        else {
            continue;
        };
        let Some(close) = scan::closing(&index.names, open) else {
            continue;
        };
        let token = index.tokens[i + 1];
        let start = scan::offset(source, token.line, token.start);
        let end_token = index.tokens[close];
        let end = scan::offset(source, end_token.line, end_token.start + end_token.length);
        let raw = &source[start..end];
        let left = raw.find('(').unwrap();
        let params = scan::parameters(&raw[left + 1..raw.len() - 1]);
        let suffix = source[end..]
            .split(['{', '\n'])
            .next()
            .unwrap_or("")
            .trim_end();
        out.push(FunctionSignature {
            name: name.into(),
            parameters: params,
            label: format!("{raw}{suffix}"),
        });
    }
    out
}
pub fn signature_help(source: &str, line: u32, character: u32) -> Option<SignatureHelp> {
    let end = scan::offset(source, line, character);
    let prefix = source.get(..end)?;
    let signatures = function_signatures(source);
    let (name, active) = scan::call(prefix, &signatures)?;
    let signature = signatures.into_iter().find(|s| s.name == name)?;
    let active_parameter = active.min(signature.parameters.len().saturating_sub(1) as u32);
    Some(SignatureHelp {
        signature,
        active_parameter,
    })
}

#[cfg(test)]
mod tests {
    use super::{function_signatures, signature_help};

    #[test]
    fn parses_a_simple_fn() {
        let sigs = function_signatures("fn greet(name: String, mood: String) -> String {\n}\n");
        assert_eq!(sigs.len(), 1);
        assert_eq!(sigs[0].name, "greet");
        assert_eq!(sigs[0].parameters, vec!["name: String", "mood: String"]);
    }

    #[test]
    fn signature_help_finds_the_active_parameter() {
        let source = "fn greet(name: String, mood: String) -> String {\n  return name\n}\n\nlet x = greet(\"hi\", \"warm\")\n";
        // cursor right after the first argument's closing quote
        let help = signature_help(source, 4, "let x = greet(\"hi\"".len() as u32).expect("help");
        assert_eq!(help.signature.name, "greet");
        assert_eq!(help.active_parameter, 0);

        // cursor after the comma — second parameter active
        let help = signature_help(source, 4, "let x = greet(\"hi\", ".len() as u32).expect("help");
        assert_eq!(help.active_parameter, 1);
    }

    #[test]
    fn cursor_outside_a_call_returns_none() {
        assert!(signature_help("let x = 1\n", 0, 5).is_none());
    }

    #[test]
    fn unknown_function_returns_none() {
        // No `fn does_not_exist` declared in this file.
        assert!(signature_help("let x = does_not_exist(1)\n", 0, 23).is_none());
    }

    #[test]
    fn no_parameter_fn_still_resolves() {
        let source = "fn noop() {\n}\n\nlet _ = noop()\n";
        let help = signature_help(source, 3, "let _ = noop(".len() as u32).expect("help");
        assert_eq!(help.signature.parameters.len(), 0);
        assert_eq!(help.active_parameter, 0);
    }

    #[test]
    fn nested_call_uses_inner_signature() {
        let source = "fn outer(x: Int) {}\nfn inner(y: Int) {}\n\nlet _ = outer(inner(42))\n";
        let help = signature_help(source, 3, "let _ = outer(inner(".len() as u32).expect("help");
        assert_eq!(help.signature.name, "inner");
    }
}
