//! Tokens.

/// Input tokens produced by lexer.
#[derive(PartialEq, Eq, Clone, Show)]
pub enum Token {
    Integer(String),
    String(String),
    Call(String),
    OpenBrace,
    CloseBrace,
    Whitespace,
    Comment,
}
