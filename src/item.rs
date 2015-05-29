//! Language items and abstract-syntax tree.

use std::fmt;
use vm;

/// The equivalent of a routine/function.
#[derive(PartialEq, Clone, Debug)]
pub struct Block<I>(pub Vec<BlockItem<I>>);

/// Language items only valid in a block.
#[derive(PartialEq, Clone, Debug)]
pub enum BlockItem<I> {
    Call(String),
    Literal(StackItem<I>),
}

impl<I> fmt::Display for Block<I> where I: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for item in &self.0 {
            try!(write!(f, "{} ", item));
        }
        Ok(())
    }
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
#[derive(PartialEq, Clone, Debug)]
pub struct Stack<I>(pub Vec<StackItem<I>>);

impl<I> Stack<I> {
    pub fn pop(&mut self) -> vm::Result<StackItem<I>> {
        self.0.pop().ok_or(vm::Error::StackUnderflow)
    }

    pub fn push(&mut self, item: StackItem<I>) {
        self.0.push(item);
    }
}

impl<I> fmt::Display for Stack<I> where I: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for item in &self.0 {
            try!(write!(f, "{} ", item));
        }
        Ok(())
    }
}

/// Language items only valid on the stack.
#[derive(PartialEq, Clone, Debug)]
pub enum StackItem<I> {
    Integer(I),
    Float(f64),
    String(String),
    Boolean(bool),
    Symbol(String),
    Block(Block<I>),
}

impl<I> fmt::Display for StackItem<I> where I: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StackItem::Integer(ref i) => write!(f, "{}", *i),
            StackItem::Float(n) => write!(f, "{}", n),
            StackItem::String(ref s) => write!(f, "\"{}\"", *s),
            StackItem::Boolean(b) => write!(f, "{}", b),
            StackItem::Symbol(ref s) => write!(f, ":{}", *s),
            StackItem::Block(ref b) => write!(f, "{{ {}}}", *b),
        }
    }
}
