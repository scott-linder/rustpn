//! Language items and abstract-syntax tree.

use std::fmt;

/// The equivalent of a routine/function.
pub type Block<I> = Vec<BlockItem<I>>;

/// Language items only valid in a block.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BlockItem<I> {
    Call(String),
    Literal(StackItem<I>),
}

impl<I> fmt::Display for BlockItem<I> where I: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlockItem::Call(ref s) => write!(f, "{}", *s),
            BlockItem::Literal(ref s) => write!(f, "{}", *s),
        }
    }
}

/// The global stack.
pub type Stack<I> = Vec<StackItem<I>>;

/// Language items only valid on the stack.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum StackItem<I> {
    Integer(I),
    String(String),
    Boolean(bool),
    Symbol(String),
    Block(Block<I>),
}

impl<I> fmt::Display for StackItem<I> where I: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StackItem::Integer(ref i) => write!(f, "{}", *i),
            StackItem::String(ref s) => write!(f, "\"{}\"", *s),
            StackItem::Boolean(b) => write!(f, "{}", b),
            StackItem::Symbol(ref s) => write!(f, ":{}", *s),
            StackItem::Block(ref b) => {
                try!(write!(f, "{{ "));
                for item in b {
                    try!(write!(f, "{}", item))
                }
                try!(write!(f, " }}"));
                Ok(())
            },
        }
    }
}
