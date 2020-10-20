# fast-fibonacci [![Crate](https://img.shields.io/crates/v/fast-fibonacci.svg)](https://crates.io/crates/fast-fibonacci) [![Build Status](https://travis-ci.org/danmedani/fast-fibonacci.svg?branch=main)](https://travis-ci.org/danmedani/fast-fibonacci)
Quickly find nth fibonacci number, with modulo.


```Rust
fn fib_with_mod(n: u64, modulo: u64) -> u64
```
> Uses linear recurrence to find nth fibonacci number with modulo.
> O(log(n))


```Rust
fn bigfib_with_mod(n: &BigUint, modulo: &BigUint) -> BigUint
```
> BigUint version of fib_with_mod. Uses linear recurrence to find nth fibonacci number with modulo.
> O(log(n))
