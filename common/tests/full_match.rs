use vec_reg_common::{CompiledRegex, Regex};

#[test]
fn match_is() {
    let reg = Regex::is(1).compile();
    assert!(reg.is_full_match(&[1]));
}

#[test]
fn match_or() {
    let reg = Regex::or(Regex::is(1), Regex::is(2)).compile();
    assert!(reg.is_full_match(&[1]));
    assert!(reg.is_full_match(&[2]));
}

#[test]
fn match_concat() {
    let reg = Regex::concat(Regex::is(1), Regex::is(2)).compile();
    assert!(reg.is_full_match(&[1, 2]));
}

#[test]
fn test_group() {
    let reg = Regex::group(Regex::concat(Regex::is(1), Regex::is(2))).compile();
    assert!(reg.is_full_match(&[1, 2]));
}

#[test]
fn match_seq() {
    let reg = Regex::seq(&[1, 2]).compile();
    assert!(reg.is_full_match(&[1, 2]));
}

#[test]
fn match_repeat0() {
    let reg = Regex::repeat0(Regex::is(1)).compile();
    assert!(reg.is_full_match(&[]));
    assert!(reg.is_full_match(&[1]));
    assert!(reg.is_full_match(&[1, 1]));
}

#[test]
fn match_repeat1() {
    let reg = Regex::repeat1(Regex::is(1)).compile();
    assert!(!reg.is_full_match(&[]));
    assert!(reg.is_full_match(&[1]));
    assert!(reg.is_full_match(&[1, 1]));
    assert!(!reg.is_full_match(&[1, 2]));
}

#[test]
fn match_repeat_n() {
    let reg = Regex::repeat_n(Regex::is(1), 2).compile();
    assert!(!reg.is_full_match(&[]));
    assert!(!reg.is_full_match(&[1]));
    assert!(reg.is_full_match(&[1, 1]));
    assert!(!reg.is_full_match(&[1, 1, 1]));
}

#[test]
fn match_repeat_n_or_more() {
    let reg = Regex::repeat_n_or_more(Regex::is(1), 3).compile();
    assert!(!reg.is_full_match(&[1, 1]));
    assert!(reg.is_full_match(&[1, 1, 1]));
    assert!(reg.is_full_match(&[1, 1, 1, 1]));
    assert!(reg.is_full_match(&[1, 1, 1, 1, 1]));
    assert!(reg.is_full_match(&[1, 1, 1, 1, 1, 1]));
}

#[test]
fn match_repeat_min_max() {
    let reg = Regex::repeat_min_max(Regex::is(1), 3, 5).compile();
    assert!(!reg.is_full_match(&[1, 1]));
    assert!(reg.is_full_match(&[1, 1, 1]));
    assert!(reg.is_full_match(&[1, 1, 1, 1]));
    assert!(reg.is_full_match(&[1, 1, 1, 1, 1]));
    assert!(!reg.is_full_match(&[1, 1, 1, 1, 1, 1]));
}

#[test]
fn match_repeat_zero_to_n_times() {
    let reg = Regex::repeat_min_max(Regex::is(1), 0, 2).compile();
    assert!(reg.is_full_match(&[]));
    assert!(reg.is_full_match(&[1]));
    assert!(reg.is_full_match(&[1, 1]));
    assert!(!reg.is_full_match(&[1, 1, 1]));
}

#[test]
fn match_zero_or_one() {
    let reg = Regex::zero_or_one(Regex::is(1)).compile();
    assert!(reg.is_full_match(&[]));
    assert!(reg.is_full_match(&[1]));
    assert!(!reg.is_full_match(&[1, 1]));
}

#[test]
fn match_statisfy() {
    let reg = Regex::satisfy(|v| v % 2 == 0).compile();
    assert!(reg.is_full_match(&[0]));
    assert!(!reg.is_full_match(&[1]));
    assert!(reg.is_full_match(&[2]));
    assert!(!reg.is_full_match(&[3]));
}

#[test]
fn match_not_statisfy() {
    let reg = Regex::not_satisfy(|v| v % 2 == 0).compile();
    assert!(!reg.is_full_match(&[0]));
    assert!(reg.is_full_match(&[1]));
    assert!(!reg.is_full_match(&[2]));
    assert!(reg.is_full_match(&[3]));
}

#[test]
fn match_any() {
    let reg = Regex::any().compile();
    assert!(reg.is_full_match(&[1]));
    assert!(reg.is_full_match(&[42]));
}

#[test]
fn match_complex() {
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
