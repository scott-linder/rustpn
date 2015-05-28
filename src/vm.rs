//! Virtual machine.

use std::rc::Rc;
use std::{error, result};
use std::fmt;
use std::collections::HashMap;
use std::error::Error as StdError;
use num::integer::Integer;
use item::{Block, BlockItem, Stack, StackItem};
use builtin;

pub type Result<T> = result::Result<T, Error>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    TypeError,
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

impl<I> fmt::Display for Vm<I> where I: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for item in &self.stack {
            try!(write!(f, "{} ", item));
        }
        Ok(())
    }
}

impl<I> Vm<I> where I: Integer + Clone {
    pub fn new() -> Vm<I> {
        Vm {
            stack: Vec::new(),
            methods: HashMap::new(),
        }
    }
    pub fn new_with_builtins() -> Vm<I> {
        let mut vm = Vm::<I>::new();
        builtin::insert(&mut vm);
        vm
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
        for item in block.iter() {
            try!(self.run(item));
        }
        Ok(())
    }

    pub fn insert_builtin<S>(&mut self, name: S, method: Box<Fn(&mut Vm<I>)
                   -> Result<()>>) where S: Into<String> {
        self.methods.insert(name.into(), Rc::new(Method::Builtin(method)));
    }

    pub fn pop_integer(&mut self) -> Result<I> {
        match self.stack.pop() {
            Some(StackItem::Integer(i)) => Ok(i),
            Some(..) => Err(Error::TypeError),
            None => Err(Error::StackUnderflow),
        }
    }
}
