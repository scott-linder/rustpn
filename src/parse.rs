//! Parse lexer tokens into an abstract-syntax tree.

use std::result::Result::{Ok, Err};
use std::{error, result};
use item::{Block, Item};
use token::Token;
use lex;

/// Result of a parser operation.
pub type Result = result::Result<Block, Error>;

/// Possible error due to parser operation.
#[derive(PartialEq, Eq, Clone, Show)]
pub enum Error {
    UnclosedBlock,
    LexError(lex::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnclosedBlock => "Unclosed block",
            Error::LexError(..) => "Lexer error",
        }
    }
}

impl error::FromError<lex::Error> for Error {
    fn from_error(err: lex::Error) -> Error {
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
fn parse_block(lexer: &mut lex::Lexer, block_level: BlockLevel) -> Result {
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
            Token::Integer(s) => block.push(Item::Integer(s.parse()
                    .expect("integer should have been rejected by lexer"))),
            Token::String(s) => block.push(Item::String(s)),
            Token::Call(s) => block.push(Item::Call(s)),
            Token::OpenBrace => 
                block.push(Item::Block(try!(parse_block(lexer,
                                                        BlockLevel::Nested)))),
            Token::CloseBrace => break,
            Token::Whitespace | Token::Comment => (),
        }
    }
    Ok(block)
}

/// Attempt to parse a source string.
pub fn parse(src: &str) -> Result {
    let mut lexer = lex::Lexer::new(src);
    parse_block(&mut lexer, BlockLevel::Top)
}

#[cfg(test)]
mod tests {
    use super::{Error, parse};
    use item::Item;

    #[test]
    fn test_all_simple() {
        assert_eq!(parse(r#"(comment) {} "string" 1 call"#),
            Ok(vec![Item::Block(vec![]),
                    Item::String("string".to_string()),
                    Item::Integer(1),
                    Item::Call("call".to_string())]));
    }
}
