use data_reg_common::Regex;
use data_reg_macro::data_reg;

fn main() {
    let is_odd = |x: &i32| x % 2 == 1;
    data_reg!((({is_odd}{|x: &i32| x % 2 == 0})*|{is_odd}?)+);
}
