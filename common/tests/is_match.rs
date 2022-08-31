use vec_reg_common::{CompiledRegex, Regex};

#[test]
fn match_is() {
    let reg = Regex::is(1).compile();
    assert!(reg.is_match(&[1]));
    assert!(reg.is_match(&[0, 1, 0]));
}

#[test]
fn match_or() {
    let reg = Regex::or(Regex::is(1), Regex::is(2)).compile();
    assert!(reg.is_match(&[1]));
    assert!(reg.is_match(&[0, 1, 0]));
    assert!(reg.is_match(&[2]));
    assert!(reg.is_match(&[0, 2, 0]));
}

#[test]
fn match_concat() {
    let reg = Regex::concat(Regex::is(1), Regex::is(2)).compile();
    assert!(reg.is_match(&[1, 2]));
    assert!(reg.is_match(&[0, 1, 2, 0]));
}

#[test]
fn test_group() {
    let reg = Regex::group(Regex::concat(Regex::is(1), Regex::is(2))).compile();
    assert!(reg.is_match(&[1, 2]));
    assert!(reg.is_match(&[0, 1, 2, 0]));
}

#[test]
fn match_seq() {
    let reg = Regex::seq(&[1, 2]).compile();
    assert!(reg.is_match(&[1, 2]));
    assert!(reg.is_match(&[0, 1, 2, 0]));
}

#[test]
fn match_repeat0() {
    let reg = Regex::repeat0(Regex::is(1), true).compile();
    assert!(reg.is_match(&[]));
    assert!(reg.is_match(&[0, 0]));
    assert!(reg.is_match(&[1]));
    assert!(reg.is_match(&[0, 1, 0]));
    assert!(reg.is_match(&[1, 1]));
    assert!(reg.is_match(&[0, 1, 1, 0]));
}

#[test]
fn match_repeat1() {
    let reg = Regex::repeat1(Regex::is(1), true).compile();
    assert!(!reg.is_match(&[]));
    assert!(!reg.is_match(&[0, 0]));
    assert!(reg.is_match(&[1]));
    assert!(reg.is_match(&[0, 1, 0]));
    assert!(reg.is_match(&[1, 1]));
    assert!(reg.is_match(&[0, 1, 1, 0]));
}

#[test]
fn match_repeat_n() {
    let reg = Regex::repeat_n(Regex::is(1), 2).compile();
    assert!(!reg.is_match(&[]));
    assert!(!reg.is_match(&[0, 0]));
    assert!(!reg.is_match(&[1]));
    assert!(!reg.is_match(&[0, 1, 0]));
    assert!(reg.is_match(&[1, 1]));
    assert!(reg.is_match(&[0, 1, 1, 0]));
}

#[test]
fn match_repeat_n_or_more() {
    let reg = Regex::repeat_n_or_more(Regex::is(1), 3, true).compile();
    assert!(!reg.is_match(&[1, 1]));
    assert!(reg.is_match(&[1, 1, 1]));
    assert!(reg.is_match(&[1, 1, 1, 1]));
    assert!(reg.is_match(&[1, 1, 1, 1, 1]));
    assert!(reg.is_match(&[1, 1, 1, 1, 1, 1]));

    // Partial match
    assert!(!reg.is_match(&[0, 1, 1, 0]));
    assert!(reg.is_match(&[0, 1, 1, 1, 0]));
    assert!(reg.is_match(&[0, 1, 1, 1, 1, 0]));
    assert!(reg.is_match(&[0, 1, 1, 1, 1, 1, 0]));
    assert!(reg.is_match(&[0, 1, 1, 1, 1, 1, 1, 0]));
}

#[test]
fn match_repeat_min_max() {
    let reg = Regex::repeat_min_max(Regex::is(1), 3, 5, true).compile();
    assert!(!reg.is_match(&[1, 1]));
    assert!(reg.is_match(&[1, 1, 1]));
    assert!(reg.is_match(&[1, 1, 1, 1]));
    assert!(reg.is_match(&[1, 1, 1, 1, 1]));

    // Partial match
    assert!(!reg.is_match(&[0, 1, 1, 0]));
    assert!(reg.is_match(&[0, 1, 1, 1, 0]));
    assert!(reg.is_match(&[0, 1, 1, 1, 1, 0]));
    assert!(reg.is_match(&[0, 1, 1, 1, 1, 1, 0]));
    assert!(reg.is_match(&[0, 1, 1, 1, 1, 1, 1, 0]));
}

#[test]
fn match_repeat_zero_to_n_times() {
    let reg = Regex::repeat_min_max(Regex::is(1), 0, 2, true).compile();
    assert!(reg.is_match(&[]));
    assert!(reg.is_match(&[1]));
    assert!(reg.is_match(&[1, 1]));

    // Partial match
    assert!(reg.is_match(&[0, 0]));
    assert!(reg.is_match(&[0, 1, 0]));
    assert!(reg.is_match(&[0, 1, 1, 0]));
}

#[test]
fn match_zero_or_one() {
    let reg = Regex::zero_or_one(Regex::is(1), true).compile();
    assert!(reg.is_match(&[]));
    assert!(reg.is_match(&[1]));

    // Partial match
    assert!(reg.is_match(&[0, 0]));
    assert!(reg.is_match(&[0, 1, 0]));
}

#[test]
fn match_statisfy() {
    let reg = Regex::satisfy(|v| v % 2 == 0).compile();
    assert!(reg.is_match(&[0]));
    assert!(!reg.is_match(&[1]));
    assert!(reg.is_match(&[2]));
    assert!(!reg.is_match(&[3]));

    // Partial match
    assert!(reg.is_match(&[1, 0, 1]));
    assert!(!reg.is_match(&[1, 1, 1]));
    assert!(reg.is_match(&[1, 2, 1]));
    assert!(!reg.is_match(&[1, 3, 1]));
}

#[test]
fn match_not_statisfy() {
    let reg = Regex::not_satisfy(|v| v % 2 == 0).compile();
    assert!(!reg.is_match(&[0, 0, 0]));
    assert!(reg.is_match(&[0, 1, 0]));
    assert!(!reg.is_match(&[0, 2, 0]));
    assert!(reg.is_match(&[0, 3, 0]));
}

#[test]
fn match_any() {
    let reg = Regex::any().compile();
    assert!(reg.is_match(&[1]));
    assert!(reg.is_match(&[42]));

    assert!(reg.is_match(&[0, 1, 0]));
    assert!(reg.is_match(&[0, 42, 0]));
}

#[test]
fn match_complex() {
    let is_fizz = |x: &i32| x % 3 == 0;
    let is_buzz = |x: &i32| x % 5 == 0;
    let is_fizz_buzz = |x: &i32| x % 15 == 0;
    let reg = Regex::concat(
        Regex::satisfy(is_fizz),
        Regex::repeat1(
            Regex::concat(Regex::satisfy(is_buzz), Regex::satisfy(is_fizz_buzz)),
            true,
        ),
    )
    .compile();
    assert!(!reg.is_match(&[1, 2, 3]));
    assert!(reg.is_match(&[3, 5, 15]));
    assert!(reg.is_match(&[6, 10, 15, 10, 30]));

    assert!(!reg.is_match(&[0, 1, 2, 3, 0]));
    assert!(reg.is_match(&[0, 3, 5, 15, 0]));
    assert!(reg.is_match(&[0, 6, 10, 15, 10, 30, 0]));
}
