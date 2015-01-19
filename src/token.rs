//! Tokens.

/// Input tokens produced by lexer.
#[derive(PartialEq, Eq, Clone, Show)]
pub enum Token {
    Literal(Literal),
    Call(String),
    OpenBrace,
    CloseBrace,
    Whitespace,
    Comment,
    Eof,
}

/// A sub-set of tokens which represent literals of a given type.
#[derive(PartialEq, Eq, Clone, Show)]
pub enum Literal {
    Integer(String),
    String(String),
}
