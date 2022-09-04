use vec_reg::{vec_reg, CompiledRegex, Regex};

fn main() {
    match_on_size(30);
}

fn match_on_size(n: usize) {
    let regex = build_target_vec_reg_with_size(n);
    let target = build_target_vec_reg_array_with_size(n);
    let compiled_regex = regex.compile();
    if compiled_regex.is_full_match(&target) {
        println!("Match!");
    } else {
        println!("No match!");
    }
}

// Build a?^na^n regex in vec_reg
fn build_target_vec_reg_with_size(n: usize) -> Regex<u32> {
    let is_one = |x: &u32| *x == 1;
    let mut regs = vec![];
    for _ in 1..=n {
        regs.push(vec_reg!([is_one]?));
    }
    for _ in 1..=n {
        regs.push(vec_reg!([is_one]));
    }

    regs.into_iter()
        .reduce(Regex::concat)
        .unwrap()
}

fn build_target_vec_reg_array_with_size(n: usize) -> Vec<u32> {
    vec![1; n]
}
