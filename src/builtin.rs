use std::rc::Rc;
use item::StackItem;
use vm::{Vm, Error, Method};
use num::{zero, one, Integer};

pub fn insert<I>(vm: &mut Vm<I>) where I: Integer + Clone {
    vm.insert_builtin("+", Box::new(|vm| {
        let n2 = try!(vm.pop_integer());
        let n1 = try!(vm.pop_integer());
        vm.stack.push(StackItem::Integer(n1 + n2));
        Ok(())
    }));
    vm.insert_builtin("-", Box::new(|vm| {
        let n2 = try!(vm.pop_integer());
        let n1 = try!(vm.pop_integer());
        vm.stack.push(StackItem::Integer(n1 - n2));
        Ok(())
    }));
    vm.insert_builtin("*", Box::new(|vm| {
        let n2 = try!(vm.pop_integer());
        let n1 = try!(vm.pop_integer());
        vm.stack.push(StackItem::Integer(n1 * n2));
        Ok(())
    }));
    vm.insert_builtin("/", Box::new(|vm| {
        let n2 = try!(vm.pop_integer());
        let n1 = try!(vm.pop_integer());
        if n2 == zero() {
            return Err(Error::DivideByZero);
        }
        vm.stack.push(StackItem::Integer(n1 / n2));
        Ok(())
    }));
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
    vm.insert_builtin("swap", Box::new(|vm| {
        let b = try!(vm.stack.pop());
        let a = try!(vm.stack.pop());
        vm.stack.push(b);
        vm.stack.push(a);
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
    vm.insert_builtin("dup", Box::new(|vm| {
        let a = try!(vm.stack.pop());
        vm.stack.push(a.clone());
        vm.stack.push(a);
        Ok(())
    }));
    vm.insert_builtin("pop", Box::new(|vm| {
        let _ = try!(vm.stack.pop());
        Ok(())
    }));
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
