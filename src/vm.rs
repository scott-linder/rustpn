//! Virtual machine.

use std::rc::Rc;
use std::{error, result};
use std::fmt;
use std::collections::HashMap;
use std::error::Error as StdError;
use item::{Block, BlockItem, Stack, StackItem};

pub type Result<T> = result::Result<T, Error>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Error {
    TypeError,
    DivideByZero,
    StackUnderflow,
    UnknownMethod,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
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
    Builtin(Box<Fn(&mut Vm) -> Result<()>>),
    Block(Block),
}

pub struct Vm {
    pub stack: Stack,
    pub methods: HashMap<String, Rc<Method>>,
}

impl Vm {
    pub fn run(&mut self, item: &BlockItem) -> Result<()> {
        match *item {
            BlockItem::Literal(ref stack_item) =>
                self.stack.push(stack_item.clone()),
            BlockItem::Call(ref name) => {
                let method = match self.methods.get(&*name) {
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

    pub fn builtin(&mut self, name: String, method: Box<Fn(&mut Vm)
                   -> Result<()>>) {
        self.methods.insert(name, Rc::new(Method::Builtin(method)));
    }

    pub fn pop_integer(&mut self) -> Result<i64> {
        match self.stack.pop() {
            Some(StackItem::Integer(i)) => Ok(i),
            Some(..) => Err(Error::TypeError),
            None => Err(Error::StackUnderflow),
        }
    }

    pub fn new() -> Vm {
        Vm {
            stack: Vec::new(),
            methods: HashMap::new(),
        }
    }

    pub fn new_with_builtins() -> Vm {
        let mut vm = Vm::new();
        vm.builtin("+".into(), Box::new(|vm| {
            let n2 = try!(vm.pop_integer());
            let n1 = try!(vm.pop_integer());
            vm.stack.push(StackItem::Integer(n1 + n2));
            Ok(())
        }));
        vm.builtin("-".into(), Box::new(|vm| {
            let n2 = try!(vm.pop_integer());
            let n1 = try!(vm.pop_integer());
            vm.stack.push(StackItem::Integer(n1 - n2));
            Ok(())
        }));
        vm.builtin("*".into(), Box::new(|vm| {
            let n2 = try!(vm.pop_integer());
            let n1 = try!(vm.pop_integer());
            vm.stack.push(StackItem::Integer(n1 * n2));
            Ok(())
        }));
        vm.builtin("/".into(), Box::new(|vm| {
            let n2 = try!(vm.pop_integer());
            let n1 = try!(vm.pop_integer());
            match n2 {
                0 => return Err(Error::DivideByZero),
                _ => vm.stack.push(StackItem::Integer(n1 / n2)),
            }
            Ok(())
        }));
        vm.builtin("fn".into(), Box::new(|vm| {
            let block = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            let name = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            match (name, block) {
                (StackItem::String(s), StackItem::Block(b)) =>
                    { vm.methods.insert(s, Rc::new(Method::Block(b))); },
                _ => return Err(Error::TypeError),
            }
            Ok(())
        }));
        vm.builtin("false".into(), Box::new(|vm| {
            vm.stack.push(StackItem::Boolean(false));
            Ok(())
        }));
        vm.builtin("true".into(), Box::new(|vm| {
            vm.stack.push(StackItem::Boolean(true));
            Ok(())
        }));
        vm.builtin("==".into(), Box::new(|vm| {
            let a = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            let b = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            vm.stack.push(StackItem::Boolean(a == b));
            Ok(())
        }));
        vm.builtin("if".into(), Box::new(|vm| {
            let block = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            let condition = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            if let (StackItem::Block(block), StackItem::Boolean(condition)) =
                    (block, condition) {
                if condition {
                    try!(vm.run_block(&block))
                }
            } else {
                return Err(Error::TypeError);
            }
            Ok(())
        }));
        vm
    }
}
