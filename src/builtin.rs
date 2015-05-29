//! Common builtins.

use std::rc::Rc;
use std::string::ToString;
use item::StackItem;
use vm::{Vm, Error, Method};
use num::{zero, one, Integer, ToPrimitive, FromPrimitive};

pub fn insert_arithmetic<I>(vm: &mut Vm<I>) where I: Integer + Clone {
    vm.insert_builtin("+", Box::new(|vm| {
        let n2 = try!(vm.stack.pop());
        let n1 = try!(vm.stack.pop());
        match (n2, n1) {
            (StackItem::Integer(n2), StackItem::Integer(n1))
                => vm.stack.push(StackItem::Integer(n1 + n2)),
            (StackItem::Float(n2), StackItem::Float(n1))
                => vm.stack.push(StackItem::Float(n1 + n2)),
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }));
    vm.insert_builtin("-", Box::new(|vm| {
        let n2 = try!(vm.stack.pop());
        let n1 = try!(vm.stack.pop());
        match (n2, n1) {
            (StackItem::Integer(n2), StackItem::Integer(n1))
                => vm.stack.push(StackItem::Integer(n1 - n2)),
            (StackItem::Float(n2), StackItem::Float(n1))
                => vm.stack.push(StackItem::Float(n1 - n2)),
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }));
    vm.insert_builtin("*", Box::new(|vm| {
        let n2 = try!(vm.stack.pop());
        let n1 = try!(vm.stack.pop());
        match (n2, n1) {
            (StackItem::Integer(n2), StackItem::Integer(n1))
                => vm.stack.push(StackItem::Integer(n1 * n2)),
            (StackItem::Float(n2), StackItem::Float(n1))
                => vm.stack.push(StackItem::Float(n1 * n2)),
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }));
    vm.insert_builtin("/", Box::new(|vm| {
        let n2 = try!(vm.stack.pop());
        let n1 = try!(vm.stack.pop());
        match (n2, n1) {
            (StackItem::Integer(n2), StackItem::Integer(n1)) => if n2 == zero() {
                    return Err(Error::DivideByZero);
                } else {
                    vm.stack.push(StackItem::Integer(n1 / n2))
                },
            (StackItem::Float(n2), StackItem::Float(n1))
                => vm.stack.push(StackItem::Float(n1 / n2)),
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }));
}

pub fn insert_conversions<I>(vm: &mut Vm<I>)
        where I: Integer + Clone + FromPrimitive + ToPrimitive + ToString {
    vm.insert_builtin("as-integer", Box::new(|vm| {
        let n = try!(vm.stack.pop());
        vm.stack.push(match n {
            i @ StackItem::Integer(_) => i,
            StackItem::Float(f) => {
                let i = try!(FromPrimitive::from_f64(f).ok_or(Error::NumericConversion));
                StackItem::Integer(i)
            },
            _ => return Err(Error::TypeError),
        });
        Ok(())
    }));
    vm.insert_builtin("as-float", Box::new(|vm| {
        let n = try!(vm.stack.pop());
        vm.stack.push(match n {
            StackItem::Integer(n) => {
                let f = try!(n.to_f64().ok_or(Error::NumericConversion));
                StackItem::Float(f)
            },
            f @ StackItem::Float(_) => f,
            _ => return Err(Error::TypeError),
        });
        Ok(())
    }));
    vm.insert_builtin("to-string", Box::new(|vm| {
        let a = try!(vm.stack.pop());
        vm.stack.push(match a {
            s @ StackItem::String(_) => s,
            StackItem::Integer(i) => StackItem::String(i.to_string()),
            StackItem::Float(f) => StackItem::String(f.to_string()),
            _ => return Err(Error::TypeError),
        });
        Ok(())
    }));
}

pub fn insert_fn<I>(vm: &mut Vm<I>) where I: Integer + Clone {
    vm.insert_builtin("fn", Box::new(|vm| {
        let block = try!(vm.stack.pop());
        let name = try!(vm.stack.pop());
        match (name, block) {
            (StackItem::Symbol(s), StackItem::Block(b)) =>
                { vm.methods.insert(s, Rc::new(Method::Block(b))); },
            _ => return Err(Error::TypeError),
        }
        Ok(())
    }));
}

pub fn insert_stack_ops<I>(vm: &mut Vm<I>)
        where I: Integer + Clone + ToPrimitive + FromPrimitive {
    vm.insert_builtin("swap", Box::new(|vm| {
        let b = try!(vm.stack.pop());
        let a = try!(vm.stack.pop());
        vm.stack.push(b);
        vm.stack.push(a);
        Ok(())
    }));
    vm.insert_builtin("clone", Box::new(|vm| {
        let a = try!(vm.stack.pop());
        vm.stack.push(a.clone());
        vm.stack.push(a);
        Ok(())
    }));
    vm.insert_builtin("clone-nth", Box::new(|vm| {
        if let StackItem::Integer(n) = try!(vm.stack.pop()) {
            if let Some(n) = n.to_usize() {
                if n <= vm.stack.0.len() {
                    let idx = vm.stack.0.len() - n;
                    let nth = vm.stack.0.get(idx).map(|i| i.clone());
                    if let Some(nth) = nth {
                        vm.stack.push(nth);
                    } else {
                        return Err(Error::OutOfBounds);
                    }
                } else {
                    return Err(Error::OutOfBounds);
                }
            } else {
                return Err(Error::IntegerOverflow);
            }
        } else {
            return Err(Error::TypeError);
        }
        Ok(())
    }));
    vm.insert_builtin("clear", Box::new(|vm| {
        vm.stack.0.clear();
        Ok(())
    }));
    vm.insert_builtin("len", Box::new(|vm| {
        let count = try!(FromPrimitive::from_usize(vm.stack.0.len())
                         .ok_or(Error::IntegerOverflow));
        vm.stack.push(StackItem::Integer(count));
        Ok(())
    }));
    vm.insert_builtin("over", Box::new(|vm| {
        let b = try!(vm.stack.pop());
        let a = try!(vm.stack.pop());
        vm.stack.push(a.clone());
        vm.stack.push(b);
        vm.stack.push(a);
        Ok(())
    }));
    vm.insert_builtin("rot", Box::new(|vm| {
        let c = try!(vm.stack.pop());
        let b = try!(vm.stack.pop());
        let a = try!(vm.stack.pop());
        vm.stack.push(b);
        vm.stack.push(c);
        vm.stack.push(a);
        Ok(())
    }));
    vm.insert_builtin("pop", Box::new(|vm| {
        let _ = try!(vm.stack.pop());
        Ok(())
    }));
}

pub fn insert_boolean_ops<I>(vm: &mut Vm<I>) where I: Integer + Clone {
    vm.insert_builtin("false", Box::new(|vm| {
        vm.stack.push(StackItem::Boolean(false));
        Ok(())
    }));
    vm.insert_builtin("true", Box::new(|vm| {
        vm.stack.push(StackItem::Boolean(true));
        Ok(())
    }));
    vm.insert_builtin("eq", Box::new(|vm| {
        let a = try!(vm.stack.pop());
        let b = try!(vm.stack.pop());
        vm.stack.push(StackItem::Boolean(a == b));
        Ok(())
    }));
    vm.insert_builtin("not", Box::new(|vm| {
        let a = try!(vm.stack.pop());
        if let StackItem::Boolean(boolean) = a {
            vm.stack.push(StackItem::Boolean(!boolean));
        } else {
            return Err(Error::TypeError)
        }
        Ok(())
    }));
    vm.insert_builtin("or", Box::new(|vm| {
        let b = try!(vm.stack.pop());
        let a = try!(vm.stack.pop());
        if let (StackItem::Boolean(a), StackItem::Boolean(b)) = (a, b) {
            vm.stack.push(StackItem::Boolean(a || b));
        } else {
            return Err(Error::TypeError);
        }
        Ok(())
    }));
}

pub fn insert_string_ops<I>(vm: &mut Vm<I>) where I: Integer + Clone {
    vm.insert_builtin("cat", Box::new(|vm| {
        let b = try!(vm.stack.pop());
        let a = try!(vm.stack.pop());
        if let (StackItem::String(b), StackItem::String(mut a)) =
                (b, a) {
            a.push_str(&b);
            vm.stack.push(StackItem::String(a));
        }
        Ok(())
    }));
}

pub fn insert_control_flow<I>(vm: &mut Vm<I>) where I: Integer + Clone {
    vm.insert_builtin("if", Box::new(|vm| {
        let block = try!(vm.stack.pop());
        let condition = try!(vm.stack.pop());
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
    vm.insert_builtin("ifelse", Box::new(|vm| {
        let else_block = try!(vm.stack.pop());
        let if_block = try!(vm.stack.pop());
        let condition = try!(vm.stack.pop());
        if let (StackItem::Block(else_block), StackItem::Block(if_block), StackItem::Boolean(condition)) =
                (else_block, if_block, condition) {
            if condition {
                try!(vm.run_block(&if_block));
            } else {
                try!(vm.run_block(&else_block));
            }
        } else {
            return Err(Error::TypeError);
        }
        Ok(())
    }));
    vm.insert_builtin("while", Box::new(|vm| {
        let action_block = try!(vm.stack.pop());
        let condition_block = try!(vm.stack.pop());
        if let (StackItem::Block(action_block), StackItem::Block(condition_block)) =
                (action_block, condition_block) {
            loop {
                try!(vm.run_block(&condition_block));
                let condition = try!(vm.stack.pop());
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
    vm.insert_builtin("times", Box::new(|vm| {
        let block = try!(vm.stack.pop());
        let times = try!(vm.stack.pop());
        if let (StackItem::Block(block), StackItem::Integer(mut times)) =
                (block, times) {
            while times > zero() {
                try!(vm.run_block(&block));
                times = times - one::<I>();
            }
        } else {
            return Err(Error::TypeError);
        }
        Ok(())
    }));
}

pub fn insert_all<I>(vm: &mut Vm<I>)
        where I: Integer + Clone + ToPrimitive + FromPrimitive + ToString {
    insert_arithmetic(vm);
    insert_conversions(vm);
    insert_fn(vm);
    insert_stack_ops(vm);
    insert_boolean_ops(vm);
    insert_string_ops(vm);
    insert_control_flow(vm);
}
