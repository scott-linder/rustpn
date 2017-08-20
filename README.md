RustPN
======

[![Build Status](https://travis-ci.org/scott-linder/rustpn.svg?branch=master)](https://travis-ci.org/scott-linder/rustpn)

RustPN is a stack-based scripting language designed to extend Rust programs
with runtime programmability.

Example Program
---------------

Here is an example of an iterative Fibonacci function written in RustPN:

```
:fib ( n -- n' ) {
    0 1 rot
    {
        over
        +
        swap
    } times
    pop
} fn

1000 fib
```

Overview
--------

All execution in RustPN shares one common data stack. It contains typed data,
referred to as "items". Item types include integers (of variable precision,
decided at the creation of the interpreter), 64-bit IEEE floating point
numbers, utf-8 encoded strings, booleans, symbols (think of ruby atoms, but
prefixed with ':'), and blocks. A block is an anonymous function body, which
can contain item literals and calls to functions.

Whenever an item is evaluated that is not a function call, the value
is pushed onto the data stack. Whenever a function call is encountered,
control is immediately transferred to either the native code implementing
that function, or the RustPN block defining it.

Comments are delimited by parenthesis and do not nest.

Building
--------

To build RustPN, you need a stable Rust 1.x compiler and a recent version of
Cargo. Both can be installed and updated via [rustup](https://www.rustup.rs/).

Once the above tools are available, RustPN can be built in release mode via:

```
cargo build --release
```

The resulting binary will be in `target/release/rustpn`, and can be copied to
somewhere in your `PATH`.

Usage
-----

RustPN expects either no arguments, in which case it will launch as a REPL, or
it expects exactly one argument, which should be a RustPN source filename.

The following will launch an interactive REPL:
```
rustpn
```

The following will execute the code contained in the file `fib.rpn`:
```
rustpn fib.rpn
```

Future Plans
------------

Desired features include:
* Improved REPL (prompt, error recovery, etc.)
* Improved error messages (for missing file, etc.)
* Larger standard library
* Improved documentation
* Larger test suite 
