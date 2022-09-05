use vec_reg_macro::vec_reg;

fn main() {
    let is_odd = |x: &i32| x % 2 == 1;
    vec_reg!([is_odd][is_odd]);
}
