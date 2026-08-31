#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticTokenKind {
    Keyword,
    String,
    Number,
    Comment,
    Variable,
    Function,
    Type,
    Property,
    Operator,
    Decorator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticToken {
    pub line: u32,
    pub start: u32,
    pub length: u32,
    pub kind: SemanticTokenKind,
}

pub fn semantic_token_legend() -> &'static [&'static str] {
    &[
        "keyword",
        "string",
        "number",
        "comment",
        "variable",
        "function",
        "type",
        "property",
        "operator",
        "decorator",
    ]
}

impl SemanticTokenKind {
    pub fn index(self) -> u32 {
        match self {
            Self::Keyword => 0,
            Self::String => 1,
            Self::Number => 2,
            Self::Comment => 3,
            Self::Variable => 4,
            Self::Function => 5,
            Self::Type => 6,
            Self::Property => 7,
            Self::Operator => 8,
            Self::Decorator => 9,
        }
    }
}
