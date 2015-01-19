//! Tokenization of source code.

use std::result::Result::{Ok, Err};
use std::str::Chars;
use std::{error, result};
use token::Token;

/// Result of a lexer operation.
pub type Result = result::Result<Token, Error>;

/// Possible errors due to a lexer operation.
#[derive(PartialEq, Eq, Clone, Show)]
pub enum Error {
    UnknownToken,
    UnclosedComment,
    UnclosedString,
    MalformedInteger,
    Unknown,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnknownToken => "Unknown token",
            Error::UnclosedComment => "Unclosed comment",
            Error::UnclosedString => "Unclosed string",
            Error::MalformedInteger => "Malformed integer",
            Error::Unknown => "Unknown lexer error",
        }
    }
}

/// Yield chars while allowing one char to be "replaced" to be yielded again.
///
/// In the case that a char must be popped in order to determine
/// if it belongs in the current token, replace can be used to
/// remember it for the start of the next token. This works
/// as long as the lexer grammar needs only one char of lookahead.
struct ReplaceOneChars<'a> {
    chars: Chars<'a>,
    replaced: Option<char>,
}

impl<'a> ReplaceOneChars<'a> {
    pub fn new(chars: Chars<'a>) -> ReplaceOneChars<'a> {
        ReplaceOneChars {
            chars: chars,
            replaced: None,
        }
    }

    pub fn replace(&mut self, c: char) {
        self.replaced = Some(c);
    }
}

impl<'a> Iterator for ReplaceOneChars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.replaced.take() {
            Some(c) => Some(c),
            None => self.chars.next(),
        }
    }
}

/// The lexer is a tokenizer implemented as an iterator over a string.
/// Iteration  proceeds until the source is successfully tokenized,
/// or an error is encountered.
pub struct Lexer<'a> {
    chars: ReplaceOneChars<'a>,
}

const DECIMAL: uint = 10u;

impl<'a> Lexer<'a> {
    /// Create a new lexer over the provided source code.
    pub fn new(src: &'a str) -> Lexer<'a> {
        Lexer {
            chars: ReplaceOneChars::new(src.chars()),
        }
    }

    /// Consume any remaining chars in underlying source and return error.
    fn error(&mut self, err: Error) -> Result {
        for _ in self.chars { }
        Err(err)
    }

    fn whitespace(&mut self) -> Result {
        loop {
            match self.chars.next() {
                Some(c) => if !c.is_whitespace() {
                    self.chars.replace(c);
                    return Ok(Token::Whitespace);
                },
                None => return Ok(Token::Whitespace),
            }
        }
    }

    fn integer(&mut self) -> Result {
        let mut s = String::new();
        loop {
            match self.chars.next() {
                Some(c) => if c.is_digit(DECIMAL) {
                    s.push(c);
                } else if c.is_whitespace() {
                    self.chars.replace(c);
                    return Ok(Token::Integer(s));
                } else {
                    return self.error(Error::MalformedInteger);
                },
                None => return Ok(Token::Integer(s)),
            }
        }
    }

    fn comment(&mut self) -> Result {
        loop {
            match self.chars.next() {
                Some(c) => if c == ')' {
                    return Ok(Token::Comment);
                },
                None => return self.error(Error::UnclosedComment),
            }
        }
    }

    fn string(&mut self) -> Result {
        let mut s = String::new();
        loop {
            match self.chars.next() {
                Some(c) => if c == '"' {
                    return Ok(Token::String(s));
                } else {
                    s.push(c);
                },
                None => return self.error(Error::UnclosedString),
            }
        }
    }

    fn call(&mut self) -> Result {
        let mut s = String::new();
        loop {
            match self.chars.next() {
                Some(c) => if c.is_whitespace() {
                    return Ok(Token::Call(s));
                } else {
                    s.push(c);
                },
                None => return Ok(Token::Call(s)),
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result;

    fn next(&mut self) -> Option<Result> {
        loop {
            let c = match self.chars.next() {
                Some(c) => c,
                None => return None,
            };
            return Some(if c.is_whitespace() {
                self.whitespace()
            } else if c.is_digit(DECIMAL) {
                self.chars.replace(c);
                self.integer()
            } else if c == '(' {
                self.comment()
            } else if c == '"' {
                self.string()
            } else if c == '{' {
                Ok(Token::OpenBrace)
            } else if c == '}' {
                Ok(Token::CloseBrace)
            } else {
                self.chars.replace(c);
                self.call()
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Error};
    use token::Token;

    #[test]
    fn test_empty_string_is_none() {
        assert_eq!(Lexer::new("").next(), None);
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(Lexer::new(" ").collect::<Vec<_>>(),
            vec![Ok(Token::Whitespace)]);
        assert_eq!(Lexer::new("  \t\t\n\n").collect::<Vec<_>>(),
            vec![Ok(Token::Whitespace)]);
    }

    #[test]
    fn test_integer() {
        assert_eq!(Lexer::new("0").collect::<Vec<_>>(),
            vec![Ok(Token::Integer("0".to_string()))]);
        assert_eq!(Lexer::new("1.0").collect::<Vec<_>>(),
            vec![Err(Error::MalformedInteger)]);
    }

    #[test]
    fn test_comment() {
        assert_eq!(Lexer::new("(this is a comment)").collect::<Vec<_>>(),
            vec![Ok(Token::Comment)]);
        assert_eq!(Lexer::new("(this is an unclosed comment")
                   .collect::<Vec<_>>(),
            vec![Err(Error::UnclosedComment)]);
    }
    #[test]
    fn test_string() {
        assert_eq!(Lexer::new("\"this is a string\"").collect::<Vec<_>>(),
            vec![Ok(Token::String("this is a string".to_string()))]);
        assert_eq!(Lexer::new("\"this is an unclosed string").collect::<Vec<_>>(),
            vec![Err(Error::UnclosedString)]);
    }

    #[test]
    fn test_call() {
        assert_eq!(Lexer::new("this-is-a-call").collect::<Vec<_>>(),
            vec![Ok(Token::Call("this-is-a-call".to_string()))]);
    }
}