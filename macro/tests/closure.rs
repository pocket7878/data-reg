use vec_reg_common::Regex;
use vec_reg_macro::vec_reg;

fn main() {
    vec_reg!([|x: &i32| x % 2 == 0]);
}
