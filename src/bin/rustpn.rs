extern crate rustpn;
extern crate num;

use rustpn::parse;
use rustpn::vm::Vm;
use std::io::{self, stdin, Read, BufRead};
use std::fs::File;
use std::env;
use std::str::FromStr;
use std::fmt::Display;
use num::bigint::BigInt;
use num::integer::Integer;

fn interactive<I>(vm: &mut Vm<I>) -> io::Result<()>
        where I: Integer + Clone + FromStr + Display {
    let stdin = stdin();
    let mut program = String::new();
    for line in stdin.lock().lines() {
        let line = try!(line);
        program.push_str(&line);
        match parse::parse(&*program) {
            Ok(ref p) => match vm.run_block(p) {
                Ok(()) => println!("{}", vm.stack),
                Err(e) => println!("runtime error: {}", e),
            },
            Err(e) => match e {
                _ if e.is_recoverable() => continue,
                parse::Error::LexError(e) =>
                    println!("lexer error: {}", e),
                _ => println!("parser error: {}", e),
            }
        }
        program.clear();
    }
    Ok(())
}

fn batch<I>(vm: &mut Vm<I>, filename: &str) -> io::Result<()>
        where I: Integer + Clone + FromStr + Display {
    let mut file = try!(File::open(filename));
    let mut program = String::new();
    try!(file.read_to_string(&mut program));
    match parse::parse(&program) {
        Ok(ref p) => match vm.run_block(p) {
            Ok(()) => println!("{}", vm.stack),
            Err(e) => println!("runtime error: {}", e),
        },
        Err(e) => match e {
            parse::Error::LexError(e) => println!("lexer error: {}", e),
            _ => println!("parser error: {}", e),
        }
    }
    Ok(())
}

fn main() {
    let mut vm = Vm::<BigInt>::new_with_builtins();
    let args = env::args();
    if let Some(filename) = args.skip(1).next() {
        batch(&mut vm, &filename).unwrap();
    } else {
        interactive(&mut vm).unwrap();
    }
}
