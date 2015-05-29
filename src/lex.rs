//! Tokenization of source code.

use std::result::Result::{Ok, Err};
use std::str::Chars;
use std::{error, result};
use std::fmt;
use std::error::Error as StdError;

/// Result of a lexer operation.
pub type Result<T> = result::Result<T, Error>;

/// Possible errors due to a lexer operation.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Error {
    UnknownEscape,
    IncompleteEscape,
    UnknownToken,
    UnclosedComment,
    UnclosedString,
    MalformedNumber,
}

impl Error {
    /// Whether this error recoverable if more input is provided.
    pub fn is_recoverable(&self) -> bool {
        match *self {
            Error::IncompleteEscape
                | Error::UnclosedComment
                | Error::UnclosedString => true,
            _ => false,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnknownEscape => "Unknown character escape",
            Error::IncompleteEscape => "Incomplete character escape",
            Error::UnknownToken => "Unknown token",
            Error::UnclosedComment => "Unclosed comment",
            Error::UnclosedString => "Unclosed string",
            Error::MalformedNumber => "Malformed integer or float",
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

/// Input tokens produced by lexer.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token {
    Integer(String),
    Float(String),
    String(String),
    Symbol(String),
    Call(String),
    OpenBrace,
    CloseBrace,
    Whitespace,
    Comment,
}

/// The lexer is a tokenizer implemented as an iterator over a string.
/// Iteration  proceeds until the source is successfully tokenized,
/// or an error is encountered.
pub struct Lexer<'a> {
    chars: ReplaceOneChars<'a>,
}

const DECIMAL: u32 = 10u32;

#[inline(always)]
fn is_whitespace_or_close_brace(c: char) -> bool {
    c.is_whitespace() || c == '}'
}

impl<'a> Lexer<'a> {
    /// Create a new lexer over the provided source code.
    pub fn new(src: &'a str) -> Lexer<'a> {
        Lexer {
            chars: ReplaceOneChars::new(src.chars()),
        }
    }

    /// Consume any remaining chars in underlying source.
    fn consume(&mut self) {
        for _ in &mut self.chars { }
    }

    fn whitespace(&mut self) -> Result<Token> {
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

    fn number(&mut self) -> Result<Token> {
        let mut s = String::new();
        let mut is_float = false;
        loop {
            match self.chars.next() {
                Some(c) => if c.is_digit(DECIMAL) {
                    s.push(c);
                } else if !is_float && c == '.' {
                    is_float = true;
                    s.push(c);
                } else if is_whitespace_or_close_brace(c) {
                    self.chars.replace(c);
                    break;
                } else {
                    return Err(Error::MalformedNumber);
                },
                None => break,
            }
        }
        Ok(if is_float {
            Token::Float(s)
        } else {
            Token::Integer(s)
        })
    }

    fn comment(&mut self) -> Result<Token> {
        loop {
            match self.chars.next() {
                Some('\n') | None => return Ok(Token::Comment),
                _ => continue,
            }
        }
    }

    fn multi_comment(&mut self) -> Result<Token> {
        loop {
            match self.chars.next() {
                Some(c) => if c == ')' {
                    return Ok(Token::Comment);
                },
                None => return Err(Error::UnclosedComment),
            }
        }
    }

    fn escape(&mut self) -> Result<char> {
        match self.chars.next() {
            Some(c) => return match c {
                '"' => Ok('"'),
                'n' => Ok('\n'),
                'r' => Ok('\r'),
                't' => Ok('\t'),
                _ => Err(Error::UnknownEscape),
            },
            None => return Err(Error::IncompleteEscape),
        }
    }

    fn string(&mut self) -> Result<Token> {
        let mut s = String::new();
        loop {
            match self.chars.next() {
                Some(c) => match c {
                    '\\' => s.push(try!(self.escape())),
                    '"' => return Ok(Token::String(s)),
                    _ => s.push(c),
                },
                None => return Err(Error::UnclosedString),
            }
        }
    }

    fn symbol(&mut self) -> Result<String> {
        let mut s = String::new();
        loop {
            match self.chars.next() {
                Some(c) => if is_whitespace_or_close_brace(c) {
                    self.chars.replace(c);
                    return Ok(s);
                } else {
                    s.push(c);
                },
                None => return Ok(s),
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Result<Token>> {
        loop {
            let c = match self.chars.next() {
                Some(c) => c,
                None => return None,
            };
            let result = if c.is_whitespace() {
                self.whitespace()
            } else if c.is_digit(DECIMAL) {
                self.chars.replace(c);
                self.number()
            } else if c == '#' {
                self.comment()
            } else if c == '(' {
                self.multi_comment()
            } else if c == '"' {
                self.string()
            } else if c == '{' {
                Ok(Token::OpenBrace)
            } else if c == '}' {
                Ok(Token::CloseBrace)
            } else if c == ':' {
                self.symbol().map(|s| Token::Symbol(s))
            } else {
                self.chars.replace(c);
                self.symbol().map(|s| Token::Call(s))
            };
            // If we have errored, the rest of the source is invalid.
            if result.is_err() {
                self.consume();
            }
            return Some(result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token, Error};

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
    fn test_number() {
        assert_eq!(Lexer::new("0").collect::<Vec<_>>(),
            vec![Ok(Token::Integer("0".to_string()))]);
        assert_eq!(Lexer::new("1.0").collect::<Vec<_>>(),
            vec![Ok(Token::Float("1.0".to_string()))]);
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
