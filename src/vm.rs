//! Virtual machine.

use std::rc::Rc;
use std::{error, result};
use std::fmt;
use std::collections::HashMap;
use std::error::Error as StdError;
use item::{Block, BlockItem, Stack};

pub type Result<T> = result::Result<T, Error>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    TypeError,
    OutOfBounds,
    IntegerOverflow,
    NumericConversion,
    DivideByZero,
    StackUnderflow,
    UnknownMethod(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnknownMethod(ref s) => write!(f, "{}: {}", self.description(), s),
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DivideByZero => "Divided by zero",
            Error::OutOfBounds => "Operation out of bounds",
            Error::IntegerOverflow => "Integer overflow or underflow",
            Error::NumericConversion => "Unable to interconvert numeric types",
            Error::TypeError => "Type error",
            Error::StackUnderflow => "Stack underflow",
            Error::UnknownMethod(_) => "Unknown method",
        }
    }
}

pub enum Method<I> {
    Builtin(Box<Fn(&mut Vm<I>) -> Result<()>>),
    Block(Block<I>),
}

pub struct Vm<I> {
    pub stack: Stack<I>,
    pub methods: HashMap<String, Rc<Method<I>>>,
}


impl<I> Vm<I> where I: Clone {
    pub fn new() -> Vm<I> {
        Vm {
            stack: Stack(Vec::new()),
            methods: HashMap::new(),
        }
    }

    pub fn run(&mut self, item: &BlockItem<I>) -> Result<()> {
        match *item {
            BlockItem::Literal(ref stack_item) =>
                self.stack.push(stack_item.clone()),
            BlockItem::Call(ref name) => {
                let method = match self.methods.get(&*name) {
                    Some(m) => m.clone(),
                    None => return Err(Error::UnknownMethod(name.clone())),
                };
                try!(match *method {
                    Method::Builtin(ref f) => (**f)(self),
                    Method::Block(ref b) => self.run_block(b),
                })
            },
        }
        Ok(())
    }

    pub fn run_block(&mut self, block: &Block<I>) -> Result<()> {
        for item in block.0.iter() {
            try!(self.run(item));
        }
        Ok(())
    }

    #[inline]
    pub fn insert_builtin<S>(&mut self, name: S, method: Box<Fn(&mut Vm<I>)
                   -> Result<()>>) where S: Into<String> {
        self.methods.insert(name.into(), Rc::new(Method::Builtin(method)));
    }
}
