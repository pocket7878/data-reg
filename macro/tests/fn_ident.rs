use data_reg_common::Regex;
use data_reg_macro::data_reg;

fn main() {
    let is_even = |x: &i32| x % 2 == 0;
    data_reg!({ is_even });
}
