# vec-reg

Generalized regex like pattern match for vector.

[![Build status](https://github.com/pocket7878/vec-reg/actions/workflows/check.yaml/badge.svg?branch=main)](https://github.com/pocket7878/vec-reg/actions/workflows/check.yml)
[![Crates.io](https://img.shields.io/crates/v/vec-reg)](https://crates.io/crates/vec-reg)
[![Documentation](https://docs.rs/vec-reg/badge.svg)](https://docs.rs/vec-reg)

## Install

```toml
# Cargo.toml
[dependencies]
vec-reg = "0.7.0"
```

## Usage

```rust
use vec_reg::{Regex, CompiledRegex, vec_reg};

let is_fizz = |x: &i32| x % 3 == 0;
let is_buzz = |x: &i32| x % 5 == 0;
let reg = vec_reg!(([is_fizz])((?:[is_buzz])(?P<"FizzBuzz">[|x| x % 15 == 0]))+).compile();    
assert!(reg.is_match(&[1, 2, 3, 5, 15, 20]));
assert!(reg.is_full_match(&[3, 5, 15, 10, 30]));

let find_result = reg.find(&[1,3,5,15]);
assert!(find_result.is_some());
assert_eq!(find_result.as_ref().unwrap().range(), 1..4);

let captures = reg.captures(&[1, 3, 5, 15, 10, 30, 2]);
assert!(captures.is_some());
// 0th capture always correspond to the entire match.
assert_eq!(captures.as_ref().unwrap().get(0).unwrap().range(), 1..6);

assert_eq!(captures.as_ref().unwrap().get(1).unwrap().range(), 1..2);
assert_eq!(captures.as_ref().unwrap().get(1).unwrap().values(), &[3]);

// Named capture can be accessed both index and name.
assert_eq!(captures.as_ref().unwrap().get(3).unwrap().values(), &[30]);
assert_eq!(captures.as_ref().unwrap().get(3).unwrap().range(), 5..6);

assert_eq!(captures.as_ref().unwrap().name("FizzBuzz").unwrap().values(), &[30]);
assert_eq!(captures.as_ref().unwrap().name("FizzBuzz").unwrap().range(), 5..6);
```

## Supported Syntax

| Syntax | Description |
|:--|:--|
| `[function_name]` | Match any values that satisfied given function. |
| `[\|x\| *x == 1]` | Match any values that satisfied given closure. |
| `[^function_name]` | Match any values that not satisfied given function. |
| `[^\|x\| *x == 1]` | Match any values that not satisfied given closure. |
| `.` | Match any values. |
| `^` | a beginning of input |
| `$` | a end of input |
| `(R)` | numbered capturing group (submatch) |
| `(?:R)` | non-capturing group |
| `(?P<"name">R)` | named & numbered capturing group (submatch) |
| `RS` | `R` followed by `S` |
| `R\|S` | `R` or `S` (prefer `R`) |
| `R?` | zero or one `R`, prefer one |
| `R??` | zero or one `R`, prefer zero |
| `R*` | zero or more `R`, prefer more |
| `R*?` | zero or more `R`, prefer fewer |
| `R+` | one or more `R`, prefer more |
| `R+?` | one or more `R`, prefer fewer |
| `R{n,m}` | `n` or `n` + 1 or ... or `m`, prefere more |
| `R{n,m}?` | `n` or `n` + 1 or ... or `m`, prefere fewer |
| `R{n,}` | `n` or more `R`, prefere more |
| `R{n,}?` | `n` or more `R`, prefere fewer |
| `R{n}` | exactly `n` `R` |
| `R{n}?` | exactly `n` `R` |
