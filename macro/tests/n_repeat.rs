use vec_reg_common::Regex;
use vec_reg_macro::vec_reg;

fn main() {
    let is_even = |x: &i32| x % 2 == 0;
    vec_reg!([is_even]{3});
    vec_reg!([is_even]{3, 4});
    vec_reg!([is_even]{3,});
}
