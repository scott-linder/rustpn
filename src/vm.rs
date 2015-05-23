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

    pub fn builtin<S>(&mut self, name: S, method: Box<Fn(&mut Vm)
                   -> Result<()>>) where S: Into<String> {
        self.methods.insert(name.into(), Rc::new(Method::Builtin(method)));
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
        vm.builtin("+", Box::new(|vm| {
            let n2 = try!(vm.pop_integer());
            let n1 = try!(vm.pop_integer());
            vm.stack.push(StackItem::Integer(n1 + n2));
            Ok(())
        }));
        vm.builtin("-", Box::new(|vm| {
            let n2 = try!(vm.pop_integer());
            let n1 = try!(vm.pop_integer());
            vm.stack.push(StackItem::Integer(n1 - n2));
            Ok(())
        }));
        vm.builtin("*", Box::new(|vm| {
            let n2 = try!(vm.pop_integer());
            let n1 = try!(vm.pop_integer());
            vm.stack.push(StackItem::Integer(n1 * n2));
            Ok(())
        }));
        vm.builtin("/", Box::new(|vm| {
            let n2 = try!(vm.pop_integer());
            let n1 = try!(vm.pop_integer());
            match n2 {
                0 => return Err(Error::DivideByZero),
                _ => vm.stack.push(StackItem::Integer(n1 / n2)),
            }
            Ok(())
        }));
        vm.builtin("fn", Box::new(|vm| {
            let block = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            let name = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            match (name, block) {
                (StackItem::String(s), StackItem::Block(b)) =>
                    { vm.methods.insert(s, Rc::new(Method::Block(b))); },
                _ => return Err(Error::TypeError),
            }
            Ok(())
        }));
        vm.builtin("swap", Box::new(|vm| {
            let a = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            let b = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            vm.stack.push(a);
            vm.stack.push(b);
            Ok(())
        }));
        vm.builtin("dup", Box::new(|vm| {
            let a = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            vm.stack.push(a.clone());
            vm.stack.push(a);
            Ok(())
        }));
        vm.builtin("pop", Box::new(|vm| {
            let _ = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            Ok(())
        }));
        vm.builtin("false", Box::new(|vm| {
            vm.stack.push(StackItem::Boolean(false));
            Ok(())
        }));
        vm.builtin("true", Box::new(|vm| {
            vm.stack.push(StackItem::Boolean(true));
            Ok(())
        }));
        vm.builtin("eq", Box::new(|vm| {
            let a = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            let b = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            vm.stack.push(StackItem::Boolean(a == b));
            Ok(())
        }));
        vm.builtin("not", Box::new(|vm| {
            let a = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            if let StackItem::Boolean(boolean) = a {
                vm.stack.push(StackItem::Boolean(!boolean));
            } else {
                return Err(Error::TypeError)
            }
            Ok(())
        }));
        vm.builtin("if", Box::new(|vm| {
            let block = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            let condition = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            if let (StackItem::Block(block), StackItem::Boolean(condition)) =
                    (block, condition) {
                if condition {
                    try!(vm.run_block(&block));
                }
            } else {
                return Err(Error::TypeError);
            }
            Ok(())
        }));
        vm.builtin("while", Box::new(|vm| {
            let action_block = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            let condition_block = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
            if let (StackItem::Block(action_block), StackItem::Block(condition_block)) =
                    (action_block, condition_block) {
                loop {
                    try!(vm.run_block(&condition_block));
                    let condition = try!(vm.stack.pop().ok_or(Error::StackUnderflow));
                    if let StackItem::Boolean(condition) = condition {
                        if condition {
                            try!(vm.run_block(&action_block));
                        } else {
                            break;
                        }
                    } else {
                        return Err(Error::TypeError);
                    }
                }
            } else {
                return Err(Error::TypeError);
            }
            Ok(())
        }));
        vm
    }
}
