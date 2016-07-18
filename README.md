# RustPN #

RustPN is a stack-based scripting language designed to extend Rust programs
with runtime programmability.

## Example ##

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
