//!
//! Generalized regex like pattern match for vector.
//!
//!  ```rust
//!  use data_reg::{Regex, CompiledRegex, data_reg};
//!
//!  fn build_without_macro() {
//!      let is_fizz = |x: &i32| x % 3 == 0;
//!      let is_buzz = |x: &i32| x % 5 == 0;
//!      let is_fizz_buzz = |x: &i32| x % 15 == 0;
//!      let reg = Regex::concat(
//!          Regex::satisfy(is_fizz),
//!          Regex::repeat1(Regex::concat(Regex::satisfy(is_buzz), Regex::satisfy(is_fizz_buzz))),
//!      )
//!      .compile();
//!      assert!(!reg.is_match(&vec![1, 2, 3]));
//!      assert!(reg.is_match(&vec![3, 5, 15]));
//!      assert!(reg.is_match(&vec![6, 10, 15, 10, 30]));
//!  }
//!  
//!  fn build_with_macro() {
//!      let is_fizz = |x: &i32| x % 3 == 0;
//!      let is_buzz = |x: &i32| x % 5 == 0;
//!      let reg = data_reg!({is_fizz}({is_buzz}{|x| x % 15 == 0})+).compile();    
//!      assert!(!reg.is_match(&vec![1, 2, 3]));
//!      assert!(reg.is_match(&vec![3, 5, 15]));
//!      assert!(reg.is_match(&vec![6, 10, 15, 10, 30]));
//!  }
//!  ```

pub use data_reg_common::{CompiledRegex, Regex};
pub use data_reg_macro::data_reg;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn match_without_macro() {
        let is_fizz = |x: &i32| x % 3 == 0;
        let is_buzz = |x: &i32| x % 5 == 0;
        let is_fizz_buzz = |x: &i32| x % 15 == 0;
        let reg = Regex::concat(
            Regex::satisfy(is_fizz),
            Regex::repeat1(Regex::concat(
                Regex::satisfy(is_buzz),
                Regex::satisfy(is_fizz_buzz),
            )),
        )
        .compile();
        assert!(!reg.is_match(&[1, 2, 3]));
        assert!(reg.is_match(&[3, 5, 15]));
        assert!(reg.is_match(&[6, 10, 15, 10, 30]));
    }

    #[test]
    fn match_with_macro() {
        let is_fizz = |x: &i32| x % 3 == 0;
        let is_buzz = |x: &i32| x % 5 == 0;
        let reg = data_reg!({is_fizz}({is_buzz}{|x| x % 15 == 0})+).compile();
        assert!(!reg.is_match(&[1, 2, 3]));
        assert!(reg.is_match(&[3, 5, 15]));
        assert!(reg.is_match(&[6, 10, 15, 10, 30]));
    }
}
