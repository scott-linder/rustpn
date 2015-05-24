extern crate rustpn;
extern crate num;

use rustpn::parse;
use rustpn::vm::Vm;
use std::io::{stdin, BufRead};
use num::bigint::BigInt;

fn main() {
    let mut vm = Vm::<BigInt>::new_with_builtins();
    let stdin = stdin();
    for program in stdin.lock().lines() {
        let program = program.unwrap();
        match parse::parse(&*program) {
            Ok(ref p) => match vm.run_block(p) {
                Ok(()) => println!("{}", vm),
                Err(e) => println!("runtime error: {}", e),
            },
            Err(e) => match e {
                parse::Error::LexError(e) =>
                    println!("lexer error: {}", e),
                _ => println!("parser error: {}", e),
            }
        }
    }
}
