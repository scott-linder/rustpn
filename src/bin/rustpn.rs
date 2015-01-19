extern crate rustpn;

use rustpn::parse::parse;
use rustpn::vm::{self, Vm, Method};
use rustpn::item::{Stack, StackItem};
use std::rc::Rc;
use std::error::Error;
use std::os;

fn pop_integer(stack: &mut Stack) -> vm::Result<i64> {
    match stack.pop() {
        Some(StackItem::Integer(i)) => Ok(i),
        Some(..) => Err(vm::Error::TypeError),
        None => Err(vm::Error::StackUnderflow),
    }
}

fn main() {
    let mut vm = Vm::new();
    vm.methods.insert(String::from_str("+"),
        Rc::new(Method::Builtin(box |vm| {
            let n2 = try!(pop_integer(&mut vm.stack));
            let n1 = try!(pop_integer(&mut vm.stack));
            vm.stack.push(StackItem::Integer(n1 + n2));
            Ok(())
        })));
    vm.methods.insert(String::from_str("-"),
        Rc::new(Method::Builtin(box |vm| {
            let n2 = try!(pop_integer(&mut vm.stack));
            let n1 = try!(pop_integer(&mut vm.stack));
            vm.stack.push(StackItem::Integer(n1 - n2));
            Ok(())
        })));
    vm.methods.insert(String::from_str("*"),
        Rc::new(Method::Builtin(box |vm| {
            let n2 = try!(pop_integer(&mut vm.stack));
            let n1 = try!(pop_integer(&mut vm.stack));
            vm.stack.push(StackItem::Integer(n1 * n2));
            Ok(())
        })));
    vm.methods.insert(String::from_str("/"),
        Rc::new(Method::Builtin(box |vm| {
            let n2 = try!(pop_integer(&mut vm.stack));
            let n1 = try!(pop_integer(&mut vm.stack));
            match n2 {
                0 => return Err(vm::Error::DivideByZero),
                _ => vm.stack.push(StackItem::Integer(n1 / n2)),
            }
            Ok(())
        })));
    vm.methods.insert(String::from_str("fn"),
        Rc::new(Method::Builtin(box |vm| {
            let block = try!(vm.stack.pop().ok_or(vm::Error::StackUnderflow));
            let name = try!(vm.stack.pop().ok_or(vm::Error::StackUnderflow));
            match (name, block) {
                (StackItem::String(s), StackItem::Block(b)) =>
                    { vm.methods.insert(s, Rc::new(Method::Block(b))); },
                _ => return Err(vm::Error::TypeError),
            }
            Ok(())
        })));
    for program in os::args().iter().skip(1) {
        print!("program: {{{}}} => ", program);
        match parse(program.as_slice()) {
            Ok(ref p) => match vm.run_block(p) {
                Ok(()) => println!("stack: {:?}", vm.stack),
                Err(e) => println!("runtime error: {}", e.description()),
            },
            Err(e) => println!("parser error: {}", e.description()),
        }
    }
}
