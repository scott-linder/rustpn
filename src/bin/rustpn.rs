extern crate rustpn;

use rustpn::parse;
use rustpn::vm::Vm;
use std::error::Error;
use std::io::{stdin, BufRead};

fn main() {
    let mut vm = Vm::new_with_builtins();
    let stdin = stdin();
    for program in stdin.lock().lines() {
        let program = program.unwrap();
        match parse::parse(&*program) {
            Ok(ref p) => match vm.run_block(p) {
                Ok(()) => println!("stack: {:?}", vm.stack),
                Err(e) => println!("runtime error: {}", e.description()),
            },
            Err(e) => match e {
                parse::Error::LexError(e) =>
                    println!("lexer error: {}", e.description()),
                _ => println!("parser error: {}", e.description()),
            }
        }
    }
}
