use data_reg_macro::data_reg;
use data_reg_common::Regex;

fn main() {
  data_reg!({|x: &i32| x % 2 == 0});
}