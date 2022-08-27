use data_reg_common::Regex;
use data_reg_macro::data_reg;

fn main() {
    data_reg!({ |x: &i32| x % 2 == 0 });
}
