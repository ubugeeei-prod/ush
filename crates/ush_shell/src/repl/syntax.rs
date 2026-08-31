#[cfg(test)]
mod tests;

pub const PATH_COMMANDS: &[&str] = &[
    ".", "cat", "cd", "cp", "diff", "find", "grep", "head", "less", "ls", "mkdir", "mv", "rm",
    "sed", "sh", "source", "tail", "touch",
];

const KEYWORDS: &[&str] = &[
    "!", ".", "break", "case", "continue", "do", "done", "elif", "else", "esac", "export", "fi",
    "for", "if", "in", "readonly", "return", "set", "shift", "then", "times", "trap", "until",
    "unset", "while",
];

pub fn keywords() -> &'static [&'static str] {
    KEYWORDS
}

pub fn is_keyword(token: &str) -> bool {
    KEYWORDS.contains(&token)
}

pub fn is_assignment(token: &str) -> bool {
    let Some((name, _)) = token.split_once('=') else {
        return false;
    };
    is_identifier(name)
}

pub fn command_position(prefix: &str, word_start: usize) -> bool {
    let tokens = tokenize(&prefix[..word_start]);
    if tokens.is_empty() {
        return true;
    }
    let start = tokens
        .iter()
        .rposition(|token| matches!(token.as_str(), "|" | "||" | "&&" | ";" | "&"))
        .map_or(0, |index| index + 1);
    let segment = &tokens[start..];
    segment.is_empty()
        || segment
            .iter()
            .all(|token| is_assignment(token) || matches!(token.as_str(), "!" | "command"))
}

pub fn previous_token(prefix: &str, word_start: usize) -> Option<String> {
    tokenize(&prefix[..word_start]).into_iter().last()
}

pub fn tokenize(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = line.chars().peekable();
    let mut single = false;
    let mut double = false;
    let mut escaped = false;

    while let Some(ch) = chars.next() {
        if single {
            current.push(ch);
            if ch == '\'' {
                single = false;
            }
            continue;
        }
        if double {
            current.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                double = false;
            }
            continue;
        }

        match ch {
            '\'' => {
                current.push(ch);
                single = true;
            }
            '"' => {
                current.push(ch);
                double = true;
            }
            '\\' => {
                current.push(ch);
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            '#' if current.is_empty() => break,
            ch if ch.is_whitespace() => flush_token(&mut tokens, &mut current),
            '|' | '&' | '<' | '>' => {
                flush_token(&mut tokens, &mut current);
                let mut op = ch.to_string();
                if chars.peek().copied() == Some(ch) {
                    op.push(chars.next().unwrap_or(ch));
                }
                tokens.push(op);
            }
            ';' | '(' | ')' | '{' | '}' => {
                flush_token(&mut tokens, &mut current);
                tokens.push(ch.to_string());
            }
            _ => current.push(ch),
        }
    }

    flush_token(&mut tokens, &mut current);
    tokens
}

pub fn word_start(line: &str, pos: usize) -> usize {
    let prefix = &line[..pos];
    prefix
        .char_indices()
        .rev()
        .find_map(|(index, ch)| {
            (ch.is_whitespace() || "|&;(){}<>".contains(ch)).then_some(index + ch.len_utf8())
        })
        .unwrap_or(0)
}

pub fn env_query(word: &str) -> Option<(usize, String, bool)> {
    if let Some(rest) = word.strip_prefix("${") {
        return Some((2, rest.to_string(), true));
    }
    word.strip_prefix('$')
        .map(|rest| (1, rest.to_string(), false))
}

pub fn needs_refresh(line: &str, pos: usize) -> bool {
    pos == line.len()
        || line.contains('$')
        || line.contains('#')
        || line.contains('"')
        || line.contains('\'')
        || line.contains('|')
}

pub fn has_unclosed_quotes(line: &str) -> bool {
    let mut single = false;
    let mut double = false;
    let mut escaped = false;

    for ch in line.chars() {
        match ch {
            '\'' if !double => single = !single,
            '"' if !single && !escaped => double = !double,
            '\\' if double => escaped = !escaped,
            _ => escaped = false,
        }
    }

    single || double
}

pub fn has_trailing_escape(line: &str) -> bool {
    let mut slashes = 0usize;
    for ch in line.chars().rev() {
        if ch == '\\' {
            slashes += 1;
            continue;
        }
        if ch.is_whitespace() {
            continue;
        }
        break;
    }
    slashes % 2 == 1
}

fn flush_token(tokens: &mut Vec<String>, current: &mut String) {
    if !current.is_empty() {
        tokens.push(std::mem::take(current));
    }
}

fn is_identifier(source: &str) -> bool {
    let mut chars = source.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}
