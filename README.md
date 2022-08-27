# data-reg

Generalized regex like pattern match for vector.

## Install

```toml
# Cargo.toml
[dependencies]
data-reg = "0.1.0"
```

## Usage

```rust
use data_reg::{Regex, data_reg};

fn build_without_macro() {
  let is_fizz = |x: &i32| x % 3 == 0;
  let is_buzz = |x: &i32| x % 5 == 0;
  let is_fizz_buzz = |x: &i32| x % 15 == 0;
  let mut reg = Regex::concat(
      Regex::satisfy(is_fizz),
      Regex::repeat1(Regex::concat(Regex::satisfy(is_buzz), Regex::satisfy(is_fizz_buzz))),
  )
  .compile();
  assert!(!reg.is_match(&vec![1, 2, 3]));
  assert!(reg.is_match(&vec![3, 5, 15]));
  assert!(reg.is_match(&vec![6, 10, 15, 10, 30]));
}

fn build_with_macro() {
  let is_fizz = |x: &i32| x % 3 == 0;
  let is_buzz = |x: &i32| x % 5 == 0;
  let mut reg = data_reg!({is_fizz}({is_buzz}{|x| x % 15 == 0})+).compile();    
  assert!(!reg.is_match(&vec![1, 2, 3]));
  assert!(reg.is_match(&vec![3, 5, 15]));
  assert!(reg.is_match(&vec![6, 10, 15, 10, 30]));
}
```
