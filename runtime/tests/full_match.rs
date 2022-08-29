use vec_reg::{vec_reg, CompiledRegex, Regex};

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
    assert!(!reg.is_full_match(&[1, 2, 3]));
    assert!(reg.is_full_match(&[3, 5, 15]));
    assert!(reg.is_full_match(&[6, 10, 15, 10, 30]));
}

#[test]
fn match_with_macro() {
    let is_fizz = |x: &i32| x % 3 == 0;
    let is_buzz = |x: &i32| x % 5 == 0;
    let reg = vec_reg!([is_fizz]([is_buzz][|x| x % 15 == 0])+).compile();
    assert!(!reg.is_full_match(&[1, 2, 3]));
    assert!(reg.is_full_match(&[3, 5, 15]));
    assert!(reg.is_full_match(&[6, 10, 15, 10, 30]));
}

#[test]
fn match_repeat_n_macro() {
    let is_even = |x: &i32| x % 2 == 0;
    let is_odd = |x: &i32| x % 2 == 1;

    let reg1 = vec_reg!([is_even]{2}).compile();
    let reg2 = vec_reg!([is_even]{2,}).compile();
    let reg3 = vec_reg!([is_even]{2,3}).compile();
    let reg4 = vec_reg!(([is_even]|[is_odd]){2,3}).compile();

    assert!(!reg1.is_full_match(&[2]));
    assert!(reg1.is_full_match(&[2, 4]));
    assert!(!reg1.is_full_match(&[2, 4, 6]));

    assert!(!reg2.is_full_match(&[2]));
    assert!(reg2.is_full_match(&[2, 4]));
    assert!(reg2.is_full_match(&[2, 4, 6]));

    assert!(!reg3.is_full_match(&[2]));
    assert!(reg3.is_full_match(&[2, 4]));
    assert!(reg3.is_full_match(&[2, 4, 6]));
    assert!(!reg3.is_full_match(&[2, 4, 6, 8]));

    assert!(!reg4.is_full_match(&[2]));
    assert!(reg4.is_full_match(&[1, 2]));
    assert!(reg4.is_full_match(&[1, 2, 3]));
    assert!(!reg4.is_full_match(&[1, 2, 3, 4]));
}

#[test]
fn match_inverse_macro() {
    let is_even = |x: &i32| x % 2 == 0;
    let reg1 = vec_reg!([^is_even]).compile();
    let reg2 = vec_reg!([^|x| x % 2 == 0]).compile();
    assert!(reg1.is_full_match(&[1]));
    assert!(reg2.is_full_match(&[1]));
}
