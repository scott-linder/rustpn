//! Parse lexer tokens into an abstract-syntax tree.

use std::result::Result::{Ok, Err};
use std::{error, result};
use std::fmt;
use std::convert::From;
use std::str::FromStr;
use item::{Block, BlockItem, StackItem};
use lex::{self, Token};
use std::error::Error as StdError;

/// Result of a parser operation.
pub type Result<I> = result::Result<Block<I>, Error>;

/// Possible error due to parser operation.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    UnclosedBlock,
    LexError(lex::Error),
}

impl Error {
    pub fn is_recoverable(&self) -> bool {
        match *self {
            Error::UnclosedBlock => true,
            Error::LexError(e) => e.is_recoverable(),
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
            Error::UnclosedBlock => "Unclosed block",
            Error::LexError(..) => "Lexer error",
        }
    }
}

impl From<lex::Error> for Error {
    fn from(err: lex::Error) -> Error {
        Error::LexError(err)
    }
}

// To reduce the burden on the programmer we just assume the top level source
// is wrapped in an implicit block. This means we have two "classes" of blocks,
// at least as far as the parser is concerned.
enum BlockLevel {
    Top,
    Nested,
}

// Recursive parsing function; could be called just "parse" but we use that
// for the public helper function which creates a lexer and creates the
// top block.
fn parse_block<I>(lexer: &mut lex::Lexer, block_level: BlockLevel) -> Result<I> 
        where I: FromStr {
    let mut block = Vec::new();
    loop {
        let token = match lexer.next() {
            None => match block_level {
                BlockLevel::Top => break,
                BlockLevel::Nested => return Err(Error::UnclosedBlock),
            },
            Some(t) => try!(t),
        };
        match token {
            Token::Integer(s) => {
                let i = s.parse().ok()
                    .expect("lexer should have rejected integer");
                block.push(BlockItem::Literal(StackItem::Integer(i)));
            },
            Token::String(s) =>
                block.push(BlockItem::Literal(StackItem::String(s))),
            Token::Symbol(s) => block.push(BlockItem::Literal(StackItem::Symbol(s))),
            Token::Call(s) => block.push(BlockItem::Call(s)),
            Token::OpenBrace => {
                let nested_block = try!(parse_block(lexer, BlockLevel::Nested));
                block.push(BlockItem::Literal(StackItem::Block(nested_block)));
            },
            Token::CloseBrace => break,
            Token::Whitespace | Token::Comment => (),
        }
    }
    Ok(block)
}

/// Attempt to parse a source string.
pub fn parse<I>(src: &str) -> Result<I>
        where I: FromStr {
    let mut lexer = lex::Lexer::new(src);
    parse_block(&mut lexer, BlockLevel::Top)
}

#[cfg(test)]
mod tests {
    use super::{Error, parse};
    use item::{BlockItem, StackItem};

    #[test]
    fn test_all_simple() {
        assert_eq!(parse(r#"(comment) {} "string" 1 call"#),
            Ok(vec![BlockItem::Literal(StackItem::Block(vec![])),
                    BlockItem::Literal(StackItem::String("string".to_string())),
                    BlockItem::Literal(StackItem::Integer(1)),
                    BlockItem::Call("call".to_string())]));
    }
}
