//! Virtual machine.

use std::rc::Rc;
use std::{error, result};
use std::collections::HashMap;
use item::{Block, BlockItem, Stack};

pub type Result<T> = result::Result<T, Error>;

#[derive(PartialEq, Eq, Copy, Show)]
pub enum Error {
    TypeError,
    DivideByZero,
    StackUnderflow,
    UnknownMethod,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DivideByZero => "Divided by zero",
            Error::TypeError => "Type error",
            Error::StackUnderflow => "Stack underflow",
            Error::UnknownMethod => "Unknown method",
        }
    }
}

pub enum Method {
    Builtin(Box<Fn(&mut Vm) -> Result<()> + 'static>),
    Block(Block),
}

pub struct Vm {
    pub stack: Stack,
    pub methods: HashMap<String, Rc<Method>>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Vec::new(),
            methods: HashMap::new(),
        }
    }

    pub fn run(&mut self, item: &BlockItem) -> Result<()> {
        match *item {
            BlockItem::Literal(ref stack_item) =>
                self.stack.push(stack_item.clone()),
            BlockItem::Call(ref name) => {
                let method = match self.methods.get(name.as_slice()) {
                    Some(m) => m.clone(),
                    None => return Err(Error::UnknownMethod),
                };
                try!(match *method {
                    Method::Builtin(ref f) => (**f)(self),
                    Method::Block(ref b) => self.run_block(b),
                })
            },
        }
        Ok(())
    }

    pub fn run_block(&mut self, block: &Block) -> Result<()> {
        for item in block.iter() {
            try!(self.run(item));
        }
        Ok(())
    }
}
