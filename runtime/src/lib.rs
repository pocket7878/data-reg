//!
//! Generalized regex like pattern match for vector.
//!
//!  ```rust
//!  use vec_reg::{Regex, CompiledRegex, vec_reg};
//!
//!  let is_fizz = |x: &i32| x % 3 == 0;
//!  let is_buzz = |x: &i32| x % 5 == 0;
//!  let reg = vec_reg!([is_fizz]([is_buzz][|x| x % 15 == 0])+).compile();    
//!  assert!(!reg.is_full_match(&vec![1, 2, 3]));
//!  assert!(reg.is_full_match(&vec![3, 5, 15]));
//!  assert!(reg.is_full_match(&vec![6, 10, 15, 10, 30]));
//!  ```
//!
//! ## Supported Syntax
//!
//! | Syntax | Description |
//! |:--|:--|
//! | `[function_name]` | Match any values that satisfied given function. |
//! | <code>[&#124;x&#124; *x == 1]</code> | Match any values that satisfied given closure. |
//! | `[^function_name]` | Match any values that not satisfied given function. |
//! | <code>[&#94;&#124;x&#124; *x == 1]</code> | Match any values that not satisfied given closure. |
//! | `.` | Match any values. |
//! | `RS` | `R` followed by `S` |
//! | <code>R&#124;S</code> | `R` or `S` (prefer `R`) |
//! | `R?` | zero or one `R`, prefer one |
//! | `R??` | zero or one `R`, prefer zero |
//! | `R*` | zero or more `R`, prefer more |
//! | `R*?` | zero or more `R`, prefer fewer |
//! | `R+` | one or more `R`, prefer more |
//! | `R+?` | one or more `R`, prefer fewer |
//! | `R{n,m}` | `n` or `n` + 1 or ... or `m`, prefere more |
//! | `R{n,m}?` | `n` or `n` + 1 or ... or `m`, prefere fewer |
//! | `R{n,}` | `n` or more `R`, prefere more |
//! | `R{n,}?` | `n` or more `R`, prefere fewer |
//! | `R{n}` | exactly `n` `R` |
//! | `R{n}?` | exactly `n` `R` |

pub use vec_reg_common::{CompiledRegex, Regex, Capture};
pub use vec_reg_macro::vec_reg;

#[cfg(doctest)]
doc_comment::doctest!("../../README.md");
