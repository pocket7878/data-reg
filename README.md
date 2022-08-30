# vec-reg

Generalized regex like pattern match for vector.

[![Build status](https://github.com/pocket7878/vec-reg/actions/workflows/check.yaml/badge.svg?branch=main)](https://github.com/pocket7878/vec-reg/actions/workflows/check.yml)
[![Crates.io](https://img.shields.io/crates/v/vec-reg)](https://crates.io/crates/vec-reg)
[![Documentation](https://docs.rs/vec-reg/badge.svg)](https://docs.rs/vec-reg)

## Install

```toml
# Cargo.toml
[dependencies]
vec-reg = "0.6.0"
```

## Usage

Example without macro:

```rust
use vec_reg::{Regex, CompiledRegex};

let is_fizz = |x: &i32| x % 3 == 0;
let is_buzz = |x: &i32| x % 5 == 0;
let is_fizz_buzz = |x: &i32| x % 15 == 0;
let reg = Regex::concat(
    Regex::satisfy(is_fizz),
    Regex::repeat1(Regex::concat(Regex::satisfy(is_buzz), Regex::satisfy(is_fizz_buzz)), true),
)
.compile();
assert!(!reg.is_full_match(&vec![1, 2, 3]));
assert!(reg.is_full_match(&vec![3, 5, 15]));
assert!(reg.is_full_match(&vec![6, 10, 15, 10, 30]));
```

Example with macro:

```rust
use vec_reg::{Regex, CompiledRegex, vec_reg};

let is_fizz = |x: &i32| x % 3 == 0;
let is_buzz = |x: &i32| x % 5 == 0;
let reg = vec_reg!([is_fizz]([is_buzz][|x| x % 15 == 0])+).compile();    
assert!(!reg.is_full_match(&vec![1, 2, 3]));
assert!(reg.is_full_match(&vec![3, 5, 15]));
assert!(reg.is_full_match(&vec![6, 10, 15, 10, 30]));
```

Example capture:

```rust
use vec_reg::{Regex, CompiledRegex, vec_reg};

let is_even = |x: &i32| x % 2 == 0;
let is_odd = |x: &i32| x % 2 == 1;
let reg = vec_reg!(([is_even]+)[is_even]([is_odd]+)).compile();
let captures = reg.captures(&[2, 4, 6, 3, 5, 7]);
assert!(captures.is_some());
assert_eq!(captures.as_ref().unwrap().len(), 2);

let capture_0 = &captures.as_ref().unwrap()[0];
assert_eq!(capture_0.range, 0..2);
assert_eq!(capture_0.values(), &[2, 4]);

let capture_1 = &captures.as_ref().unwrap()[1];
assert_eq!(capture_1.range, 3..6);
assert_eq!(capture_1.values(), &[3, 5, 7]);
```

## Supported Syntax

| Syntax | Description |
|:--|:--|
| `[function_name]` | Match any values that satisfied given function. |
| `[\|x\| *x == 1]` | Match any values that satisfied given closure. |
| `[^function_name]` | Match any values that not satisfied given function. |
| `[^\|x\| *x == 1]` | Match any values that not satisfied given closure. |
| `.` | Match any values. |
| `(R)` | numbered capturing group (submatch) |
| `(?:R)` | non-capturing group |
| `RS` | `R` followed by `S` |
| <code>R&#124;S</code> | `R` or `S` (prefer `R`) |
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
