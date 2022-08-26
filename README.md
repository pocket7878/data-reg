# data-reg

Regex like pattern match for data sequence.

```rust
use data_reg::Regex;

fn main() {
  let is_fizz = |x: &i32| x % 3 == 0;
  let is_buzz = |x: &i32| x % 5 == 0;
  let is_fizz_buzz = |x: &i32| x % 15 == 0;
  let mut reg = Regex::concat(
      Regex::satisfy(is_fizz),
      Regex::some(Regex::concat(Regex::satisfy(is_buzz), Regex::satisfy(is_fizz_buzz))),
  )
  .compile();
  assert!(!reg.is_match(&vec![1, 2, 3]));
  assert!(reg.is_match(&vec![3, 5, 15]));
  assert!(reg.is_match(&vec![6, 10, 15, 10, 30]));
}
```
