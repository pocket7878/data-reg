use data_reg_macro::data_reg;
use data_reg_common::Regex;

fn main() {
  let is_even = |x: &i32| x % 2 == 0;
  data_reg!({is_even});
}